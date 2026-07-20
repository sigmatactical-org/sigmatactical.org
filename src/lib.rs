//! sigmatactical.org: open-source showcase with shared theme and GitHub org listing.

#![forbid(unsafe_code)]

mod catalog;
mod config;
mod repos;
mod templates;

use std::convert::Infallible;
use std::sync::{Arc, OnceLock};

use warp::{Filter, Rejection, Reply};

pub use sigma_theme::copyright_years;

/// Identity origin for the CSP `connect-src`, resolved once.
///
/// `sigma_theme::warp::security_headers` returns `impl Filter + 'static`, which
/// (edition 2024 RPIT capture rules) makes its `&str` argument `&'static str`;
/// caching the value here supplies that lifetime.
fn identity_origin() -> &'static str {
    static ORIGIN: OnceLock<String> = OnceLock::new();
    ORIGIN.get_or_init(config::identity_public_origin)
}

/// Render the home page from the cached repository listing. A listing failure
/// with nothing cached degrades to the "temporarily unavailable" copy rather
/// than an error page.
async fn index_page() -> Result<impl Reply, Rejection> {
    let (repos, repos_error) = match repos::list_public_repos().await {
        Ok(repos) => (repos, String::new()),
        Err(err) => {
            tracing::error!(%err, "github repo list failed");
            (Arc::new(Vec::new()), err.to_string())
        }
    };
    templates::render_index_html(&repos, &repos_error)
        .map(warp::reply::html)
        .map_err(|_| warp::reject::custom(sigma_theme::warp::TemplateError))
}

fn index_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get().and(warp::path::end()).and_then(index_page)
}

/// Full site filter: `/`, `/up`, `/health`, `/static/*`, `/favicon.ico`,
/// security headers, plus themed rejection recovery.
pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone + Send + 'static
{
    sigma_theme::warp::security_headers(
        sigma_theme::warp::site_routes(
            index_route(),
            sigma_pg::health::warp::health_routes("sigmatactical-org", None),
        ),
        identity_origin(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_repo() -> repos::RepoView {
        repos::RepoView {
            name: "dbc-rs".to_string(),
            url: "https://github.com/sigmatactical-org/dbc-rs".to_string(),
            description: "DBC library".to_string(),
            language: "Rust".to_string(),
            stars: 12,
            default_branch: "main".to_string(),
            build: None,
        }
    }

    #[test]
    fn rendered_html_contains_ownership_message_and_sections() {
        let html = templates::render_index_html(&[sample_repo()], "").expect("template");
        assert!(html.contains("<title>Open Source — Sigma Tactical Group</title>"));
        assert!(html.contains("Own your vehicle"));
        assert!(html.contains("Why we open-source"));
        assert!(html.contains("Data formats & bench tooling"));
        assert!(html.contains(&format!(
            "&copy; {} Sigma Tactical Group",
            copyright_years()
        )));
    }

    #[test]
    fn rendered_html_lists_repo_cards() {
        let html = templates::render_index_html(&[sample_repo()], "").expect("template");
        assert!(html.contains("dbc-rs"));
        assert!(html.contains("Why it matters:"));
        assert!(html.contains("CAN networks"));
        assert!(html.contains("12 stars"));
    }

    #[test]
    fn listing_failure_renders_the_unavailable_notice() {
        let html = templates::render_index_html(&[], "github is down").expect("template");
        assert!(html.contains("Repository list is temporarily unavailable"));
    }
}

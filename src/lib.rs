//! sigmatactical.org: open-source showcase with shared theme and GitHub org listing.

mod catalog;
mod config;
mod repos;
mod templates;

use std::convert::Infallible;

use warp::Filter;
use warp::{Rejection, Reply};

/// Resolve listen address from **`PORT`** (default **8080**). Binds IPv4 **`0.0.0.0`**.
#[must_use]
pub fn listen_socket_addr_from_env() -> std::net::SocketAddr {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port)
}

pub use sigma_theme::{COPYRIGHT_START_YEAR, copyright_years, current_year};

#[derive(Debug)]
struct TemplateError;
impl warp::reject::Reject for TemplateError {}

async fn index_page() -> Result<impl Reply, Rejection> {
    let (repos, repos_error) = match repos::list_public_repos().await {
        Ok(repos) => (repos, String::new()),
        Err(err) => {
            eprintln!("github repo list failed: {err}");
            (Vec::new(), err.to_string())
        }
    };
    templates::render_index_html(&repos, &repos_error)
        .map(warp::reply::html)
        .map_err(|_| warp::reject::custom(TemplateError))
}

fn index_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get().and(warp::path::end()).and_then(index_page)
}

fn content_security_policy() -> String {
    let identity_origin = config::identity_public_origin();
    format!(
        "default-src 'self'; base-uri 'self'; object-src 'none'; frame-ancestors 'none'; \
         img-src 'self' data:; style-src 'self'; script-src 'self'; font-src 'self'; \
         connect-src 'self' {identity_origin}; form-action 'self'; upgrade-insecure-requests"
    )
}

/// Full site filter: `/`, `/up`, `/static/*`, `/favicon.ico`, security headers, plus rejection recovery.
pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone + Send + 'static
{
    use warp::reply::with::header;

    warp::path("up")
        .and(warp::get())
        .map(|| warp::reply::with_status("up", warp::http::StatusCode::OK))
        .or(sigma_pg::health::warp::health_routes(
            "sigmatactical-org",
            None,
        ))
        .or(index_route())
        .or(sigma_theme::warp::static_files())
        .or(sigma_theme::warp::favicon())
        .recover(sigma_theme::warp::handle_rejection)
        .with(header("content-security-policy", content_security_policy()))
        .with(header("x-content-type-options", "nosniff"))
        .with(header("x-frame-options", "DENY"))
        .with(header("referrer-policy", "strict-origin-when-cross-origin"))
        .with(header("cross-origin-opener-policy", "same-origin"))
        .with(header(
            "permissions-policy",
            "geolocation=(), microphone=(), camera=()",
        ))
        .with(header(
            "strict-transport-security",
            "max-age=63072000; includeSubDomains; preload",
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sigma_theme::templates::render_internal_server_error_html;
    use warp::http::StatusCode;

    #[test]
    fn rendered_html_contains_ownership_message_and_sections() {
        let repos = vec![repos::RepoView {
            name: "dbc-rs".to_string(),
            url: "https://github.com/sigmatactical-org/dbc-rs".to_string(),
            description: "DBC library".to_string(),
            language: "Rust".to_string(),
            stars: 12,
            default_branch: "main".to_string(),
            build: None,
        }];
        let html = templates::render_index_html(&repos, "").expect("template should render");
        assert!(html.contains("<title>Open Source — Sigma Tactical Group</title>"));
        assert!(html.contains("Own your vehicle"));
        assert!(html.contains("Why we open-source"));
        assert!(html.contains("Data formats & bench tooling"));
        assert!(html.contains("Why it matters:"));
        assert!(html.contains("dbc-rs"));
    }

    #[test]
    fn rendered_html_lists_repo_cards() {
        let repos = vec![repos::RepoView {
            name: "dbc-rs".to_string(),
            url: "https://github.com/sigmatactical-org/dbc-rs".to_string(),
            description: "DBC tooling".to_string(),
            language: "Rust".to_string(),
            stars: 12,
            default_branch: "main".to_string(),
            build: None,
        }];
        let html = templates::render_index_html(&repos, "").expect("template");
        assert!(html.contains("dbc-rs"));
        assert!(html.contains("CAN networks"));
        assert!(html.contains("12 stars"));
    }

    #[tokio::test]
    async fn routes_up_returns_200() {
        let res = warp::test::request()
            .method("GET")
            .path("/up")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), 200);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "up");
    }

    #[tokio::test]
    async fn routes_root_returns_200() {
        let res = warp::test::request()
            .method("GET")
            .path("/")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), 200);
    }

    #[tokio::test]
    async fn routes_set_security_headers() {
        let res = warp::test::request()
            .method("GET")
            .path("/")
            .reply(&routes())
            .await;
        let headers = res.headers();
        assert!(headers.contains_key("content-security-policy"));
        assert_eq!(headers.get("x-content-type-options").unwrap(), "nosniff");
        assert_eq!(headers.get("x-frame-options").unwrap(), "DENY");
    }

    #[tokio::test]
    async fn routes_unknown_returns_404_html() {
        let res = warp::test::request()
            .method("GET")
            .path("/missing-page")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), 404);
        let body = std::str::from_utf8(res.body()).expect("utf-8");
        assert!(body.contains("Oops"));
    }

    #[tokio::test]
    async fn routes_wrong_method_returns_405() {
        let res = warp::test::request()
            .method("POST")
            .path("/")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), 405);
        let body = std::str::from_utf8(res.body()).expect("utf-8");
        assert!(body.contains("Method not allowed"));
    }

    #[tokio::test]
    async fn static_assets_get_cache_control() {
        let res = warp::test::request()
            .method("GET")
            .path("/static/vendor/bootstrap-5.3.3/bootstrap.min.css")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), 200);
        assert_eq!(
            res.headers().get("cache-control").unwrap(),
            "public, max-age=31536000, immutable"
        );
    }

    #[test]
    fn internal_error_template_renders_html() {
        let html = render_internal_server_error_html().expect("500 template");
        assert!(html.contains("Something went wrong"));
        assert!(html.contains("Oops"));
        assert!(html.contains("<title>Something went wrong — Sigma Tactical Group</title>"));
    }

    #[tokio::test]
    async fn handle_rejection_renders_500_for_non_not_found() {
        let reply = sigma_theme::warp::handle_rejection(warp::reject::custom(TemplateError))
            .await
            .expect("recovery is infallible");
        let resp = warp::Reply::into_response(reply);
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}

//! Home page rendering: site chrome plus the curated repository sections.

mod index_template;

use std::sync::OnceLock;

use askama::Template;
use sigma_theme::copyright_years;
use sigma_theme::nav::SiteHeader;
use sigma_theme::site_nav::SiteChrome;

use index_template::IndexTemplate;

use crate::config;
use crate::repos::RepoView;

fn chrome() -> SiteChrome {
    SiteChrome {
        title: "Open Source".to_string(),
        identity_base: config::identity_public_base_url(),
        app_base: config::public_base_url(),
        contact_base: config::contact_public_base_url(),
        cart_url: config::cart_public_base_url(),
        show_cart: true,
    }
}

/// Header actions, rendered once and reused for the process lifetime.
/// Only a successful render is cached, so a failure retries next request.
fn site_nav() -> Result<&'static str, askama::Error> {
    static NAV: OnceLock<String> = OnceLock::new();
    if let Some(nav) = NAV.get() {
        return Ok(nav);
    }
    let nav = chrome().site_nav("/", 0)?;
    Ok(NAV.get_or_init(|| nav))
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_index_html(repos: &[RepoView], repos_error: &str) -> Result<String, askama::Error> {
    IndexTemplate {
        site_header: SiteHeader::home(),
        site_nav: site_nav()?,
        github_org: config::github_org(),
        sections: crate::catalog::build_sections(repos),
        repos_error,
        copyright_years: copyright_years(),
    }
    .render()
}

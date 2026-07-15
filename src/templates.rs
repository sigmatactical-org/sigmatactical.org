use askama::Template;

use sigma_theme::copyright_years;
use sigma_theme::nav::SiteHeader;
use sigma_theme::site_nav::{AppSiteNav, render_app_site_nav};

use crate::catalog::RepoSection;
use crate::repos::RepoView;

fn site_nav() -> Result<String, askama::Error> {
    render_app_site_nav(&AppSiteNav {
        identity_base: &crate::config::identity_public_base_url(),
        app_base: &crate::config::public_base_url(),
        contact_base: &crate::config::contact_public_base_url(),
        cart_url: &crate::config::cart_public_base_url(),
        cart_count: 0,
        return_path: "/",
        show_cart: true,
        show_contact_us: false,
        leading_html: "",
    })
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    site_header: SiteHeader,
    site_nav: String,
    github_org: String,
    sections: Vec<RepoSection>,
    repos_error: String,
    copyright_years: String,
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_index_html(repos: &[RepoView], repos_error: &str) -> Result<String, askama::Error> {
    IndexTemplate {
        site_header: SiteHeader::home(),
        site_nav: site_nav()?,
        github_org: crate::config::github_org(),
        sections: crate::catalog::build_sections(repos.to_vec()),
        repos_error: repos_error.to_string(),
        copyright_years: copyright_years(),
    }
    .render()
}

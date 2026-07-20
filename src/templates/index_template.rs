//! [`IndexTemplate`].

use askama::Template;
use sigma_theme::nav::SiteHeader;

use crate::catalog::RepoSection;

/// Home page: the curated open-source repository directory.
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub site_header: SiteHeader,
    pub site_nav: &'static str,
    pub github_org: String,
    pub sections: Vec<RepoSection>,
    pub repos_error: &'a str,
    pub copyright_years: String,
}

//! Environment-driven configuration: public base URLs and GitHub API access.

use sigma_pg::clients::http::env_url;

/// Public base URL of this site (e.g. `http://127.0.0.1:8080/`).
#[must_use]
pub fn public_base_url() -> String {
    env_url("STORG_PUBLIC_BASE_URL", "http://127.0.0.1:8080/")
}

/// Public base URL of the identity BFF (e.g. `http://127.0.0.1:3000/`).
#[must_use]
pub fn identity_public_base_url() -> String {
    env_url("STORG_IDENTITY_PUBLIC_URL", "http://127.0.0.1:3000/")
}

/// Browser origin of the identity BFF for CSP `connect-src` (no trailing slash).
#[must_use]
pub fn identity_public_origin() -> String {
    identity_public_base_url().trim_end_matches('/').to_string()
}

/// Public base URL of the contact service for navbar links.
#[must_use]
pub fn contact_public_base_url() -> String {
    env_url("STORG_CONTACT_PUBLIC_URL", "http://127.0.0.1:8083/")
}

/// Public base URL of the cart service for navbar links.
#[must_use]
pub fn cart_public_base_url() -> String {
    env_url("STORG_CART_PUBLIC_URL", "http://127.0.0.1:8084/")
}

/// GitHub organization whose public repositories are listed on the home page.
#[must_use]
pub fn github_org() -> String {
    std::env::var("STORG_GITHUB_ORG")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "sigmatactical-org".to_string())
}

/// Optional GitHub token for authenticated API calls (raises rate limits).
///
/// Reads `STORG_GITHUB_TOKEN`, falling back to `GITHUB_TOKEN`. Unset means
/// unauthenticated requests (fine for public data, lower rate limit).
#[must_use]
pub fn github_token() -> Option<String> {
    std::env::var("STORG_GITHUB_TOKEN")
        .or_else(|_| std::env::var("GITHUB_TOKEN"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Optional override for the GitHub REST API base (testing only).
#[must_use]
pub fn github_api_base() -> String {
    std::env::var("STORG_GITHUB_API_BASE")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim_end_matches('/').to_string())
        .unwrap_or_else(|| "https://api.github.com".to_string())
}

//! Environment-driven configuration: public base URLs and GitHub API access.
//!
//! Required values are declared in the [`sigma_config::service!`] block and
//! checked by [`validate`] at startup.

sigma_config::service! {
    prefix = "STORG";
    role = "storg";
    urls {
        /// Public base URL of this site.
        public_base_url = "PUBLIC_BASE_URL" => "http://127.0.0.1:8080/";
        /// Public base URL of the identity BFF.
        identity_public_base_url = "IDENTITY_PUBLIC_URL" => "http://127.0.0.1:3000/";
        /// Public base URL of the contact service for navbar links.
        contact_public_base_url = "CONTACT_PUBLIC_URL" => "http://127.0.0.1:8083/";
        /// Public base URL of the cart service for navbar links.
        cart_public_base_url = "CART_PUBLIC_URL" => "http://127.0.0.1:8084/";
    }
}

/// Browser origin of the identity BFF for CSP `connect-src` (no trailing slash).
#[must_use]
pub fn identity_public_origin() -> String {
    sigma_config::origin_of(&identity_public_base_url())
}

/// GitHub organization whose public repositories are listed on the home page.
#[must_use]
pub fn github_org() -> String {
    SERVICE
        .opt_str("GITHUB_ORG")
        .unwrap_or_else(|| "sigmatactical-org".to_string())
}

/// Optional GitHub token for authenticated API calls (raises rate limits).
///
/// Reads `STORG_GITHUB_TOKEN`, falling back to `GITHUB_TOKEN`. Unset means
/// unauthenticated requests (fine for public data, lower rate limit).
#[must_use]
pub fn github_token() -> Option<String> {
    SERVICE
        .opt_str("GITHUB_TOKEN")
        .or_else(|| sigma_config::var("GITHUB_TOKEN"))
}

/// Optional override for the GitHub REST API base (testing only).
#[must_use]
pub fn github_api_base() -> String {
    SERVICE
        .opt_str("GITHUB_API_BASE")
        .map(|s| s.trim_end_matches('/').to_string())
        .unwrap_or_else(|| "https://api.github.com".to_string())
}

#![forbid(unsafe_code)]

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sigmatactical_org::config::validate()?;
    sigma_theme::warp::run_service("sigmatactical.org", sigmatactical_org::routes())?;
    Ok(())
}

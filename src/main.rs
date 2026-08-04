#![forbid(unsafe_code)]

fn main() -> std::io::Result<()> {
    sigma_theme::warp::run_service("sigmatactical.org", sigmatactical_org::routes())
}

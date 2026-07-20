//! Integration tests: site-specific content and assets served over the HTTP surface.
//! Shared scaffold behavior (/up, 404/405, security headers, cache-control) is
//! tested in `sigma-theme`.

use sigmatactical_org::routes;

#[tokio::test]
async fn get_root_is_html() {
    let res = warp::test::request()
        .method("GET")
        .path("/")
        .reply(&routes())
        .await;
    assert_eq!(res.status(), 200);
    let ctype = res
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok());
    assert!(
        ctype.is_some_and(|c| c.starts_with("text/html")),
        "expected text/html, got {ctype:?}"
    );
    let body = std::str::from_utf8(res.body()).expect("utf-8 body");
    assert!(body.contains("sigma-dial-root"));
    assert!(body.contains("Own your vehicle"));
}

#[tokio::test]
async fn static_favicon_png_served() {
    let res = warp::test::request()
        .method("GET")
        .path("/static/sigma-favicon-32.png")
        .reply(&routes())
        .await;
    assert_eq!(res.status(), 200);
    assert!(!res.body().is_empty());
}

#[tokio::test]
async fn favicon_ico_alias_served() {
    let res = warp::test::request()
        .method("GET")
        .path("/favicon.ico")
        .reply(&routes())
        .await;
    assert_eq!(res.status(), 200);
    assert!(!res.body().is_empty());
}

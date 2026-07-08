//! Integration tests: exercise the HTTP surface from the crate root (static files, paths).

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
    assert!(body.contains("Open Source"));
}

#[tokio::test]
async fn get_unknown_path_is_404() {
    let res = warp::test::request()
        .method("GET")
        .path("/no-such-page")
        .reply(&routes())
        .await;
    assert_eq!(res.status(), 404);
    let body = std::str::from_utf8(res.body()).expect("utf-8 body");
    assert!(body.contains("Oops"), "expected friendly 404 copy");
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

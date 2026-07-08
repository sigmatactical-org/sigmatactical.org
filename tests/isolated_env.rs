//! Integration tests that touch process environment use **`temp-env`** scopes and **`serial_test::serial`**
//! so mutations never overlap across threads.

use serial_test::serial;

#[test]
#[serial]
fn listen_port_defaults_to_8080() {
    temp_env::with_vars(vec![("PORT", None::<&str>)], || {
        assert_eq!(
            sigmatactical_org::listen_socket_addr_from_env().port(),
            8080
        );
    });
}

#[test]
#[serial]
fn listen_port_parses_env() {
    temp_env::with_vars(vec![("PORT", Some("9555"))], || {
        assert_eq!(
            sigmatactical_org::listen_socket_addr_from_env().port(),
            9555
        );
    });
}

#[test]
#[serial]
fn listen_invalid_port_falls_back_to_8080() {
    temp_env::with_vars(vec![("PORT", Some("not-a-port"))], || {
        assert_eq!(
            sigmatactical_org::listen_socket_addr_from_env().port(),
            8080
        );
    });
}

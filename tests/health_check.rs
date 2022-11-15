use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let addrs = spawn_app();
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/health_check", &addrs))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

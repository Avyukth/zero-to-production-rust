
#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to start app");
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8080/health_check").send().await.expect("Failed to send request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


async  fn spawn_app() -> std::io::Result<()>{
    zero2production::run().await
}

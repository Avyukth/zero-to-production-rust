async  fn spawn_app(){
    let server =  zero2production::run("127.0.0.1:0").expect("Failed to start server");
    let _ = tokio::spawn(server);
}



#[tokio::test]
async fn health_check_works() {
    spawn_app();
}



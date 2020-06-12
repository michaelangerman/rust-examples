use redis_client::send;

#[tokio::main]
async fn main() {
    send::set_test().await;

    let res1 = send::send("set me ohmy").await;
    println!("{:?}", res1);

    let res2 = send::send("get me").await;
    println!("{:?}", res2);
}

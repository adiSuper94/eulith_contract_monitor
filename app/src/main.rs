#[tokio::main]
async fn main() {
    async_fn().await;
}

async fn async_fn() {
    println!("Hello, world!");
}

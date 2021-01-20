use futures::FutureExt;
use tokio::time::{self, Duration};
use tuple_fut::{Join, Select};

async fn wait1(t: u64) -> u64 {
    println!("wait {}", t);
    time::sleep(Duration::from_millis(t)).await;
    t
}

async fn wait2(t: u64) -> &'static str {
    println!("wait {}", t);
    time::sleep(Duration::from_millis(t)).await;
    "hello"
}

#[tokio::main]
async fn main() {
    tokio::pin! {
        let f1 = wait1(2);
        let f2 = wait2(100);
    }
    let r = (f1, f2).join().await;
    println!("{:?}", r);

    tokio::pin! {
        let f1 = wait1(100);
        let f2 = wait2(2).map(|s| s.len() as u64);
    }
    let r = (f1, f2).select().await;
    println!("{:?}", r);
}


// Rust asynchronous IO thread
use std::thread;

pub struct Runtime {

}

// impl Runtime {
//     pub fn new ()
// }

pub fn create_basic_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_io()
        .enable_time()
        .build()
        .unwrap()
}

pub fn run_basic<F, R>(future: F) -> R
where
    F: std::future::Future<Output = R> + 'static,
{
    let mut rt = create_basic_runtime();
    rt.block_on(future)
}

#[test]
fn test_future() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

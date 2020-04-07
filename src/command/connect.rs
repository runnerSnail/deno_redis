use crate::*;

use futures::future;
use redis::aio::{Connection, MultiplexedConnection};
use redis::RedisResult;
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, Debug)]
struct ConnectArgs {
    host: Option<String>,
    port: Option<u32>,
    db: Option<u32>,
}

pub fn get_connection(command: Command) -> CoreOp {
    let args: ConnectArgs = serde_json::from_slice(command.data.unwrap().as_ref()).unwrap();
    let url = format!(
        "redis://{}:{}/{}",
        args.host.unwrap(),
        args.port.unwrap(),
        args.db.unwrap()
    );
    let client = redis::Client::open(url).unwrap();
    CLIENT_ID.fetch_add(1, Ordering::SeqCst);
    CLIENTS.lock().unwrap().insert(CLIENT_ID.load(Ordering::SeqCst), client.clone());
    CoreOp::Sync(Buf::from(CLIENT_ID.load(Ordering::SeqCst).to_string().as_bytes()))
}

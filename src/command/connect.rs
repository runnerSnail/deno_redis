use crate::*;
use redis::IntoConnectionInfo;
use redis::RedisError;
use std::time::Duration;

use futures::future;
use redis::aio::ConnectionLike;
use redis::RedisResult;

#[derive(Serialize, Deserialize, Debug)]
struct ConnectArgs {
    host: Option<String>,
    port: Option<u32>,
    db: Option<u32>,
}

pub fn get_connection(command: Command) -> CoreOp {
    println!("test--->");

    println!("runtime:{}", "xxxx");

    let fut = async move {
        let args: ConnectArgs = serde_json::from_slice(command.data.unwrap().as_ref()).unwrap();
        let url = format!(
            "redis://{}:{}/{}",
            args.host.unwrap(),
            args.port.unwrap(),
            args.db.unwrap()
        );
        let client = redis::Client::open(url).unwrap();
        let client_id: usize = CLIENT_ID.fetch_add(1, Ordering::SeqCst);
        CLIENTS.lock().unwrap().insert(client_id, client.clone());
        let mut write_conns = Arc::clone(&CLIENTS_CONNECT);
        println!("error===>");
        let (con, driver) = client.get_multiplexed_async_connection().await.unwrap();
        // .and_then(|(con, driver)| {
        tokio::spawn(driver);
        // let cmds = (0..100).map(move |i| my_test(i));
        Ok(Buf::from(client_id.to_string().as_bytes()))
        // })
        // .map_err(|err| panic!("{}", err))
        // .await
        // .unwrap();
        // let  connect: Arc<MultiplexedConnection> = Arc::new(client.get_multiplexed_tokio_connection().await.unwrap());
        // let mut write_conns = write_conns.write().unwrap();
        // // write_conns.insert(client_id, connect.clone());
        // Ok(Buf::from(client_id.to_string().as_bytes()))
    };
    CoreOp::Async(fut.boxed())
}

async fn run_multi<C: ConnectionLike + Clone>(mut con: C) -> RedisResult<()> {
    // let mut interval = interval(Duration::from_millis(100));
    loop {
        // interval.tick().await;
        println!();
        println!("> PING");
        println!("> PING");
        println!("> PING");
        let results: (
            RedisResult<String>,
            RedisResult<String>,
            RedisResult<String>,
        ) = future::join3(
            redis::cmd("PING").query_async(&mut con.clone()),
            redis::cmd("PING").query_async(&mut con.clone()),
            redis::cmd("PING").query_async(&mut con),
        )
        .await;
        println!("< {:?}", results.0);
        println!("< {:?}", results.1);
        println!("< {:?}", results.2);
    }
}

// pub struct RedisCache {
//     client: redis::Client,
//     pool: Arc<Mutex<Option<MultiplexedConnection>>>,
// }

// impl RedisCache {
//     pub fn new<T: IntoConnectionInfo>(url: T) -> Self {
//         let client = redis::Client::open(url).unwrap();
//         let pool = Arc::new(Mutex::new(None));
//         RedisCache { client, pool }
//     }

//     fn checkout(&self) -> Box<Future<Item=MultiplexedConnection, Error=RedisError> + Send> {
//         let guard = self.pool.lock().unwrap();
//         if let Some(conn) = guard.clone() {
//             Box::new(Ok(conn))
//         } else {
//             let f = self.client.get_multiplexed_tokio_connection()
//                 .map(|conn| {
//                         let mut guard = pool_to_update.lock().unwrap();
//                         *guard = Some(conn.clone());
//                         conn
//                 });
//              Box::new(f)
//         }
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
// }

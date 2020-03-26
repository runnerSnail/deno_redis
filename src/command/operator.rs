use crate::*;
use futures::{executor, future, prelude::*};
use redis::{self, aio::MultiplexedConnection, RedisResult};
use serde_json::Value;
use std::time::Duration;
use std::{thread, time};
pub async fn get_multiplexed_tokio_connection_from_client_id(
    client_id: usize,
) -> MultiplexedConnection {
    get_client(client_id)
        .clone()
        .get_multiplexed_tokio_connection()
        .await
        .unwrap()
}

pub async fn get_multiplexed_async_connection_from_client_id(
    client_id: usize,
) -> (MultiplexedConnection, impl Future<Output = ()>) {
    get_client(client_id)
        .get_multiplexed_async_connection()
        .await
        .unwrap()
}

async fn test_cmd(con: &MultiplexedConnection) -> RedisResult<()> {
    let mut con = con.clone();

    let key = format!("key{}", 1);
    let key2 = format!("key{}_2", 2);
    let value = format!("foo{}", 2);

    redis::cmd("SET")
        .arg(&key[..])
        .arg(&value)
        .query_async(&mut con)
        .await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SetArgs {
    value: Value,
    key: Value,
}

pub fn set(command: Command) -> CoreOp {
    println!("===>set");
    let fut1 = async move {
        // let connect = get_client(0).clone();
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        // println!("=====<<<<");
        let mut connect: MultiplexedConnection =
          client.get_multiplexed_tokio_connection().await.unwrap();
        
        let data:String = redis::cmd("SET")
          .arg("key1")
          .arg(b"foo")
          .query_async(&mut connect)
          .await
          .unwrap();
        let result = b"test";
        let result_box: Buf = Box::new(*result);
        Ok(result_box)
        // .await
      };
    CoreOp::Async(fut1.boxed())
}

use crate::*;
use futures::{prelude::*};
use redis::{self, aio::MultiplexedConnection, RedisResult};
use serde_json::Value;

// todo 
pub async fn get_multiplexed_tokio_connection_from_client_id(
    client_id: usize,
) -> MultiplexedConnection {
    get_client(client_id)
        .clone()
        .get_multiplexed_tokio_connection()
        .await
        .unwrap()
}

// todo
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
    let fut1 = async move {
        // let connect = get_connection(0);
        // let data:String = redis::cmd("SET")
        //   .arg("key1")
        //   .arg(b"foo")
        //   .query_async(&mut connect.clone())
        //   .await
        //   .unwrap();
        // println!("result:{}",data);
        let result = b"test";
        let result_box: Buf = Box::new(*result);
        Ok(result_box)
        // .await
      };
    CoreOp::Async(fut1.boxed())
}
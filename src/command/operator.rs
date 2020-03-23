use crate::*;
use futures::{future, prelude::*};
use redis::{self, aio::MultiplexedConnection, RedisResult};
use serde_json::Value;

pub async fn get_multiplexed_tokio_connection_from_client_id(
    client_id: usize,
) -> MultiplexedConnection {
    get_client(client_id)
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
    let fut = async move {
        let connect:MultiplexedConnection = get_multiplexed_tokio_connection_from_client_id(
            command.identity.client_id.expect("no get client id"),
        )
        .await;

        let data = command.data;
        let args: SetArgs = serde_json::from_slice(data.unwrap().as_ref()).unwrap();
        let key = args.key;
        let value = args.value;

        let cmd = redis::cmd("SET")
            .arg("12")
            .arg("122")
            .query_async(&mut connect)
            .await;

        Ok(util::async_result(
            &command.identity,
            "OK"
        ))
    };
    CoreOp::Async(fut.boxed())
}

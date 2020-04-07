use crate::*;
use futures::prelude::*;
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
    value: String,
    key: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetArgs {
    key: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HsetArgs {
    fileds: Vec<String>,
    key: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HgetArgs {
    key: String,
    filed: String,
}

pub fn set(command: Command) -> CoreOp {
    let fut1 = async move {
        let mut client = get_client(command.identity.client_id.unwrap());
        let args: SetArgs = serde_json::from_slice(command.data.unwrap().as_ref()).unwrap();
        let mut connect = client.get_connection().unwrap();
        let _: () = redis::cmd("SET")
            .arg(args.key)
            .arg(args.value)
            .query(&mut connect)
            .unwrap();
        Ok(util::async_result(&command.identity, "OK"))
    };
    CoreOp::Async(fut1.boxed())
}

pub fn get(command: Command) -> CoreOp {
    let fut1 = async move {
        let mut client = get_client(command.identity.client_id.unwrap());
        let args: GetArgs = serde_json::from_slice(command.data.unwrap().as_ref()).unwrap();
        let mut connect = client.get_connection().unwrap();
        let result: String = redis::cmd("GET").arg(args.key).query(&mut connect).unwrap();
        Ok(util::async_result(&command.identity, result))
    };
    CoreOp::Async(fut1.boxed())
}

pub fn hget(command: Command) -> CoreOp {
    let fut1 = async move {
        let mut client = get_client(command.identity.client_id.unwrap());
        let args: HgetArgs = serde_json::from_slice(command.data.unwrap().as_ref()).unwrap();
        let mut connect = client.get_connection().unwrap();
        let result: String = redis::cmd("HGET")
            .arg(args.key)
            .arg(args.filed)
            .query(&mut connect)
            .unwrap();
        Ok(util::async_result(&command.identity, result))
    };
    CoreOp::Async(fut1.boxed())
}

pub fn hset(command: Command) -> CoreOp {
    let fut1 = async move {
        let mut client = get_client(command.identity.client_id.unwrap());
        let args: HsetArgs = serde_json::from_slice(command.data.unwrap().as_ref()).unwrap();
        let mut connect = client.get_connection().unwrap();
        let mut redis = redis::cmd("HSET");
        let mut cur = 0;
        let mut redis = redis.arg(args.key);
        while cur < args.fileds.len() {
            let mut redis = redis.arg(&args.fileds[cur]);
            let mut redis = redis.arg(&args.fileds[cur + 1]);
            cur += 2;
        }
        let _: () = redis.query(&mut connect).unwrap();
        Ok(util::async_result(&command.identity, "OK"))
    };
    CoreOp::Async(fut1.boxed())
}

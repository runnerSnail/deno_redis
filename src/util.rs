use crate::*;
use futures::Future;

pub fn async_result<T>(args: &CommandArgs, data: T) -> Buf
where
    T: Serialize,
{
    let result = AsyncResult {
        command_id: args.command_id.unwrap(),
        data,
    };
    let json = json!(result);
    let data = serde_json::to_vec(&json).unwrap();
    Buf::from(data)
}

// TODO

// pub fn create_basic_runtime() -> tokio::runtime::Runtime {
//     tokio::runtime::Builder::new()
//         .basic_scheduler()
//         .enable_io()
//         .enable_time()
//         .build()
//         .unwrap()
// }

// pub fn run_basic<F, R>(future: F) -> R
// where
//     F: std::future::Future<Output = R> + 'static,
// {
//     let mut rt = create_basic_runtime();
//     rt.block_on(future)
// }


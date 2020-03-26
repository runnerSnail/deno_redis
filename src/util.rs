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

// pub fn create_basic_runtime() -> tokio::runtime::Runtime {
//     let mut builder = tokio::runtime::Builder::new();

//         builder
//         .basic_scheduler()
//         .enable_io()
//         .enable_time()
//         .build()
//         .unwrap()
// }

// pub fn block_on_all<F>(f: F) -> F::Output
// where
//     F: Future,
// {
//     create_basic_runtime().block_on(f)
// }
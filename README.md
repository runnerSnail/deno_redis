# deno_redis

## perform the test

> RUST_BACKTRACE=full deno run --allow-all test.ts 

## The problem record

The deno load plug-in has no tokio runtime in it。[issue](https://github.com/denoland/deno/issues/4479)

The solution

> A new thread with tokio runtime interacts with dispatch_op for mpsc


## question

> After the initial test performance is not synchronized well。So it's synchronized

### My original deno design

![deno_redis](./readme/redis_plugin.png)

> If you can do better through asynchrony, please contact me


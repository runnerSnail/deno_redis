import { assert, assertEquals } from "https://deno.land/std/testing/asserts.ts";
import { cargoBuild } from "./build.ts";
import { localInit, RedisClient } from "./mod.ts";

const { test, runTests } = Deno;

test(async function testSet() {
  const client: RedisClient = new RedisClient({
    host: '127.0.0.1',
    port: 6379,
    db: 0,
  });
  assert(client.client_id == 1);
  const param = ["key1", "foo", "key2", "foo", "key3", "foo", "key4", "foo", "key5", "foo", "key6", "foo", "key7", "foo", "key7", "foo", "key8", "foo", "key9", "foo"];
  const time = new Date().getTime();
  await client.connection.hmset("test", param);
  console.log(new Date().getTime() - time);
});

test(async function test_set_get() {
  const client = new RedisClient({
    host: '127.0.0.1',
    port: 6379,
    db: 0,
  });
  assert(client.client_id == 2);
  await client.connection.set('test1', '121313');
  let result = await client.connection.get('test1');
  assert(result === '121313');
});

test(async function test_hset_hget() {
  const client = new RedisClient({
    host: '127.0.0.1',
    port: 6379,
    db: 0,
  });
  assert(client.client_id == 3);
  await client.connection.hmset('test', ['key1', '123']);
  let result = await client.connection.hmget('test', 'key1');
  assert(result === '123');
});

// deno env compile rust
// if (Deno.env()['rebuild']) {
await cargoBuild();
// }

localInit();
await runTests();

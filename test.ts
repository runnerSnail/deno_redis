import { assert, assertEquals } from "https://deno.land/std/testing/asserts.ts";
import { cargoBuild } from "./build.ts";
import { localInit, RedisClient } from "./mod.ts";

const { test, runTests } = Deno;

test(async function testSet() {
  console.log('result--->1');
  const client: RedisClient = new RedisClient({
    host: '127.0.0.1',
    port: 6379,
    db: 0,
  });
  console.log(client.client_id);
});

test(async function testSet() {
  const client: RedisClient = new RedisClient({
    host: '127.0.0.1',
    port: 6379,
    db: 0,
  });
  console.log(client.client_id);
});

await cargoBuild();
localInit();
await runTests();

import { assert, assertEquals } from "https://deno.land/std/testing/asserts.ts";
import { cargoBuild } from "./build.ts";
import { localInit, RedisClient } from "./mod.ts";

const { test, runTests } = Deno;

test(async function testSet() {
  console.log('result--->1');
  const client: RedisClient = new RedisClient();
  client.establishConnection({
    host: '127.0.0.1',
    port: 6379,
    db: 0,
  });
  let connection = client.getConenction();
  setTimeout(async ()=>{
    // const result = await connection.set('deno_redis','123');
  },1000);
});

await cargoBuild();
localInit();
await runTests();

import { assert, assertEquals } from "https://deno.land/std/testing/asserts.ts";
import { cargoBuild } from "./build.ts";
import { localInit,  RedisClient} from "./mod.ts";

const { test, runTests } = Deno;

test(async function testGetConnect() {
  const client:RedisClient = new RedisClient();
  client.get_connection({
    host:'127.0.0.1',
    port: 6379,
    db:0,
  });
  console.log(client.client_id);
});

await cargoBuild();
localInit();
await runTests();

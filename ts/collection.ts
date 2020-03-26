import { RedisClient } from "./client.ts";
import { CommandType } from "./types.ts";
import { dispatchAsync, encode } from "./operator.ts";

export class Collection {
  constructor(
    private readonly client_id: number,
  ) { }

  public async set(key: string, value: string): Promise<any> {
    console.log('doc1');
    const doc = await dispatchAsync(
      {
        command_type: CommandType.Cmd,
        client_id: this.client_id,
      },
      encode(
        JSON.stringify({
          key,
          value,
        })
      )
    );
    console.log('doc',doc);
    return doc;
  }
}

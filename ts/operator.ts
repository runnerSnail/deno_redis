// import { prepare } from "https://raw.githubusercontent.com/manyuanrong/deno-plugin-prepare/master/mod.ts";
import { Command } from "./types.ts";
import { VERSION } from "../mod.ts";

const PLUGIN_NAME = "deno_redis";
const pendingOperator: Map<number, (data: unknown) => void> = new Map();
let dispatcher: Deno.PluginOp | null = null;


const decoder = new TextDecoder();
const encoder = new TextEncoder();


export function encode(str: string): Uint8Array {
    return encoder.encode(str);
}

export function decode(data: Uint8Array): string {
    return decoder.decode(data);
}


// export async function init(libReleaseUrl: string = 'https://github.com/runnerSnail/deno_redis/releases/download/', libVersion: string = VERSION ) {
//     let releaseLibraryUrl = '';
//     if(libReleaseUrl.startsWith('local')){
//         releaseLibraryUrl = libReleaseUrl;
//         return;
//     }else{
//         releaseLibraryUrl = `${libReleaseUrl}${libVersion}`;
//     }

//     const options = {
//         name: PLUGIN_NAME,
//         urls: {
//             mac: `${releaseLibraryUrl}/lib${PLUGIN_NAME}.dylib`,
//             win: `${releaseLibraryUrl}/${PLUGIN_NAME}.dll`,
//             linux: `${releaseLibraryUrl}/lib${PLUGIN_NAME}.so`
//         }
//     };

//     const Redis = await prepare(options);
//     dispatcher = Redis.ops["redis_command"];
//     dispatcher.setAsyncHandler((msg: Uint8Array) => {
//         const { command_id, data } = JSON.parse(decoder.decode(msg));
//         const resolver = pendingOperator.get(command_id);
//         resolver && resolver(data);
//     });
// }

export function localInit(localPath:string){
    const Redis = Deno.openPlugin(localPath);
    dispatcher = Redis.ops["redis_command"];
    dispatcher.setAsyncHandler((msg: Uint8Array) => {
        const { command_id, data } = JSON.parse(decoder.decode(msg));
        const resolver = pendingOperator.get(command_id);
        resolver && resolver(data);
    });
}

export function dispatch(command: Command, data?: ArrayBufferView): Uint8Array {
    const control = encoder.encode(JSON.stringify(command));
    if (!dispatcher) {
        throw new Error("The plugin must be initialized before use");
    }
    return dispatcher.dispatch(control, data)!;
}
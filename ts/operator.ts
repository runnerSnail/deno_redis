import { prepare } from "https://raw.githubusercontent.com/manyuanrong/deno-plugin-prepare/master/mod.ts";
import { Command } from "./types.ts";
import { VERSION } from "../mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const os = Deno.build.os;
const PLUGIN_NAME = "deno_redis";
const pendingOperator: Map<number, (data: unknown) => void> = new Map();
const PLUGIN_SUFFIX_MAP: { [os in Deno.OperatingSystem]: string } = {
    mac: ".dylib",
    win: ".dll",
    linux: ".so"
};

let dispatcher: Deno.PluginOp | null = null;
let commandId = 0;


const decoder = new TextDecoder();
const encoder = new TextEncoder();


export function encode(str: string): Uint8Array {
    return encoder.encode(str);
}

export function decode(data: Uint8Array): string {
    return decoder.decode(data);
}


export async function init(libReleaseUrl: string = 'https://github.com/runnerSnail/deno_redis/releases/download/', libVersion: string = VERSION) {
    let releaseLibraryUrl = '';
    if (libReleaseUrl.startsWith('local')) {
        releaseLibraryUrl = libReleaseUrl;
        return;
    } else {
        releaseLibraryUrl = `${libReleaseUrl}${libVersion}`;
    }

    const options = {
        name: PLUGIN_NAME,
        urls: {
            mac: `${releaseLibraryUrl}/lib${PLUGIN_NAME}.dylib`,
            win: `${releaseLibraryUrl}/${PLUGIN_NAME}.dll`,
            linux: `${releaseLibraryUrl}/lib${PLUGIN_NAME}.so`
        }
    };

    const Redis = await prepare(options);
    dispatcher = Redis.ops["redis_command"];
    dispatcher.setAsyncHandler((msg: Uint8Array) => {
        const { command_id, data } = JSON.parse(decoder.decode(msg));
        const resolver = pendingOperator.get(command_id);
        resolver && resolver(data);
    });
}

export function localInit() {
    const urls = {
        mac: `${PLUGIN_NAME}.dylib`,
        win: `${PLUGIN_NAME}.dll`,
        linux: `${PLUGIN_NAME}.so`
    };
    const localPath = path.resolve(".deno_plugins", urls[os]);
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

export function dispatchAsync(
    command: Command,
    data?: ArrayBufferView
): Promise<unknown> {
    return new Promise(resolve => {
        pendingOperator.set(++commandId, resolve);
        console.log('commondId',commandId);
        const control = encoder.encode(
            JSON.stringify({
                ...command,
                command_id: commandId
            })
        );
        console.log('control',JSON.stringify({
            ...command,
            command_id: commandId
        }));
        if (!dispatcher) {
            if (!dispatcher) {
                throw new Error("The plugin must be initialized before use");
            }
        }
        dispatcher.dispatch(control, data);
    });
}

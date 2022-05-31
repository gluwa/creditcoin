import { creditcoinApi } from 'creditcoin-js';
import { Keyring } from '@polkadot/api';
import * as fs from 'fs';
import * as child_process from 'child_process';
import { promisify } from 'util';
import * as _ from '@polkadot/api/augment';

type WasmRuntimeInfo = {
    size: number;
    compression: {
        size_compressed: number;
        size_decompressed: number;
        compressed: boolean;
    };
    reserved_meta: Array<number>;
    reserved_meta_valid: boolean;
    metadata_version: number;
    core_version: string;
    proposal_hash: string;
    parachain_authorize_upgrade_hash: string;
    ipfs_hash: string;
    blake2_256: string;
};

const readFile = promisify(fs.readFile);
const exec = promisify(child_process.exec);

async function doRuntimeUpgrade(
    wsUrl: string,
    wasmBlobPath: string,
    sudoKeyUri: string,
    hasSubwasm: boolean = false,
): Promise<void> {
    const { api } = await creditcoinApi(wsUrl);
    try {
        const keyring = new Keyring({ type: 'sr25519' }).createFromUri(sudoKeyUri);

        if (hasSubwasm) {
            // subwasm needs to be installed with `cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.17.1`
            const output = await exec(`subwasm info -j ${wasmBlobPath}`);
            if (output.stderr.length > 0) {
                throw new Error(`subwasm info failed: ${output.stderr}`);
            }
            const info = JSON.parse(output.stdout) as WasmRuntimeInfo;
            // should probably do some checks here to see that the runtime is right
            // e.g. the core version is reasonable, it's compressed, etc.
        }

        const wasmBlob = await readFile(wasmBlobPath);

        const u8aToHex = (bytes: Uint8Array | Buffer): string => {
            const byteArray = Uint8Array.from(bytes);
            return byteArray.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '0x');
        };

        const unsub = await api.tx.sudo
            .sudoUncheckedWeight(api.tx.system.setCode(u8aToHex(wasmBlob)), 1)
            .signAndSend(keyring, { nonce: -1 }, (result) => {
                if (result.isInBlock && !result.isError) {
                    console.log('Runtime upgrade successful');
                    unsub();
                } else if (result.isError) {
                    console.error(`Runtime upgrade failed: ${result}`);
                    unsub();
                }
            });
    } finally {
        await api.disconnect();
    }
}

if (process.argv.length < 5) {
    console.error('runtimeUpgrade.ts <wsUrl> <wasmBlobPath> <sudoKeyUri>');
    process.exit(1);
}

console.log(process.argv);
const wsUrl = process.argv[2];
const wasmBlobPath = process.argv[3];
const sudoKeyUri = process.argv[4];

doRuntimeUpgrade(wsUrl, wasmBlobPath, sudoKeyUri).catch(console.error);

import { ApiPromise, CREDO_PER_CTC, creditcoinApi, Keyring } from 'creditcoin-js';
import { Option } from 'creditcoin-js';
import { Vec, Bytes } from '@polkadot/types';
import { createOverrideWeight } from 'creditcoin-js/lib/utils';
import { decodeAddress, cryptoWaitReady } from '@polkadot/util-crypto';
import { u8aToHex } from './common';

import { ITuple } from '@polkadot/types/types/interfaces';
import { KeyTypeId } from '@polkadot/types/interfaces';

type Keys = {
    grandpa: string;
    babe: string;
    imOnline: string;
};

async function rotateKeys(api: ApiPromise): Promise<Keys> {
    const rotatedKeys = await api.rpc.author.rotateKeys();
    console.log('Rotated` keys:', rotatedKeys.toHex());
    const decoded = await api.call.sessionKeys.decodeSessionKeys<Option<Vec<ITuple<[Bytes, KeyTypeId]>>>>(rotatedKeys);
    const unwrapped = decoded.unwrap();
    const [grandpaKey] = unwrapped[0];
    const [babeKey] = unwrapped[1];
    const [imOnlineKey] = unwrapped[2];
    console.log('Grandpa key:', grandpaKey.toHex());
    console.log('Babe key:', babeKey.toHex());
    console.log('ImOnline key:', imOnlineKey.toHex());
    return {
        grandpa: grandpaKey.toHex(),
        babe: babeKey.toHex(),
        imOnline: imOnlineKey.toHex(),
    };
}

function ctc(credo: number | bigint | string) {
    return BigInt(credo) * BigInt(CREDO_PER_CTC);
}

async function waitBlocks(wsUrl: string, n: number): Promise<void> {
    const { api } = await creditcoinApi(wsUrl);

    const {
        block: {
            header: { number: startBlock },
        },
    } = await api.rpc.chain.getBlock();
    const targetBlock = startBlock.toNumber() + n;
    await new Promise<void>((resolve, reject) => {
        const unsubscribe = api.rpc.chain.subscribeNewHeads((header) => {
            const currentBlock = header.number.toNumber();
            if (currentBlock >= targetBlock) {
                unsubscribe.then(
                    (unsub) => {
                        unsub();
                        resolve();
                    },
                    (err) => {
                        console.error(err);
                        reject(err);
                    },
                );
            }
        });
    });
}

async function doSwitchToPos(wsUrl: string, sudoKeyUri: string): Promise<void> {
    // init the api client
    const { api } = await creditcoinApi(wsUrl);
    await cryptoWaitReady();
    try {
        // make the keyring for the sudo account
        const keyring = new Keyring({ type: 'sr25519' }).createFromUri(sudoKeyUri);
        const ed25519Keyring = new Keyring({ type: 'ed25519' }).createFromUri(sudoKeyUri);
        const overrideWeight = createOverrideWeight(api);
        let grandpa: string;
        let babe: string;
        let imOnline: string;

        // if it's not Alice, rotate the keys (the proper method).
        // if it's alice, just use alice's keys as session keys (not the proper method)
        // for the ease of having a known initial grandpa key (in tests)
        if (sudoKeyUri.trim() !== '//Alice') {
            const keys = await rotateKeys(api);
            grandpa = keys.grandpa;
            babe = keys.babe;
            imOnline = keys.imOnline;
        } else {
            const decode = (ss58: string) => u8aToHex(decodeAddress(ss58));
            grandpa = decode(ed25519Keyring.address);
            babe = decode(keyring.address);
            imOnline = decode(keyring.address);
        }
        const initialValidators = [
            {
                stash: keyring.address,
                controller: keyring.address,
                grandpa,
                babe,
                imOnline,
                bonded: ctc('1000000'),
                controllerBalance: 0,
                invulnerable: true,
            },
        ];
        console.log(initialValidators);
        const callback = api.tx.posSwitch.switchToPos(initialValidators);

        await new Promise<void>((resolve, reject) => {
            const unsubscribe = api.tx.sudo
                .sudoUncheckedWeight(callback, overrideWeight)
                .signAndSend(keyring, { nonce: -1 }, (result) => {
                    const finish = (fn: () => void) => {
                        console.log('finish called');
                        unsubscribe
                            .then((unsub) => {
                                console.log('unsubscribing');
                                unsub();
                                console.log('unsubscribed');
                                fn();
                            })
                            .catch(reject);
                    };
                    if (result.isInBlock && !result.isError) {
                        console.log('switchToPos called');
                        finish(resolve);
                    } else if (result.isError) {
                        // eslint-disable-next-line @typescript-eslint/no-base-to-string
                        const error = new Error(`Failed calling switchToPos: ${result.toString()}`);
                        finish(() => reject(error));
                    }
                });
        });
        console.log('switchToPos finished');
        process.exit(0);
    } finally {
        await api.disconnect();
    }
}

if (process.argv.length < 3) {
    console.error('switchToPos.ts <wsUrl> <sudoKeyUri> [waitNBlocks]');
    process.exit(1);
}

const inputWsUrl = process.argv[2];
const inputSudoKeyUri = process.argv[3];
const waitNBlocks = process.argv[4] ? parseInt(process.argv[4].trim(), 10) : 0;

const preSwitch = waitNBlocks > 0 ? waitBlocks(inputWsUrl, waitNBlocks) : Promise.resolve();
preSwitch.then(
    () => doSwitchToPos(inputWsUrl, inputSudoKeyUri),
    (reason) => {
        console.error(reason);
        process.exit(1);
    },
);

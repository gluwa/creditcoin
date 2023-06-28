import { ApiPromise, CREDO_PER_CTC, creditcoinApi, Keyring } from 'creditcoin-js';
import { Option, Vec, Bytes } from 'creditcoin-js';
import { createOverrideWeight } from 'creditcoin-js/lib/utils';

import { ITuple } from '@polkadot/types/types/interfaces';
import { KeyTypeId } from '@polkadot/types/interfaces';
import { decodeAddress } from '@polkadot/util-crypto';

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

async function doSwitchToPos(wsUrl: string, sudoKeyUri: string): Promise<void> {
    // init the api client
    const { api } = await creditcoinApi(wsUrl);
    try {
        // make the keyring for the sudo account
        const keyring = new Keyring({ type: 'sr25519' }).createFromUri(sudoKeyUri);
        const overrideWeight = createOverrideWeight(api);

        const { grandpa, babe, imOnline } = await rotateKeys(api);
        const initialValidators = [
            {
                stash: keyring.address,
                controller: keyring.address,
                grandpa,
                babe,
                imOnline,
                bonded: ctc('1_000_000'),
                controllerBalance: 0,
                invulnerable: true,
            },
        ];
        const callback = api.tx.posSwitch.switchToPos(initialValidators);

        await new Promise<void>((resolve, reject) => {
            const unsubscribe = api.tx.sudo
                .sudoUncheckedWeight(callback, overrideWeight)
                .signAndSend(keyring, { nonce: -1 }, (result) => {
                    const finish = (fn: () => void) => {
                        unsubscribe
                            .then((unsub) => {
                                unsub();
                                fn();
                            })
                            .catch(reject);
                    };
                    if (result.isInBlock && !result.isError) {
                        console.log('switchToPos called');
                        finish(resolve);
                    } else if (result.isError) {
                        const error = new Error(`Failed calling switchToPos: ${result.toString()}`);
                        finish(() => reject(error));
                    }
                });
        });
    } finally {
        await api.disconnect();
    }
}

if (process.argv.length < 3) {
    console.error('switchToPos.ts <wsUrl> <sudoKeyUri>');
    process.exit(1);
}

const inputWsUrl = process.argv[2];
const inputSudoKeyUri = process.argv[3];

doSwitchToPos(inputWsUrl, inputSudoKeyUri).catch((reason) => {
    console.error(reason);
    process.exit(1);
});

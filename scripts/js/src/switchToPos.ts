import { creditcoinApi, Keyring } from 'creditcoin-js';
import { createOverrideWeight } from 'creditcoin-js/lib/utils';

async function doSwitchToPos(wsUrl: string, sudoKeyUri: string): Promise<void> {
    // init the api client
    const { api } = await creditcoinApi(wsUrl);
    try {
        // make the keyring for the sudo account
        const keyring = new Keyring({ type: 'sr25519' }).createFromUri(sudoKeyUri);
        const overrideWeight = createOverrideWeight(api);
        const callback = api.tx.posSwitch.switchToPos();

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

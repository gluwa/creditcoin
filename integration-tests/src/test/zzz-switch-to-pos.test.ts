import fs from 'fs';
import { U256 } from '@polkadot/types-codec';
import { Option, Result, Null } from '@polkadot/types';
import { SpRuntimeDispatchError } from '@polkadot/types/lookup';
import { Blockchain, KeyringPair } from 'creditcoin-js';
import { creditcoinApi, ApiPromise } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { createOverrideWeight } from 'creditcoin-js/lib/utils';
import { testData } from 'creditcoin-js/lib/testUtils';
import { describeIf, testIf } from '../utils';

describeIf((global as any).CREDITCOIN_SWITCH_TO_POS_ALREADY_CALLED !== true, 'switch_to_post()', (): void => {
    let ccApi: CreditcoinApi;
    let root: KeyringPair;

    // this file is created by the calling CI environment
    const { keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    /// Send a sudo transaction with overridden weight
    /// and wait for its result. If successful, the promise
    /// resolves. If unsuccessful, the promise rejects with
    /// the name of the error (e.g. "posSwitch.AlreadySwitched").
    const sendSudo = (api: ApiPromise, keypair: KeyringPair, callback: any): Promise<void> => {
        const weight = createOverrideWeight(api);
        return new Promise((resolve, reject) => {
            const unsub = api.tx.sudo
                .sudoUncheckedWeight(callback, weight)
                .signAndSend(keypair, { nonce: -1 }, ({ status, events }) => {
                    const unsubAndResolve = () => {
                        unsub.then((us) => us()).catch(console.error);
                        resolve();
                    };
                    const unsubAndReject = (s: string) => {
                        unsub.then((us) => us()).catch(console.error);
                        reject(new Error(s));
                    };
                    if (status.isInBlock || status.isFinalized) {
                        events
                            // We know this tx should result in `Sudid` event.
                            .filter(({ event }) => api.events.sudo.Sudid.is(event))
                            // We know that `Sudid` returns just a `Result`
                            .forEach(
                                ({
                                    event: {
                                        data: [sudoResult],
                                    },
                                }) => {
                                    const result = sudoResult as Result<Null, SpRuntimeDispatchError>;

                                    // Now we look to see if the extrinsic was actually successful or not...
                                    if (result.isErr) {
                                        const error = result.asErr;
                                        if (error.isModule) {
                                            // for module errors, we have the section indexed, lookup
                                            const decoded = api.registry.findMetaError(error.asModule);
                                            const { name, section } = decoded;

                                            unsubAndReject(`${section}.${name}`);
                                        } else {
                                            // Other, CannotLookup, BadOrigin, no extra info
                                            unsubAndReject(error.toString());
                                        }
                                    } else {
                                        unsubAndResolve();
                                    }
                                },
                            );
                        unsub.then((us) => us()).catch(console.error);
                        reject(new Error("Didn't find sudo result event"));
                    }
                });
        });
    };

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        root = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'sudo');
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    // LAST_POW_BLOCK_NUMBER will be configured from the outside b/c this test only
    // makes sense when someone has called switch_to_pos() explicitly, e.g. as part of an upgrade
    testIf(process.env.LAST_POW_BLOCK_NUMBER !== undefined, 'was already called', async (): Promise<void> => {
        const { api } = ccApi;
        // note: this value is exported as hex string inside CI
        const lastPoWBlockNumber = parseInt(process.env.LAST_POW_BLOCK_NUMBER as string, 16);

        const result = await api.query.posSwitch.switchBlockNumber<Option<U256>>();
        expect(result.isSome).toBeTruthy();
        const switchBlockNumber = result.unwrap().toNumber();
        expect(switchBlockNumber).toBeGreaterThan(lastPoWBlockNumber);

        const currentDifficulty = await api.query.difficulty.currentDifficulty<U256>();
        expect(currentDifficulty.isMax()).toBeTruthy();
    });

    testIf(process.env.LAST_POW_BLOCK_NUMBER !== undefined, 'fails when called again', async (): Promise<void> => {
        const { api } = ccApi;

        const callback = api.tx.posSwitch.switchToPos();
        await expect(sendSudo(api, root, callback)).rejects.toThrow('posSwitch.AlreadySwitched');
    });

    testIf(
        process.env.LAST_POW_BLOCK_NUMBER !== undefined &&
            process.env.LAST_POW_BLOCK_INFO_PATH !== undefined &&
            fs.existsSync(process.env.LAST_POW_BLOCK_INFO_PATH),
        'block history is preserved',
        async (): Promise<void> => {
            const { api } = ccApi;

            const powBlockData = JSON.parse(fs.readFileSync(process.env.LAST_POW_BLOCK_INFO_PATH as string, 'utf-8'));

            const blockHash = await api.rpc.chain.getBlockHash(process.env.LAST_POW_BLOCK_NUMBER);
            const blockInfo = await api.rpc.chain.getBlock(blockHash);
            const posBlockData = JSON.parse(JSON.stringify(blockInfo));

            // returned as decimal integer in blockInfo but hex string before PoS switch
            powBlockData.block.header.number = parseInt(powBlockData.block.header.number, 16); // eslint-disable-line
            // the digest field differs before and after the switch
            delete powBlockData.block.header.digest;
            delete posBlockData.block.header.digest;

            expect(posBlockData).toEqual(powBlockData);
        },
    );
});

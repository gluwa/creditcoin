import { CreditcoinApi } from '../types';
import { CollectCoinsContract } from '../model';
import { KeyringPair } from '@polkadot/keyring/types';

export async function collectCoinsV2Example(
    ccApi: CreditcoinApi,
    burnDetails: CollectCoinsContract,
    creditcoinSigner: KeyringPair,
) {
    const {
        extrinsics: { requestCollectCoinsV2 },
    } = ccApi;

    // Submit the swap request, adding it to the task queue of the off chain worker
    const collectCoins = await requestCollectCoinsV2(burnDetails, creditcoinSigner);

    // Wait for the offchain worker to finish processing this request
    // Under the hood waitForVerification tracks CollectedCoinsMinted and CollectedCoinsFailedVerification events using the TaskId as a unique key
    // 900_000 (milliseconds) comes from an assumed 60 block task timeout deadline and assumed 15 second blocktime (check the constants provided by the runtime in production code)
    return await collectCoins.waitForVerification(900_000);
}

import { ISubmittableResult } from "@polkadot/types/types";
import { ApiPromise, KeyringPair } from "creditcoin-js";

import { SubmittableExtrinsic } from "@polkadot/api/types";

export async function signSendAndWatch(
  tx: SubmittableExtrinsic<"promise", ISubmittableResult>,
  api: ApiPromise,
  signer: KeyringPair
): Promise<TxResult> {
  let result;

  return new Promise(async (resolve) => {
    let result = await new Promise(
      await tx.signAndSend(signer, ({ status, dispatchError }) => {
        if (status.isFinalized) {
          const result = {
            status: TxStatus.ok,
            info: `Transaction included at blockHash ${status.asFinalized.toString()}`,
          };

          // resolve(result);
        }
        if (dispatchError) {
          if (dispatchError.isModule) {
            // for module errors, the section is indexed, lookup
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;

            const error = `${section}.${name}: ${docs.join(" ")}`;

            const result = {
              status: TxStatus.failed,
              info: `Transaction failed with error: "${error}"`,
            };

            //   resolve(result);
          } else {
            // Other, CannotLookup, BadOrigin, no extra info
            const result = {
              status: TxStatus.failed,
              info: `Transaction failed with error: "${dispatchError.toString()}"`,
            };
            //   resolve(result);
          }
        }
      })
    );
  });
}

export enum TxStatus {
  ok,
  failed,
}

export interface TxResult {
  status: TxStatus;
  info: string;
}

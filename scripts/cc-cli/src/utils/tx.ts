import { ISubmittableResult } from "@polkadot/types/types";
import { ApiPromise, KeyringPair } from "creditcoin-js";

import { SubmittableExtrinsic } from "@polkadot/api/types";

export async function signSendAndWatch(
  tx: SubmittableExtrinsic<"promise", ISubmittableResult>,
  api: ApiPromise,
  signer: KeyringPair
): Promise<TxResult> {
  // Top-Level Promise

  // eslint-disable-next-line @typescript-eslint/no-misused-promises
  return new Promise(async (resolve) => {
    // eslint-disable-next-line @typescript-eslint/no-misused-promises
    const out: TxResult = await new Promise(async (resolveInner) => {
      console.log("Sending transaction...");
      // Sign and send with callback
      await tx.signAndSend(signer, ({ status, dispatchError }) => {
        // Called every time the status changes
        if (status.isFinalized) {
          const result = {
            status: TxStatus.ok,
            info: `Transaction included at blockHash ${status.asFinalized.toString()}`,
          };

          resolveInner(result);
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

            resolveInner(result);
          } else {
            // Other, CannotLookup, BadOrigin, no extra info
            const result = {
              status: TxStatus.failed,
              info: `Transaction failed with error: "${dispatchError.toString()}"`,
            };
            resolveInner(result);
          }
        }
      });
    });
    resolve(out);
  });
}

// eslint-disable-next-line no-shadow
export enum TxStatus {
  ok,
  failed,
}

export interface TxResult {
  status: TxStatus;
  info: string;
}

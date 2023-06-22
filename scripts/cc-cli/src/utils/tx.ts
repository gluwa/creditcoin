import { ISubmittableResult } from "@polkadot/types/types";
import { ApiPromise, KeyringPair } from "creditcoin-js";

import { SubmittableExtrinsic } from "@polkadot/api/types";

export async function signSendAndWatch(
  tx: SubmittableExtrinsic<"promise", ISubmittableResult>,
  api: ApiPromise,
  signer: KeyringPair
): Promise<TxResult> {
  return new Promise((resolve, reject) => {
    console.log("Sending transaction...");
    let maybeUnsub: (() => void) | undefined;
    const unsubAndResolve = (result: TxResult) => {
      if (maybeUnsub) {
        maybeUnsub();
      }
      resolve(result);
    };
    // Sign and send with callback
    tx.signAndSend(signer, ({ status, dispatchError }) => {
      // Called every time the status changes
      if (status.isFinalized) {
        const result = {
          status: TxStatus.ok,
          info: `Transaction included at blockHash ${status.asFinalized.toString()}`,
        };
        unsubAndResolve(result);
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
          unsubAndResolve(result);
        } else {
          // Other, CannotLookup, BadOrigin, no extra info
          const result = {
            status: TxStatus.failed,
            info: `Transaction failed with error: "${dispatchError.toString()}"`,
          };
          unsubAndResolve(result);
        }
      }
    })
      .then((unsub) => {
        maybeUnsub = unsub;
      })
      .catch((err) => {
        reject(err);
      });
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

import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { ISubmittableResult } from "@polkadot/types/types";
import { u8aConcat } from "@polkadot/util";
import { blake2AsHex } from "@polkadot/util-crypto";
import { PalletCreditcoinBlockchain } from "@polkadot/types/lookup";

export const handleTransaction = async (api: ApiPromise, unsubscribe: (() => void), result: ISubmittableResult) => {
    const { status, events, dispatchError } = result;
    console.log(`current status is ${status}`);
    if (dispatchError) {
        if (dispatchError.isModule) {
            // for module errors, we have the section indexed, lookup
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;

            console.log(`${section}.${name}: ${docs.join(' ')}`);
        } else {
            // Other, CannotLookup, BadOrigin, no extra info
            console.log(dispatchError.toString());
        }
        unsubscribe();
        api.disconnect();
    }
    if (status.isInBlock) {
        events.forEach(({ event }) => {
            const types = event.typeDef;
            event.data.forEach((data, index) => {
                console.log(`pallet: ${event.section} event name: ${event.method}`)
                console.log(`event types ${types[index].type} event data: ${data.toString()}`);
            });
        });
        unsubscribe();
        api.disconnect();
    }
}

export const getAddressId = (blockchain: PalletCreditcoinBlockchain | string, externalAddress: string) => {
    const addressId = u8aConcat(
        Buffer.from(blockchain.toString().toLowerCase()),
        Buffer.from(externalAddress));
    console.log(`addressId: ${addressId}`);
    return blake2AsHex(addressId);
}
// [object Object]
// SPDX-License-Identifier: Apache-2.0

import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { GenericEventData } from '@polkadot/types/';
import { PalletCreditcoinAddress } from '@polkadot/types/lookup';

import { handleTransaction, TxOnFail, TxOnSuccess } from '../utils';

type Blockchain = 'Ethereum' | 'Rinkeby' | 'Luniverse' | 'Bitcoin' | 'Other';

type Address = {
  accountId: string;
  blockchain: Blockchain;
  externalAddress: string;
};

type RegisteredAddress = {
  addressId: string;
  address: Address;
};

export const registerAddress = async (
  api: ApiPromise,
  externalAddress: string,
  blockchain: string,
  signer: KeyringPair,
  onSuccess: TxOnSuccess,
  onFail: TxOnFail
) => {
  const unsubscribe: () => void = await api.tx.creditcoin
    .registerAddress(blockchain, externalAddress)
    .signAndSend(signer, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

const processRegisteredAddress = (api: ApiPromise, result: SubmittableResult): RegisteredAddress | undefined => {
  const { events } = result;
  const addressRegistered = events.find(({ event }) => event.method == 'AddressRegistered');

  const getData = (data: GenericEventData) => {
    const addressId = data[0].toString();
    const { blockchain, owner, value } = api.createType<PalletCreditcoinAddress>(
      'PalletCreditcoinAddress',
      data[1]
    );
    const address = { accountId: owner.toString(), blockchain: blockchain.type, externalAddress: value.toString() };

    return { addressId, address };
  };

  return addressRegistered && getData(addressRegistered.event.data);
};

export const registerAddressAsync = async (
  api: ApiPromise,
  externalAddress: string,
  blockchain: string,
  signer: KeyringPair
) => {
  return new Promise<RegisteredAddress | undefined>(async (resolve) => {
    const onFail = () => resolve(undefined);
    const onSuccess = (result: SubmittableResult) => resolve(processRegisteredAddress(api, result));

    await registerAddress(api, externalAddress, blockchain, signer, onSuccess, onFail);
  });
};

import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { Currency, CurrencyId, EventReturnJoinType } from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, processEvents } from './common';
import { TxCallback, TxFailureCallback } from '../types';
import { createCreditcoinCurrency, createCurrency } from '../transforms';
import { PalletCreditcoinPlatformCurrency } from '@polkadot/types/lookup';
import { blake2AsHex } from '@polkadot/util-crypto';

export type CurrencyRegistered = EventReturnJoinType<CurrencyId, Currency>;

export const createCurrencyId = (api: ApiPromise, currency: Currency): CurrencyId => {
    switch (currency.platform) {
        case 'Evm':
            switch (currency.type) {
                case 'SmartContract':
                    const { contract, chainId } = currency;
                    const tup = api.createType('(Bytes, PalletCreditcoinPlatformEvmChainId)', [contract, chainId]);
                    const bytes = tup.toU8a();
                    return blake2AsHex(bytes);
            }
    }
};

export const registerCurrency = async (
    api: ApiPromise,
    currency: Currency,
    sudoSigner: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const unsubscribe: () => void = await api.tx.sudo
        .sudo(api.tx.creditcoin.registerCurrency(createCreditcoinCurrency(api, currency)))
        .signAndSend(sudoSigner, { nonce: -1 }, (result) =>
            handleTransaction(api, unsubscribe, result, onSuccess, onFail),
        );
};

export const registerCurrencyAsync = async (
    api: ApiPromise,
    currency: Currency,
    sudoSigner: KeyringPair,
): Promise<CurrencyRegistered> => {
    return new Promise<CurrencyRegistered>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) => resolve(processCurrencyRegistered(api, result)); // eslint-disable-line @typescript-eslint/no-unused-vars
        registerCurrency(api, currency, sudoSigner, onSuccess, reject).catch(reject);
    });
};

const processCurrencyRegistered = (api: ApiPromise, result: SubmittableResult): CurrencyRegistered => {
    return processEvents<CurrencyId, Currency, PalletCreditcoinPlatformCurrency>(
        api,
        result,
        'CurrencyRegistered',
        'PalletCreditcoinPlatformCurrency',
        createCurrency,
    ) as CurrencyRegistered;
};

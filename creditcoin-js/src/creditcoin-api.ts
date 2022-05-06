import { ApiPromise, WsProvider } from '@polkadot/api';
import { extrinsics } from './extrinsics/extrinsics';
import { utils } from './utils';
import { CreditcoinApi } from './types';

export const creditcoinApi = async (wsUrl: string): Promise<CreditcoinApi> => {
    const provider = new WsProvider(wsUrl);
    const api = await ApiPromise.create({ provider });

    return { api, extrinsics: extrinsics(api), utils: utils(api) };
};

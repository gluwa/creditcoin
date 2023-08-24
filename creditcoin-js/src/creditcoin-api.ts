import { ApiPromise, WsProvider } from '@polkadot/api';
import { extrinsics } from './extrinsics/extrinsics';
import { utils } from './utils';
import type { CreditcoinApi } from './types';

export const creditcoinApi = async (wsUrl: string, noInitWarn = false): Promise<CreditcoinApi> => {
    const provider = new WsProvider(wsUrl);
    const api = await ApiPromise.create({ provider, noInitWarn });

    return { api, extrinsics: extrinsics(api), utils: utils(api) };
};

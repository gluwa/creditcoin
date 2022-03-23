// [object Object]
// SPDX-License-Identifier: Apache-2.0

import { ApiPromise, WsProvider } from '@polkadot/api';

describe('System RPC sanity test', (): void => {
  let api: ApiPromise;

  beforeEach(async () => {
    process.env.NODE_ENV = 'test';

    const provider = new WsProvider('ws://127.0.0.1:9944');

    api = await ApiPromise.create({ provider });
  });

  afterEach(async () => {
    await api.disconnect();
  });

  it('rpc.system.chain() works', (): void => {
    return api.rpc.system.chain().then(result => {
      expect(result.toString()).toBe('Development');
    });
  });

  it('rpc.system.name() works', (): void => {
    return api.rpc.system.name().then(result => {
      expect(result.toString()).toBe('Creditcoin Node');
    });
  });

  it('rpc.system.version() works', (): void => {
    return api.rpc.system.version().then(result => {
      expect(result.toString()).toEqual(expect.anything());
    });
  });

});

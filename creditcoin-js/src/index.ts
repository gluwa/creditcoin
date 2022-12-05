export * from './creditcoin-api';
export * from './types';
export * from './model';

export { providers, Wallet } from 'ethers';
export { Guid } from 'js-guid';

export { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
export { Option } from '@polkadot/types';
export { BN } from '@polkadot/util';
export { KeyringPair } from '@polkadot/keyring/types';
export type { Balance, DispatchError, DispatchResult } from '@polkadot/types/interfaces';
export { PalletCreditcoinAddress } from '@polkadot/types/lookup';
export type { EventRecord } from '@polkadot/types/interfaces/system';
export * as common from './extrinsics/common';

export const CREDO_PER_CTC = 1_000_000_000_000_000_000;
export const POINT_01_CTC = 0.01 * CREDO_PER_CTC;

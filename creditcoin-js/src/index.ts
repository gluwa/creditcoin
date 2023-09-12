export * from './creditcoin-api';
export * from './types';
export * from './model';

export { providers, Wallet, FixedNumber, BigNumber } from 'ethers';
export { parseUnits } from 'ethers/lib/utils';
export { Guid } from 'js-guid';

export { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
export { Option, Vec, Bytes } from '@polkadot/types';
export { BN } from '@polkadot/util';
export { KeyringPair } from '@polkadot/keyring/types';
export type { Balance, DispatchError, DispatchResult } from '@polkadot/types/interfaces';
export { PalletCreditcoinAddress } from '@polkadot/types/lookup';
export type { EventRecord } from '@polkadot/types/interfaces/system';
export * as common from './extrinsics/common';

import { CREDO_PER_CTC } from './ctc-deploy';
export { CREDO_PER_CTC };
export const POINT_01_CTC = 0.01 * CREDO_PER_CTC;

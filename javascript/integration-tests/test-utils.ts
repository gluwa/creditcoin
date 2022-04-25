// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Wallet } from 'ethers';
import { Guid } from 'js-guid';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { Null, Option } from '@polkadot/types';
import { PalletCreditcoinTransfer } from '@polkadot/types/lookup';
import { BN } from '@polkadot/util';

import { ethConnection } from 'credal-js/lib/examples/ethereum';
import {
    AddressId,
    AskOrderId,
    BidOrderId,
    Blockchain,
    DealOrderId,
    DealOrderAdded,
    DealOrderFunded,
    DealOrderLocked,
    LoanTerms,
    OfferId,
    Transfer,
    TransferId,
    TransferKind,
    TransferProcessed,
} from 'credal-js/lib/model';
import { createCreditcoinTransferKind } from 'credal-js/lib/transforms';
import { addAskOrderAsync, AskOrderAdded } from 'credal-js/lib/extrinsics/add-ask-order';
import { addAuthorityAsync } from 'credal-js/lib/extrinsics/add-authority';
import { addBidOrderAsync, BidOrderAdded } from 'credal-js/lib/extrinsics/add-bid-order';
import { addDealOrderAsync } from 'credal-js/lib/extrinsics/add-deal-order';
import { addOfferAsync, OfferAdded } from 'credal-js/lib/extrinsics/add-offer';
import { lockDealOrderAsync } from 'credal-js/lib/extrinsics/lock-deal-order';
import { registerAddressAsync, AddressRegistered } from 'credal-js/lib/extrinsics/register-address';
import { registerDealOrderAsync, DealOrderRegistered } from 'credal-js/lib/extrinsics/register-deal-order';
import {
    registerFundingTransferAsync,
    registerRepaymentTransferAsync,
    TransferEvent,
} from 'credal-js/lib/extrinsics/register-transfers';
import { fundDealOrderAsync } from 'credal-js/lib/extrinsics/fund-deal-order';

const ETHEREUM_ADDRESS = 'http://localhost:8545';
export const AUTHORITY_PUBKEY = '0xcce7c3c86f7e4431cdefca6c328bab69af12010a4a9fa0d91be37a24776afd4a';
export const AUTHORITY_SURI = 'blade city surround refuse fold spring trip enlist myself wild elevator coil';
export const AUTHORITY_ACCOUNTID = '5GhNUTKw9xkTN5Za4torEe1SAGPhXjM78oNZWAXrFymhB6oZ';

export const setupAuthority = async (api: ApiPromise, sudoSigner: KeyringPair) => {
    const u8aToHex = (bytes: Uint8Array): string => {
        return bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '0x');
    };
    const rpcUri = u8aToHex(api.createType('String', ETHEREUM_ADDRESS).toU8a());
    await api.rpc.offchain.localStorageSet('PERSISTENT', 'ethereum-rpc-uri', rpcUri);

    const hasAuthKey = await api.rpc.author.hasKey(AUTHORITY_PUBKEY, 'ctcs');
    if (hasAuthKey.isFalse) {
        await api.rpc.author.insertKey('ctcs', AUTHORITY_SURI, AUTHORITY_PUBKEY);
    }
    const auth = await api.query.creditcoin.authorities<Option<Null>>(AUTHORITY_ACCOUNTID);
    if (auth.isNone) {
        await addAuthorityAsync(api, AUTHORITY_ACCOUNTID, sudoSigner);
    }

    await api.tx.sudo
        .sudo(api.tx.balances.setBalance(AUTHORITY_ACCOUNTID, '10000000000000000000', '0'))
        .signAndSend(sudoSigner, { nonce: -1 });

    // bump balance for Alice b/c we need enough to be able to pay fees
    await api.tx.sudo
        .sudo(api.tx.balances.setBalance(sudoSigner.address, '10000000000000000000', '0'))
        .signAndSend(sudoSigner, { nonce: -1 });
};

export const expectNoDispatchError = (api: ApiPromise, dispatchError: any): void => {
    if (dispatchError) {
        if (dispatchError.isModule) {
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;

            expect(`${section}.${name}: ${docs.join(' ')}`).toBe('');
        } else {
            expect(dispatchError.toString()).toBe('');
        }
    }
};

export const registerAddress = async (
    api: ApiPromise,
    ethAddress: string,
    blockchain: Blockchain,
    signer: KeyringPair,
): Promise<AddressRegistered> => {
    const addr = await registerAddressAsync(api, ethAddress, blockchain, signer);
    expect(addr).toBeTruthy();

    if (addr) {
        expect(addr.addressId).toBeTruthy();
        return addr;
    } else {
        throw new Error("Address wasn't registered successfully");
    }
};

export const addAskOrder = async (
    api: ApiPromise,
    externalAddress: string,
    loanTerms: LoanTerms,
    expirationBlock: number,
    askGuid: Guid,
    signer: KeyringPair,
): Promise<AskOrderAdded> => {
    const result = await addAskOrderAsync(api, externalAddress, loanTerms, expirationBlock, askGuid, signer);
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('askOrder not found');
    }
};

export const addBidOrder = async (
    api: ApiPromise,
    externalAddress: string,
    loanTerms: LoanTerms,
    expirationBlock: number,
    bidGuid: Guid,
    signer: KeyringPair,
): Promise<BidOrderAdded> => {
    const result = await addBidOrderAsync(api, externalAddress, loanTerms, expirationBlock, bidGuid, signer);
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('bidOrder not found');
    }
};

export const addOffer = async (
    api: ApiPromise,
    askOrderId: AskOrderId,
    bidOrderId: BidOrderId,
    expirationBlock: number,
    signer: KeyringPair,
): Promise<OfferAdded> => {
    const result = await addOfferAsync(api, askOrderId, bidOrderId, expirationBlock, signer);
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('AddOffer failed');
    }
};

export const addDealOrder = async (
    api: ApiPromise,
    offerId: OfferId,
    expirationBlock: number,
    signer: KeyringPair,
): Promise<DealOrderAdded> => {
    const result = await addDealOrderAsync(api, offerId, expirationBlock, signer);
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('AddDealOrder failed');
    }
};

export const registerDealOrder = async (
    api: ApiPromise,
    lenderAddressId: AddressId,
    borrowerAddressId: AddressId,
    loanTerms: LoanTerms,
    expBlock: number,
    askGuid: Guid,
    bidGuid: Guid,
    borrowerKey: Uint8Array,
    signedParams: Uint8Array,
    signer: KeyringPair,
): Promise<DealOrderRegistered> => {
    const result = await registerDealOrderAsync(
        api,
        lenderAddressId,
        borrowerAddressId,
        loanTerms,
        expBlock,
        askGuid,
        bidGuid,
        borrowerKey,
        signedParams,
        signer,
    );
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('RegisterDealOrder failed');
    }
};

export const sleep = (delay: number) => new Promise((resolve) => setTimeout(resolve, delay));

export const prepareEthTransfer = async (
    lenderWallet: Wallet,
    borrowerWallet: Wallet,
    dealOrderId: DealOrderId,
    loanTerms: LoanTerms,
): Promise<[TransferKind, string]> => {
    // Note: this is Account #0 from gluwa/hardhat-dev !!!
    process.env.PK1 = '0xabf82ff96b463e9d82b83cb9bb450fe87e6166d4db6d7021d0c71d7e960d5abe';

    // connect to ethereum to lend and repay
    const { lend, repay, waitUntilTip } = await ethConnection();
    expect(repay).toBeTruthy();

    // Lender lends to borrower on ethereum
    const [tokenAddress, lendTxHash, lendBlockNumber] = await lend(
        lenderWallet,
        borrowerWallet.address,
        dealOrderId[1],
        loanTerms.amount,
    );

    // wait 15 blocks on Ethereum
    await waitUntilTip(lendBlockNumber + 15);

    const transferKind: TransferKind = { kind: 'Ethless', contractAddress: tokenAddress };
console.log(`**** DEBUG, ethTransfer tx_id=${lendTxHash}`);
    return [transferKind, lendTxHash];
};

export const registerFundingTransfer = async (
    api: ApiPromise,
    transferKind: TransferKind,
    dealOrderId: DealOrderId,
    txHash: string,
    signer: KeyringPair,
    waitForVerification = true,
): Promise<TransferEvent> => {
    const result = await registerFundingTransferAsync(api, transferKind, dealOrderId, txHash, signer);
    expect(result).toBeTruthy();

    if (result) {
        if (waitForVerification) {
            const verifiedTransfer = await result.waitForVerification().catch();
            expect(verifiedTransfer).toBeTruthy();
        }
console.log(`**** DEBUG, registerFundingTransfer transferId=${result.transferId}`);
        return result;
    } else {
        throw new Error('RegisterFundingTransfer failed');
    }
};

export const registerRepaymentTransfer = async (
    api: ApiPromise,
    transferKind: TransferKind,
    repaymentAmount: BN,
    dealOrderId: DealOrderId,
    txHash: string,
    signer: KeyringPair,
    waitForVerification = true,
): Promise<TransferEvent> => {
    const result = await registerRepaymentTransferAsync(
        api,
        transferKind,
        repaymentAmount,
        dealOrderId,
        txHash,
        signer,
    );
    expect(result).toBeTruthy();

    if (result) {
        if (waitForVerification) {
            const verifiedTransfer = await result.waitForVerification().catch();
            expect(verifiedTransfer).toBeTruthy();
        }
console.log(`**** DEBUG, registerRepaymentTransfer transferId=${result.transferId}`);
        return result;
    } else {
        throw new Error('RegisterRepaymentTransfer failed');
    }
};

export const fundDealOrder = async (
    api: ApiPromise,
    dealOrderId: DealOrderId,
    transferId: TransferId,
    signer: KeyringPair,
): Promise<[DealOrderFunded, TransferProcessed]> => {
    const result = await fundDealOrderAsync(api, dealOrderId, transferId, signer);
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('FundDealOrder failed');
    }
};

export const createCreditcoinTransfer = (api: ApiPromise, transfer: Transfer): PalletCreditcoinTransfer => {
    const toType = (): unknown => {
        return {
            blockchain: transfer.blockchain,
            kind: createCreditcoinTransferKind(api, transfer.kind),
            from: transfer.from,
            to: transfer.to,
            order_id: api.createType('PalletCreditcoinOrderId', { Deal: transfer.orderId }),
            amount: transfer.amount,
            tx_id: transfer.txHash,
            block: transfer.blockNumber,
            is_processed: transfer.processed,
            account_id: transfer.accountId,
            timestamp: transfer.timestamp,
        };
    };

    return api.createType('PalletCreditcoinTransfer', toType());
};

export const lockDealOrder = async (
    api: ApiPromise,
    dealOrderId: DealOrderId,
    signer: KeyringPair,
): Promise<DealOrderLocked> => {
    const result = await lockDealOrderAsync(api, dealOrderId, signer);
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('LockDealOrder failed');
    }
};

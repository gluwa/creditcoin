
import { ApiPromise, ApiRx, Keyring, WsProvider, } from '@polkadot/api';
import { PalletCreditcoinBlockchain, } from '@polkadot/types/lookup';
import { u8aConcat } from '@polkadot/util';
import { Guid } from 'js-guid';
import { handleTransaction, getAddressId } from '../utils';

export const registerDealOrder = async () => {
    const provider = new WsProvider("ws://localhost:9944");
    const api = await ApiPromise.create({ provider });

    const keyring = new Keyring({ type: 'sr25519' });
    const lender = keyring.addFromUri('//Alice', { name: 'Alice' });
    const borrower = keyring.addFromUri('//Bob', { name: 'Bob' });

    const lenderAddress = Math.random().toString(36);
    const borrowerAddress = Math.random().toString(36);
    const blockchain: PalletCreditcoinBlockchain = api.createType('PalletCreditcoinBlockchain', 'ethereum');
    const lenderId = getAddressId('Ethereum', lenderAddress);
    const borrowerId = getAddressId(blockchain, borrowerAddress);

    const registerLender: () => void = await api.tx.creditcoin
        .registerAddress(blockchain, lenderAddress)
        .signAndSend(lender, { nonce: -1 }, (result) => handleTransaction(api, registerLender, result));
    const registerBorrower: () => void = await api.tx.creditcoin
        .registerAddress(blockchain, borrowerAddress)
        .signAndSend(borrower, { nonce: -1 }, (result) => handleTransaction(api, registerBorrower, result));

    const askGuid = Guid.newGuid().toString();
    const bidGuid = Guid.newGuid().toString();
    const expBlock = 5;

    const loanTerms = api.createType(
        'PalletCreditcoinLoanTerms',
        { amount: 1_000, interestRate: 10, maturity: 10 });

    const bytesParams = u8aConcat(
        api.createType("u32", expBlock).toU8a(),
        api.createType("String", askGuid).toU8a(),
        api.createType("String", bidGuid).toU8a(),
        loanTerms.toU8a())

    const signedParams = borrower.sign(bytesParams);

    const registerDealOrder: () => void = await api.tx.creditcoin
        .registerDealOrder(
            lenderId,
            borrowerId,
            loanTerms,
            expBlock,
            askGuid,
            bidGuid,
            { Sr25519: borrower.publicKey },
            { Sr25519: signedParams })
        .signAndSend(lender, { nonce: -1 }, (result) =>
            handleTransaction(api, registerDealOrder, result));
}

import { ContractFactory, ethers, Signer, Wallet } from 'ethers';
import { TestToken } from './ethless/typechain';
import TestTokenArtifact from './ethless/contracts/TestToken.sol/TestToken.json';
import { JsonRpcProvider } from '@ethersproject/providers';
import { BN } from '@polkadot/util';

// Private key for Account #0: from gluwa/hardhat-dev (10000 ETH)
const MINTER = '0xabf82ff96b463e9d82b83cb9bb450fe87e6166d4db6d7021d0c71d7e960d5abe';

const signTransfer = async (
    domain: number,
    chainId: number,
    tokenAddress: string,
    from: Signer,
    to: string,
    amount: BN,
    fee: number,
    nonce: bigint,
) => {
    const fromAddress = await from.getAddress();
    const hash = ethers.utils.solidityKeccak256(
        ['uint8', 'uint256', 'address', 'address', 'address', 'uint256', 'uint256', 'uint256'],
        [domain, chainId, tokenAddress, fromAddress, to, amount.toString(), fee, nonce],
    );
    return from.signMessage(ethers.utils.arrayify(hash));
};

const deployTestToken = async (deployer: Signer) => {
    const factory = new ContractFactory(TestTokenArtifact.abi, TestTokenArtifact.bytecode, deployer);
    const testToken = (await factory.deploy()) as TestToken;
    return testToken;
};

const fundAccount = (token: TestToken, minter: Signer, recipient: string, amount: number) =>
    token
        .connect(minter)
        .mint(recipient, amount)
        .then((tx) => tx.wait());

const ethlessTransfer = async (
    signer: Signer,
    domain: number,
    token: TestToken,
    fromSigner: Signer,
    to: string,
    amount: BN,
    fee: number,
    nonce: bigint,
) => {
    const chainId = await token.chainID();
    const fromAddress = await fromSigner.getAddress();
    const signature = await signTransfer(domain, chainId.toNumber(), token.address, fromSigner, to, amount, fee, nonce);

    const receipt = await token
        .connect(signer)
        ['transfer(address,address,uint256,uint256,uint256,bytes)'](
            fromAddress,
            to,
            amount.toString(),
            fee,
            nonce.valueOf(),
            signature,
        )
        .then((tx) => tx.wait());
    return receipt;
};

export type EthConnection = {
    lend: (lender: Wallet, borrower: string, dealOrderId: string, amount: BN) => Promise<[string, string, number]>;
    repay: (borrower: Wallet, lender: string, dealOrderId: string, amount: BN) => Promise<[string, string, number]>;
    waitUntilTip: (tip: number) => Promise<void>;
    testTokenAddress: string;
};

export const ethConnection = async (
    providerRpcUrl = 'http://localhost:8545',
    decreaseMiningInterval = true,
    minterWallet?: Wallet,
): Promise<EthConnection> => {
    const provider = new JsonRpcProvider(providerRpcUrl);
    const minter = minterWallet || new Wallet(MINTER, provider);
    const testToken = await deployTestToken(minter);

    if (decreaseMiningInterval) {
        await provider.send('evm_setIntervalMining', [500]);
    }

    const sleep = (delay: number) => new Promise((resolve) => setTimeout(resolve, delay));

    const waitUntilTip = async (tip: number) => {
        while ((await provider.getBlockNumber()) <= tip) await sleep(1000);
    };

    const lend = async (
        lender: Wallet,
        borrower: string,
        dealOrderId: string,
        amount: BN,
    ): Promise<[string, string, number]> => {
        await fundAccount(testToken, minter, lender.address, 1_000_000);

        const nonce = BigInt(dealOrderId);

        const transferReceipt = await ethlessTransfer(minter, 3, testToken, lender, borrower, amount, 0, nonce);

        console.log(transferReceipt);
        return [testToken.address, transferReceipt.transactionHash, transferReceipt.blockNumber];
    };

    const repay = async (
        borrower: Wallet,
        lender: string,
        dealOrderId: string,
        amount: BN,
    ): Promise<[string, string, number]> => {
        // calls lend() with swapped arguments
        const result = await lend(borrower, lender, dealOrderId, amount);
        return result;
    };
    return { lend, repay, waitUntilTip, testTokenAddress: testToken.address };
};

import { ContractFactory, Signer, Wallet } from 'ethers';
import { GluwaCreditVestingToken } from './ethless/typechain';
import CtcArtifact from './ethless/contracts/GluwaCreditVestingToken.sol/GluwaCreditVestingToken.json';
import { JsonRpcProvider } from '@ethersproject/providers';
import { BN } from '@polkadot/util';

// Private key for Account #0: from gluwa/hardhat-dev (10000 ETH)
const MINTER = '0xabf82ff96b463e9d82b83cb9bb450fe87e6166d4db6d7021d0c71d7e960d5abe';

const deployTestToken = async (deployer: Signer) => {
    const factory = new ContractFactory(CtcArtifact.abi, CtcArtifact.bytecode, deployer);

    const deployerAddress = await deployer.getAddress();
    const testToken = (await factory.deploy(deployerAddress, deployerAddress)) as GluwaCreditVestingToken;
    return testToken;
};

const performTransfer = async (token: GluwaCreditVestingToken, fromSigner: Signer, to: string, amount: number) => {
    const tx = await token.connect(fromSigner).transfer(to, amount);
console.log('++++ DEBUG: amount=', amount);
console.log('**** DEBUG: tx=', tx);
    const receipt = await tx.wait();
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
    tstToken?: GluwaCreditVestingToken,
): Promise<EthConnection> => {
    const provider = new JsonRpcProvider(providerRpcUrl);
    const minter = minterWallet || new Wallet(MINTER, provider);
    const testToken = tstToken || (await deployTestToken(minter));

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
        if (!tstToken && !minterWallet) {
            // initialize balance only when we had deployed a test contract as part of this function
            // don't initialize if using pre-existing contract
            await performTransfer(testToken, minter, lender.address, 1_000_000);
        }

        const transferReceipt = await performTransfer(testToken, lender, borrower, amount.toNumber());

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

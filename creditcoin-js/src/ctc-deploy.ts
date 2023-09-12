import { ContractFactory, Signer, Wallet } from 'ethers';
import { GluwaCreditVestingToken } from './examples/ctc/typechain';
import CtcArtifact from './examples/ctc/contracts/GluwaCreditVestingToken.sol/GluwaCreditVestingToken.json';
import { JsonRpcProvider } from '@ethersproject/providers';

export const CREDO_PER_CTC = 1_000_000_000_000_000_000;

const deployCtcToken = async (deployer: Signer, existingAddress: string | undefined) => {
    const factory = new ContractFactory(CtcArtifact.abi, CtcArtifact.bytecode, deployer);
    let ctcToken: GluwaCreditVestingToken;

    if (existingAddress !== undefined) {
        ctcToken = factory.attach(existingAddress) as GluwaCreditVestingToken;
        console.log('Using existing contract', ctcToken.address);
    } else {
        const deployerAddress = await deployer.getAddress();
        ctcToken = (await factory.deploy(deployerAddress, deployerAddress)) as GluwaCreditVestingToken;
        console.log('Deployed to', ctcToken.address);
    }

    process.env.CREDITCOIN_CTC_CONTRACT_ADDRESS = ctcToken.address;

    return ctcToken;
};

const burnCtc = async (ctcToken: GluwaCreditVestingToken, howMuch: string) => {
    const tx = await ctcToken.burn(howMuch);
    const txHash = tx.hash;

    // wait for tx to be mined and get receipt
    await tx.wait();

    console.log('Burn Tx hash', txHash);
    process.env.CREDITCOIN_CTC_BURN_TX_HASH = txHash;
};

export const deployCtcContract = async (
    existingAddress: string | undefined,
    ethereumUrl: string | undefined,
    deployerPrivateKey: string,
    howMuchToBurn: string = (1 * CREDO_PER_CTC).toString(), // 1 CTC
) => {
    const provider = new JsonRpcProvider(ethereumUrl);
    const deployer = new Wallet(deployerPrivateKey, provider);
    const ctcToken = await deployCtcToken(deployer, existingAddress);

    await burnCtc(ctcToken, howMuchToBurn);
};

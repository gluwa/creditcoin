import { ContractFactory, Signer, Wallet } from 'ethers';
import { GluwaCreditVestingToken } from './examples/ctc/typechain';
import CtcArtifact from './examples/ctc/contracts/GluwaCreditVestingToken.sol/GluwaCreditVestingToken.json';
import { JsonRpcProvider } from '@ethersproject/providers';
import GATEArtifact from './examples/ctc/contracts/GluwaGateToken.sol/GluwaGateToken.json';

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

export const deployGATEToken = async (deployer: Signer, existingAddress: string | undefined): Promise<any> => {
    const factory = new ContractFactory(GATEArtifact.abi, GATEArtifact.bytecode, deployer);
    let gateToken: any;

    if (existingAddress !== undefined) {
        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
        gateToken = factory.attach(existingAddress);
        // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
        console.log('Using existing contract', gateToken.address);
    } else {
        // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
        gateToken = await factory.deploy();
        // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
        console.log('Deployed GATE Token to', gateToken.address);
    }

    // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-assignment
    process.env.CREDITCOIN_GATE_CONTRACT_ADDRESS = gateToken.address;

    // eslint-disable-next-line @typescript-eslint/no-unsafe-return
    return gateToken;
};

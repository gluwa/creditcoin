import { ContractFactory, Signer, Wallet } from 'ethers';
import { GluwaCreditVestingToken } from './ethereum/ctc/typechain';
import CtcArtifact from './ethereum/ctc/contracts/GluwaCreditVestingToken.sol/GluwaCreditVestingToken.json';
import { JsonRpcProvider } from '@ethersproject/providers';

import GATEArtifact from './ethereum/ctc/contracts/GluwaGateToken.sol/GluwaGateToken.json';
import { BigNumber } from 'creditcoin-js';

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

export const deployGATEToken = async (deployer: Signer, existingAddress: string | undefined) => {
    const factory = new ContractFactory(GATEArtifact.abi, GATEArtifact.bytecode, deployer);
    let gateToken: any;

    if (existingAddress !== undefined) {
        gateToken = factory.attach(existingAddress);
        console.log('Using existing contract', gateToken.address);
    } else {
        gateToken = await factory.deploy();
        console.log('Deployed GATE Token to', gateToken.address);
    }

    process.env.CREDITCOIN_GATE_CONTRACT_ADDRESS = gateToken.address;

    return gateToken;
};
const burnCtc = async (ctcToken: GluwaCreditVestingToken) => {
    // Burn 1 Credo == 10^-18 CTC
    const tx = await ctcToken.burn(1);
    const txHash = tx.hash;

    // wait for tx to be mined and get receipt
    await tx.wait();

    console.log('Burn Tx hash', txHash);
    process.env.CREDITCOIN_CTC_BURN_TX_HASH = txHash;
};

export const main = async (existingAddress: string | undefined) => {
    const provider = new JsonRpcProvider((global as any).CREDITCOIN_ETHEREUM_NODE_URL);
    const deployer = new Wallet((global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY, provider);
    const ctcToken = await deployCtcToken(deployer, existingAddress);

    await burnCtc(ctcToken);
};

if (require.main === module) {
    main(undefined).catch(console.error);
}

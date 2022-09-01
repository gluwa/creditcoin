import { ContractFactory, Signer, Wallet } from 'ethers';
import { GluwaCreditVestingToken } from './ethereum/ctc/typechain';
import CtcArtifact from './ethereum/ctc/contracts/GluwaCreditVestingToken.sol/GluwaCreditVestingToken.json';
import { JsonRpcProvider } from '@ethersproject/providers';

// Private key for Account #0: from gluwa/hardhat-dev (10000 ETH)
export const DEPLOYER_PRIVATE_KEY = '0xabf82ff96b463e9d82b83cb9bb450fe87e6166d4db6d7021d0c71d7e960d5abe';

const deployCtcToken = async (deployer: Signer) => {
    const factory = new ContractFactory(CtcArtifact.abi, CtcArtifact.bytecode, deployer);
    const deployerAddress = await deployer.getAddress();
    const ctcToken = (await factory.deploy(deployerAddress, deployerAddress)) as GluwaCreditVestingToken;
    return ctcToken;
};

export const main = async () => {
    const provider = new JsonRpcProvider('http://localhost:8545');
    const deployer = new Wallet(DEPLOYER_PRIVATE_KEY, provider);
    const ctcToken = await deployCtcToken(deployer);

    const tx = await ctcToken.burn(500);
    const txHash = tx.hash;
    // wait for tx to be mined and get receipt
    await tx.wait();

    console.log('Deployed to', ctcToken.address);
    console.log('Burn Tx hash', txHash);

    process.env.CREDITCOIN_CTC_CONTRACT_ADDRESS = ctcToken.address;
    process.env.CREDITCOIN_CTC_BURN_TX_HASH = txHash;
};

if (require.main === module) {
    main().catch(console.error);
}

import { ContractFactory, ethers, Signer, Wallet } from 'ethers';
import { GluwaCreditVestingToken } from './ethereum/ctc/typechain';
import CtcArtifact from './ethereum/ctc/contracts/GluwaCreditVestingToken.sol/GluwaCreditVestingToken.json';
import { JsonRpcProvider } from '@ethersproject/providers';

// Private key for Account #0: from gluwa/hardhat-dev (10000 ETH)
const MINTER = '0xabf82ff96b463e9d82b83cb9bb450fe87e6166d4db6d7021d0c71d7e960d5abe';

const deployTestToken = async (deployer: Signer) => {
    const factory = new ContractFactory(CtcArtifact.abi, CtcArtifact.bytecode, deployer);
    const deployerAddress = await deployer.getAddress();
    const ctcToken = (await factory.deploy(deployerAddress, deployerAddress)) as GluwaCreditVestingToken;
    return ctcToken;
};

const main = async () => {
    const provider = new JsonRpcProvider('http://localhost:8545');
    const minter = new Wallet(MINTER, provider);
    const testToken = await deployTestToken(minter);
    console.log('Deployed to', testToken.address);
};

if (require.main === module) {
    main().catch(console.error);
}

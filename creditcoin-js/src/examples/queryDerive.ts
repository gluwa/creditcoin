import { creditcoinApi } from 'src';

export const queryExample = async () => {
    // Connect to a local node
    const { api } = await creditcoinApi('ws://localhost:9944');

    // The derives are just helpers that define certain functions
    // and combine results from multiple sources

    // Query staking information using derive
    const stakingInfo = await api.derive.staking.account('5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY');

    console.log('accountId:', stakingInfo.accountId.toString());
    console.log('nominators:', stakingInfo.nominators.toString());
    console.log('rewardDestination:', stakingInfo.rewardDestination.toString());
    console.log('stakingLedger:', stakingInfo.stakingLedger.toString());

    // Disconnect from the local node
    await api.disconnect();
};

if (require.main === module) {
    queryExample().catch(console.error);
}

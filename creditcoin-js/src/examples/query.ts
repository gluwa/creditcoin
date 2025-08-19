import { creditcoinApi } from 'src';

export const queryExample = async () => {
    // Connect to a local node
    const { api } = await creditcoinApi('ws://localhost:9944');

    // The query method is followed by the name of the module,
    // then the name of the getter function

    // Query the current block number
    const blockNumber = await api.query.system.number();
    console.log('blockNumber:', blockNumber.toString());

    // Query the current block hash
    const blockHash = await api.query.system.blockHash(blockNumber);
    console.log('blockHash:', blockHash.toString());

    // Query the epoch
    const currentSession = await api.query.session.currentIndex();
    console.log('currentSession:', currentSession.toString());

    // Disconnect from the local node
    await api.disconnect();
};

if (require.main === module) {
    queryExample().catch(console.error);
}

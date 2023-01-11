# Creditcoin Pallet

The creditcoin pallet contains the logic for the loan flow (as outlined in the creditcoin whitepaper). That means
it defines all of the data structures used in the process of recording loans, it also maintains the storage
of loans and their associated data, and provides extrinsics to interact with the loan cycle. In general, each operation
in the loan cycle maps to an extrinsic, for instance registering an external address corresponds with the `register_address`
extrinsic.

For the most part the business logic is pretty straightforward, and isn't changed often since it's meant to adhere to the
specification of the white paper (mostly), so I won't talk too much about it. The primary exception to this is
our interactions with external blockchains.

## Verifying External Transactions

As part of the loan cycle, users can report a transfer of funds that occurred on a different blockchain. For instance,
Alice and Bob are in a loan where Bob leant Alice 100 ETH (Bob is a high-roller). Upon initiating the loan, Bob would
send the funds to Alice on ethereum, and then report the transfer on creditcoin to indicate he had funded the loan.
Instead of just trusting that Bob really did send 100 ETH through ethereum, we want to verify that is the case.
The idea is simple - call some ethereum RPCs to pull information about the transaction and verify that it matches
what Bob claimed. That RPC call, though, requires some extra work.

### Offchain Interactions

Since an RPC call requires network access, it is inherently non-deterministic. You might perform the call and get one result,
and someone else might perform the same exact call to the same exact RPC node only to receive a different result (due to transient network
errors, solar flares, bad luck, whatever else). This is bad because if we can't repeat the exact execution, it would be impossible to
replicate and verify which defeats the purpose of blockchains! Because of this, substrate disallows non-determinism in code
executing in an on-chain context. That means that the non-deterministic work needs to occur off-chain. To facilitate this pattern,
substrate provides "offchain workers." Basically it's just a special function (a runtime hook, to be precise) defined by a pallet that runs on each block, and it
runs in an off-chain context. This means that code running under offchain workers can access, for instance, the network.

Offchain workers can freely read from on-chain storage, but they cannot write to on-chain storage. This means that the only way for
the offchain worker to communicate back to the chain is by sending transactions. The general flow for an external transfer is then

1. Transfer registered
2. Entry put into storage indicating there's a new task to be executed
3. Offchain worker starts
4. Offchain worker looks at tasks, picks one up
5. Transfer verification occurs
6. Offchain worker sends transaction indicating the verification succeeded (or failed)

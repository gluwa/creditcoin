# Runtime

## What is a runtime?

At a high level, the runtime is the "business logic" of the chain.
More precisely, the runtime primarily acts as the state transition function for the blockchain. Roughly you can think of each block
as containing a set of operations to perform (transactions) and a snapshot of the state of the world after those operations
have been carried out. Then, to link a series of blocks together each block has a pointer to its predecessor.
The runtime, then, is what dictates how to actually execute those operations and how the state of the world is
modified in the process. You may also see the runtime and its components referred to as the on-chain logic. On-chain in this context means
an execution environment that can modify the state of the chain, and all modifications to state will be tracked and recorded.

## How is the runtime organized?

Substrate (which creditcoin builds on) uses the FRAME framework to define independent modules (called _pallets_)
which can be composed to build up the runtime. So, for instance, you might have a [pallet](https://github.com/paritytech/substrate/tree/master/frame/balances)
which maintains balances for a set of accounts and provides operations on those balances (transfer funds, deposit funds, withdrawals, etc.).

The `creditcoin-node-runtime` crate (found in the `runtime` directory) is where we glue together all of the pieces of on-chain logic, and
basically consists of configuring all of the pallets we use and incorporating them into the runtime.

For each pallet, you'll probably have a `Config` trait which allows pallets to be generic over certain types
that can be configured at the runtime level. For instance, here is where we decide that block numbers are
represented by a `u32` and balances by a `u128`. For each pallet the runtime has to implement the `Config` trait.

Some pallets also define runtime APIs. For the most part you can ignore these, but for context runtime APIs are essentially an interface
for the outer node/client to call into the runtime. So, for instance, if you wanted to expose an RPC that needed some information from the runtime,
you would define a runtime API, implement the logic in the runtime, and then call your runtime API from the RPC handler.

Pretty much the entirety of the runtime is in [`lib.rs`](https://github.com/gluwa/creditcoin/tree/dev/runtime/src/lib.rs). If you need to increase the runtime version
(required after non-backwards-compatible changes to the runtime, i.e. any consensus-affecting change) that's in [`version.rs`](https://github.com/gluwa/creditcoin/tree/dev/runtime/src/version.rs)

## What pallets do we use?

A bunch.

### External Pallets

These pallets are all part of substrate and aren't maintained by the creditcoin developers:

- [Balances](https://paritytech.github.io/polkadot-sdk/master/pallet_balances/index.html)
- [FRAME System](https://paritytech.github.io/polkadot-sdk/master/frame_system/index.html)
- [Scheduler](https://paritytech.github.io/polkadot-sdk/master/pallet_scheduler/index.html)
- [Sudo](https://paritytech.github.io/polkadot-sdk/master/pallet_sudo/index.html)
- [Timestamp](https://paritytech.github.io/polkadot-sdk/master/pallet_timestamp/index.html)
- [Transaction Payment](https://paritytech.github.io/polkadot-sdk/master/pallet_transaction_payment/index.html)

### Internal Pallets

These pallets are written and maintained by the creditcoin developers:

- [Creditcoin](./pallet-creditcoin.md)
- [Difficulty](pallet-difficulty.md)
- [Rewards](https://github.com/gluwa/creditcoin/tree/dev/pallets/rewards)
- [Offchain Task Scheduler](https://github.com/gluwa/creditcoin/tree/dev/pallets/offchain-task-scheduler)

The majority of the creditcoin developers' work is spent developing these pallets (and probably more in the future).

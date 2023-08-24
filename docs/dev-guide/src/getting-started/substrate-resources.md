# Learning about substrate

The Creditcoin blockchain is built on the [substrate framework](https://docs.substrate.io/), which provides most of the underlying
blockchain functionality (P2P networking, block production, RPC server, storage, etc.). This allows us
to focus on the functionality specific to creditcoin and additionally we benefit from
existing tooling developed for the polkadot/substrate ecosystem (such as the polkadot explorer, polkadotJS, telemetry, etc.).

## Helpful resources

The official substrate documentation provides a good starting point, I would recommend (at a minimum) reading through
all of the material in the [fundamentals section](https://docs.substrate.io/fundamentals/).

That should give you a rough understanding of substrate's architecture, and how the pieces fit together.

For learning about FRAME and best practices, the [substrate repository](https://github.com/paritytech/substrate) has a bunch of pallets of varying complexity that serve as good reference points.
For starters, the [sudo pallet](https://github.com/paritytech/substrate/tree/polkadot-v0.9.32/frame/sudo) is fairly small and
digestible.

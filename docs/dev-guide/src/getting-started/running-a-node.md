# Running a Creditcoin node

## Running a development node

Now that you've built a `creditcoin-node` from source, you can get a minimal development node running with:

```bash
./target/release/creditcoin-node --dev --mining-key 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --mining-threads 1
```

The node should start running and produce output similar to below:

```plaintext
2023-01-05 19:54:53 Creditcoin Node
2023-01-05 19:54:53 ✌️  version 2.211.2-e972d650ee6
2023-01-05 19:54:53 ❤️  by Gluwa Inc.:Nathan Whitaker <nathan.whitaker@gluwa.com>, 2017-2023
2023-01-05 19:54:53 📋 Chain specification: Development
2023-01-05 19:54:53 🏷  Node name: cute-geese-8080
2023-01-05 19:54:53 👤 Role: AUTHORITY
2023-01-05 19:54:53 💾 Database: RocksDb at /var/folders/jw/4ykz4cmj7q7fkjp9t6pv6z7h0000gn/T/substrateLol6Jy/chains/dev/db/full
2023-01-05 19:54:53 ⛓  Native runtime: creditcoin-node-212 (creditcoin-node-0.tx10.au1)
2023-01-05 19:54:53 🔨 Initializing Genesis block/state (state: 0x652a…44ab, header-hash: 0x6dd1…2b4e)
2023-01-05 19:54:53 Using default protocol ID "sup" because none is configured in the chain specs
2023-01-05 19:54:53 🏷  Local node identity is: 12D3KooWCMzU5LdWErgqjLZxVSwveDjGrRQ7q4zcuycjHocPCNDs
2023-01-05 19:54:53 💻 Operating system: macos
2023-01-05 19:54:53 💻 CPU architecture: aarch64
2023-01-05 19:54:53 📦 Highest known block at #0
2023-01-05 19:54:53 Running JSON-RPC HTTP server: addr=127.0.0.1:9933, allowed origins=None
2023-01-05 19:54:53 Running JSON-RPC WS server: addr=127.0.0.1:9944, allowed origins=None
2023-01-05 19:54:53 〽️ Prometheus exporter started at 127.0.0.1:9615
2023-01-05 19:54:58 💤 Idle (0 peers), best: #0 (0x6dd1…2b4e), finalized #0 (0x6dd1…2b4e), ⬇ 0 ⬆ 0
2023-01-05 19:55:03 💤 Idle (0 peers), best: #0 (0x6dd1…2b4e), finalized #0 (0x6dd1…2b4e), ⬇ 0 ⬆ 0
2023-01-05 19:55:03 🙌 Starting consensus session on top of parent 0x6dd1a66ff1b0b6482f8da72b829420f10eafa99a6fda25c9f8992fa381d92b4e
2023-01-05 19:55:03 🎁 Prepared block for proposing at 1 (0 ms) [hash: 0x1293ab00b882c45fc6ebf312992cc127e596d1350b3d2202f3d48dd64ac7d88b; parent_hash: 0x6dd1…2b4e; extrinsics (1): [0x93be…9e86]]
2023-01-05 19:55:04 🙌 Starting consensus session on top of parent 0x1029bb84cb03783d8b927e8b98b48f65b04f0afdb06abc08f2428503b5078572
2023-01-05 19:55:04 ✅ Successfully mined block on top of: 0x6dd1…2b4e
2023-01-05 19:55:04 ✨ Imported #1 (0x1029…8572)
2023-01-05 19:55:04 🎁 Prepared block for proposing at 2 (0 ms) [hash: 0x07fbf7a2b55b414e4e208a0b75e25735a0855455660667c2834bfcb2e7a2d74f; parent_hash: 0x1029…8572; extrinsics (1): [0x51fb…557f]]
```

By default this is a temporary chain, so when you stop your development node the chain will be wiped out. If you want a local development
chain that is persistent, you can use the `local` chain specification:

```bash
./target/release/creditcoin-node --chain local --validator --mining-key 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --mining-threads 2
```

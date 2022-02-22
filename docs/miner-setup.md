# Creditcoin Mining Node Setup

## Prerequisites

- Working [Docker](https://www.docker.com) installation

## Setup Steps

1) In order to receive mining rewards you will need an account on the Creditcoin network. Each account has an address and a balance associated with it. The account is backed by a keypair. You can use an existing ECDSA keypair (e.g. from pre-Creditcoin 2.0) or you can generate a new keypair. You can use [subkey](https://docs.substrate.io/v3/tools/subkey/) to retrieve the account address from an existing private key (e.g. from pre-Creditcoin 2.0) or to generate a new keypair.

    - Using an existing ECDSA keypair:
        - Your private key should be formatted as hex and start with `0x`, for example `0x3351b11eca7b5c78c0f55c681d9a2e8a0630bcc7a95a35a4a87615c916771774`
        - Run `docker run -it docker.io/parity/subkey inspect --scheme Ecdsa <private key>` which will display the account information fot the keypair. For example:
            ```
            Secret Key URI `0x3351b11eca7b5c78c0f55c681d9a2e8a0630bcc7a95a35a4a87615c916771774` is account:
            Secret seed:       0x3351b11eca7b5c78c0f55c681d9a2e8a0630bcc7a95a35a4a87615c916771774
            Public key (hex):  0x02abf7befd96f80ce3a27772e7903f45a930c54ede2f0b9e052bfb21e90e0a4b40
            Account ID:        0xe37a568057962e95990cbba46c68f8d5b0d0d614abc8bc9f4e46af3e7aa8880c
            Public key (SS58): KW6p8XTkd6pLhTnwfSfr3hUcVSKTQhJHZxTVD8RrpfUhUTrvn
            SS58 Address:      5HCy4x9b5mW28EYheGn14bWidQkhab5VMiNakia7i4VfxTKs 
            ```
        - Copy the `SS58 Address` for later use
    - Generate a new keypair
        - Run `docker run -it docker.io/parity/subkey:latest generate --scheme Ecdsa`. This will generate a new keypair and print the account information, for example:
            ```
            Secret phrase:       toss frown run relief book lift aunt guard reduce shell genuine alarm
            Network ID:        substrate
            Secret seed:       0x5ad92bddf82eae47f5c9cc77a749fd175d9d80aadeab6555e3126a087f5eb5f1
            Public key (hex):  0x03084078b5d3633f53ceb103199332aaf86e7ebc1b2975e697dd5dc8653692b7b9
            Account ID:        0x7bbf1daa8ccb9aedccade233879f299a5485fbd0922d9458b19a5dbfde71da3c
            Public key (SS58): KW8u8Y1GgAGWtTfU5o92imPsYVkowfbsKE7hosQHwJ2E7gF9h
            SS58 Address:      5ErxX8PgVYVE3WbCkSs9mvioFHVrsc4uXFwkF3G9Pyv4FC2w
            ```
        - Store your secret phrase (a 12-word mnemonic) in a secure location. We won’t use this phrase directly, but you’ll need it to access the account or recover your private key.
        - Copy the `SS58 Address` for later use
2) Start mining node
    - Make make sure that your port 30333 is accessible by external connections
    - Obtain your public IP address
    - Run 
        ```
        docker run -p 30333:30333 -e FQDN=<publicip> -e BOOTNODE_FQDN='cct-bootnode.centralus.azurecontainer.io' -e MINING_KEY=<SS58address> <TODO: PUBLIC DOCKER IMAGE>`
        ```
    - By default all data will be stored in `/data` within the docker container.  If you’d like to persist this data or access it from your host system you can mount a docker volume at `/data` by passing `-v` `<yourlocalpath>:/data` in the command above
    - You may optionally give your node a name so it’s more easily identifiable in telemetry by passing `-e NODE_NAME=<name>` in the command above.
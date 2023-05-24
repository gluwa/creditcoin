# Creditcoin CLI Tool

## Building

This tool depends on `creditcoin-js`. Make sure to pack the library using `yarn pack` in the `creditcoin-js` folder before building. It should be available as `creditcoin-js-vX.Y.Z.tgz`.

Build using yarn.

```
yarn install
yarn build
```

Install globally using npm.

```
npm install -g .
```

Or use with node from the project directory.

```
node dist/index.js
```

## Commands

- **new-seed**: Create a new seed and print it or save it to a file.
- **receive**: Show address for a particular account.
- **balance**: Check the balance of an account.
- **bond**: Bond CTC tokens using a stash account.
- **rotate-keys**: Rotates the node keys used for validating.
- **set-keys**: Set new keys for the controller account.
- **validate**: Signal intention to start validating.
- **chill**: Signal validator to _chill_ and stop producing blocks.
- **wizard**: Run the validator setup wizard.

To view all commands run the tool with the `--help` flag.

## Examples

### Running from the Creditcoin Docker container

From the root of the Creditcoin repository, build and run the image.

```
docker build -t creditcoin-node .
docker run --rm --name creditcoin creditcoin-node --chain test
```

Execute the CLI tool with the `exec` Docker command like so:

```
docker exec creditcoin creditcoin-cli --help
```

### Create a new seed

```
creditcoin-cli new
```

### Create a new seed and save it to a file

```
creditcoin-cli new --save seed.txt
```

### Show address for a particular account

```
creditcoin-cli receive -f seed.txt
```

### Check the balance of an account

```
creditcoin-cli balance -f seed.txt
```

### Bond CTC tokens using a stash account

```
creditcoin-cli bond -f seed.txt --amount 1000 --controller 5DJ8qkxAbSVfyvorNBKt4BwDR9hUUzH8aqofuTAMTkLZtpv9
```

### Rotates of a particular node

```
creditcoin-cli rotate-keys -u http://localhost:8000
```

### Run the validator setup wizard

This example asumes seeds are saved in `stashseed` and `controllerseed` files and a node is running on `ws://localhost:9944`.

```
creditcoin-cli wizard -sf stashseed -cf controllerseed -a 1000 -u ws://localhost:9944
```

⚠️🔧 Warning: This tool is currently under development! 🔧⚠️

Please be aware that the command line tool you are using is still in active development. It may contain bugs, incomplete features, or unexpected behavior. Exercise caution and use it at your own risk. Feedback and bug reports are greatly appreciated to help improve the tool and ensure its stability.

# Creditcoin zombienet

This is a template for using the [zombienet](https://github.com/paritytech/zombienet) tool for spawning and testing
ephemeral networks. Zombienet will launch a local blockchain instance with multiple nodes and parameters
specified in a text file. You can then use this local blockchain to run experiments
against it or use the Zombienet DSL language to assert against the spawned network!

## Prerequisites

- Docker
- Kubectl: [docs](https://kubernetes.io/releases/download/#kubectl)
- Kind: [docs](https://kind.sigs.k8s.io/docs/user/quick-start/#installation)
- Zombienet binary: [releases](https://github.com/paritytech/zombienet/releases). Download the appropriate binary and put it somewhere accessible.
- (optional) Caddy: [install](https://caddyserver.com/docs/install)

Build `creditcoin-node` using `--features "fast-runtime zombienet"`!
That affects block time and epoch duration!

## Hardware requirements

Launching multiple Creditcoin nodes on a single machine would put a strain on it.
Mostly it needs lots of memory in order to fit all of the separate creditcoin-node
processes. It also needs CPU because block finalization is computationally intensive.

A small network, e.g. 5 validators should spawn comfortably within 16 GiB of RAM,
see `network.yaml`.
Experiments have shown that launching 200 validator nodes natively requires around
64 GiB of RAM and leaves very little memory to spare. Realistic experiments have been
executed on machines with 128 GiB of RAM. For example a `D32s_v3` VM in Azure will
utilize most of its memory and load all of its 32 CPU cores to nearly 100% in a 200
validator scenario.

## Architecture

Zombinet will launch nodes in accordance with the configuration passed to it.
All of them will be listening on localhost and their port numbers will be assigned randomly.
Informnation about spawned nodes is printed on the terminal.

For more information check-out the
[upstream documentation](https://paritytech.github.io/zombienet/). See
[Chapter 5. Guide (examples)](https://paritytech.github.io/zombienet/guide.html) for
practical examples.

### How to use

The folowing is intended as a quick-start guide.
For more detailed instructions, take a look at the [zombienet docs](https://paritytech.github.io/zombienet/intro.html).

There is a base network specification ([docs](https://paritytech.github.io/zombienet/network-definition-spec.html)) here to start off with, you can edit it as needed. To spawn a network:

1. Make sure you have a kind cluster created

   You can start one by running

   ```bash
   ./setup-kind-cluster.sh
   ```

   This will create a new cluster, and also start up and configure a local
   image repository for you to push development images to.

   Alternatively, if you don't care about using development images (i.e. you
   only want the nodes to run publicly pushed images) then you can just skip the image repository and start only the cluster with

   ```bash
   kind cluster create
   ```

2. Spawn the network

   Assuming the zombienet binary is in the current directory, you can
   spawn a network with

   ```bash
   ./zombienet --dir /var/tmp/zombinet --spawn-concurrency <N-CPUs> spawn network.yaml
   ```

   This will start up a new network, and will print out all of the info
   you need to connect/interact with the network.

   _(Optional) Run tests_

   You can also specify tests in a DSL [docs](https://paritytech.github.io/zombienet/cli/test-dsl-definition-spec.html), and then run it against
   an ephemeral network. There is a simple example test included here.

   You can run a test with

   ```bash
   ./zombienet --dir /var/tmp/zombienet --spawn-concurrency <N-CPUs> test ./tests/test.zndsl
   ```

   This will bring up the network, run the specified test, then
   tear down the network.

3. Using a local/development image

   For development, you often want to use a local image e.g. one
   built on a development branch. Normally docker will be able to find
   and use local images, but our kind cluster can't see our local images
   that aren't pushed to a repository.

   To get this working, we run a local image repository, configure our kind
   cluster to use it, (these first two steps are performed automatically
   by `setup-kind-cluster.sh`) and then push the image to the local image repository.

   By default, the setup script will make the image repository available at
   `localhost:5001`. So to push a local image (call it `creditcoin-dev:dev`) just
   tag and push it:

   ```bash
   docker tag creditcoin-dev:dev localhost:5001/creditcoin-dev:dev
   docker push localhost:5001/creditcoin-dev:dev
   ```

   Then configure the network specification to use that image
   (there is a commented out line in the included `network.yaml` that does this).

### Using native provider on a VM

This is an easier way of using Zombienet without having to setup a Kubernetes cluster.
Just execute the command:

```bash
zombienet spawn tests/0001-load-test-with-200-validators.yaml
```

and observe the terminal output for the names, URLs and log locations of the launched nodes.
It may be useful to utilize the `--dir /path/to/directory` command line option if you want
log filenames to be more deterministic.


#### Reverse proxy internal ports to the outside world

In case you are using the native provider (on a VM) you will have to reverse proxy the
WebSockets RPC because Polakdot JS Apps will refuse plain/text connections over the Internet.

Select an arbitrary node and search in its log file, e.g. `first.log` lines similar to

```text
2023-09-13 16:31:40 Running JSON-RPC HTTP server: addr=0.0.0.0:32943, allowed origins=["*"]
2023-09-13 16:31:40 Running JSON-RPC WS server: addr=0.0.0.0:37271, allowed origins=["*"]
```
- Edit the port mappings [and domain name] in `Caddyfile.rpc-proxy`
- Execute `caddy`

  ```bash
  sudo caddy run --config Caddyfile.rpc-proxy
  ```
- (Possibly) adjust DNS configuration for the domain name used to access your VM. We've got
  `zombienet.creditcoin.network` operated by DevOps and pointing to one of our test VMs
- Go to <https://polkadot.js.org/apps/?rpc=wss://zombienet.creditcoin.network:8443#/explorer>
  and explore the newly spawned chain!

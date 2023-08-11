# Creditcoin zombienet

This is a template for using the [zombienet](https://github.com/paritytech/zombienet) tool for spawning and testing
ephemeral networks.

## Prerequisites

- Docker
- Kubectl: [docs](https://kubernetes.io/releases/download/#kubectl)
- Kind: [docs](https://kind.sigs.k8s.io/docs/user/quick-start/#installation)
- Zombienet binary: [releases](https://github.com/paritytech/zombienet/releases). Download the appropriate binary and put it somewhere accessible.
- (optional) Caddy: [install](https://caddyserver.com/docs/install)

Build `creditcoin-node` using `--features "fast-runtime zombienet`!
That affects block time and epoch duration!

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

In case you are using the native provider (on a VM) you will have to reverse proxy the
WebSockets RPC because Polakdot JS Apps will refuse plain/text connections outside of
`127.0.0.1`.

- (Possibly) adjust DNS configuration for the domain name used to access your VM
- Edit the port mappings [and domain name] in `Caddyfile.rpc-proxy`
- Execute `caddy`

    ```bash
    sudo caddy run --config Caddyfile.rpc-proxy
    ```

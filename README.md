# CosmWasm Substreams Example Project

## Overview
This is an substream for reading cosmwasm messages from a cosmwasm-enabled blockchain in the Cosmos ecosystem.

## Firehose configuration

In order to read cosmwasm blocks we need a firehose configured to emit substreams.

### Compilation


Install Go
### Install Rust
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` to install

### Protobuf Compiler `protoc`(https://grpc.io/docs/protoc-installation/#install-using-a-package-manager)


Put `$GOPATH/bin` on your `$PATH`. If you have homebrew and might have installed substreams, make sure `$GOPATH/bin` precedes the installed version. Commands below assume you're running the firehose and substreams executables that you have built locally.

```
git clone https://github.com/streamingfast/firehose-cosmos.git
cd firehose-cosmos
git checkout feature/substreams
```

Then build with `make && make install`

Now that you have a build of firehose built with substream support, you'll want to download some cosmos testnet blocks locally so firehose can serve them to the substream:

```
# Local directory for downloaded firehose blocks
export BLOCKS_DIR="./cosmos-blocks"
# Your firehose API token
export FIREHOSE_API_TOKEN="1751e03cd2034d4b18f9365661875b49"
# The firehose API GRPC endpoint for cosmos juno
export COSMOS_GRPC_ENDPOINT="firehose--cosmoshub-4--testnet.grpc.datahub.figment.io:443"

# Download blocks
export START_BLOCK=9034670
export END_BLOCK=9100000
firehose-cosmos tools download-from-firehose $COSMOS_GRPC_ENDPOINT  --common-first-streamable-block=$START_BLOCK $END_BLOCK $BLOCKS_DIR

# Serve blocks for substreams
# Node that this bypasses everything but already downloaded blocks
firehose-cosmos start firehose --common-first-streamable-block=$START_BLOCK --common-blockstream-addr= --common-blocks-store-url=$BLOCKS_DIR --substreams-enabled`
```

## Building substreams

As of 6/4/2022, substreams needs a custom build instead of the shipping releases. To do this you'll need the wasmer (wasmer.io) library installed locally.

`curl https://get.wasmer.io/ -sSfL | sh`

Once wasmer is installed you can build the substreams binary from source.

### Compilation

To build the cosmwasm substream reader in this example, run the script:

```
./build.sh
```
 
## Running your substreams code
 
`substreams run -p -e 127.0.0.1:9030 substream.yaml map_hello_world -s 9100300 -t +10`

At this point, the `map_hello_world` entry point will be called with with firehose blocks starting at `9100300` (the cosmos hub testned in this example).


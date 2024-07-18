# Nullifier Counter in RISC Zero

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/MIT)

This repository is a module for Identity Provider in [FreedomTool by Rarimo](https://freedomtool.org/).

Particularly, it contains necessary functionality to construct ZKP for counting duplicates of nullifiers
for some document (passport) hash.

Reference implementation is [nullifiersCounter](https://github.com/rarimo/passport-zk-circuits/tree/main/nullifiersCounter) in Circom.

## TL;DR in Risc0

`Risc0` is a technology allowing to construct ZKP logic in arbitrary language, which can get compiled down to
`RISC-V` architecture. Currently, flagship support is for `Rust`.

### Guest

In a `methods/guest` directory is located rust code, whose trace of execution is constructing ZKP.
It is compiled to `RISC-V ELF` binary, which is then getting run by zkVM.

### Host

The entity, responsible for running zkVM and handling I/O with guest, is known as `host`.

In this repository is present necessary functionality for running the host locally, meaning private inputs
won't leave machine (versa using remote proving server known as `Bonsai`).

### Core

`core` directory is a convention in Risc0 projects for placing the models shared between host and guest.

## Data flow and Testing

In order to test everything, [SageMath](https://www.sagemath.org/) script is present in `host/sage` directory. It allows to randomly generate input for `guest` program to run on. In a `host/test_values` directory
you can find generated inputs and expected _journal_ - public information in Risc0 ZKP.

The host program expects CLI argument for filepath to `JSON` input. It is then read and passed to `guest`
for ZKP generation.

## Quick Launch

After cloning this repository and making sure you've got installed rust and risc0, run:

```bash
make run-prove
```

## Development mode and Execution Statistics

During development, faster iteration upon code changes can be achieved by leveraging [dev-mode], we strongly suggest activating it during your early development phase. Furthermore, you might want to get insights into the execution statistics of your project, and this can be achieved by specifying the environment variable `RUST_LOG="[executor]=info"` before running your project.

Put together, the command to run your project in development mode while getting execution statistics is:

```bash
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run -- <path-to-input-file>
```

## Cryptography Acceleration

The main cryptographical primitive in this ZK scheme is a hashing function.
Even though examples like `Pedersen` or `Poseidon` are known to be ZK-friendly, Risc0 has extensive support
for `SHA256`. It is hardware accellerated and utilized in this project unlike circom implementation.

## Benchmarks

Project has been tested for computational efficiency with a variety of inputs.
You can examine results in [Benchmark.md](./BENCHMARK.md)

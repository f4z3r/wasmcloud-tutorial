<div align="center">

<img src="./assets/img/wasmcloud-logo.png" alt="wasmCloud" width="25%">

# wasmCloud Tutorials

![GitHub last commit](https://img.shields.io/github/last-commit/f4z3r/wasmcloud-tutorial)
![GitHub License](https://img.shields.io/github/license/f4z3r/wasmcloud-tutorial)

### This repository contains assets to start understanding wasmCloud as a technology.

[About](#about) | [WASM](#webassembly) | [Component Model](#the-component-model) | [NATS](#nats) |
[wasmCloud Components](#wasmcloud-components) | [wasmCloud Providers](#wasmcloud-providers) |
[Kubernetes Demo](#kubernetes-demo) | [License](#license)

<hr />
</div>

## About

TODO: add link to blog posts on wasmcloud architecture

## WebAssembly

This repository does not contain an explicit introduction into WebAssembly, but build on top of it.
If you are unfamiliar with WebAssembly, have a look at
[the official website](https://webassembly.org/).

## The Component Model

The component model is central to wasmCloud. Check out a small tutorial on it under
[`component-model/`](./component-model/).

For more information on the component model, visit
[its page from the Bytecode Alliance](https://component-model.bytecodealliance.org/introduction.html).

## NATS

[NATS](https://nats.io/) is a central technology to wasmCloud, essentially enabling the
communication between WebAssembly components running on the platform. Check out a small tutorial on
NATS under [`nats/`](./nats/).

## wasmCloud Components

WasmCloud components are essentially identical to components in the component model. We have not
designed a separate tutorial for wasmCloud components, but they can be seen in action in the
provider tutorial or in the Kubernetes demo.

## wasmCloud Providers

Providers expands the functionality space of components. Check out the tutorial at
[`provider/`](./provider/) to learn more about it.

## Kubernetes Demo

WasmCloud is a very capable application platform. However, in most cases, one requires more
flexibility to deploy services on a platform, especially since the WebAssembly adoption is not
currently very strong. This it can make a lot of sense to combine wasmCloud with a Kubernetes
platform to host standardized container workloads for services. We have designed a small demo on
this in the [`kubernetes/`](./kubernetes/) directory.

## License

![GitHub License](https://img.shields.io/github/license/f4z3r/wasmcloud-tutorial)

The license can be found under [`./LICENSE`](./LICENSE).

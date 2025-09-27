# Inventory Component

This component is a simple inventory management system that exposes an HTTP API. It uses a key-value
store to persist the inventory data.

## Building the component

To build the component, you need to have the `wasm-tools` and `cargo-component` installed. You can
install them by running:

```sh
cargo install wasm-tools cargo-component
```

Once you have the tools installed, you can build the component by running:

```sh
cargo component build --release
```

This will generate a `http_hello_world_s.wasm` file in the `build/` directory.

## Running the component

To run the component, you can use the `wadm` CLI. You can find the application manifest in the
`wadm.yaml` file.

To deploy the application, run:

```sh
wash app deploy -f wadm.yaml
```

This will deploy the component and the HTTP server provider. You can then add items to your
inventory by using simple curl commands:

```bash
curl http://localhost:8000?item=apple
```

For simplicity reasons you can only add items to the inventory under unique names and inspect how
the counter increases.

# `wit/component.wit`

The `wit/component.wit` file defines the world that the component implements. In this case, the
component implements the `hello` world, which imports the `wasi:keyvalue/atomics`,
`wasi:keyvalue/store` and `wasi:http/incoming-handler` interfaces.

```wit
package wasmcloud-tutorial:inventory-component;

world hello {
  import wasi:keyvalue/atomics@0.2.0-draft;
  import wasi:keyvalue/store@0.2.0-draft;
  export wasi:http/incoming-handler@0.2.2;
}
```

The `wasi:keyvalue/store` interface is used to interact with the key-value store, and the
`wasi:http/incoming-handler` interface is used to handle HTTP requests.

# `wadm.yaml`

The `wadm.yaml` file is the application manifest that describes the application. It defines the
components and the links between them.

In this example, we have two components:

- `inventory-component`: The inventory component.
- `httpserver`: The HTTP server provider.

The `link` trait is used to link the `inventory-component` to the `httpserver` provider. This allows
the component to receive HTTP requests.

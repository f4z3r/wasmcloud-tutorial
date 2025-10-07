# Writing Your Own Provider

> This directory provides a devbox environment. run `devbox shell` to have all the tools required to
> execute the commands listed here and in subdirectories.

In this tutorial we will be creating a custom `wasmCloud` provider named `key-value-provider` that
exposes an in-memory key-value store. The goal of this tutorial is to understand the core concepts
of a provider. In order to get the most out of this exercise, try to complete the coding sections on
your own and only consult the solutions when you are stuck.

## What is a Provider

A provider is a key part of the WebAssembly ecosystem that enables components to access external
resources and services such as databases, secret stores, or notification systems. This extends a
component's functionality beyond a stateless application and enables integration with a broader
system infrastructure.

### Component - Provider - Backend

A provider acts as a crucial interface or "glue layer" between a component and a backend service.
Sometimes like in our case the provider does not rely on any backend service. It translates the
component's generic, simple calls (like `get` or `set` for a key-value store) into the specific
commands required by the chosen backend (like a `redis` query or a `postgresql` SQL statement). This
simplifies backend changes to the point where no component code needs to be modified when a backend
is swapped.

![component-provider-backend](./assets/ProviderSetup.excalidraw.svg)

## The Provider Configuration

### The Provider's Contract (WIT)

Providers define their interface using the **Wasm Interface Type (WIT)** language. This interface is
the contract between the component and the provider. It's a key part of the `wasmCloud` philosophy
of separating the implementation from the interface. In our example, we define the `get` and `set`
functions that the provider implements and makes available for other components to use. Generally it
is good practice to first think about which functions your provider should expose. Hence the `wit`
is generally a good starting point.

> **💡 Info:** Usually when starting from scratch a provider template can be created using the
> `wash-cli`:
>
> ```bash
> wash new provider my-provider --template-name custom-template-rust
> ```
>
> However this tutorial already provides the base structures and hence we can skip this step.

Head over to `./key-value-provider/wit` and try to define the `wit` that exposes an interface
`store` and two functions `get` and `set`.

<details>
  <summary>Solution</summary>

```wit
package wasmcloud-tutorial:key-value-provider@0.1.0;

interface store {
    // Retrieve a value associated with a key
    get: func(key: string) -> option<string>;

    // Store a value associated with a key
    set: func(key: string, value: string);
}

// All imports and exports our provider can use / must implement.
world provider {
    export store;
}
```

</details>

## Provider Logic Definition

The `wit` inteface is the contract between the component and the provider. In our example, we define
the `get` and `set` functions that the provider implements and makes available for other components
to use (for now we did not implement any logic, we only defined the interface that our provider will
expose for other components to use).

The next step would be to implement the functions defined in that contract. For this head over to
`key-value-provider/src/provider.rs` file. This involves creating a thread-safe, in-memory store and
implementing the required trait. Follow the instructions below to implement the missing parts of our
provider.

1. **Add a `HashMap` to the `KeyValueStoreProvider` struct:** This will act as our in-memory
   key-value store.

   > 💡 **Hint:** To make the shared, mutable HashMap thread-safe in Rust, wrap it in the
   > combination of `Arc` (for shared ownership across threads) and `RwLock` (for safe, concurrent
   > access control).

<details>
  <summary>Solution</summary>

```rs
#[derive(Default, Clone)]
/// Your provider struct is where you can store any state or configuration that your provider needs to keep track of.
pub struct KeyValueStoreProvider {
    config: Arc<RwLock<ProviderConfig>>,
    /// All components linked to this provider and their config.
    linked_from: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    /// All components this provider is linked to and their config
    linked_to: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    /// Add hashmap to store key value pairs
    store: Arc<RwLock<HashMap<String, String>>>,
}
```

</details>

2.  **Implement the Interface Handler:** Use the `async_trait::Handler` trait on your provider
    struct (`KeyValueStoreProvider`). This trait is the mechanism through which the provider exposes
    the `get` and `set` functions defined in the `wit` contract, fulfilling the provider's
    capability role. By implementing this trait, our provider fulfills its contract.

<details>
  <summary>Solution</summary>

```rs
impl Handler<Option<Context>> for KeyValueStoreProvider {
    async fn get(
        &self,
        _ctx: Option<Context>,
        key: String,
    ) -> Result<Option<String>, anyhow::Error> {
        let store = self.store.read().await;
        Ok(store.get(&key).cloned())
    }

    async fn set(
        &self,
        _ctx: Option<Context>,
        key: String,
        value: String,
    ) -> Result<(), anyhow::Error> {
        let mut store = self.store.write().await;
        store.insert(key.clone(), value.clone());
        Ok(())
    }
}
```

</details>

## Building the Provider

Now that our provider's logic is complete, we can build it! The `wash` cli handles all the
compilation and packaging for us. In `wasmCloud`, a provider is packaged as a `.par.gz` file (a
provider archive), which is a signed, gzipped tarball.

First, let's build the provider. For this navigate to the root folder of the provider and run the
following command:

```bash
wash build
```

This will generate the necessary `.par.gz` file in the build directory. The build process also signs
the artifacts with a developer key, a critical step for security and trust on a `wasmCloud` host.

## Custom Component

As mentioned before a provider provides functionalities for components. Hence, to showcase the usage
of these functionalities we will build a minimal custom component that incorporates the before
implemented functionalities of a `key-value` store. Check out the `custom-component/` folder and
make yourself familiar with the structure of it.

### Wasm Interface Type

As with all WebAssembly components we need to define the interface to setup the contract between the
component and the provider from the component side. Go to `custom-component/wit/component.wit` and
define the contract to import the `key-value` exposed `store` interface. Additionally in order to
interact more easily with the component we also want to use the `httpserver` provider.

<details>
  <summary>Solution</summary>

```wit
package wasmcloud-tutorial:custom-component;

world custom-component {
  import wasmcloud-tutorial:key-value-provider/store@0.1.0;
  export wasi:http/incoming-handler@0.2.2;
}
```

</details>

### Use provider capabilities

Now we need to implement the logic in our `custom-component`. The goal is to listen to HTTP requests
and react with response that shows the capabilities of our `key-value` provider. Let's implement the
logic to respond to queries that contain query parameters of the form `key=value` to store and `key`
to retrieve values. (**Note:** Usually we would handle this with different HTTP methods but for
simplicity reasons we will only use a `GET` method.)

<details>
  <summary>Solution</summary>

```rust
let (status, response_body) = match query.split_once('=') {
    // Case 1: Query contains '=', implying SET operation (e.g., ?key=value)
    Some((key, value)) => {
        store::set(&key, &value);
        (
            http::StatusCode::CREATED,
            format!("{key} added with value: {value}!\n"),
        )
    }
    // Case 2: Query does not contain '=', implying GET operation or Welcome message
    None => {
        let key = query.trim();

        if key.is_empty() {
            (
                http::StatusCode::BAD_REQUEST,
                format!("Use the query string: ?key=value (SET) or ?key (GET)."),
            )
        } else {
            match store::get(key) {
                // Success: Value found
                Some(value) => {
                    (http::StatusCode::OK, format!("Value for '{key}': {value}"))
                }
                // Success: Key not found
                None => (
                    http::StatusCode::NOT_FOUND,
                    format!("Key '{key}' not found."),
                ),
            }
        }
    }
};
```

</details>

## Building the Component

You can then build the component using:

```bash
wash wit deps
wash build
```

## Deploying with WADM

For deployment, we'll use a `wadm.yaml` (wasmCloud application deployment manifest) file to
declaratively define, deploy, and manage our application. This manifest tells the `wasmCloud` host
which components and providers to start, and how to link them together. Check out the manifest and
define the link between the custom-component and our custom `key-value` provider.

> 💡 **Hint:** As we are in development mode we can reference to our custom provider by linking to
> the local file path. In a production environment the provider would be stored in a registry. A
> local file can be referenced as follows:
>
> ```
> image: file://../key-value-provider/build/key-value-provider.par.gz
> ```

<details>
  <summary>Solution</summary>

```yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: custom-component
  annotations:
    version: v0.0.1
    description: 'Example component to understand the connection between components, providers and backends.'
spec:
  components:
    - name: custom-component
      type: component
      properties:
        image: file://./build/custom_component.wasm
      traits:
        - type: spreadscaler
          properties:
            instances: 1
        - type: link
          properties:
            target:
              name: key-value-provider
            namespace: wasmcloud-tutorial
            package: key-value-provider
            interfaces: [store]
    - name: httpserver
      type: capability
      properties:
        image: ghcr.io/wasmcloud/http-server:0.27.0
      traits:
        - type: link
          properties:
            target:
              name: custom-component
            namespace: wasi
            package: http
            interfaces: [incoming-handler]
            source:
              config:
                - name: default-http
                  properties:
                    address: 0.0.0.0:8000
    - name: key-value-provider
      type: capability
      properties:
        image: file://../key-value-provider/build/key-value-provider.par.gz
```

</details>

After we ran the following commands we are ready to deploy the component with the provider on the
`wasmtime-host`. We start a `wasmtime-host` and deploy the application:

```bash
wash up -d
wash app deploy ./wadm.yaml
```

Using a `curl` we can test the functionality:

```bash
curl localhost:8000?test-key # status code 404: result in error as key not found
curl localhost:8000?test-key=test-value # status code 201: result in stored message
curl localhost:8000?test-key # status code 200: return stored value
```

## Teardown

You can then tear down the wasmCloud host using:

```bash
# first delete the application, otherwise it will remain when restarting wasmcloud
wash app delete custom-component
# shutdown wasmcloud
wash down
```

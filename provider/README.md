# WasmCloud Providers

> This directory provides a DevBox environment. Run `devbox shell` to have all the tools required to
> execute the commands listed here and in subdirectories.

## What is a Provider

A provider is a key part of the WebAssembly ecosystem that enables components to access external resources and services such as databases, secret stores, or notification systems. This extends the component's functionality beyond a stateless application and enables integration with a broader system infrastructure.

### Component - Provider - Backend

A provider acts as a crucial interface or "glue layer" between a component and a backend service. It translates the component's generic, simple calls (like `get` or `set` for a key-value store) into the specific commands required by the chosen backend (like a `Redis` query or a `PostgreSQL` SQL statement). This simplifies backend changes to the point where no component code needs to be modified.

![Component-Provider-Backend](./assets/ProviderSetup.excalidraw.svg)

### Provider Configuration

As described in the [Component-Model](./../component-model/README.md) every component exposes a well defined interface. This is no different when using a Provider, hence the need to add the desired Providers in the .wit (wasm interface type) as follows:

```wit
package wasmcloud-tutorial:inventory-component;

world hello {
  import wasi:keyvalue/atomics@0.2.0-draft;
  import wasi:keyvalue/store@0.2.0-draft;
  export wasi:http/incoming-handler@0.2.2;
}
```

In the example above, the inventory-component has imported two interfaces from the key-value capability and one from the http capability.

### Application Manifest

While the `.wit` file declares the need for a capability, the `wadm.yaml` manifest defines how that capability is provided in an application. Here, the links from a component to a specific provider, including its backend, are configured.

By default the key-value provider uses `NATS`, which is already integrated with WasmCloud hosts. If a specific backend like `Redis` is needed or a different HTTP server is desired, it must be explicitly deployed and linked to the component. The example below shows how to configure a link that connects the `inventory-component` to the `http-server` provider. 

```yaml
    - name: httpserver
      type: capability
      properties:
        image: ghcr.io/wasmcloud/http-server:0.27.0
      traits:
        - type: link
          properties:
            target:
              name: inventory-component
            namespace: wasi
            package: http
            interfaces: [incoming-handler]
            source:
              config:
                - name: default-http
                  properties:
                    address: 0.0.0.0:8000

```
Check out the example [inventory-component](./inventory-component/README.md) to deepen your understanding about providers.

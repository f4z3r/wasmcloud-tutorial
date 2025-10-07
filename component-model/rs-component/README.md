# A Rust Component

Cargo (the Rust build tooling) provides a nice convenient subcommand `component` to setup
components. This directory was generated using the following command (you don't need to run this):

```sh
cargo component new rs-component --name calculator
```

Again, just like with the Python component, we declared the WIT in the [`wit/`](./wit/) directory.
Moreover, in order for Rust to know the contents of `wasmcloud-tutorial:adder/add@0.1.0` declared in
the WIT, we have a line in the `Cargo.toml` file to reference it:

```toml
[package.metadata.component.target.dependencies]
"wasmcloud-tutorial:adder" = { path = "../py-component/wit" }
```

We can then use the following command to generate the required bindings:

```sh
cargo component bindings
```

Running this should provide you with a file under [`./src/bindings.rs`](./src/bindings.rs) which
contains the type definitions that you need to be able to compile your component. We now have to
implement the `main` function using the `add` function from the `add` interface from the `adder`
world. Head over to the [main.rs](./rs-component/src/main.rs) and try to implement the main
function.

<details>
    <summary>Solution</summary>

```rs
mod bindings;

use bindings::wasmcloud_tutorial::adder::add::add;

fn main() {
    let result = add(1, 5);
    println!("1 + 5 = {result}");
}
```

</details>

We can then build this component:

```sh
cargo component build --release
```

Cargo will put this under `./target/wasm32-wasip1/release/calculator.wasm`. We can again check the
WIT directly on the component:

```sh
wasm-tools component wit ./target/wasm32-wasip1/release/calculator.wasm
```

Note that it exports an interface. This enables running the component as a CLI tool via the WASM
System Interface (WASI).

```wit
world root {
  // ...
  export wasi:cli/run@0.2.3;
}
```

Now you are ready to continue the main tutorial and compose you two components together!

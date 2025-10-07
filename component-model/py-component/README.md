# A Python Component

Building WASM components from Python code is very simple. We create the `WIT` definition (in our
case we already did this under the [`wit/`](./wit/) directory), and then use `componentize-py` to
generate the boilerplatecode from the WIT definition. The boilerplatecode is needed because it acts
as a translator for the interface, ensuring the Python interpreter understands the `WIT` interface
definitions. This is achieved by running the following command:

```sh
uv run componentize-py --wit-path wit --world adder bindings .
```

Essentially, this says to generate the bindings for the `adder` world defined somewhere within the
`wit` path. This will generate code under `wit_world` for Python to use. The interesting is the
`add` interface we need to implement, which is generated under
[`./wit_world/exports/__init__.py`](./wit_world/exports/__init__.py):

```py
class Add(Protocol):

    @abstractmethod
    def add(self, x: int, y: int) -> int:
        raise NotImplementedError
```

This is the abtract method we then implement in our `app.py`. As this is Python, implementing an
interface is done by subtyping an abstract class. Now we need to actually implement the logic of our
component. Head to [./app.py](./app.py) and add the required logic for the add function.

<details>
    <summary>Solution</summary>

```py
class Add(exports.Add):
    def add(self, x: int, y: int) -> int:
        return x + y
```

</details>

Now our component is complete and we can generate the WASM component:

```sh
uv run componentize-py \
    --wit-path wit/component.wit \
    --world adder \
    componentize \
    app \
    -o adder.wasm
```

This will generate a `adder.wasm` core module, which metadata attached to it to make it a component.
For instance, you can check the WIT of the component:

```sh
wasm-tools component wit adder.wasm
```

You should see all kinds of interfaces that the module uses (injected by `componentize-py`), but it
should include the following blocks:

- Somewhere at the top of the output:
  ```wit
  world root {
    // ...
    export wasmcloud-tutorial:adder/add@0.1.0;
  }
  ```
  Indicating that the modules exports this interface.
- At the very botton of the output:
  ```wit
  package wasmcloud-tutorial:adder@0.1.0 {
    interface add {
      add: func(x: u32, y: u32) -> u32;
    }
  }
  ```
  The definition of the interface being exported.

Head back to the main tutorial and continue with the next steps to complete your application!

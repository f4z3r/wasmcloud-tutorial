```sh
wash wit deps
uv run componentize-py --wit-path wit --world echo --world-module echo bindings .
wash build
wash up -d --wasmcloud-version 1.9.0 --wadm-version v0.21.0
wash app deploy wadm.yaml
```

```sh
wash build
wash up -d --wasmcloud-version 1.9.0 --wadm-version v0.21.0
wash app deploy wadm.yaml
```

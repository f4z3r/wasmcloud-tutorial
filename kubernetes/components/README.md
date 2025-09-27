# Components

We are developing an application that consists of several components, which are all linked together
via WIT imports/exports. This directory contains the implementation of the individual components.

## Architecture

TODO(@f4z3r): add architecture diagram once application fully done.

## Local Testing

In each component directory, run a `wash build`, this should generate the WASM modules required to
run. Once these exist, launch a local version of wasmCloud:

```sh
wash up -d
```

Then deploy the application:

```sh
wash app deploy local.wadm.yaml
```

At this point, connect to your local NATS cluster to subscribe to the `echo.response` subject:

```sh
nats -s 127.0.0.1:4222 sub "echo.response"
```

Then you should be able to access the application under `localhost:8000`

```console
$ curl http://localhost:8000
Published a message to echo.response subject
```

You should see a message being published:

```console
$ nats -s 127.0.0.1:4222 sub "echo.response"
15:20:16 Subscribing on echo.response
[#1] Received on "echo.response"
This is a test
```

Tear down the local wasmCloud instance:

```sh
wash down
```

## Check Logs

```sh
cat ~/.wash/downloads/wasmcloud.log | grep demo.booking-master
```

## Deploying on Kubernetes

You can deploy the application on a Kubernetes hosted wasmCloud platform using the following
command. For a step by step guide on how to set this up, see [`../README.md`](../README.md).

```sh
kubectl apply -f k8s.wadm.yaml
```

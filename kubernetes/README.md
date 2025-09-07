# Kubernetes

> This directory provides a DevBox environment. Run `devbox shell` to have all the tools required to
> execute the commands listed here and in subdirectories.

This showcases how to combine Kubernetes and wasmCloud to build a platform with high compute density
and easy combination of applications, while providing the full flexibility of Kubernetes to deploy
backing services, and to manage wasmCloud itself of course.

## Creating a Kubernetes Cluster

First, we need to create a Kubernetes cluster. This will host our wasmCloud runtime and some
backing services. The cluster we create here is a single node cluster for demonstration purposes.

```bash
kind create cluster -n wasmcloud
```

## Deploy an Ingress

Next, we deploy an ingress. This is important as we will want to speak to our application once it is
deployed. We will use a NginX ingress controller for this, and use an external simulated
loadbalancer to route the traffic into the cluster:

```bash
kubectl apply -f https://kind.sigs.k8s.io/examples/ingress/deploy-ingress-nginx.yaml
# generate the loadbalancer
cloud-provider-kind
```

## Deploy the Application Platform

Finally, let us deploy wasmCloud as the application runtime. We will do this in the `wasmcloud`
namespace. First we create the namespace, then we install the infrastructure required for wasmCloud,
specifically NATS, wadm, and the wasmCloud operator. Once this is done, we can add a host
configuration for the operator to deploy a host on Kubernetes:

```bash
kubectl create namespace wasmcloud

# install NATS, wadm, and the operator
helm upgrade --install \
  -n wasmcloud \
  wasmcloud-platform \
  --values ./assets/platform-values.yaml \
  oci://ghcr.io/wasmcloud/charts/wasmcloud-platform:0.1.2 \
  --dependency-update

# update to get host, cannot be done in one because of missing CRDs
helm upgrade --install \
  -n wasmcloud \
  wasmcloud-platform \
  --values ./assets/platform-values.yaml \
  oci://ghcr.io/wasmcloud/charts/wasmcloud-platform:0.1.2 \
  --dependency-update \
  --set "hostConfig.enabled=true"
```

## Launch wash UI

First, we need to expose the NATS endpoints to our local machine to ensure that `wash` can retrive
the necessary information from NATS:

```bash
kubectl port-forward -n wasmcloud service/nats 4222:4222 4223:4223
```

Then, one can simply run the following command to access the wash UI under `localhost:3030`:

```bash
wash ui
```

## Todos

- install redis on the cluster
- deploy an application that uses 2 components and the redis capability provider
- deploy opa on the platform
- define securiy policies via policy server

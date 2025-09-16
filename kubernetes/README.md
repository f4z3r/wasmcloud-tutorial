# Kubernetes

> This directory provides a DevBox environment. Run `devbox shell` to have all the tools required to
> execute the commands listed here and in subdirectories.

This showcases how to combine Kubernetes and wasmCloud to build a platform with high compute density
and easy combination of applications, while providing the full flexibility of Kubernetes to deploy
backing services, and to manage wasmCloud itself of course.

## Creating a Kubernetes Cluster

First, we need to create a Kubernetes cluster. This will host our wasmCloud runtime and some
backing services.

```bash
k3d registry create wasmcloud-demo.localhost --port 5000
k3d cluster create wasmcloud-demo -c assets/k3d-config.yaml
```

The cluster we create here is a five node cluster for demonstration purposes. One node is for the
Kubernetes itself, and of the remaining four worker nodes we will use two for our application
platform and two for our service provisioning. Once the cluster is up and running, let us label the
nodes accordingly:

```bash
kubectl label node k3d-wasmcloud-demo-agent-0 "node-role.kubernetes.io/worker=true"
kubectl label node k3d-wasmcloud-demo-agent-0 "node-role.kubernetes.io/application-platform=true"
kubectl label node k3d-wasmcloud-demo-agent-1 "node-role.kubernetes.io/worker=true"
kubectl label node k3d-wasmcloud-demo-agent-1 "node-role.kubernetes.io/application-platform=true"
kubectl label node k3d-wasmcloud-demo-agent-2 "node-role.kubernetes.io/worker=true"
kubectl label node k3d-wasmcloud-demo-agent-2 "node-role.kubernetes.io/infra-platform=true"
kubectl label node k3d-wasmcloud-demo-agent-3 "node-role.kubernetes.io/worker=true"
kubectl label node k3d-wasmcloud-demo-agent-3 "node-role.kubernetes.io/infra-platform=true"
```

K3d will by default deploy a loadbalancer, meaning that any ingress route you expose will be
available on `localhost:8081` on your machine.

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

# wait for the NATS and the operator to come up, then
kubectl apply -f ./assets/wasmcloud-hostconfig.yaml
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

## Launch a Demo Application

```bash
kubectl apply -f ./assets/wasmcloud-hello-world-app.yaml
```

## Todos

- install redis on the cluster
- deploy an application that uses 2 components and the redis capability provider
- deploy opa on the platform
- define securiy policies via policy server

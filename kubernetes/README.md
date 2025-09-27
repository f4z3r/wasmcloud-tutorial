# Kubernetes

> This directory provides a DevBox environment. Run `devbox shell` to have all the tools required to
> execute the commands listed here and in subdirectories.

This showcases how to combine Kubernetes and wasmCloud to build a platform with high compute density
and easy combination of applications, while providing the full flexibility of Kubernetes to deploy
backing services, and to manage wasmCloud itself of course.

## The Application Platform

### Creating a Kubernetes Cluster

First, we need to create a Kubernetes cluster. This will host our wasmCloud runtime and some backing
services.

```bash
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

### Deploy the Application Platform

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

### Launch wash UI

Now that our wasmCloud platform is up and running, we will launch a small UI just to be able to
validate that everything is as it should. For this we need to be able to talk to NATS to check what
is running. Expose these ports locally:

```bash
kubectl port-forward -n wasmcloud service/nats 4222:4222 4223:4223
```

Then, one can simply run the following command to expose the wash UI under `localhost:3030`:

```bash
wash ui
```

## Applications

Now that the application platform is up and running, we can start deploying applications to it.

### Deploy a Demo Application

The demo application consists of the following:

- An HTTP server to accept requests. This is provided as a host capability.
- A NATS client to publish a message to a NATS subject. This is also provided as a host capability.
- Our custom component which publishes a message to the `echo.response` subject on every HTTP
  request it receives.

You can deploy the entire application using:

```bash
kubectl apply -f ./components/k8s.wadm.yaml
# deploy an ingress to make it reachable from outside Kubernetes
kubectl apply -f ./assets/ingress.yaml
```

### Access the Application

Before accessing the application, make sure that you listen to the `echo.response` NATS subject to
see that the application does what it is supposed to do. For this we use the port-forward from the
wash UI and subscribe to that subject:

```sh
nats -s 127.0.0.1:4222 sub "echo.response"
```

Then either run the command below or access the URL in your browser:

```sh
curl http://localhost:8081/hello
```

You should get a response in the browser:

```
Published a message to echo.response subject
```

And see a message being published on the subject:

```console
$ nats -s 127.0.0.1:4222 sub "echo.response"
14:52:13 Subscribing on echo.response
[#1] Received on "echo.response"
This is a test
```

## Teardown

To teardown the Kubernetes cluster, run:

```bash
k3d cluster delete wasmcloud-demo
```

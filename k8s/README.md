# Setting up Kubernetes Cluster

## Network Mesh

### Downloading Istio

1) Download and extract the latest release with:

```sh
curl -L https://istio.io/downloadIstio | sh -
```

2) Move to the `Istio` directory that contains:

* Sample applications in `samples/`
* The `istioctl` client binary in the `bin/` directory.

```sh
cd istio-1.6.7
```

3) Add the istioctl client to your path (Linux or macOS):

```sh
export PATH=$PWD/bin:$PATH
```

### Installing Istio

1) For this installation, we use the demo configuration profile. It’s selected to have a good set of defaults for testing, but there are other profiles for production or performance testing.

```sh
istioctl install --set profile=demo
```

2) Add a namespace label to instruct Istio to automatically inject Envoy sidecar proxies when you deploy your application later:

```sh
kubectl label namespace default istio-injection=enabled
```

3) We can view all of the K8s resources created by istio with:

```sh
kubectl get all -n istio-system
```

# :frog: Tode

_Tode is a play on the words Tonic and node_.

---
Tode is a minimal docker container that's purpose is to run a Tonic gRPC server and facilitate Kubernetes API integration.

## Documentation

### Tode Architecture

#### Tonic gRPC Server

#### Kubernetes Operator Integration

### Getting Started

#### Building the Container Image

To build a new tagged container for my [dockerhub](https://hub.docker.com/) repo use one of the following depending on which image base we want:

```sh
docker build -t lekrow/tode:latest -t lekrow/tode:v0.0.1 -f ./debian/Dockerfile .
```

If you just want to build the image locally, use the following:

```sh
docker build -t tode -f ./debian/Dockerfile .
```

#### Running Container from Image

To run a built image of *tode* use:

```sh
docker run -p 8000:8000 lekrow/tode:latest
docker run -p 8000:8000 tode
```

This will run our Tode web application, and make its health-check API accessible locally at <http://localhost:8000/health>.

#### Useful Development Tools

##### Dive

A tool for exploring a docker image, layer contents, and discovering ways to shrink the size of your Docker/OCI image.

```sh
wget https://github.com/wagoodman/dive/releases/download/v0.9.2/dive_0.9.2_linux_amd64.deb
sudo apt install ./dive_0.9.2_linux_amd64.deb
```

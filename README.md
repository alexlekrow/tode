# :frog: Tode

_Tode is a play on the words Tonic and node_.

---
Tode is a minimal docker container that's purpose is to run a Tonic gRPC server in a docker container.

## Documentation

### Getting Started

#### Running and Building a Container from Repo Image

To build a new tagged container for my [dockerhub](https://hub.docker.com/) repo use one of the following depending on which image base we want:

```sh
docker build -t lekrow/tode:latest -t lekrow/tode:v0.0.1 -f ./debian/Dockerfile .
docker run -p 50051:50051 lekrow/tode:latest
```

If you just want to build the image locally, use the following:

```sh
docker build -t tode -f ./debian/Dockerfile .
docker run -p 50051:50051/tcp tode
```

### Testing the gRPC Server

To test the gRPC server we can use:

```sh
cd client
cargo run
```

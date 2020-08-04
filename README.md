# :frog: Tode

_Tode is a play on the words Tonic and node_.

---
Tode is a minimal docker container that's purpose is to run a Tonic gRPC server node in a docker container.

## Documentation

### Getting Started

To build or run the image for my [dockerhub](https://hub.docker.com/r/lekrow/tode) registry, use:

```sh
docker build -t lekrow/tode:latest -t lekrow/tode:v0.0.1 -f ./debian/Dockerfile .
docker run -p 50051:50051 lekrow/tode:latest
```

If you just want to build or run the image locally, use:

```sh
docker build -t tode -f ./debian/Dockerfile .
docker run -p 50051:50051/tcp tode
```

### Testing the gRPC Server

To test the gRPC server, run the rust client:

```sh
cd client
cargo run
```

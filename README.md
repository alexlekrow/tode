# :frog: Tode

[![newest version docker image version](https://img.shields.io/docker/v/lekrow/tode?sort=semver)](https://hub.docker.com/r/lekrow/tode)

_Tode is a play on the words Tonic and node_.

---
Tode is a minimal docker container that's purpose is to run a Tonic gRPC server node in a docker container.

## Documentation

Should impliment Readiness and Liveness checks

### Getting Started

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

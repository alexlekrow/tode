# :frog: Tode

[![newest version docker image version](https://img.shields.io/docker/v/lekrow/tode?sort=semver)](https://hub.docker.com/r/lekrow/tode)

_Tode is a play on the words Tonic and node_.

---
Tode is a minimal docker container that's purpose is to run a Tonic gRPC server node in a docker container.

## Documentation

### Getting Started

If you just want to build or run the image locally, use:

```sh
# For rootless docker be sure to define the current Host socket with:
# export DOCKER_HOST=unix://$XDG_RUNTIME_DIR/docker.sock
docker build -t tode -f ./debian/Dockerfile .
docker run -p 50051:50051/tcp tode
```

### Testing the gRPC Server

To test the gRPC server, run the rust client:

```sh
cd client
cargo run
```

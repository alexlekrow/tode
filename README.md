# :frog: Tode

_Tode is a play on the words Tonic and node_.

---
Tode is a minimal docker container that's purpose is to run a Tonic gRPC server and facilitate Kubernetes API integration.

## Building the Container Image

To build a new tagged container for my [dockerhub](https://hub.docker.com/) repo use one of the following depending on which image base we want:

```sh
# (1)
docker build -t lekrow/tode-alpine:latest -t lekrow/tode-alpine:v0.0.1 -f ./alpine/Dockerfile .
```

```sh
# (2)
docker build -t lekrow/tode-debian:latest -t lekrow/tode-debian:v0.0.1 -f ./debian/Dockerfile .
```

If you just want to build the image locally, use either of the following:

```sh
# (3)
docker build -t tode-alpine -f ./alpine/Dockerfile .
```

```sh
# (4)
docker build -t tode-debian -f ./debian/Dockerfile .
```

## Running Container from Image

To run a built image of *tode* use:

```sh
# (1)
docker run -p 8000:8000 lekrow/tode-alpine:latest
# (2)
docker run -p 8000:8000 lekrow/tode-debian:latest
# (3)
docker run -p 8000:8000 tode-alpine
# (4)
docker run -p 8000:8000 tode-debian
```

This will run our Tode web application, and make its API accessible locally at <http://localhost:8000/>.

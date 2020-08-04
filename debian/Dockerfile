# Base image and labels
FROM rust:1.43 as builder
LABEL Name=tode Version=0.0.2

# Create a new empty Rust project
RUN USER=root cargo new --bin tode
WORKDIR /tode

# Copy over our cargo manifests
COPY ./Cargo.lock ./Cargo.lock 
COPY ./Cargo.toml ./Cargo.toml 

# Build and cache dependencies seperately
RUN cargo build --release
RUN rm src/*.rs

# Copy our project source
ADD ./build.rs ./build.rs
ADD ./src ./src
ADD ./proto ./proto

# Build for release with Tonic dependencies added to our toolchain
RUN rm ./target/release/deps/tode*
RUN rustup component add rustfmt --toolchain 1.43.1-x86_64-unknown-linux-gnu
RUN cargo build --release

################################################
FROM debian:buster-slim
ARG APP=/usr/src/app
EXPOSE 50051
ENV TZ=Etc/UTC \
    APP_USER=todeuser \
    TODE_PORT=50051

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /tode/target/release/tode ${APP}/tode

RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}
CMD ["./tode"]

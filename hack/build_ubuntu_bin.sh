#!/usr/bin/bash
D=$(dirname $(dirname $(realpath $0)))

buildah pull "docker.io/ubuntu:22.04"
container=$(buildah from "docker.io/ubuntu:22.04")

# install rust
buildah run $container apt-get update
buildah run -e "DEBIAN_FRONTEND=noninteractive" $container apt-get install -y curl libgpgme-dev libgit2-dev build-essential pkg-config
buildah run $container bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /root/rustup.sh"
buildah run $container chmod +x /root/rustup.sh
buildah run $container /root/rustup.sh -y

# compile kustomize-pass
rm -rf $D/target/release
buildah run $container mkdir -p /app/src
buildah run -v $D:/app/src $container bash -c "cd /app/src && source /root/.cargo/env && cargo build --profile=release"

buildah rm $container

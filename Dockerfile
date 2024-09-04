# SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
#
# SPDX-License-Identifier: GPL-3.0-or-later

# This Dockerfile is used to build a Docker image for the iptmnet_client project
# The image is built in two stages:
# 1. The first stage uses the official Rust image as the builder image
#    It builds the Rust project and creates a binary
# 2. The second stage uses the official Debian image as the base image
#    It copies the binary from the builder image and sets it as the entry point of the container

# Use the official Rust image as the builder image
# Use the 1.74.1 version of the Rust image since it's the MSRV (Minimum Supported Rust Version) for the iptmnet_client project
FROM rust:1.74.1 AS builder

# Set the working directory in the builder image to /usr/src
WORKDIR /usr/src

# Create a new Rust project named iptmnet_client
RUN USER=root cargo new iptmnet

# Create a new cargo project to source the binary file from
RUN user=root cargo init --lib iptmlib

# Change the working directory to the iptmnet_client directory
WORKDIR /usr/src/iptmnet

# Create the appropriate directory structure for the first build
RUN mkdir -p src/iptmnet && mv -v src/main.rs src/iptmnet/main.rs
RUN mkdir -p src/iptmlib && mv -v /usr/src/iptmlib/src/lib.rs src/iptmlib/lib.rs

# Copy the Cargo.toml and Cargo.lock files to the iptmnet_client directory
COPY Cargo.toml Cargo.lock ./

# Build the Rust project
# This step is done separately to take advantage of Docker's layer caching
# Any changes in the source code will not invalidate the cached dependencies
RUN cargo build --release

# Remove the auto-generated main.rs file
# This file will be replaced with the actual source code
RUN rm -rfv src/*

# Remove the auto-generated binary and dependencies
# These will be replaced with the actual binary and dependencies
RUN rm -rfv target/release/deps/iptm*
RUN rm -rfv target/release/deps/libiptm*

# Add the actual source code to the src directory
ADD src src

# Build the Rust project with the actual source code
RUN cargo build --release

# Use the official distroless image as the base image
FROM gcr.io/distroless/cc-debian12@sha256:e1065a1d58800a7294f74e67c32ec4146d09d6cbe471c1fa7ed456b2d2bf06e0

# Copy the binary from the builder image to the base image
COPY --from=builder /usr/src/iptmnet/target/release/iptmnet /usr/local/bin/iptmnet

# Change the user to a non-root user for security
USER 1000

# Set the binary as the entry point of the container
# When the container starts, it will execute this binary
ENTRYPOINT [ "/usr/local/bin/iptmnet" ]

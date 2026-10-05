# SPDX-FileCopyrightText: 2023 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
# 
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# SPDX-License-Identifier: Apache-2.0

FROM ghcr.io/rust-cross/rust-musl-cross:x86_64-musl AS chef-amd64
ENV BUILDTARGET="x86_64-unknown-linux-musl"

FROM ghcr.io/rust-cross/rust-musl-cross:aarch64-musl AS chef-arm64
ENV BUILDTARGET="aarch64-unknown-linux-musl"

FROM chef-$TARGETARCH AS chef
ARG TARGETARCH
RUN apt-get update && apt-get install -y ca-certificates \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# This will speed up fetching the crate.io index in the future, see
# https://blog.rust-lang.org/2022/06/22/sparse-registry-testing.html
ENV CARGO_UNSTABLE_SPARSE_REGISTRY=true
# This is supposedly required for successfully building for arm64 using buildx with QEMU
# see https://github.com/rust-lang/cargo/issues/10583
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN cargo install --locked --features cli cargo-about
RUN cargo install --locked cargo-chef
WORKDIR /home/rust
RUN echo "Building for $TARGETARCH"

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /home/rust/recipe.json recipe.json
RUN cargo chef cook --release --target $BUILDTARGET --recipe-path recipe.json
COPY . .
RUN cargo build --release --target $BUILDTARGET --package fms-consumer
RUN cargo about generate -o target/licenses.html about.hbs
RUN mv target/${BUILDTARGET}/release/fms-consumer target/

FROM scratch

COPY --from=builder /home/rust/target/fms-consumer /app/fms-consumer
COPY --from=builder /home/rust/target/licenses.html /app/
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENTRYPOINT [ "/app/fms-consumer" ]

FROM lukemathwalker/cargo-chef:latest-rust-1-trixie AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
# Keep this in sync with the `dioxus` version in Cargo.lock -- a mismatched CLI
# changes the bundle layout and mangles asset symbols.
# `--locked` keeps the build-from-source fallback reproducible when binstall can't
# fetch a prebuilt binary; without it the CLI resolves newer semver-compatible deps
# and fails to compile.
RUN cargo binstall dioxus-cli@0.7.10 --locked --root /.cargo -y --force
# RUN curl -fsSL https://dioxus.dev/install.sh | bash

# Create the final bundle folder. Bundle always executes in release mode with optimizations enabled
RUN /.cargo/bin/dx bundle --release --package shadcn_dioxus_web


#FROM scratch
FROM gcr.io/distroless/cc-debian13
COPY --from=builder /app/target/dx/shadcn_dioxus_web/release/web/ /usr/local/app
COPY --from=builder /app/shadcn_dioxus_web/assets /usr/local/app/assets

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]
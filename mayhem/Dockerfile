# ARGS
ARG project=alumina

# Setup Fuzz-Rust Builder
FROM ghcr.io/nottirb/fuzz-rust:latest as builder

RUN apt-get update && apt-get install -y software-properties-common curl build-essential git \
	&& curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
	&& apt-get install -y nodejs \
	&& cargo install tree-sitter-cli

ADD . /${project}
WORKDIR /${project}

# Inject a lib.rs file and build the fuzz targets
RUN mv ./mayhem/inject/lib.rs src/alumina-boot/src/lib.rs \
    && cd ./fuzz && cargo +nightly hfuzz build

# Setup Fuzz-Env
FROM ghcr.io/nottirb/fuzz-env:latest
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/parse_for_errors /
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/parse_no_errors /

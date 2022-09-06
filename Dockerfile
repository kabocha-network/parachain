# This is the build stage for Kabocha. Here we create the binary in a temporary image.
FROM paritytech/ci-linux:production as builder

WORKDIR /kabocha
COPY . /kabocha

RUN rustup update stable
RUN rustup update nightly
RUN cargo update
RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the Polkadot binary."
FROM docker.io/library/ubuntu:20.04

LABEL description="Multistage Docker image for Kabocha: sovereign creative socities on web3." \
	io.parity.image.type="builder" \
	io.parity.image.authors="ramsey@decentration.org, chevdor@gmail.com, devops-team@parity.io, " \
	io.parity.image.vendor="Decentration" \
	io.parity.image.description="Kabocha: creator network on web3." \
	io.parity.image.source="https://github.com/paritytech/polkadot/blob/${VCS_REF}/scripts/ci/dockerfiles/polkadot/polkadot_builder.Dockerfile" \
	io.parity.image.documentation="https://github.com/kabocha-network/parachain"

COPY --from=builder /kabocha/target/release/parachain-collator /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /kabocha kabocha && \
	mkdir -p /data /kabocha/.local/share && \
	chown -R kabocha:kabocha /data && \
	ln -s /data /kabocha/.local/share/kabocha && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/kabocha --version

USER kabocha

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/kabocha"]
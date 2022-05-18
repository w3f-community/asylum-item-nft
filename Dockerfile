FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /asylum
COPY . /asylum
RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the Substrate binary."
FROM docker.io/library/ubuntu:20.04

COPY --from=builder /asylum/target/release/node-asylum /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /asylum asylum && \
	mkdir -p /data /asylum/.local/share/asylum && \
	chown -R asylum:asylum /data && \
	ln -s /data /asylum/.local/share/asylum && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin

USER asylum
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
FROM docker.io/library/rust:1.98-bookworm AS builder

WORKDIR /build

ARG DATABASE_URL

COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx/
COPY .cargo ./.cargo/
COPY beer-api/Cargo.toml beer-api/build.rs ./beer-api/
COPY beer-api/migrations ./beer-api/migrations
COPY beer-api/src ./beer-api/src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/build/target \
	cargo build --locked --release --package beer-api \
	&& cp /build/target/release/beer-api /build/beer-api

FROM docker.io/library/debian:bookworm-slim AS runtime

ARG DATABASE_URL

RUN apt-get update \
	&& apt-get install --yes --no-install-recommends ca-certificates \
	&& rm -rf /var/lib/apt/lists/* \
	&& useradd --system --uid 10001 --create-home api \
	&& install --directory --owner api --group api /data

COPY --from=builder /build/beer-api/beer-api /usr/local/bin/beer-api

ENV PORT=3000 \
	DEVELOPMENT=false \
	DATABASE_URL=${DATABASE_URL}

VOLUME ["/data"]
EXPOSE 3000

USER api

ENTRYPOINT ["beer-api"]

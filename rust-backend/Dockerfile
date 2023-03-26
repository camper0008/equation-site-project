FROM rust:1.68.1-buster AS builder
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN apt update && apt install sqlite3 -y
WORKDIR /builder
COPY . .
RUN sqlite3 equation-site.db < prepare.sql
ARG DATABASE_URL=sqlite://equation-site.db
RUN cargo build --release

FROM debian:buster AS runner
RUN apt update && apt install sqlite3 libssl1.1 -y
ENV DATABASE_URL=sqlite://equation-site.db
WORKDIR /runner
COPY --from=builder /builder/target/release/equation-site-backend .
COPY --from=builder /builder/equation-site.db .
ENTRYPOINT [ "/runner/equation-site-backend" ]

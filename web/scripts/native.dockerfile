FROM rust:latest AS binary-builder

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src/ src/

RUN ["cargo", "build", "--release"]


FROM node:18.4 AS tailwind-builder

WORKDIR /tailwind

COPY config/styles.css ./
COPY tailwind.config.js ./
COPY src/ src/

RUN npm i -g tailwindcss
RUN tailwindcss -i styles.css -o styles.css -m


FROM rust:latest AS wasm-builder

WORKDIR /wasm


FROM debian:bookworm-slim AS runner

WORKDIR /app

COPY public/ public/
COPY blogs/ blogs/

COPY --from=binary-builder /build/target/release/pi-web .
COPY --from=tailwind-builder /tailwind/styles.css public/

RUN find public \( -name "*.js" -or -name "*.css" \) -exec gzip -k {} \;

COPY config/docker.yaml .

CMD /app/pi-web docker.yaml

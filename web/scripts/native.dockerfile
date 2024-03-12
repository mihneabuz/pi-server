FROM --platform=$BUILDPLATFORM rust:latest AS binary-builder

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src/ src/

RUN ["cargo", "build", "--release"]


FROM --platform=$BUILDPLATFORM node:18.4 AS tailwind-builder

WORKDIR /tailwind

COPY config/styles.css ./
COPY tailwind.config.js ./
COPY src/ src/

RUN npm i -g tailwindcss
RUN tailwindcss -i styles.css -o styles.css -m


FROM --platform=$TARGETPLATFORM debian:bookworm-slim AS runner

WORKDIR /app

COPY public/ public/
COPY blogs/ blogs/

RUN gzip -k -r public/*

COPY --from=binary-builder /build/target/release/pi-web .
COPY --from=tailwind-builder /tailwind/styles.css .

COPY config/docker.yaml .

CMD /app/pi-web docker.yaml

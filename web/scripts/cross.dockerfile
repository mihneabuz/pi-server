FROM --platform=$BUILDPLATFORM rust:latest AS binary-builder

RUN apt update && apt upgrade -y
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross

RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup toolchain install stable-aarch64-unknown-linux-gnu

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src/ src/

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

RUN ["cargo", "build", "--release", "--target", "aarch64-unknown-linux-gnu"]


FROM --platform=$BUILDPLATFORM node:18.4 AS tailwind-builder

WORKDIR /tailwind

COPY config/styles.css ./
COPY tailwind.config.js ./
COPY src/ src/

RUN npm i -g tailwindcss
RUN tailwindcss -i styles.css -o styles.css -m


FROM --platform=$BUILDPLATFORM rust:latest AS wasm-builder

WORKDIR /wasm

RUN cargo install wasm-pack

COPY wasm ./

RUN for app in *; do wasm-pack build --release --target web ${app}; done


FROM --platform=$TARGETPLATFORM debian:bookworm-slim AS runner

WORKDIR /app

COPY public/ public/
COPY blogs/ blogs/

COPY --from=binary-builder /build/target/aarch64-unknown-linux-gnu/release/pi-web .
COPY --from=tailwind-builder /tailwind/styles.css public/
COPY --from=wasm-builder /wasm/*/pkg/*.js /wasm/*/pkg/*.wasm public/wasm/

RUN find public \( -name "*.js" -or -name "*.css" -or -name "*.wasm" \) -exec gzip -k {} \;

COPY config/docker.yaml .

CMD /app/pi-web docker.yaml

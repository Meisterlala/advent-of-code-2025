# Stage 1: Build Rust WASM
FROM rust:1.91@sha256:867f1d1162913c401378a8504fb17fe2032c760dc316448766f150a130204aad AS rust-builder

# Install wasm-pack
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install wasm-pack

RUN rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install wasm-bindgen-cli@0.2.105

WORKDIR /app/rust-wasm
COPY rust-wasm .

# Build rust
# RUN --mount=type=cache,target=/app/rust-wasm/target \
#     cargo build --target wasm32-unknown-unknown --release

# Build WASM
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/rust-wasm/target \
    wasm-pack build --target web --release

# Stage 2: Build Angular Frontend
FROM node:22-slim@sha256:773413f36941ce1e4baf74b4a6110c03dcc4f968daffc389d4caef3f01412d2a AS frontend-builder

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

# Install pnpm
RUN corepack enable && corepack prepare pnpm@10.22.0 --activate

# Copy frontend source
WORKDIR /app/frontend
COPY frontend .

# Copy WASM artifacts from previous stage
COPY --from=rust-builder /app/rust-wasm/pkg /app/rust-wasm/pkg

# Install dependencies
ENV CI=true
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

# Build Angular application
RUN npx ng build --configuration production

# Stage 3: Serve with Nginx
FROM nginx:alpine@sha256:8491795299c8e739b7fcc6285d531d9812ce2666e07bd3dd8db00020ad132295

# Copy built assets from frontend-builder
COPY --from=frontend-builder /app/frontend/dist/frontend/browser /usr/share/nginx/html

# Copy custom Nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

FROM rust:bullseye as builder
WORKDIR /usr/src/quests-tracker

# Copy files to the image
COPY . .

# Install package and build release
RUN cargo install --path .

FROM debian:bullseye-slim

# Update and install package
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy build file from builder
COPY --from=builder /usr/local/cargo/bin/quests-tracker /usr/local/bin/quests-tracker

CMD ["quests-tracker"]

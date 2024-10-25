# Dockerfile
FROM rust:1.75-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    libaio1 \
    libncurses5 \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set up CSDK environment
ENV INFORMIXDIR=/opt/IBM/informix
ENV CSDK_HOME=/opt/IBM/informix
ENV LD_LIBRARY_PATH=${INFORMIXDIR}/lib:${INFORMIXDIR}/lib/esql:${INFORMIXDIR}/lib/cli
ENV INFORMIXSQLHOSTS=${INFORMIXDIR}/etc/sqlhosts

# Create necessary directories
RUN mkdir -p ${INFORMIXDIR}/lib ${INFORMIXDIR}/lib/esql ${INFORMIXDIR}/lib/cli ${INFORMIXDIR}/etc

# Create and set working directory
WORKDIR /usr/src/app

# Copy the CSDK files first
COPY csdk/ ${INFORMIXDIR}/

# Copy sqlhosts file explicitly
COPY sqlhosts ${INFORMIXDIR}/etc/

# Copy your application files
COPY . .

# Build the application
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libaio1 \
    libncurses5 \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Set up CSDK environment
ENV INFORMIXDIR=/opt/IBM/informix
ENV CSDK_HOME=/opt/IBM/informix
ENV LD_LIBRARY_PATH=${INFORMIXDIR}/lib:${INFORMIXDIR}/lib/esql:${INFORMIXDIR}/lib/cli
ENV INFORMIXSQLHOSTS=${INFORMIXDIR}/etc/sqlhosts

# Create app directory
WORKDIR /app

# Create CSDK directories
RUN mkdir -p ${INFORMIXDIR}/lib ${INFORMIXDIR}/lib/esql ${INFORMIXDIR}/lib/cli ${INFORMIXDIR}/etc

# Copy CSDK files and sqlhosts from builder
COPY --from=builder ${INFORMIXDIR} ${INFORMIXDIR}

# Copy the built binary
COPY --from=builder /usr/src/app/target/release/rust-start /app/

# Create volume mount points
VOLUME ["/app/data", "/app/logs"]

# Run the application
CMD ["./rust-start"]
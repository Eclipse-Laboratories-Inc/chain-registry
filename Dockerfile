FROM rust:1.65.0

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo install --path .
EXPOSE 8000

# Run the web service on container startup.
CMD ["eclipse-chain-registry"]
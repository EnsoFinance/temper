FROM rust:1.69.0-slim-buster AS build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM centos:8

# Copy the binary from the build stage to the current directory in the new stage
COPY --from=build /app/target/release/enso-temper /enso-temper
EXPOSE 80
CMD ["./enso-temper"]
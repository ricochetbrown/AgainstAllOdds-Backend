FROM rust:alpine
COPY --from=build  dist/target/myapp/debug/* .
RUN cargo install --path .
CMD ["myapp"]
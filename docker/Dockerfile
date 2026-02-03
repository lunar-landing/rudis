FROM rust:latest
WORKDIR /rudis
RUN cargo install --git https://github.com/lunar-landing/rudis.git
EXPOSE 6379
ENTRYPOINT ["rudis-server"]

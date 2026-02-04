FROM rust:1.93.0
WORKDIR /rudis
RUN cargo install --git https://github.com/lunar-landing/rudis.git
EXPOSE 6379
ENTRYPOINT ["rudis-server"]

FROM rustlang/rust:nightly
ENV http_proxy http://proxy.lsy.bud.dlh.de:3128
ENV https_proxy http://proxy.lsy.bud.dlh.de:3128
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
WORKDIR /rust_app
ADD . /rust_app
#RUN cargo install cargo-watch
#ENTRYPOINT ["cargo", "watch", "-x", "run"]
CMD ["cargo", "run"]
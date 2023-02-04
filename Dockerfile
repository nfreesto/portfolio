FROM rust
COPY ./target/release/server /bin/server
COPY ./app/pkg /bin/app/pkg
COPY ./app/res /bin/app/res
COPY ./app/index.html /bin/app/index.html
CMD ["/bin/server"]
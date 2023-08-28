FROM rust:1.71

WORKDIR /usr/src/app

COPY . . 

RUN cargo install --path .
EXPOSE 8080 8080

CMD [ "email-collector-rs" ]
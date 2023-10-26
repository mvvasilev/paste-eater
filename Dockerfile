FROM rust:1.73-slim-buster as rustbuild

WORKDIR /paste-eater

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM node:lts as nodebuilder

WORKDIR /app

COPY /paste-eater-frontend .

RUN yarn install --prefer-offline --frozen-lockfile --non-interactive --production=true

RUN yarn build

RUN rm -rf node_modules

FROM rust:1.73-slim-buster
COPY --from=rustbuild /paste-eater/target/release/paste-eater ./paste-eater/
COPY --from=nodebuilder /app/build ./paste-eater/paste-eater-frontend/build

ENV ROCKET_ADDRESS 0.0.0.0

EXPOSE 8000

VOLUME /root/.config/paste-eater/data

CMD ["./paste-eater/paste-eater"]
FROM rust:latest as builder

RUN cargo install --locked trunk

WORKDIR /usr/src/frontend
COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN trunk build --release

FROM nginx:alpine as runtime

COPY --from=builder /usr/src/frontend/dist /usr/share/nginx/html
COPY --from=builder /usr/src/frontend/nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx"]

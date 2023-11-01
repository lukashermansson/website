# Production image, copy all the files and run next
FROM debian

COPY target/armv7-unknown-linux-gnueabi/release/website-2 ./
COPY target/site ./site

ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

CMD ["./website-2"]
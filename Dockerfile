# Production image, copy all the files and run next
FROM debian

COPY target/server/release/website-2 ./
COPY target/site ./site

ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000

ENV PORT 3000

CMD ["/website-2"]
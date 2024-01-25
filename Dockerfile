# Production image, copy all the files and run next
FROM debian

COPY target/aarch64-unknown-linux-gnu/release/website ./
COPY target/site ./site
COPY public ./public
COPY projects ./projects

ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000

CMD ["./website"]
FROM clojure:tools-deps-trixie AS builder

RUN apt update && apt install -y nodejs npm

WORKDIR /build/

COPY . /build/

RUN npx shadow-cljs release meetinizer


FROM nginx:1.29.4

COPY --from=builder /build/public/js /usr/share/nginx/html/app-js
COPY resources/public/index.html resources/public/styles.css /usr/share/nginx/html/
COPY docker-nginx.conf /etc/nginx/conf.d/default.conf

RUN rm /usr/share/nginx/html/50x.html

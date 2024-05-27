### gasket-gui image: Pushed to pierrelefevreneti/gasket-gui by GitHub Actions

# Build with Bun
FROM docker.io/oven/bun:latest as build

ARG RELEASE_BRANCH
ARG RELEASE_DATE
ARG RELEASE_COMMIT

ENV VITE_RELEASE_BRANCH=${RELEASE_BRANCH}
ENV VITE_RELEASE_DATE=${RELEASE_DATE}
ENV VITE_RELEASE_COMMIT=${RELEASE_COMMIT}

ENV NODE_ENV="production"

WORKDIR /app
COPY . /app

RUN bun install
RUN bun run build

# Serve with NGINX
FROM nginx
COPY --from=build /app/dist /usr/share/nginx/html
RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
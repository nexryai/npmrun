## npmrun
A low-memory replacement for "npm run" written in Rust for Docker containers.

### Usage
```Dockerfile
FROM rust:1-alpine as npmrun-builder
WORKDIR /src

RUN apk add --no-cache git alpine-sdk

RUN git clone https://github.com/nexryai/npmrun.git .
RUN cargo build --release

FROM node:20-alpine3.19 AS builder

ARG NODE_ENV=production

########
# <Command to build your app>
########

FROM node:20-alpine3.19 AS runner

########
# <Copy files from builder>
########

COPY --from=npmrun-builder /src/target/release/npmrun /usr/local/bin/npmrun

ENV NODE_ENV=production
ENTRYPOINT ["/sbin/tini", "--"]
CMD ["npmrun", "docker:start"]
```

### Effects

#### Before
![Before](https://raw.githubusercontent.com/nexryai/npmrun/main/docs/before.png)

#### After
![After](https://raw.githubusercontent.com/nexryai/npmrun/main/docs/after.png)
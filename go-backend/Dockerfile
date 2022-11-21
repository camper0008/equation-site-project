FROM golang:1.19.2-alpine AS builder

COPY . /src 

WORKDIR /src

# in order to statically link dependencies
ENV CGO_ENABLED=0

RUN go build .

FROM scratch AS runner

COPY --from=builder /src/equation-site-backend /equation-site-backend

ENV GIN_MODE=$GIN_MODE

ENV DOMAIN=$DOMAIN

CMD ["/equation-site-backend"]

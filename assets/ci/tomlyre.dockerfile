FROM busybox:musl
ARG PKG
# ARG BIN
RUN mkdir -p /data
COPY $PKG /data

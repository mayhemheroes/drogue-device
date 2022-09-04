# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd device/fuzz && cargo -Z sparse-registry fuzz build && popd" && \
    mv device/fuzz/target/x86_64-unknown-linux-gnu/release/esp8266_wifi_parse /esp8266_wifi_parse && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd device/fuzz && cargo -Z sparse-registry build --release && popd" && \
    mv device/fuzz/target/release/esp8266_wifi_parse /esp8266_wifi_parse_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /esp8266_wifi_parse /esp8266_wifi_parse_no_inst /

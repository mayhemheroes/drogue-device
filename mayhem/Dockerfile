# Build Stage
FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz --version 0.11.2

# Pre-clone embassy at the pinned rev without submodules to avoid stm32-data fetch failure
RUN git clone --no-recurse-submodules https://github.com/embassy-rs/embassy.git /embassy && \
    git -C /embassy checkout 6baddaf53982b75149cb7e91280c571f7fe2e7bc

# Pre-clone ector at the pinned rev without submodules
RUN git clone --no-recurse-submodules https://github.com/drogue-iot/ector.git /ector && \
    git -C /ector checkout 63c08489da17ff92c493f21411411ce5ddd6154a

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd device/fuzz && cargo fuzz build && popd" && \
    mv device/fuzz/target/x86_64-unknown-linux-gnu/release/esp8266_wifi_parse /esp8266_wifi_parse && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd device/fuzz && cargo build --release && popd" && \
    mv device/fuzz/target/release/esp8266_wifi_parse /esp8266_wifi_parse_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /esp8266_wifi_parse /esp8266_wifi_parse_no_inst /

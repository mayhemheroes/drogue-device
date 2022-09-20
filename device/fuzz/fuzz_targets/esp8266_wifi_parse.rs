#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    _ = drogue_device::drivers::wifi::esp8266::parse(data.as_bytes());
});

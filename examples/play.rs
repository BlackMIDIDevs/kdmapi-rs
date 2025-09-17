use std::time::Duration;

use kdmapi::KDMAPI;

fn main() {
    let kdmapi = KDMAPI
        .as_ref()
        .map_err(|e| {
            eprintln!("Failed to initialize KDMAPI: {e}");
            std::process::exit(1);
        })
        .unwrap();

    let stream = kdmapi
        .open_stream()
        .map_err(|e| {
            eprintln!("Failed to start streaming: {e}");
            std::process::exit(1);
        })
        .unwrap();

    stream.send_direct_data(0x7F4090);

    std::thread::sleep(Duration::from_secs(5));

    // kdmapi dropped, terminating the stream here
}

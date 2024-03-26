use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Set an environment variable for `cargo test` to pick up
    // This will be used for the previous exercise (the one with the timestamp)
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let timestamp_str = timestamp.to_string();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp_str);

    // Check if the feature "pass" should be enabled
    // This is for the current exercise (drive4.rs)
    // The logic of setting this feature could be based on any condition or environment variable.
    // For demonstration purposes, let's always enable this feature here.
    // You might need to set this conditionally based on your actual exercise requirements.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
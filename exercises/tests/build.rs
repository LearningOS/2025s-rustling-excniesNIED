//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable called `TEST_FOO`.
    // Print in the standard output to let Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Set the environment variable TEST_FOO with the current timestamp
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the testcase return early.
    // Fill in the command to tell Cargo about that.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
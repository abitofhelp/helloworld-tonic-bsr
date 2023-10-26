use std::error::Error;
use std::process::{exit, Command};

// We manually generate the gRPC/ProtoBuf code, so the following should
// not be available when performing a CICD release build.  The conditional
// is determining whether this is a release build.
#[cfg(not(debug_assertions))]
fn main() -> Result<(), Box<dyn Error>> {
    let status = Command::new("buf")
        .arg("generate")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap();

    if !status.success() {
        exit(status.code().unwrap_or(-1))
    }

    Ok(())
}

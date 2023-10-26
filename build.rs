use std::error::Error;
use std::process::{exit, Command};

// Uncomment when we need to generate gRPC/Protobuf code.
// FIXME: Determine how to conditionally compile this code.  We don't want it to be used during a CICD build because we generate the code manually -- not the release build.
// fn main() -> Result<(), Box<dyn Error>> {
//     let status = Command::new("buf")
//         .arg("generate")
//         .current_dir(env!("CARGO_MANIFEST_DIR"))
//         .status()
//         .unwrap();
//
//     if !status.success() {
//         exit(status.code().unwrap_or(-1))
//     }
//
//     Ok(())
// }

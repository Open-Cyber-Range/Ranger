use std::env;
use std::path::PathBuf;
use tonic_build::configure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let proto_path = PathBuf::from("grpc-proto");
    let proto_path_string = proto_path.to_str().unwrap();
    configure()
        .emit_rerun_if_changed(true)
        .out_dir(out_dir)
        .compile(
            &[
                proto_path
                    .join("src")
                    .join("common.proto")
                    .to_str()
                    .unwrap(),
                proto_path
                    .join("src")
                    .join("virtual-machine.proto")
                    .to_str()
                    .unwrap(),
                proto_path
                    .join("src")
                    .join("switch.proto")
                    .to_str()
                    .unwrap(),
                proto_path
                    .join("src")
                    .join("capability.proto")
                    .to_str()
                    .unwrap(),
                proto_path
                    .join("src")
                    .join("template.proto")
                    .to_str()
                    .unwrap(),
                proto_path
                    .join("src")
                    .join("feature.proto")
                    .to_str()
                    .unwrap(),
                proto_path
                    .join("src")
                    .join("condition.proto")
                    .to_str()
                    .unwrap(),
                proto_path
                    .join("src")
                    .join("inject.proto")
                    .to_str()
                    .unwrap(),
            ],
            &[proto_path_string],
        )?;

    Ok(())
}

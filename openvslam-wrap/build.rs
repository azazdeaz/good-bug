use std::{
    fs::canonicalize,
    process::{Command, Stdio},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos(
        canonicalize("openvslam/zmq_api/proto/openvslam_api.proto")
            .expect("can't find the openvslam_api.proto file"),
    )?;
    println!("here>{:?}", std::fs::canonicalize(".")?);

    //TODO handle initial build
    let status = Command::new("make")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .current_dir(
            canonicalize("./openvslam/build").expect("can't find the openvslam build folder"),
        )
        .arg("-j3")
        .status()
        .expect("failed to run make for openvslam");
    if !status.success() {
        panic!("failed to build openvslam {:?}", status);
    }
    Ok(())
}

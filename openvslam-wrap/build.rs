fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("openvslam/zmq_api/proto/openvslam_api.proto")?;
    Ok(())
}
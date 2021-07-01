use prost_build;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["protos/map_segment.proto"], &["protos/"])?;
    tonic_build::compile_protos("../proto/helloworld.proto")?;
    Ok(())
}
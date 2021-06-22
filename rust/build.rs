use prost_build;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["protos/map_segment.proto"], &["protos/"])?;
    tonic_build::compile_protos("../proto/helloworld.proto")?;
    Ok(())
}

// extern crate protoc_rust;

// use protoc_rust::Customize;

// fn main() {
//     protoc_rust::Codegen::new()
//         .out_dir("src/protos")
//         .inputs(&["protos/map_segment.proto"])
//         .include("protos")
//         .run()
//         .expect("protoc");
// }

// extern crate protobuf_codegen_pure;
// fn main() {
//   protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
//     out_dir: "src/protos",
//     input: &["protos/map_segment.proto"],
//     includes: &["protos"],
//     customize: protobuf_codegen_pure::Customize {
//       ..Default::default()
//     },
//   }).expect("protoc");
// }
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/order.proto")?;
    tonic_build::configure()
        .out_dir("src/order_protobuf")
        .compile(&["proto/order.proto"], &["proto"])?;
    Ok(())
}

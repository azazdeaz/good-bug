
fn main() -> Result<()> {
    let settings = Settings::new()?;
    let buf = fs::read(settings.detecor_model)?;
    test_mnist(&FlatBufferModel::build_from_buffer(buf)?)
}

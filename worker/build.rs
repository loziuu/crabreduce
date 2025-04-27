use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["proto/crab_master_service.proto"], &["proto"])?;

    Ok(())
}

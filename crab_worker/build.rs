use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile_protos(
            &["../crab_master/proto/crab_master_service.proto"],
            &["../crab_master/proto/"],
        )?;

    Ok(())
}

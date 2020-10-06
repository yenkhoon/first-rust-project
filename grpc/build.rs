use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = env::current_dir().unwrap();
    let proto_path = dir.join("src/proto/say.proto");
    let out_dir = dir.join("src");

    // compiling protos using path on build time
    //tonic_build::compile_protos("proto/say.proto")?;
    tonic_build::configure()
        .format(true)
        .out_dir(&out_dir)
        .compile(&[proto_path], &[out_dir])?;

    let dir = env::current_dir().unwrap();
    let mangostine_path = dir.join("src/proto/mangostine_v0.proto");
    let out_dir = dir.join("src");

    tonic_build::configure()
        .format(true)
        .out_dir(&out_dir)
        .compile(&[mangostine_path], &[out_dir])?;
    Ok(())
}

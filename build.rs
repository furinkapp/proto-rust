fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "proto/posts.proto",
            "proto/session.proto",
            "proto/users.proto",
        ],
        &["proto"],
    )?;
    Ok(())
}

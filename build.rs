fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);

    tonic_build::configure().format(false).compile_with_config(
        config,
        &[
            "googleapis/google/cloud/bigquery/storage/v1/arrow.proto",
            "googleapis/google/cloud/bigquery/storage/v1/avro.proto",
            "googleapis/google/cloud/bigquery/storage/v1/storage.proto",
            "googleapis/google/cloud/bigquery/storage/v1/stream.proto",
        ],
        &["googleapis"],
    )?;
    Ok(())
}

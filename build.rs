use schema_rust::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-schema",
        "0.1.0",
        "SIGNAL_SCHEMA_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}

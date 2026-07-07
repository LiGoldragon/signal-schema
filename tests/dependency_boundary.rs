use std::process::Command;

#[test]
fn default_dependency_tree_has_no_runtime_or_schema_bridge_crates() {
    let output = Command::new(env!("CARGO"))
        .args(["tree", "--edges", "normal", "--no-default-features"])
        .output()
        .expect("run cargo tree");

    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("cargo tree output");

    for forbidden in ["tokio v", "redb v", "kameo v", "schema v", "schema-rust v"] {
        assert!(
            !tree.lines().skip(1).any(|line| line.contains(forbidden)),
            "default dependency tree unexpectedly contains {forbidden}:\n{tree}"
        );
    }
}

#[test]
fn nota_text_feature_is_the_text_codec_boundary() {
    let output = Command::new(env!("CARGO"))
        .args([
            "tree",
            "--edges",
            "normal",
            "--no-default-features",
            "--features",
            "nota-text",
        ])
        .output()
        .expect("run cargo tree");

    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("cargo tree output");
    assert!(
        tree.contains("nota"),
        "nota-text feature should pull nota:\n{tree}"
    );
}

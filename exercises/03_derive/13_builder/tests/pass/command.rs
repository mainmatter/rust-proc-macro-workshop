use builder_exercise::Builder;

#[derive(Builder, Debug)]
struct Command {
    executable: String,
    args: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    // `current_dir` is `Option<_>`, so it can be left unset.
    let cmd = Command::builder()
        .executable("cargo".to_string())
        .args(vec!["build".to_string()])
        .build()
        .unwrap();
    assert_eq!(cmd.executable, "cargo");
    assert_eq!(cmd.args, vec!["build".to_string()]);
    assert_eq!(cmd.current_dir, None);

    // The optional field can also be set.
    let cmd = Command::builder()
        .executable("ls".to_string())
        .args(vec![])
        .current_dir("/tmp".to_string())
        .build()
        .unwrap();
    assert_eq!(cmd.current_dir, Some("/tmp".to_string()));

    // A missing *required* field makes `build()` fail, naming the field.
    let err = Command::builder().args(vec![]).build().unwrap_err();
    assert!(
        err.to_string().contains("executable"),
        "error should name the missing field: {err}"
    );
}

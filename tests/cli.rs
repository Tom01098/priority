use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin;
use tempfile::NamedTempFile;

fn temp_database() -> (String, NamedTempFile) {
    let temp_file = NamedTempFile::new().unwrap();
    let db_url = format!("sqlite://{}", temp_file.path().display());
    (db_url, temp_file)
}

fn base_command(db_url: &str) -> Command {
    let mut cmd = Command::new(cargo_bin!("priority"));
    cmd.arg("--database-url").arg(db_url);

    cmd
}

#[test]
fn add_single_todo() {
    let (db_url, _temp_db) = temp_database();

    let mut cmd = base_command(&db_url);
    cmd.arg("add").arg("Test todo item").assert().success();

    let mut cmd = base_command(&db_url);
    let assert = cmd.arg("list").assert().success();

    let expected_output = "\
+----+----------------+
| ID | Title          |
+----+----------------+
| 1  | Test todo item |
+----+----------------+
";

    assert.stdout(expected_output);
}

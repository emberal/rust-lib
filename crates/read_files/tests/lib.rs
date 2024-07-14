use read_files::read_files_to_string;

#[test]
fn test_load_files() {
    let files = read_files_to_string!("./src", ".rs$");
    assert!(!files.is_empty());
}

#[test]
fn test_load_all_files() {
    let files = read_files_to_string!("./src");
    assert!(!files.is_empty());
}

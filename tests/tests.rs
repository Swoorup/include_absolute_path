use include_absolute_path::include_absolute_path;

#[test]
fn test_absolute_include_absolute_path() {
    const PATH: &str = include_absolute_path!("/home");
    assert!(PATH.ends_with("/home"));
}

#[test]
fn test_relative_include_absolute_path() {
    const PATH: &str = include_absolute_path!("test_file.txt");
    let contents = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(contents, "Hello World!");
}

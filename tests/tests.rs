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

#[test]
fn test_containing_env_variable_include_absolute_path() {
    const ACTUAL: &str = include_absolute_path!("$HOME");
    let expected = std::env::var("HOME").unwrap();
    assert_eq!(ACTUAL, expected);
}

#[test]
fn test_containing_env_variable_with_subpath_include_absolute_path() {
    const ACTUAL: &str = include_absolute_path!("$HOME/../");
    let expected_path =
        std::path::absolute(format!("{}/../", std::env::var("HOME").unwrap())).unwrap();
    let expected_canocalized = expected_path.canonicalize().unwrap();
    let expected = expected_canocalized.to_str().unwrap();
    assert_eq!(ACTUAL, expected);
}

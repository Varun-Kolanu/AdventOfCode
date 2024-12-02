pub fn read_file(file_path: String) -> String {
    let result = std::fs::read_to_string(file_path);
    match result {
        Ok(s) => s,
        Err(_) => panic!("Contents not found"),
    }
}

fn main() {
    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg("touch /tmp/test-deps-file")
        .output()
        .expect("failed to execute process");

}

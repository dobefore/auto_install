use std::process::Command;
fn main() {
    let _cmd = Command::new("taskkill")
        .args(&["/f", "/im", "nginx.exe"])
        .status()
        .expect("fail to run");
}

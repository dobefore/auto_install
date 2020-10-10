pub use check_python::*;
pub mod check_python {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::process::Command;
    use std::time::Duration;
    pub fn check_python() -> io::Result<()> {
        loop {
            let mut file = File::create("check_python.txt")?;
            file.write_all(b"")?;

            let mut file = File::create("check_python.txt")?;
            let py_in = Command::new("python")
                .args(&["check_python.py"])
                .output()
                .expect("Failed to install python");
            let mut fk = String::new();
            File::open("check_python.txt")?.read_to_string(&mut fk)?;
            match fk.trim() {
                "1" => {
                    println!("确认python已安装");
                    break Ok(());
                }
                _ => {
                    println!("未正确安装python或未正确识别py文件,请重新打开auto_install安装python");
                    Duration::from_secs(1);
                }
            };
        }
    }
}

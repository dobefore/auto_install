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
    .expect("Failed to start print py-version process");
    let py_str=String::from_utf8(py_in_.stdout).unwrap();
   
   if py_str !=""{
   let sp_py:Vec<&str>=py_str.trim().split(|c| c==' ' || c=='.').collect();
let py_ver=format!("{}.{}",sp_py[1],sp_py[2]);
match py_ver.as_str() {
    "3.9" => println!(),
    _ => {println!("检测到安装了3.9以外的python，请卸载重装3.9")},   
}
                _ => {
                    println!("未正确安装python或未正确识别py文件,请关闭本页面重新打开auto_install");
                    Duration::from_secs(1);
}
            };
        }
    }
}

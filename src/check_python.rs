pub use check_python::check_python;
pub mod check_python {

    use std::io::*;
    use std::process::Command;
    use std::str;

    fn install_py() {
        let _py_in = Command::new(r"pre_install\python-3.9.0-amd64.exe")
            .status()
            .expect("Failed to install python");
    }

    pub fn check_python() -> Result<()> {
        let py_in_ = Command::new("python")
            .arg("-V")
            .output()
            .expect("Failed to start print py-version process");
        let py_str = String::from_utf8(py_in_.stdout).unwrap();

        if py_str != "" {
            let sp_py: Vec<&str> = py_str.trim().split(|c| c == ' ' || c == '.').collect();
            let py_ver = format!("{}.{}", sp_py[1], sp_py[2]);
            match py_ver.as_str() {
                "3.9" => println!(),
                _ => println!("检测到安装了3.9以外的python，请卸载重装3.9"),
            }
        } else {
            println!("未安装python,接下来启动安装程序");
            install_py();
            print!("安装后，请按回车键/enter退出重新点击auto_install");
            stdout().flush().unwrap();
            let mut input1 = String::new();
            stdin().read_line(&mut input1)?;
            panic!("exit python install");
        }

        Ok(())
    }
}

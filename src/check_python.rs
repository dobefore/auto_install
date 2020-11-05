pub use check_python::*;
pub mod check_python {

<<<<<<< HEAD
    use std::io::{*};
    use std::str;
    use std::process::Command;
    
fn install_py(){
    let _py_in = Command::new(r"pre_install\python-3.9.0-amd64.exe")
                .status()
                .expect("Failed to install python");
}

    pub fn check_python() -> Result<()> {
        let py_in_ = Command::new("python")
    .arg("-V")
    .output()
=======
            let mut file = File::create("check_python.txt")?;
            let py_in = Command::new("python")
                .args(&["check_python.py"])
                .output()
>>>>>>> b55fc6f95b61f55b10a6b9d2e26558af1982988a
    .expect("Failed to start print py-version process");
    let py_str=String::from_utf8(py_in_.stdout).unwrap();
   
   if py_str !=""{
   let sp_py:Vec<&str>=py_str.trim().split(|c| c==' ' || c=='.').collect();
let py_ver=format!("{}.{}",sp_py[1],sp_py[2]);
match py_ver.as_str() {
    "3.9" => println!(),
    _ => {println!("检测到安装了3.9以外的python，请卸载重装3.9")},   
}
<<<<<<< HEAD
}
else{println!("未安装python,接下来启动安装程序");
install_py();
print!("安装后，请按回车键/enter退出重新点击auto_install");
stdout().flush().unwrap();
let mut input1 = String::new();
    stdin().read_line(&mut input1)?;
    panic!("exit python install");
}

    Ok(())
}

=======
                _ => {
                    println!("未正确安装python或未正确识别py文件,请关闭本页面重新打开auto_install");
                    Duration::from_secs(1);
}
            };
        }
    }
>>>>>>> b55fc6f95b61f55b10a6b9d2e26558af1982988a
}

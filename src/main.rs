use std::io::*;
use std::process::Command;
mod Anki_exe;
#[warn(non_snake_case)]
mod Anki_ip;
mod check_python;
mod print_colors;
fn main() -> Result<()> {
    println!("开始运行Anki服务器首次配置引导程序。。。");
    println!("请和教程结合使用(选中下面教程网址鼠标右键单击复制)");
    println!("https://dobefore.github.io/%E6%90%AD%E5%BB%BA-Anki%E5%B1%80%E5%9F%9F%E7%BD%91%E5%90%8C%E6%AD%A5%E6%9C%8D%E5%8A%A1%E5%99%A8.html");
    print_colors::write_green(&"-------------------------------------------")?;
    print_colors::print_green("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    println!(
        "如果你的PC已安装了 {} 或除 {} 以外的py3版本，请卸载它们。。。",
        print_colors::format_green("py2"),
        print_colors::format_green("py3.8")
    );
    println!("如果你已卸载py或者没有安装python，输入数字  {}  并按回车键/enter安装py3.8(请根据教程操作)：",print_colors::format_green("1"));
    println!(
        "如果你已安装 {} ，输入数字  {}  并按回车键/enter进行下一步操作",
        print_colors::format_green("python3.8"),
        print_colors::format_green("2")
    );
    print!("你输入的数字为：");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    match input.trim() {
        "1" => {
            let _py_in = Command::new(r"pre_install\python-3.8.5-amd64.exe")
                .output()
                .expect("Failed to install python");
        }
        _ => {}
    }
    check_python::check_python()?;
    print!(
        "输入数字 {} 进行复制模块操作：",
        print_colors::format_green("1")
    );
    stdout().flush().unwrap();
    let mut input1 = String::new();
    stdin().read_line(&mut input1)?;
    if let "1" = input1.trim() {
        //copy modules to python PATH
        println!("copy modules");
        let _py_in_ = Command::new("python")
            .arg(r"pre_install\copy.py")
            .spawn()
            .expect("Failed to start copy module process")
            .wait_with_output();
    }
    println!("--------------------------------------------------------------------");
    //check Ankidroid version
    print!(
        "复制modules完成，输入数字  {}  并按回车键/enter继续下一步配置：",
        print_colors::format_green("1")
    );
    stdout().flush().unwrap();
    let mut input2 = String::new();
    stdin().read_line(&mut input2)?;
    if let "1" = input2.trim() {
        println!("请打开手机Anki查询版本（设置->高级设置-关于（往下翻））");
        println!(
            "如果 {} ,输入数字 {} 并按回车键/enter继续下一步配置：",
            print_colors::format_green("<2.10"),
            print_colors::format_green("1")
        );
        println!(
            "如果 {} ,输入数字 {} 并按回车键/enter继续下一步配置：",
            print_colors::format_green(">=2.10"),
            print_colors::format_green("2")
        );
        print!("你要输入的数字为：");
        stdout().flush().unwrap();
        let mut input3 = String::new();
        stdin().read_line(&mut input3)?;
        match input3.trim() {
            "1" => {
                println!("根据你的输入，确认手机Anki 版本<2.10");
                println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
                Anki_ip::win_anki_ver_handle("1");
                print!(
                    "如手机Anki IP以修改，输入数字 {} 并按回车键/enter继续下一步配置：",
                    print_colors::format_green("1")
                );
                stdout().flush().unwrap();
                let mut input4 = String::new();
                stdin().read_line(&mut input4)?;
                if let "1" = input4.trim() {
                    Anki_exe::anki_send_desktop();
                }
            }
            "2" => {
                println!("根据你的输入，确认手机Anki 版本>=2.10");
                //add PC Anki env variant
                println!(
                    "请按照教程图示添加 {}",
                    print_colors::format_green("Anki环境变量")
                );
                println!(
                    "如在本教程找不到相关内容,请把教程向上滑，点击Ankidroid2.10开头的网页链接"
                );
                println!(
                    "{}",
                    print_colors::format_green("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<")
                );
                print!(
                    "完成后输入数字 {} 并按回车键/enter继续下一步配置：",
                    print_colors::format_green("1")
                );
                stdout().flush().unwrap();
                let mut input4 = String::new();
                stdin().read_line(&mut input4)?;
                if let "1" = input4.trim() {
                    Anki_ip::win_anki_ver_handle("2");
                    // install local CA
                    Anki_ip::install_CA()?;
                    print!(
                        "如手机Anki IP以修改，输入数字 {} 并按回车键/enter继续下一步配置：",
                        print_colors::format_green("1")
                    );
                    stdout().flush().unwrap();
                    let mut input4 = String::new();
                    stdin().read_line(&mut input4)?;
                    if let "1" = input4.trim() {
                        Anki_exe::anki_send_desktop();
                    }
                }
            }
            _ => {
                println!("输入错误，请退出重新开始");
                // system break and exit
            }
        }
    }

    print!("引导程序完成，输入任意键退出安装过程。。。");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(())
}

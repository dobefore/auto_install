use std::io::*;
use std::process::Command;
mod Anki_exe;
#[warn(non_snake_case)]
mod Anki_ip;
mod check_python;
fn main() -> Result<()> {
    println!("开始运行Anki服务器首次配置引导程序。。。");
    println!("如你在程序运行过程中遇到阻挠，请咸鱼联系我哟");
    println!("请和教程结合使用(选中下面教程网址鼠标右键单击复制)");
    println!("https://sourl.cn/MQMR8w");
    println!("-------------------------------------------");
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    check_python::check_python()?;
        //copy modules to python PATH
        println!("start copying modules");
        let _py_in_ = Command::new("python")
            .arg(r"pre_install\copy.py")
            .spawn()
            .expect("Failed to start copy module process")
            .wait_with_output();
    
    println!("--------------------------------------------------------------------");
    //check Ankidroid version
        println!("请打开手机Anki查询版本（设置->高级设置-关于（往下翻））");
        println!(
            "如果 {} ,输入数字 {} 并按回车键/enter继续下一步配置：",
            "<2.10",
            "1"
        );
        println!(
            "如果 {} ,输入数字 {} 并按回车键/enter继续下一步配置：",
            ">=2.10",
            "2"
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
                    "1"
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
                    "自动添加 {}到USER PATH",
                    "Anki环境变量"
                );
                let _set_noverify_ssl_path = Command::new("setx")
                    .args(&["ANKI_NOVERIFYSSL", "1"])
                    .status()
                    .unwrap();
                println!(
                    "{}",
                    "<<<<<<<<<<<<<<<<<<<<<<<<<<<<<"
                );
                    Anki_ip::win_anki_ver_handle("2");
                    // install local CA
                    Anki_ip::install_CA()?;
                    print!(
                        "如手机Anki IP以修改，输入数字 {} 并按回车键/enter继续下一步配置：",
                        "1"
                    );
                    stdout().flush().unwrap();
                    let mut input4 = String::new();
                    stdin().read_line(&mut input4)?;
                    if let "1" = input4.trim() {
                        Anki_exe::anki_send_desktop();
                    }
                
            }
            _ => {
                println!("输入错误，请退出重新开始");
                // system break and exit
            }
        }
    

    print!("引导程序完成，输入任意键退出安装过程。。。");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(())
}

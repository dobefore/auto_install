use std::io::*;
use std::path::Path;
use std::process::Command;
#[allow(non_snake_case)]
mod Anki_exe;
#[allow(non_snake_case)]
mod Anki_ip;
mod check_pc_anki_ver;
mod check_python;
mod print_colors;
mod walk_dir;

// .py to exe?
// modify nginx config add cert path ?
fn check_python_cp_mod() {
    check_python::check_python().unwrap();
    //copy modules to python PATH
    println!("start copying modules");
    let _py_in_ = Command::new(r"pre_install\copy.exe")
        .status()
        .expect("Failed to start copy module process");
}

fn pc_ver_15_26_36(pc_anki_ver: &str) {
    println!("--------------------------------------------------------------------");
    //check Ankidroid version
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
    stdin().read_line(&mut input3).unwrap();
    match input3.trim() {
        "1" => {
            println!("根据你的输入，确认手机Anki 版本<2.10");
            println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
            Anki_ip::win_anki_ver_handle("1", pc_anki_ver);
            Anki_ip::print_server_ipaddress_http().unwrap();
            print!(
                "如手机Anki IP以修改，输入数字 {} 并按回车键/enter继续下一步配置：",
                print_colors::format_green("1")
            );
            stdout().flush().unwrap();
            let mut input4 = String::new();
            stdin().read_line(&mut input4).unwrap();
            if let "1" = input4.trim() {
                Anki_exe::anki_send_desktop();
            }
        }
        "2" => {
            println!("根据你的输入，确认手机Anki 版本>=2.10");
            //add PC Anki env variant
            println!(
                "自动添加 {}到USER PATH",
                print_colors::format_green("Anki环境变量")
            );
            let _set_noverify_ssl_path = Command::new("setx")
                .args(&["ANKI_NOVERIFYSSL", "1"])
                .status()
                .unwrap();

            println!(
                "{}",
                print_colors::format_green("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<")
            );
            Anki_ip::win_anki_ver_handle("2", pc_anki_ver);
            // install local CA
            Anki_ip::install_CA_print_server_ipaddress_https().unwrap();
            print!(
                "如手机Anki IP以修改，输入数字 {} 并按回车键/enter继续下一步配置：",
                print_colors::format_green("1")
            );
            stdout().flush().unwrap();
            let mut input4 = String::new();
            stdin().read_line(&mut input4).unwrap();
            if let "1" = input4.trim() {
                Anki_exe::anki_send_desktop();
            }
        }
        _ => {
            println!("输入错误，请退出重新开始");
        }
    }
}

fn main() -> Result<()> {
    println!("开始运行Anki服务器首次配置引导程序。。。");
    println!("如你在程序运行过程中遇到阻挠，请咸鱼联系我哟");
    println!("请和教程结合使用(选中下面教程网址鼠标右键单击复制)");
    println!("https://sourl.cn/MQMR8w");
    print_colors::write_green(&"-------------------------------------------")?;
    print_colors::print_green("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");

    let anki_ver_txt_path = Path::new(r"pre_install\pc_anki_ver.txt");
    let anki_ver = check_pc_anki_ver::output_pc_anki_ver(anki_ver_txt_path);
    match anki_ver {
        check_pc_anki_ver::PcAnkiVer::Ver22 => {
            check_python_cp_mod();
            pc_ver_15_26_36("1");
        }
        check_pc_anki_ver::PcAnkiVer::Ver26 => {
            check_python_cp_mod();
            pc_ver_15_26_36("2");
        }
        check_pc_anki_ver::PcAnkiVer::Ver36 => {
            pc_ver_15_26_36("3");
        }
        check_pc_anki_ver::PcAnkiVer::VerErr => println!("请卸载安装其他PC Anki版本。。。"),
    }

    print!("引导程序完成，输入任意键退出安装过程。。。");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(())
}

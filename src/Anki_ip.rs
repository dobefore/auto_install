pub use Anki_ip::*;

pub mod Anki_ip {
    use crate::print_colors::*;
    use std::fs;
    use std::io::{*};
    use std::io::{self, Read};
    use std::path::{Path, PathBuf};
    use std::process::Command;

    fn  change_Ankidroid_ip_http() -> Result<()> {
        println!(
            "{}",
            print_colors::format_green("----------------------------------------------------")
        );
        println!("修改手机Anki IP");
            let mut ipa_ = String::new();
            fs::File::open(r"anki_server_v_2.1.26\anki-sync-server\server_txts\ip.txt")?
                .read_to_string(&mut ipa_)?;
            println!("请在文件夹目录-->电脑端和手机端anki的配置里打开安卓手机anki设置的图片");
            println!("并将下行内容填在手机Anki的指定位置");
            println!(
                "{}{}{}    (同步地址)",
                format_green("http://"),
                format_green(&ipa_.trim()),
                format_green(":27701")
            );
            println!(
                "{}{}{}   (媒体文件同步地址)",
                format_green("http://"),
                format_green(&ipa_.trim()),
                format_green(":27701/msync")
            );
        
        Ok(())
    }
    fn print_anki_options<'a>(ankidroid_ver: &'a str) -> io::Result<()> {
        println!(
            "如果你的anki版本为 {}，输入数字 {} 并按回车键/enter复制插件",
            print_colors::format_green("2.1.1~2.1.21"),
            print_colors::format_green("1")
        );
        println!(
            "如果你的anki版本为 {}，输入数字 {} 并按回车键/enter复制插件",
            print_colors::format_green("2.1.22~2.1.26"),
            print_colors::format_green("2")
        );
        //println!("如果你的anki版本为 {}，输入数字 {} 并按回车键/enter复制插件",print_colors::format_green("2.1.1~2.1.21"),print_colors::format_green("3"));
        print!("你要输入的数字为：");
        stdout().flush().unwrap();
        let mut PC_anki_ver = String::new();
        io::stdin().read_line(&mut PC_anki_ver)?;
        match ankidroid_ver {
            "1" => {
                //http

                match PC_anki_ver.trim() {
                    "1" => {
                        //modify PC anki IP
                        println!(
                            "接下来将自动修改 {}，请稍等。。。",
                            format_green("电脑Anki IP")
                        );
                        fs::write(r"pre_install\write_nu.txt", b"1")?;
                        let _py_in_ = Command::new("python")
                            .arg(r"anki_server_v_2.1.26\anki-sync-server\auto_ch_IP.py")
                            .spawn()
                            .expect("Failed to start change PC anki IP process")
                            .wait_with_output();
                    }
                    "2" => {
                        //modify PC anki IP
                        println!(
                            "接下来将自动修改 {}，请稍等。。。",
                            format_green("电脑Anki IP")
                        );
                        fs::write(r"pre_install\write_nu.txt", b"2")?;
                        let _py_in_ = Command::new("python")
                            .arg(r"anki_server_v_2.1.26\anki-sync-server\auto_ch_IP.py")
                            .spawn()
                            .expect("Failed to start change PC IP process")
                            .wait_with_output();
                    }
                    _ => {}
                }
            }
            "2" => {
                //https
                match PC_anki_ver.trim() {
                    "1" => {
                        //modify PC anki IP
                        println!(
                            "接下来将自动修改 {}，请稍等。。。",
                            format_green("电脑Anki IP")
                        );
                        fs::write(r"pre_install\write_nu.txt", b"1")?;
                        let _py_in_ = Command::new("python")
                            .arg(r"anki_server_v_2.1.26\anki-sync-server\auto_ch_IP_https.py")
                            .spawn()
                            .expect("Failed to start change PC anki IP process")
                            .wait_with_output();
                    }

                    "2" => {
                        //modify PC anki IP
                        println!(
                            "接下来将自动修改 {}，请稍等。。。",
                            format_green("电脑Anki IP")
                        );
                        fs::write(r"pre_install\write_nu.txt", b"2")?;
                        let _py_in_ = Command::new("python")
                            .arg(r"anki_server_v_2.1.26\anki-sync-server\auto_ch_IP_https.py")
                            .spawn()
                            .expect("Failed to start change PC anki IP process")
                            .wait_with_output();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn install_CA() -> Result<()> {
        println!(
            "{}",
            print_colors::format_green("----------------------------------------------")
        );
        println!("安装Local CA。。。,如有弹出框，点击确认");
        let _py_in_ = Command::new(r"ssl certificate\mkcert-v1.4.1-windows-amd64.exe")
            .arg(r"-install")
            .spawn()
            .expect("Failed to start install local CA process")
            .wait_with_output();
        print!(
            "确认安装后输入数字 {} 并按回车键/enter继续下一步配置：",
            format_green("1")
        );
        stdout().flush().unwrap();
        let mut inp0 = String::new();
        stdin().read_line(&mut inp0)?;
        if let "1" = inp0.trim() {
            println!("复制CA证书到桌面。。。");
            // get win_usr name and rename rootca.pem to .crt send to desktop
            let mut win_usr_name = String::new();
            fs::File::open(
                r".\anki_server_v_2.1.26\anki-sync-server\server_txts\win_username.txt",
            )?
            .read_to_string(&mut win_usr_name)?;
            let rootca_file_path = Path::new(r"C:\Users")
                .join(&win_usr_name.trim())
                .join(r"AppData\Local\mkcert")
                .join("rootCA.pem");
            let rootca_desktop_path = Path::new(r"C:\Users")
                .join(&win_usr_name.trim())
                .join(r"Desktop\rootCA.crt");
            fs::copy(rootca_file_path, rootca_desktop_path)?;
            println!(
                "请在桌面找到文件 {} ，双击文件进行导入证书，具体参考教程",
                format_green("rootCA.crt")
            );
            println!(
                "请把桌面上的文件 {} 复制到手机（QQ或USB数据线）",
                format_green("rootCA.crt")
            );
        }
        println!(
            "{}",
            print_colors::format_green("------------------------------------------------")
        );
        print!(
            "确认PC导入证书且将证书复制到手机，输入数字 {} 并按回车键/enter继续下一步配置：",
            format_green("1")
        );
        stdout().flush().unwrap();
        //generate server ca and key
        let mut inp2 = String::new();
        stdin().read_line(&mut inp2)?;
        if let "1" = inp2.trim() {
            let mut ipa_ = String::new();
            fs::File::open(r"anki_server_v_2.1.26\anki-sync-server\server_txts\ip.txt")?
                .read_to_string(&mut ipa_)?;
            println!("生成服务器证书和密钥");
            let _py_in_ = Command::new(r"ssl certificate\mkcert-v1.4.1-windows-amd64.exe")
                .args(&["localhost", "127.0.0.1", "::1", ipa_.trim()])
                .spawn()
                .expect("Failed to start generating server ca and key process")
                .wait_with_output();
            let _py_in_1 = Command::new("python")
                .arg("get_set_server_ca_key.py")
                .spawn()
                .expect("Failed to start generating server ca and key process")
                .wait_with_output();
            println!("已导入服务器证书和密钥");
            println!(
                "{}",
                print_colors::format_green("---------------------------------------------")
            );
            //Ankidroid ca import
            println!("{}", format_green("--------------------------------------"));
            println!("接下来是手机CA证书安装");
            println!(
                "进入手机QQ/TIM或文件浏览器，找到文件 {}，触摸打开",
                format_green("rootCA.crt")
            );
            println!("证书名称随意填");
            print!(
                "如手机证书已安装，输入数字 {} 并按回车键/enter继续下一步配置：",
                format_green("1")
            );
            stdout().flush().unwrap();
            //modify Ankidroid IP
            let mut inp3 = String::new();
            stdin().read_line(&mut inp3)?;
            if let "1" = inp3.trim() {
                println!("-------------------------------------");
                println!("请打开手机Anki以修改IP");
                println!(
                    "请在本软件文件夹目录-->电脑端和手机端anki的配置里打开安卓手机anki设置的图片"
                );
                println!("并将下行内容填在手机Anki的指定位置");
                let mut ipa_ = String::new();
                fs::File::open(r"anki_server_v_2.1.26\anki-sync-server\server_txts\ip.txt")?
                    .read_to_string(&mut ipa_)?;
                println!("请在文件夹目录-->电脑端和手机端anki的配置里打开安卓手机anki设置的图片");
                println!("并将下行内容填在手机Anki的指定位置");
                println!(
                    "{}{}{}    (同步地址)",
                    format_green("https://"),
                    format_green(&ipa_.trim()),
                    format_green(":27701")
                );
                println!(
                    "{}{}{}   (媒体文件同步地址)",
                    format_green("https://"),
                    format_green(&ipa_.trim()),
                    format_green(":27701/msync")
                );
            }
        }

        Ok(())
    }
    pub fn win_anki_ver_handle(ver: &str) {
        println!("请打开电脑版 Anki 查看Anki版本：帮助-->关于");
        println!("----------------------------------------");

        match ver {
            "1" => {
                //use http protocol

                print_anki_options("1").unwrap();
                //modify Ankidroid IP
                change_Ankidroid_ip_http().unwrap();
            }
            "2" => {
                //use https protocol
                print_anki_options("2").unwrap();
            }
            _ => {}
        }
    }
}

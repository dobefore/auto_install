pub use Anki_ip::*;

pub mod Anki_ip {
    use crate::print_colors::*;
    use crate::walk_dir::*;
    use std::fs::{self, OpenOptions};
    use std::io::{self, stdin, stdout, BufRead, BufReader, Read, Result, Write};
    use std::path::Path;
    use std::process::Command;

    pub fn print_server_ipaddress_http() -> Result<()> {
        // read addon choice num from txt
        let addon_choice_num = fs::read_to_string(r"pre_install\addon_choice_num.txt")
            .unwrap()
            .trim()
            .parse::<u8>()
            .unwrap();
        let (ip_addr_path, port, port_media) = if addon_choice_num == 3 {
            (
                r"anki_server_v_2.1.36\server_txts\ip.txt",
                ":27702",
                ":27702/msync",
            )
        } else {
            (
                r"anki_server_v_2.1.26\anki-sync-server\server_txts\ip.txt",
                ":27701",
                ":27701/msync",
            )
        };

        println!(
            "{}",
            print_colors::format_green("----------------------------------------------------")
        );
        println!("修改手机Anki IP");
        let mut ipa_ = String::new();
        fs::File::open(&ip_addr_path)?.read_to_string(&mut ipa_)?;
        println!("请在文件夹目录-->电脑端和手机端anki的配置里打开安卓手机anki设置的图片");
        println!("并将下行内容填在手机Anki的指定位置");
        println!(
            "{}{}{}    (同步地址)",
            format_green("http://"),
            format_green(&ipa_.trim()),
            format_green(&port)
        );
        println!(
            "{}{}{}   (媒体文件同步地址)",
            format_green("http://"),
            format_green(&ipa_.trim()),
            format_green(&port_media)
        );

        Ok(())
    }
    fn write_start_server_choice_num(choice_num: u8) {
        if choice_num == 1 {
            // pc anki 15-26
            let server_start_choice_path_rel =
                r"anki_server_v_2.1.26\anki-sync-server\server_txts\server_startup_choice.txt";
            fs::write(server_start_choice_path_rel, "1").unwrap();
        } else if choice_num == 2 {
            //pc anki 36 http
            let server_start_choice_path_rel =
                r"anki_server_v_2.1.36\server_txts\server_startup_choice.txt";
            fs::write(server_start_choice_path_rel, "2").unwrap();
        } else {
            //pc anki 36 https
            // choice_num==3
            let server_start_choice_path_rel =
                r"anki_server_v_2.1.36\server_txts\server_startup_choice.txt";
            fs::write(server_start_choice_path_rel, "3").unwrap();
        }
    }
    fn excuete_change_pc_anki_ip_https(pc_anki_version: &str) {
        fs::write(r"pre_install\addon_choice_num.txt", pc_anki_version).unwrap();

        if pc_anki_version == "3" {
            let _py_in_ = Command::new(r"anki_server_v_2.1.36\auto_ch_IP_https.exe")
                .spawn()
                .expect("Failed to start change PC anki IP process")
                .wait_with_output();
        } else {
            let _py_in_ =
                Command::new(r"anki_server_v_2.1.26\anki-sync-server\auto_ch_IP_https.exe")
                    .spawn()
                    .expect("Failed to start change PC anki IP process")
                    .wait_with_output();
        }
    }
    fn excuete_change_pc_anki_ip_http(pc_anki_version: &str) {
        fs::write(r"pre_install\addon_choice_num.txt", pc_anki_version).unwrap();
        if pc_anki_version == "3" {
            let _py_in_ = Command::new(r"anki_server_v_2.1.36\auto_ch_IP_http.exe")
                .spawn()
                .expect("Failed to start change PC anki IP process")
                .wait_with_output();
        } else {
            let _py_in_ =
                Command::new(r"anki_server_v_2.1.26\anki-sync-server\auto_ch_IP_http.exe")
                    .spawn()
                    .expect("Failed to start change PC anki IP process")
                    .wait_with_output();
        }
    }
    fn cp_nginx_https_to_nginx_conf() {
        // only for pc anki ver >=2.1.36
        //    cp nginx_https to nginx (rename)
        let nginx_conf_http_path_rel = r"anki_server_v_2.1.36\conf\nginx_https.conf";
        let nginx_conf_path_rel = r"anki_server_v_2.1.36\conf\nginx.conf";
        fs::copy(nginx_conf_http_path_rel, nginx_conf_path_rel).unwrap();
    }
    fn change_nginx_conf_http() {
        // only for pc anki ver >=2.1.36

        // read ip from txt
        let ip_addr_path_rel = r"anki_server_v_2.1.36\server_txts\ip.txt";
        let ip_string = String::from(fs::read_to_string(&ip_addr_path_rel).unwrap().trim());
        //    cp nginx_http to nginx (rename)
        let nginx_conf_http_path_rel = r"anki_server_v_2.1.36\conf\nginx_http.conf";
        let nginx_conf_path_rel = r"anki_server_v_2.1.36\conf\nginx.conf";
        fs::copy(nginx_conf_http_path_rel, nginx_conf_path_rel).unwrap();
        // change ip addr in nginx conf
        let mut f_conf = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(nginx_conf_path_rel)
            .unwrap();
        let f_http_conf = fs::File::open(nginx_conf_http_path_rel).unwrap();
        let reader_http = BufReader::new(f_http_conf);
        for line in reader_http.lines() {
            let line_ = line.unwrap();
            if line_.contains("server_name") {
                writeln!(f_conf, "{}   {};", "server_name", &ip_string).unwrap();
            } else {
                writeln!(f_conf, "{}", line_.trim()).unwrap();
            }
        }
    }
    fn cp_addon_modify_pc_anki_ip<'a>(ankidroid_ver: &'a str, pc_anki_ver: &str) -> io::Result<()> {
        println!(
            "接下来将自动修改 {}，请稍等。。。",
            format_green("电脑Anki IP")
        );
        match ankidroid_ver {
            "1" => {
                //http

                match pc_anki_ver {
                    "1" => {
                        //modify PC anki IP
                        write_start_server_choice_num(1);
                        excuete_change_pc_anki_ip_http("1")
                    }
                    "2" => {
                        //modify PC anki IP
                        write_start_server_choice_num(1);
                        excuete_change_pc_anki_ip_http("2")
                    }
                    "3" => {
                        //modify PC anki IP
                        write_start_server_choice_num(2);
                        excuete_change_pc_anki_ip_http("3");
                        // change nginx conf:ip addr
                        change_nginx_conf_http();
                    }
                    _ => {}
                }
            }
            "2" => {
                //https
                match pc_anki_ver {
                    "1" => {
                        //modify PC anki IP
                        write_start_server_choice_num(1);
                        excuete_change_pc_anki_ip_https("1")
                    }

                    "2" => {
                        //modify PC anki IP
                        write_start_server_choice_num(1);
                        excuete_change_pc_anki_ip_https("2")
                    }
                    "3" => {
                        //modify PC anki IP
                        write_start_server_choice_num(3);
                        excuete_change_pc_anki_ip_https("3");
                        cp_nginx_https_to_nginx_conf();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn install_CA_print_server_ipaddress_https() -> Result<()> {
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
        // read addon choice num from txt
        let addon_choice_num = fs::read_to_string(r"pre_install\addon_choice_num.txt")
            .unwrap()
            .trim()
            .parse::<u8>()
            .unwrap();
        let (win_user_name_path_rel, ip_txt_path, sync_addr, sync_media_addr) =
            if addon_choice_num == 3 {
                (
                    r".\anki_server_v_2.1.36\server_txts\win_username.txt",
                    r".\anki_server_v_2.1.36\server_txts\ip.txt",
                    ":27702",
                    ":27702/msync",
                )
            } else {
                (
                    r".\anki_server_v_2.1.26\anki-sync-server\server_txts\win_username.txt",
                    r".\anki_server_v_2.1.26\anki-sync-server\server_txts\ip.txt",
                    ":27701",
                    ":27701/msync",
                )
            };

        //    read ip_address from txt
        let mut ipa_ = String::new();
        fs::File::open(&ip_txt_path)?.read_to_string(&mut ipa_)?;
        if let "1" = inp0.trim() {
            println!("复制CA证书到桌面。。。");
            // get win_usr name and rename rootca.pem to .crt
            // and then send it to desktop

            let mut win_usr_name = String::new();
            fs::File::open(&win_user_name_path_rel)?.read_to_string(&mut win_usr_name)?;
            let rootca_file_path = Path::new(r"C:\Users")
                .join(&win_usr_name.trim())
                .join(r"AppData\Local\mkcert")
                .join("rootCA.pem");
            let rootca_desktop_path = Path::new(r"C:\Users")
                .join(&win_usr_name.trim())
                .join(r"Desktop\rootCA.crt");
            fs::copy(rootca_file_path, rootca_desktop_path)?;
            // add a function to check if rootCA.crt is in desktop
            // if not,use walkdir in C:\\ to search for rootCA.pem
            // and then rename it

            // return false,if rootCA.crt not in Desktop,
            // then follow two-line codes will be excueted!
            if !walk_dir::check_CA_Desktop() {
                walk_dir::resend_CA_desk_use_py();
            }

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
            println!("生成服务器证书和密钥");
            let _py_in_ = Command::new(r"ssl certificate\mkcert-v1.4.1-windows-amd64.exe")
                .args(&["localhost", "127.0.0.1", "::1", ipa_.trim()])
                .spawn()
                .expect("Failed to start generating server ca and key process")
                .wait_with_output();
            //add absolute ssl ca and key path to sync_app.py or nginx config
            let _py_in_1 = Command::new("get_set_server_ca_key.exe")
                .status()
                .expect("Failed to start generating server ca and key process");
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
                println!("请在文件夹目录-->电脑端和手机端anki的配置里打开安卓手机anki设置的图片");
                println!("并将下行内容填在手机Anki的指定位置");
                println!(
                    "{}{}{}    (同步地址)",
                    format_green("https://"),
                    format_green(&ipa_.trim()),
                    format_green(sync_addr)
                );
                println!(
                    "{}{}{}   (媒体文件同步地址)",
                    format_green("https://"),
                    format_green(&ipa_.trim()),
                    format_green(sync_media_addr)
                );
            }
        }

        Ok(())
    }
    pub fn win_anki_ver_handle(ankidroid_ver: &str, pc_anki_ver: &str) {
        match ankidroid_ver {
            "1" => {
                //use http protocol

                cp_addon_modify_pc_anki_ip("1", pc_anki_ver).unwrap();
            }
            "2" => {
                //use https protocol
                cp_addon_modify_pc_anki_ip("2", pc_anki_ver).unwrap();
            }
            _ => {}
        }
    }
}

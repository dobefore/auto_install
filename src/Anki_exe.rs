pub use Anki_exe::*;
pub mod Anki_exe {
    use crate::print_colors::*;
    use std::io::*;
    use std::process::Command;

    pub fn anki_send_desktop() {
        let _py_in_ = Command::new("python")
            .arg(r"anki_server_v_2.1.26\anki-sync-server\shortcut_server.py")
            .spawn()
            .expect("Failed to start change PC anki IP process")
            .wait_with_output();
        println!("已将anki服务器软件发送到桌面快捷方式");
        println!(
            "在桌面找到刚发送的anki_server，双击打开,点击按钮 {}，如教程图所示",
            format_green("添加账号")
        );
        println!(
            "点击按钮{}，即可打开同步服务，如教程图所示即表示正常",
            format_green("打开服务器")
        );
        println!(
            "{}",
            format_green("------------------------------------------------------------")
        );
        println!("如出现命令行窗口闪退，输入数字 {}", format_green("1"));
        println!("如能正常打开，输入数字 {}", format_green("2"));
        print!("输入的数字为：");
        stdout().flush().unwrap();
        let mut inp1 = String::new();
        stdin().read_line(&mut inp1).unwrap();
        match inp1.trim() {
            "1" => {
                let _py_in_1 = Command::new("python")
                    .arg(r"anki_server_v_2.1.26\anki-sync-server\shortcut_alt.py")
                    .spawn()
                    .expect("Failed to start change PC anki IP process")
                    .wait_with_output();
                println!("已重新将anki服务器软件发送到桌面快捷方式");
                println!(
                    "在桌面找到刚发送的{}，双击打开,点击按钮 {}，如教程图所示",
                    format_green("anki_server_alt"),
                    format_green("打开服务器")
                );
            }
            _ => {}
        }
    }
}

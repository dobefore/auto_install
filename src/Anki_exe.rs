pub use Anki_exe::anki_send_desktop;
#[allow(non_snake_case)]
pub mod Anki_exe {
    use std::fs;
    use std::process::Command;

    pub fn anki_send_desktop() {
        // send anki_server.exe to desktop using py scripts

        //read addon choice num from txt
        let addon_num = fs::read_to_string(r"pre_install\addon_choice_num.txt")
            .unwrap()
            .trim()
            .parse::<u8>()
            .unwrap();

        let _py_in_ = if addon_num == 3 {
            Command::new(r"anki_server_v_2.1.36\shortcut_server.exe")
                .status()
                .expect("Failed to send anki-server to desktop")
        } else {
            Command::new(r"anki_server_v_2.1.26\anki-sync-server\shortcut_server.exe")
                .status()
                .expect("Failed to send anki-server to desktop")
        };
        println!("已将anki服务器软件发送到桌面快捷方式");
        println!("在桌面找到刚发送的anki_server，双击打开,点击按钮 添加账号，如教程图所示");
        println!("点击按钮 打开服务器，即可打开同步服务，如教程图所示即表示正常");
        println!("------------------------------------------------------------");
    }
}

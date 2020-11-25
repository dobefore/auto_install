pub use Anki_exe::anki_send_desktop;
#[allow(non_snake_case)]
pub mod Anki_exe {
    use std::process::Command;

    pub fn anki_send_desktop() {
        let _py_in_ = Command::new("python")
            .arg(r"anki_server_v_2.1.26\anki-sync-server\shortcut_server.py")
            .spawn()
            .expect("Failed to start change PC anki IP process")
            .wait_with_output();
        println!("已将anki服务器软件发送到桌面快捷方式");
        println!(
            "在桌面找到刚发送的anki_server，双击打开,点击按钮 添加账号，如教程图所示"
        );
        println!(
            "点击按钮 打开服务器，即可打开同步服务，如教程图所示即表示正常"
        );
        println!(
            "------------------------------------------------------------");
        
    }
}

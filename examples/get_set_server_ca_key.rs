use std::{
    env,
    ffi::OsStr,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};
// filename get_set_server_ca_key.rs
fn txt_list<P: AsRef<Path>>(filepath: P) -> Vec<String> {
    // read file line to vectoe
    let mut vectorname = Vec::new();
    let f = File::open(filepath).unwrap();
    let reader = BufReader::new(f);
    for i in reader.lines() {
        let i = i.unwrap();
        vectorname.push(i.trim().to_owned())
    }
    vectorname
}
fn read_ip_from_txt<U: AsRef<OsStr>>(cwd: U) -> String {
    // 2.1.36
    let server_ip_path_rel = r"anki_server_v_2.1.36\server_txts\ip.txt";
    let abs_server_ip_path = Path::new(&cwd).join(server_ip_path_rel);
    let ipaddr_vec = txt_list(abs_server_ip_path);
    ipaddr_vec.get(0).unwrap().to_owned()
}
fn main() {
    let cwd = env::current_dir().unwrap();

    let ser_ca_path = Path::new(&cwd).join("localhost+3.pem");
    let ser_key_path = Path::new(&cwd).join("localhost+3-key.pem");
    let ser_ca_path_str = format!("{}", ser_ca_path.display());
    let ser_key_path_str = format!("{}", ser_key_path.display());
    // pc anki version according to 15-36
    let ver_nu_path = Path::new(&cwd).join(r"pre_install\addon_choice_num.txt");

    let ver_nu = txt_list(ver_nu_path);
    if ver_nu[0] == "1" || ver_nu[0] == "2" {
        let sync_app_path = r"anki_server_v_2.1.26\anki-sync-server\ankisyncd";
        let abs_sync_app_dir = Path::new(&cwd).join(sync_app_path);
        let abs_sync_app_py = Path::new(&abs_sync_app_dir).join("sync_app.py");
        let abs_sync_app_file = Path::new(&abs_sync_app_dir).join("sync_app.txt");
        //   write ssl ca and key to sync_app.py

        // .py lines and ssl-added to txt file
        let mut f = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&abs_sync_app_file)
            .unwrap();

        let mut n = 0;
        let g = File::open(&abs_sync_app_py).unwrap();
        let reader = BufReader::new(g);
        for i in reader.lines() {
            n += 1;
            if n == 671 {
                let rep_str = " ".repeat(4);
                let q = format!(
                    "{}{}{}{}{}{}",
                    rep_str,
                    "httpd.socket = ssl.wrap_socket(httpd.socket, certfile=r\"",
                    ser_ca_path_str,
                    "\",keyfile=r\"",
                    ser_key_path_str,
                    "\",server_side=True, ssl_version=ssl.PROTOCOL_TLSv1_2)"
                );
                writeln!(f, "{}", q).unwrap();
            } else {
                write!(f, "{}", i.unwrap()).unwrap();
            }
            // txt lines to .py
            let mut a = OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(&abs_sync_app_py)
                .unwrap();
            let h = File::open(&abs_sync_app_file).unwrap();
            let reader = BufReader::new(h);
            for i in reader.lines() {
                write!(a, "{}", i.unwrap()).unwrap();
            }
        }
    } else if ver_nu[0] == "3" {
        let nginx_config_path_rel = r"\anki_server_v_2.1.36\conf";
        let abs_nginx_config_dir = format!("{}{}", cwd.display(), nginx_config_path_rel);
        let abs_nginx_config_file_path = format!("{}{}", abs_nginx_config_dir, r"\nginx.conf");
        let abs_nginx_conf_txt_file = format!("{}{}", abs_nginx_config_dir, r"\nginx.txt");
        let ip_addr = read_ip_from_txt(cwd);
        // nginx conf lines ssl-added to txt
        // only for 2.1.36 https
        let mut f = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&abs_nginx_conf_txt_file)
            .unwrap();

        let g = File::open(&abs_nginx_config_file_path).unwrap();
        let reader = BufReader::new(g);
        for i in reader.lines() {
            let l = i.unwrap();
            if l.contains("server_name") {
                let q = format!(
                    "{}{}{}{};",
                    " ".repeat(7),
                    "server_name",
                    " ".repeat(3),
                    ip_addr
                );
                writeln!(f, "{}", q).unwrap();
            } else if l.contains("ssl_certificate") {
                let q = format!(
                    "{}{}{}{};",
                    " ".repeat(7),
                    "ssl_certificate",
                    " ".repeat(3),
                    ser_ca_path.display()
                );
                writeln!(f, "{}", q).unwrap();
            } else if l.contains("ssl_certificate_key") {
                let q = format!(
                    "{}{}{}{};",
                    " ".repeat(7),
                    "ssl_certificate_key",
                    " ".repeat(3),
                    ser_key_path.display()
                );
                writeln!(f, "{}", q).unwrap();
            } else {
                write!(f, "{}", l).unwrap();
            }

            //txt lines to nginx conf
            let mut k = OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(&abs_nginx_config_file_path)
                .unwrap();
            let p = File::open(&abs_nginx_conf_txt_file).unwrap();
            let reader = BufReader::new(p);
            for oo in reader.lines() {
                write!(k, "{}", oo.unwrap()).unwrap();
            }
        }
    }
}

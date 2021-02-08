use std::{
    env,
    path::Path,
    fs::{self,File,OpenOptions},
    io::{self,BufReader,BufRead,Write},
    net::UdpSocket,
    ffi::OsStr
};
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

pub fn txt_list<P: AsRef<Path>>(filepath: P) -> Vec<String> {
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
pub fn write_IP_txt<P:AsRef<Path>>(to_path:P)->io::Result<()>{
    // look up local ipaddr

let socket =UdpSocket::bind("0.0.0.0:0").unwrap();
socket.connect("8.8.8.8:80")
.expect("Couldn't connect to the server...");
let ipaddr= socket.local_addr().unwrap().ip();
let ipaddr_str=format!("{}",ipaddr);
fs::write(to_path, ipaddr_str)?;
Ok(())
}

pub fn output_pc_username()->io::Result<Vec<String>>{
// # return Admin
let mut usr_vec=Vec::new();
let hklm = RegKey::predef(HKEY_CURRENT_USER);
let cur_ver = hklm.open_subkey("Volatile Environment")?;
let username: String = cur_ver.get_value("USERNAME")?;
usr_vec.push(username);
Ok(usr_vec)
}
pub fn output_user_profile()->io::Result<String>{
// return
// userprofile:C:\\Users\\Admin 
let hklm = RegKey::predef(HKEY_CURRENT_USER);
let cur_ver = hklm.open_subkey("Volatile Environment")?;
let userprofile: String = cur_ver.get_value("USERPROFILE")?;
Ok(userprofile)

}

pub fn list_txt<P: AsRef<Path>>(to_path:P,vec:&Vec<String>)->io::Result<()>{

let mut f=OpenOptions::new().truncate(true).write(true).open(&to_path).unwrap();
for item in vec{
    writeln!(f,"{}",item)?;
}
Ok(())
}
pub fn change_addon_ip_https<P: AsRef<OsStr>>(addon_ankisyncd_dir:P,ipadr:&str,addon_choice_num:i8)->io::Result<()> {
// change addon ip addr on pc anki
let init_path=Path::new(&addon_ankisyncd_dir).join("__init__.py");
let temp_path=Path::new(&addon_ankisyncd_dir).join("temp.txt");
// .py with ipaddr to temp.txt
let mut f=OpenOptions::new().truncate(true).write(true).open(&temp_path).unwrap();
let g = File::open(&init_path).unwrap();
    let reader = BufReader::new(g);
    for i in reader.lines() {
        let i = i.unwrap();
    if i.contains("27701") || i.contains("27702") {
        if addon_choice_num==3 {
            let q=format!("addr = \"https://{}:27702/\"",&ipadr);
            writeln!(f,"{}",q)?;
        }else{
            let q=format!("addr = \"https://{}:27701/\"",&ipadr);
            writeln!(f,"{}",q)?;
        }
    }else{

        write!(f,"{}",i)?;
    }
    }
    // temp.txt to init,py
let mut e=OpenOptions::new().truncate(true).write(true).open(&init_path).unwrap();
let h = File::open(&temp_path).unwrap();
    let reader = BufReader::new(h);
    for i in reader.lines() {
        let i = i.unwrap();
    write!(e,"{}",i)?;
    }


    Ok(())
}
pub fn change_addon_ip_http<P: AsRef<OsStr>>(addon_ankisyncd_dir:P,ipadr:&str,addon_choice_num:i8)->io::Result<()>{
// change addon ip addr on pc anki
let init_path=Path::new(&addon_ankisyncd_dir).join("__init__.py");
let temp_path=Path::new(&addon_ankisyncd_dir).join("temp.txt");
// .py with ipaddr to temp.txt
let mut f=OpenOptions::new().truncate(true).write(true).open(&temp_path).unwrap();
let g = File::open(&init_path).unwrap();
    let reader = BufReader::new(g);
    for i in reader.lines() {
        let i = i.unwrap();
    if i.contains("27701") || i.contains("27702") {
        if addon_choice_num==3 {
            let q=format!("addr = \"http://{}:27702/\"",&ipadr);
            writeln!(f,"{}",q)?;
        }else{
            let q=format!("addr = \"http://{}:27701/\"",&ipadr);
            writeln!(f,"{}",q)?;
        }
    }else{

        write!(f,"{}",i)?;
    }
    }
    // temp.txt to init,py
let mut e=OpenOptions::new().truncate(true).write(true).open(&init_path).unwrap();
let h = File::open(&temp_path).unwrap();
    let reader = BufReader::new(h);
    for i in reader.lines() {
        let i = i.unwrap();
    write!(e,"{}",i)?;
    }
Ok(())
}
fn read_addon_choice_num() ->i8{
    // read addon choice num from txt
let cwd=env::current_dir().unwrap();
let addon_choice_txt_path=Path::new(&cwd).join(r"pre_install\addon_choice_num.txt");
let addon_choice_vec= txt_list(addon_choice_txt_path);
let addon_choice_num=addon_choice_vec.get(0).unwrap().parse::<i8>().unwrap();
addon_choice_num

}


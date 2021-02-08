use std::io;
use std::path::Path;
use std::fs;
use winreg::enums::{HKEY_CURRENT_USER,HKEY_LOCAL_MACHINE};
use winreg::RegKey;
fn output_pc_username()->io::Result<String>{
// # return Admin
let hklm = RegKey::predef(HKEY_CURRENT_USER);
let cur_ver = hklm.open_subkey("Volatile Environment")?;
let username: String = cur_ver.get_value("USERNAME")?;
Ok(username)
}
fn output_user_profile()->io::Result<String>{
// return
// userprofile:C:\\Users\\Admin 
let hklm = RegKey::predef(HKEY_CURRENT_USER);
let cur_ver = hklm.open_subkey("Volatile Environment")?;
let userprofile: String = cur_ver.get_value("USERPROFILE")?;
Ok(userprofile)

}
fn output_pc_anki_ver() ->io::Result<String>{
// read anki ver 2.1.36
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
let cur_ver = hklm.open_subkey(r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Anki")?;
let ankiver: String = cur_ver.get_value("DisplayVersion")?;
Ok(ankiver)
}
fn write_ver_txt<P:AsRef<Path>>(ver:String,txt_pat_rel:P)->io::Result<()> {
    // write 46 to txt
// ver : 2.1.36
let ver_s:Vec<&str>=ver.split(".").collect();
let min_ver=ver_s.get(2).unwrap().to_owned();
fs::write(txt_pat_rel, min_ver)?;
Ok(())
}
fn main()->io::Result<()>{
  let   txt_path=r"pre_install\pc_anki_ver.txt";
let anki_ver_v=output_pc_anki_ver()?;
write_ver_txt(anki_ver_v, txt_path)?;
Ok(())
}


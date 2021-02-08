use std::{path::Path,
env,
io,
collections::HashMap,
fs
};
use preclude::{txt_list,write_IP_txt,output_pc_username,list_txt,change_addon_ip_http};

fn exec_cp_addon_change_pc_anki_ip<P: AsRef<Path>>(addon_value:i8,server_txts_dir_rel:P) ->io::Result<()>{

    let cwd=env::current_dir().unwrap();
    let txt_dir=Path::new(&cwd).join(server_txts_dir_rel);
    let ip_txt_path=Path::new(&txt_dir).join("ip.txt");
    // write and read ip
    write_IP_txt(&ip_txt_path)?;
    let ip_addr= txt_list(&ip_txt_path).get(0).unwrap().to_owned();
// read and write pc username 

let pc_usrname=output_pc_username()?;
let pc_usrname_txt_path=Path::new(&txt_dir).join("win_username.txt");
list_txt(&pc_usrname_txt_path, &pc_usrname)?;

let addon_dir=Path::new(r"C:\Users").join(&pc_usrname.get(0).unwrap()).join(r"AppData\Roaming\Anki2\addons21");

    // mkdir of sync and cp init.py to anki addon folder
let mut hp_ankisyncd=HashMap::new();
hp_ankisyncd.insert(1, "ankisyncd_old");
hp_ankisyncd.insert(2, "ankisyncd_22");
hp_ankisyncd.insert(3, "ankisyncd_36");
let ankisyncd_v=hp_ankisyncd.get(&addon_value).unwrap().to_owned();
let ankisyncd_dir=Path::new(&addon_dir).join(&ankisyncd_v);
let src_init_path=Path::new(&cwd).join(r"pre_install\addons").join(&ankisyncd_v).join("__init__.py");
let dst_inti_path=Path::new(&ankisyncd_dir).join(&"__init__.py");
if  ankisyncd_dir.exists(){
    fs::copy(src_init_path, dst_inti_path)?;
}else{
fs::create_dir(&ankisyncd_dir)?;
fs::copy(src_init_path, dst_inti_path)?;

}
change_addon_ip_http(&ankisyncd_dir, ip_addr.as_str(), addon_value)?;
println!("{}","PC Anki IP修改成功，请重启Anki");
    Ok(())
}
// auto_ch_IP_http
fn main()->io::Result<()>{
    let cwd=env::current_dir().unwrap();
let addon_num_txt_path=Path::new(&cwd).join(r"pre_install\addon_choice_num.txt");
let addon_num= txt_list(&addon_num_txt_path).get(0).unwrap().parse::<i8>().unwrap();
let server_txts_path_26=r"anki_server_v_2.1.26\anki-sync-server\server_txts";
let server_txts_path_36=r"anki_server_v_2.1.36\server_txts";
if addon_num==3 {
    exec_cp_addon_change_pc_anki_ip(addon_num, &server_txts_path_36)?;

}else{
    exec_cp_addon_change_pc_anki_ip(addon_num, &server_txts_path_26)?;

}

Ok(())
}
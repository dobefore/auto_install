use std::{collections::HashSet, env, fs, io, path::Path,
};

use winreg::enums::{HKEY_CURRENT_USER};
use winreg::RegKey;


fn  handle_dir_copy(from_dir:&String,to_dir:&String)->io::Result<()> {
// create dir in to_dir
fs::create_dir(&to_dir)?;
for i in fs::read_dir(&from_dir)?{
let i=i?.path();
let entry_=&i.file_name().unwrap().to_str().unwrap().to_owned();
let from_path=i.to_str().unwrap().to_string();
let to_path=Path::new(to_dir).join(entry_).to_str().unwrap().to_owned();
if i.is_dir() {
   
    handle_dir_copy(&from_path, &to_path)?;
}else{
   
    fs::copy(from_path, to_path)?;
}
}

Ok(())
}
fn cp_mod(from_dir:String,to_dir:String)->io::Result<()> {
    let  to_set = fs::read_dir(&to_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<HashSet<_>, io::Error>>()?;
let new_to_set:HashSet<_>=to_set.iter().map(|item| format!("{}", item.file_name().unwrap().to_str().unwrap().to_owned())).collect();

        let  from_set = fs::read_dir(&from_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<HashSet<_>, io::Error>>()?;
        let new_from_set:HashSet<_>=from_set.iter().map(|item| format!("{}", item.file_name().unwrap().to_str().unwrap().to_owned())).collect();
        // cp mods which not exist in to_dir but do in from_dir
        let mut new_intersect_set:HashSet<String>=HashSet::new();
        let intersect_set:HashSet<_>=new_from_set.intersection(&new_to_set).collect();
        for i in intersect_set{
            new_intersect_set.insert(i.to_owned());
        }
       
        let differ_set:HashSet<_>=new_from_set.difference(&new_intersect_set).collect();
for q in differ_set{
    let a=q.to_owned();
//    impl cp if path is dir
    let from_path=Path::new(&from_dir).join(&a).to_str().unwrap().to_owned();
    let to_path=Path::new(&to_dir).join(&a).to_str().unwrap().to_owned();
    let from_path_p=Path::new(&from_path);
    println!("copy module {},please waiting...",&a);
    if from_path_p.is_file(){
    fs::copy(from_path, to_path)?;
    }else{

        handle_dir_copy(&from_path,&to_path)?;
    }
    
}
Ok(())
}
fn return_install_path() ->io::Result<String>{
    let remin_path=r"Lib\site-packages";
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let cur_ver = hklm.open_subkey(r"SOFTWARE\Python\PythonCore\3.9\InstallPath")?;
    let exepath: String = cur_ver.get_value("ExecutablePath")?;
    let to_be_trimmed:&[_]=&['p','y','t','h','o','n','.','e','x','e'];
let f_path=exepath.trim_end_matches(to_be_trimmed).to_string();
let full_path=format!("{}{}",f_path,remin_path);
    Ok(full_path)
}
// copy_for_26
/// use win reg-edit to get python install path 
/// and imply site-packages path
/// cp module to above path
fn main() ->io::Result<()>{
// cp mod from server_path to pathon path
let to_dir= return_install_path().unwrap();
let cur_dir=env::current_dir().unwrap();
let remain_dir=r"pre_install\copy";
let from_dir=Path::new(&cur_dir).join(remain_dir);
// read mod from pathon_path and server_path into a vec
let from_dir_s=format!("{}",from_dir.display());
cp_mod(from_dir_s, to_dir)?;

        Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, io};
    

    
    use std::path::Path;

    use crate::cp_mod;
    #[test]
    fn test_copytree() ->io::Result<()>{
        let from_d=r"C:\Users\Admin\Desktop\from".to_owned();
        let to_d=r"C:\Users\Admin\Desktop\to".to_owned();
      
        cp_mod(from_d, to_d)?;
        Ok(())
    }
    #[test]
    fn finame() {
        let s=r"C:\Users";
        let aa=r"C:\Users\a.txt";
let ss=Path::new(s).file_name().unwrap().to_str().unwrap().to_owned();
let aaa=Path::new(aa).file_name();
println!("{}{:?}",ss,aaa);
    }
}
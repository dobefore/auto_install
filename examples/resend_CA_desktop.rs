use std::env;
use std::path::Path;
use std::fs;
// filebame resend_CA_desktop.rs
// cargo build --example resend_CA_desktop --release
fn main(){

 let pem_path= env::args().last().unwrap() ;

let split_str:Vec<&str>=pem_path.split("\\").collect();
let user_name=split_str.get(2).unwrap().to_owned();
let desktop_crt_path=Path::new("C:\\Users").join(user_name).join(r"Desktop\rootCA.crt");

fs::copy(pem_path, desktop_crt_path).unwrap();
}


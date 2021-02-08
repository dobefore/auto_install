pub use walk_dir::{check_CA_Desktop, resend_CA_desk_use_py, walkdir_find_roorCA_path};
pub mod walk_dir {
    use std::path::Path;
    use std::process::Command;
    #[allow(non_snake_case)]
    pub fn resend_CA_desk_use_py() {
        // resend rootCA.crt(first rename it:use copy method) using .exe
        // to desktop
        let exe_path = r"pre_install\resend_CA_desktop.exe";
        let rootCA_path = walkdir_find_roorCA_path("C:\\Users", "rootCA.pem");
        let _resend_desktop = Command::new(exe_path)
            .arg( rootCA_path)
            .status()
            .expect("fail to run send to CA to desktop");
    }
    #[allow(non_snake_case)]
    pub fn check_CA_Desktop() -> bool {
        //check if rootCA.crt is sent to destop path
        let path_ = walkdir_find_roorCA_path("C:\\Users", "rootCA.pem");
        let desk_path_split: Vec<&str> = path_.split("\\").collect();
        let usr_name = desk_path_split[2];
        let CA_desk_path = Path::new("C:\\Users")
            .join(usr_name)
            .join("Desktop")
            .join("rootCA.crt");
        CA_desk_path.exists()
    }
    #[allow(non_snake_case)]
    pub fn walkdir_find_roorCA_path(path_root: &str, pat_str: &str) -> String {
        // walkdir C:\\ to find rootCA.pem path
        // path_root:C:\\Users;
        // pat_str: rootCA.pem;

        use walkdir::WalkDir;

        let mut match_entry_path = String::new();
        for entry in WalkDir::new(path_root) {
            let entry = entry.unwrap();
            let entr = format!("{}", entry.path().display());
            let ss: Vec<&str> = entr.rmatches(pat_str).collect();
            let vv = vec![pat_str];
            if ss == vv {
                println!("{}", &entr);
                &match_entry_path.push_str(entr.as_str());

                break;
            }
        }
        match_entry_path
    }
}

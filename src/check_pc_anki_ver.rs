pub use check_pc_anki_ver::{output_pc_anki_ver, PcAnkiVer};
//types: 2.1.36 、 2.1.26 and before
pub mod check_pc_anki_ver {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::Path;
    use std::process::Command;
    pub enum PcAnkiVer {
        Ver22,
        Ver36,
        Ver26,
        VerErr, //28-35
    }

    pub fn output_pc_anki_ver<S: AsRef<Path>>(pc_anki_ver_path: S) -> PcAnkiVer {
        //get ver_num(ie,36) from txt
        let ver_value = {
            let _cmd = Command::new(r"pre_install\get_pc_anki_ver.exe")
                .status()
                .expect("error");
            let mut ver_ = String::new();
            let f = File::open(pc_anki_ver_path).unwrap();
            let mut reader = BufReader::new(f);
            reader.read_to_string(&mut ver_).unwrap();
            ver_
        };

        let ver = ver_value.trim().parse::<u32>();
        match ver {
            Ok(_x @ 10..=22) => return PcAnkiVer::Ver22,
            Ok(_x @ 23..=26) => return PcAnkiVer::Ver26,
            Ok(36) => return PcAnkiVer::Ver36,
            //28-35
            _ => return PcAnkiVer::VerErr,
        }
    }
}

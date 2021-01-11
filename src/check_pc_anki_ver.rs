pub use check_pc_anki_ver::{output_pc_anki_ver, PcAnkiVer};
//types: 2.1.36 and later 、 2.1.26 and before
pub mod check_pc_anki_ver {
    use std::io::{self, Write};
    pub enum PcAnkiVer {
        Ver22,
        Ver36, //36 and later
        Ver26,
        VerErr, //28-35
    }

    pub fn output_pc_anki_ver() -> PcAnkiVer {
        //input pc anki ver_num_code(ie,1,2,3) for
        // different vers 1:10~22,2:23~26,3:>=36
        let ver_value = {
            println!("{}", "输入电脑anki 版本对应的数字代码");
            println!("{}", "电脑Anki版本在2.1.1~2.1.22（包含）内，输入数字 1");
            println!("{}", "电脑Anki版本在2.1.23~2.1.26（包含）内，输入数字 2");
            println!("{}", "电脑Anki版本在2.1.36（包含）及以后，输入数字 3");

            print!("{}", "输入数字并按enter/回车键结束：");
            io::stdout().flush().unwrap();
            let mut anki_num_code = String::new();
            io::stdin().read_line(&mut anki_num_code).unwrap();
            // match anki_num_code with assumed actual pc anki ver
            // ie: 1:22,2:26,3:36
            let temp_anki_num = anki_num_code.trim();
            let ver_ = if temp_anki_num == "1" {
                "22"
            } else if temp_anki_num == "2" {
                "26"
            } else if temp_anki_num == "3" {
                "36"
            } else {
                "0"
            };
            ver_
        };

        let ver = ver_value.parse::<u32>();
        match ver {
            Ok(_x @ 10..=22) => return PcAnkiVer::Ver22,
            Ok(_x @ 23..=26) => return PcAnkiVer::Ver26,
            Ok(_x) if _x >= 36 => return PcAnkiVer::Ver36,
            //28-35
            _ => return PcAnkiVer::VerErr,
        }
    }
}

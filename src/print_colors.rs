pub use print_colors::{format_green, print_green, write_green};
pub mod print_colors {
    use std::io::{self, Write};
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
    #[allow(non_camel_case_types)]
    pub fn write_green<str: std::marker::Sized + std::fmt::Debug>(string: &str) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut stdout, "{:#?}", string)
    }
    pub fn print_green(string: &str) {
        println!("{}{}{}", "\x1b[32;1m", string, "\x1b[0m");
    }
    pub fn format_green(string: &str) -> String {
        return format!("{}{}{}", "\x1b[32;1m", string, "\x1b[0m");
    }
}

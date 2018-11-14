#[macro_use]
extern crate clap;
use clap::{AppSettings::TrailingVarArg, Arg};
use std::io::BufRead;
use std::process::exit;
use std::process::{Command, Stdio};
use std::str::from_utf8;

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("separator")
                .short("s")
                .long("separator")
                .help("Single character delimiter beween input values.")
                .default_value("\\n")
                .takes_value(true),
        ).arg(
            Arg::with_name("argname")
                .help("Arbitrary string to be inserted into command")
                .required(true),
        ).setting(TrailingVarArg)
        .arg(
            Arg::with_name("command")
                .help("Command template to be run for every input")
                .multiple(true)
                .required(true),
        ).get_matches();

    let sepb = {
        let sep = matches.value_of("separator").expect("Checked by clap.");
        match unescape_delimiter(sep) {
            Ok(del) => del,
            Err(_) => {
                eprintln!("couldn't interpret delimiter as single byte character, try something like \"\\n\" or \",\"");
                exit(1);
            }
        }
    };

    let argname = matches.value_of("argname").expect("Checked by clap.");

    let command = matches
        .values_of("command")
        .expect("Checked by clap.")
        .collect::<Vec<&str>>();

    exit(match run(sepb, argname, &command) {
        Ok(exit_code) => exit_code,
        Err(maperr) => {
            eprintln!("{}", maperr.0);
            1
        }
    });
}

fn run(separator: u8, argname: &str, command: &[&str]) -> Result<i32, MapErr> {
    let stdin = std::io::stdin();
    for value in stdin.lock().split(separator) {
        let value = value.map_err(|_| MapErr("io err"))?;
        let value =
            from_utf8(&value).map_err(|_| MapErr("recieved invalid utf8 as argument on stdin"))?;
        let command: Vec<String> = command
            .iter()
            .map(|s| s.to_string().replace(argname, value.into()))
            .collect();
        let (prog, args) = command.split_first().ok_or(MapErr("no command supplied"))?;
        let status = Command::new(prog)
            .args(args)
            .stdin(Stdio::null())
            .status()
            .map_err(|_| MapErr("couldn't execute command"))?;
        if !status.success() {
            return Ok(status.code().unwrap_or(1));
        }
    }
    Ok(0)
}

#[derive(PartialEq, Debug)]
struct MapErr(&'static str);

fn unescape_delimiter(input: &str) -> Result<u8, UnescapeErr> {
    if input.as_bytes().len() == 1 {
        return Ok(input.as_bytes()[0]);
    }
    let result = match input {
        "\\a" => 7,
        "\\b" => 8,
        "\\f" => 0xc,
        "\\n" => b'\n',
        "\\r" => b'\r',
        "\\t" => b'\t',
        "\\v" => 0x0b,
        "\\\\" => b'\\',
        "\\'" => b'\'',
        "\\\"" => b'"',
        "\\?" => b'?',
        "\\e" => 0x1b,
        "\\0" => 0,
        _ => return Err(UnescapeErr),
    };
    return Ok(result);
}

#[derive(PartialEq, Debug)]
struct UnescapeErr;

#[cfg(test)]
mod test {
    use unescape_delimiter;

    #[test]
    fn unescape() {
        unescape_delimiter("").unwrap_err();
        unescape_delimiter("\\n\\n").unwrap_err();
        unescape_delimiter("  ").unwrap_err();
        unescape_delimiter("aa").unwrap_err();
        assert_eq!(unescape_delimiter("\\n"), Ok(b'\n'));
        assert_eq!(unescape_delimiter(" "), Ok(b' '));
        assert_eq!(unescape_delimiter("a"), Ok(b'a'));
        assert_eq!(unescape_delimiter("\\0"), Ok(b'\0'));
    }
}

use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use stdin_readlines::stdin_readlines;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    in_place: bool,
    file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let s: String = {
        if let Some(file) = &opt.file {
            std::fs::read_to_string(file).unwrap()
        } else {
            let mut s = String::new();
            stdin_readlines(&mut s);
            s
        }
    };
    let out: String = match serde_json::from_str::<Value>(&s) {
        Ok(json) => serde_yaml::to_string(&json).unwrap(),
        Err(json_e) => match serde_yaml::from_str::<Value>(&s) {
            Ok(yaml) => serde_json::to_string_pretty(&yaml).unwrap(),
            Err(yaml_e) => panic!("Could not decode json: {json_e}, nor yaml: {yaml_e}"),
        },
    };
    if opt.in_place {
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(opt.file.unwrap())
            .unwrap();
        f.write_all(out.as_bytes()).unwrap();
        f.flush().unwrap();
    } else {
        println!("{out}");
    }
}

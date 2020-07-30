use simplemad::Decoder;
use std::fs::{File, remove_file, remove_dir_all};
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "mp3deleter", about = "Deletes mp3 files with bitrate below 192 kbps.")]
struct Options {
    #[structopt(parse(from_os_str))]
    input: Vec<PathBuf>,

    #[structopt(short = "r", about = "Delete parent directory.")]
    directory: bool,
}


fn main() {
    let opt = Options::from_args();

    for filepath in opt.input {

        let file = File::open(&filepath);
        match file {
            Ok(file) => decode(file, filepath),
            Err(_file) => (),
        }
    }
}


fn decode(file: File, filepath: PathBuf) {
    let opt = Options::from_args();
    let headers = Decoder::decode_interval(file, Duration::from_secs(0), Duration::from_secs(1)).unwrap();

    let mut bitrate = 0;

    for decoding_result in headers {
        match decoding_result {
            Err(_e) => (),
            Ok(frame) => {
                bitrate = frame.bit_rate;
            },
        }
    } 

    if bitrate < 191000 {
        if opt.directory {
            remove_dir_all(&filepath.parent().unwrap());
            println!("Deleting directory {}", &filepath.parent().unwrap().to_str().unwrap());
        }
        else {
            remove_file(&filepath); 
            println!("Deleting {}", &filepath.to_str().unwrap()); 
        }
    }
}

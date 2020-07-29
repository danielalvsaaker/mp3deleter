use simplemad::Decoder;
use std::fs::{File, remove_file};
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mp3deleter", about = "Deletes mp3 files with bitrate below 192 kbps.")]
struct Options {
    #[structopt(parse(from_os_str))]
    input: Vec<PathBuf>,
}

fn main() {
    let opt = Options::from_args();

    for file in opt.input {

        let files = File::open(&file).unwrap();
        let headers = Decoder::decode_interval(files, Duration::from_secs(0), Duration::from_secs(1)).unwrap();

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
            remove_file(&file); 
            println!("Deleting {}", &file.to_str().unwrap()); 
        }
    }
}

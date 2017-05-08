extern crate engiffen;
extern crate image;
extern crate getopts;

use std::io;
use std::io::Write;
use std::{env, fmt, process};
use std::fs::{read_dir, File};
use std::path::PathBuf;
use std::time::{Instant, Duration};
use parse_args::{parse_args, Args, SourceImages};

mod parse_args;

#[derive(Debug)]
enum RuntimeError {
    Directory(PathBuf),
    Destination(String),
    Image(image::ImageError),
    Engiffen(engiffen::Error),
}

impl From<image::ImageError> for RuntimeError {
    fn from(err: image::ImageError) -> RuntimeError {
        RuntimeError::Image(err)
    }
}

impl From<engiffen::Error> for RuntimeError {
    fn from(err: engiffen::Error) -> RuntimeError {
        RuntimeError::Engiffen(err)
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeError::Directory(ref dir) => write!(f, "No such directory {:?}", dir),
            RuntimeError::Destination(ref dst) => write!(f, "Couldn't write to output '{}'", dst),
            RuntimeError::Image(ref e) => write!(f, "Image load error: {}", e),
            RuntimeError::Engiffen(ref e) => e.fmt(f,)
        }
    }
}

fn run_engiffen(args: &Args) -> Result<((String, Duration)), RuntimeError> {
    let source_images = match args.source {
        SourceImages::StartEnd(ref dir, ref start_path, ref end_path) => {
            let start_string = start_path.as_os_str();
            let end_string = end_path.as_os_str();

            let mut files: Vec<_> = read_dir(dir)
                .map_err(|_| RuntimeError::Directory(dir.clone()))?
                .filter_map(|e| e.ok())
                .collect();

            // Filesystem probably already sorted by name, but just in case
            files.sort_by_key(|f| f.file_name());

            files.iter()
            .skip_while(|path| path.file_name() < start_string)
            .take_while(|path| path.file_name() <= end_string)
            .map(|e| e.path())
            .collect()
        },
        SourceImages::List(ref list) => list.into_iter().map(PathBuf::from).collect(),
        SourceImages::StdIn => vec![],
    };

    let imgs: Vec<_> = source_images.iter()
        .map(|path| image::open(&path).ok() )
        .filter_map(|i| i)
        .collect();

    let mut out = File::create(&args.out_file)
        .map_err(|_| RuntimeError::Destination(args.out_file.to_owned()))?;

    let now = Instant::now();
    let gif = engiffen::engiffen(&imgs, args.fps)?;
    gif.write(&mut out)
        .map_err(|_| RuntimeError::Destination(args.out_file.to_owned()))?;
    let duration = now.elapsed();
    Ok((args.out_file.clone(), duration))
}

#[allow(unused_must_use)]
fn main() {
    let arg_strings: Vec<String> = env::args().collect();
    let args = parse_args(&arg_strings).map_err(|e| {
        writeln!(&mut io::stderr(), "{}", e);
        process::exit(1);
    }).unwrap();

    match run_engiffen(&args) {
        Ok((file, duration)) => {
            let ms = duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000000;
            println!("Wrote {} in {} ms", file, ms);
        },
        Err(e) => {
            writeln!(&mut io::stderr(), "{}", e);
            process::exit(1);
        },
    }
}

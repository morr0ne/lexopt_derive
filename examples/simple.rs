use std::{
    ffi::OsString,
    io::{stdout, Write},
    path::PathBuf,
};

use lexopt::Error;
use lexopt_derive::Parser;

#[derive(Debug, Parser)]
struct Args {
    name: String,
    path: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = std::env::args_os();
    let args = Args::parse(args, "Help message")?;

    dbg!(args);

    Ok(())
}

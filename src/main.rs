use std::{ error::Error, process, env, ffi::OsString };
use serde::Deserialize;

mod allocations;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        let result: allocations::AllocationRow = result?;
        let allocation = allocations::convert_row_to_allocation(result);
        println!("{:?}", allocation);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

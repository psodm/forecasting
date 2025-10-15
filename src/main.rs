use std::{ error::Error, process, env, ffi::OsString };
use dialoguer::{Select, Input, theme::ColorfulTheme};
use std::collections::HashMap;

mod allocations;

fn run() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = vec!["Forecast Utilization","Forecast Bench","Show Overallocated","Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&selections)
            .interact()?;
        match selection {
                0 => {
                    let file_path: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter the path to the CSV file")
                        .interact_text()?;
                    let mut rdr = csv::Reader::from_path(file_path)?;
                    for result in rdr.deserialize() {
                        let result: allocations::AllocationRow = result?;
                        let allocation = allocations::convert_row_to_allocation(result);
                        println!("{:?}", allocation);
                    }
                }
                1 => {
                    let file_path: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter the path to the Hard Allocations CSV file")
                        .interact_text()?;
                    let mut allocations = Vec::new();
                    let mut names = Vec::new();
                    let mut rdr = csv::Reader::from_path(file_path)?;
                    for result in rdr.deserialize() {
                        let result: allocations::AllocationRow = result?;
                        let allocation = allocations::convert_row_to_allocation(result);
                        allocations.push(allocation);
                    }
                    for name in allocations::get_unique_names(&allocations) {
                        names.push(name);
                    }
                    println!("\n{:<30}|{:>10}{:>10}{:>10}{:>10}", "Resource", "Week 0", "Week 1", "Week 2", "Week 3");
                    println!("-----------------------------------------------------------------------");
                    for name in names {
                        get_benched_resources(&name, &allocations);
                    }
                    println!("\n");
                }
                2 => {
                    let file_path: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter the path to the CSV file")
                        .interact_text()?;
                    let mut allocations = Vec::new();
                    let mut names = Vec::new();
                    let mut rdr = csv::Reader::from_path(file_path)?;
                    for result in rdr.deserialize() {
                        let result: allocations::AllocationRow = result?;
                        let allocation = allocations::convert_row_to_allocation(result);
                        allocations.push(allocation);
                    }
                    for name in allocations::get_unique_names(&allocations) {
                        names.push(name);
                    }
                    println!("\n{:<30}|{:>10}{:>10}{:>10}{:>10}", "Resource", "Week 0", "Week 1", "Week 2", "Week 3");
                    println!("-----------------------------------------------------------------------");
                    for name in names {
                        get_overallocated_resources(&name, &allocations);
                    }
                    println!("\n");
                }
                3 => {
                    println!("Exiting application.");
                    break;
                }
                _ => unreachable!(), // Should not happen with `Select`
        }
    }
    Ok(())
}

struct WeeklyAllocations {
    week0: f64,
    week1: f64,
    week2: f64,
    week3: f64,
}

fn get_overallocated_resources(name: &String, allocations: &Vec<allocations::Allocation>) {
    let mut allocation_map: HashMap<String, WeeklyAllocations> = HashMap::new();
    for allocation in allocations {
        if &allocation.resource_name == name {
            if allocation_map.get(name).is_none() {
                allocation_map.insert(name.clone(), WeeklyAllocations{
                    week0: allocation.a0,
                    week1: allocation.a1,
                    week2: allocation.a2,
                    week3: allocation.a3,
                });
            } else {
                let entry = allocation_map.get_mut(name).unwrap();
                entry.week0 += allocation.a0;
                entry.week1 += allocation.a1;
                entry.week2 += allocation.a2;
                entry.week3 += allocation.a3;
            }
        }
    }
    for (key, value) in allocation_map.iter() {
        if value.week0 > 41.0 || value.week1 > 41.0 || value.week2 > 41.0 || value.week3 > 41.0 {
            println!("{:<30}|{:>10.0}{:>10.0}{:>10.0}{:>10.0}", key, value.week0, value.week1, value.week2, value.week3);
        }
    }
}

fn get_benched_resources(name: &String, allocations: &Vec<allocations::Allocation>) {
    let mut allocation_map: HashMap<String, WeeklyAllocations> = HashMap::new();
    for allocation in allocations {
        if &allocation.resource_name == name {
            if allocation_map.get(name).is_none() {
                allocation_map.insert(name.clone(), WeeklyAllocations{
                    week0: allocation.a0,
                    week1: allocation.a1,
                    week2: allocation.a2,
                    week3: allocation.a3,
                });
            } else {
                let entry = allocation_map.get_mut(name).unwrap();
                entry.week0 += allocation.a0;
                entry.week1 += allocation.a1;
                entry.week2 += allocation.a2;
                entry.week3 += allocation.a3;
            }
        }
    }
    for (key, value) in allocation_map.iter() {
        if value.week0 < 40.0 || value.week1 < 40.0 || value.week2 < 40.0 || value.week3 < 40.0 {
            println!("{:<30}|{:>10.0}{:>10.0}{:>10.0}{:>10.0}", key, (40.0 - value.week0).abs(), (40.0 - value.week1).abs(), (40.0 - value.week2).abs(), (40.0 - value.week3).abs());
        }
    }
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

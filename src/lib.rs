use std::{env, fs, io};

#[derive(Debug)]
pub struct DataHolder{
    query: String,
    dir_path: String,
    case_sensitive: bool
}

impl DataHolder {
    /// Creates new DataHolder struct
    fn new(mut args: impl Iterator<Item = String>) -> Result<DataHolder, &'static str>{
        let env_var = env::var("IGNORE_CASE").unwrap_or(String::new());
        let b;
        // Program args handling
        args.next(); // program dir
        let dir_path = match args.next() {
            Some(s) => s,
            None => return Err("No directory path given")
        };
        let query = match args.next() {
            Some(s) => s,
            None => return Err("No query given")
        }; 
        if env_var.to_lowercase() == "true" {b = false;}
        else {b = true;}
        Ok(DataHolder{
            query,
            dir_path,
            case_sensitive: b
        })
    }
}

pub fn read_args() -> Result<DataHolder, &'static str>{
    let args = env::args();
    if args.len() != 3 { return Err("This program only accepts 2 arguments: <Directory path> <Query>"); }

    DataHolder::new(args)
}

pub fn return_file_contents(data: DataHolder) -> Result<Vec<String>, io::Error> {
    let file_cont = fs::read_to_string(data.dir_path)?;
    let output;
    let mut output_real: Vec<String> = Vec::new();
    if data.case_sensitive {output = find_in_file_query(&file_cont, &data.query);}
    else {output = find_in_file_query_insensitive(&file_cont, &data.query);}
    for l in output.iter(){
        output_real.push(l.to_string());
    }
    Ok(output_real)
}


fn find_in_file_query_insensitive<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|l| {l.to_lowercase().contains(&query)})
        .collect()
}

fn find_in_file_query<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|l| {l.contains(query)})
        .collect()
}


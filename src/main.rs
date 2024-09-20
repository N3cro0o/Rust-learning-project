use std::process;

fn main() {
    let data = grep_v1::read_args().unwrap_or_else(|err| {
        eprintln!("Error!\n{err}");
        process::exit(1);
    });
    let result = grep_v1::return_file_contents(data).unwrap_or_else(|err| {
        eprintln!("Error!\n{err}");
        process::exit(2);
    });
    let mut mess = String::new();
    if result.is_empty() {mess = "Couldn't find anything".to_string()}
    else {
        for s in result{
            mess.push_str(&s);
            mess.push('\n');
        }
        
    }
    eprintln!("Result:\n{mess}");
}

use std::env;

mod task;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = utils::setup();

    if args.len() == 1 {
        println!("Invalid number of arguments!");
        usage(&args[0], false);
        return;
    } else if args.len() >= 2 {
        let mut tasks = task::Tasks::new();

        match args[1].as_str() {
            "--help" | "-h" => usage(&args[0], true),
            "-a" | "--add" => {
                if let Err(e) = tasks.add_task(&args[2]) {
                    println!("error adding new task: {:?}", e);
                }
            }
            "-r" | "--remove" => {
                let num_str = &args[2];
                let num = num_str.parse().expect("expected argument to be a number");

                if let Err(e) = tasks.remove_task(num) {
                    println!("error removing task: {}", e);
                }
            }
            "-t" | "--todo" => tasks.list_tasks(),
            _ => {
                println!("Invalid argument");
                usage(&args[0], false);
            }
        }
    }
}

fn usage(name: &str, help: bool) {
    if help {
        println!("Command line options: ");
        println!("-h, --help                display this help");
        println!("-a, --add                 add a new item to the todo list");
        println!("-d, --done <item number>  mark a list item as done");
        println!("-l, --list                list itens not completed yet");
        println!("-ld, --list-done          list completed itens");
        return;
    }

    println!("run {} --help for more information", name);
}

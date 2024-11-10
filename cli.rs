use evaluator::eval;
use object::{
    env::Env,
    object::{EvalError, Object},
};
use parser::Parser;
use std::{
    cell::RefCell,
    env, fs,
    io::{self, Write},
    ops::Index,
    rc::Rc,
};

static PROMPT: &str = "(taha) > ";
static RESULT: &str = "=>";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let file_path = args.index(1);

        match fs::read_to_string(file_path) {
            Err(err) => {
                write_line(&err.to_string());
            }
            Ok(file_content) => {
                match run(&file_content) {
                    Err(e) => {
                        write_line(e.as_str());
                    }
                    Ok(result) => {
                        write_line(result.to_string().as_str());
                    },
                }            
            }
        }
    } else if args.len() == 1 {
        start_repl();
    } else {
        write_line("no such file or directory");
        std::process::exit(1);
    }
}

fn write_line(input: &str) {
    println!("{} {}", RESULT, input);
}

pub fn start_repl() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => {
                let result = run(&buf);

                match result {
                    Ok(obj) => {
                        write_line(&obj.to_string());
                    }
                    Err(e) => write_line(&e),
                }
            }
            Err(_) => {
                write_line("failed to read from stdin");
            }
        }
    }
}

fn run(input: &str) -> Result<Rc<Object>, EvalError> {
    let env: Env = Rc::new(RefCell::new(Default::default()));
    let node = Parser::parse(input.to_string()).unwrap();

    eval(node, &env)
}

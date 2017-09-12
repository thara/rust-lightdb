use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
    loop {
        print!("db > ");
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();

        match &input[..1] {
            "." => {
                if !do_meta_command(cmd) {
                    println!("Unrecognized command : {}", cmd);
                    continue
                }
            },
            _ => {
            }
        }

        match prepare_statement(cmd) {
            Ok(statement) => {
                execute_statement(statement);
            },
            Err(msg) => {
                println!("Unrecognized keyword at start of '{}'", msg);
                continue;
            }
        }

        println!("Executed.");
    }
}

fn do_meta_command(cmd: &str) -> bool  {
    match cmd {
        ".exit" => {
            println!("Bye.");
            process::exit(0);
        },
        _ => {
            false
        }
    }
}

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType
}

fn prepare_statement(cmd: &str) -> Result<Statement, &str> {
    match &cmd[..6] {
        "insert" => Ok(Statement{statement_type: StatementType::Insert}),
        "select" => Ok(Statement{statement_type: StatementType::Select}),
        _ => {
            Err(cmd)
        }
    }
}

fn execute_statement(s: Statement) {
    match s.statement_type {
        StatementType::Insert => {
            println!("This is where we would do an insert.")
        },
        StatementType::Select => {
            println!("This is where we would do a select.")
        },
    }

}

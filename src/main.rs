extern crate itertools;
#[macro_use]
extern crate lazy_static;

use std::slice;
use std::io;
use std::io::prelude::*;
use std::process;
use itertools::Itertools;
use itertools::Tuples;
use std::mem;

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
                println!("{}", msg);
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

const COLUMN_USERNAME_SIZE : usize = 32;
const COLUMN_EMAIL_SIZE : usize = 32;

type Id = u32;
type UserName = [u8; COLUMN_USERNAME_SIZE];
type EMail = [u8; COLUMN_EMAIL_SIZE];

struct Row {
    id: u32,
    username: UserName,
    email: EMail,
}

struct RowSize {
    id_size : usize,
    username_size : usize,
    email_size : usize,
}

struct RowLayout {
    id_offset : usize,
    username_offset : usize,
    email_offset : usize,
}

const ID_OFFSET : usize = 0;

lazy_static! {
    static ref ROW_SIZE: RowSize = {
        let id_size = mem::size_of::<Id>();
        let username_size = mem::size_of::<UserName>();
        let email_size = mem::size_of::<EMail>();
        RowSize{
            id_size: id_size,
            username_size: username_size,
            email_size: email_size,
        }
    };

    static ref ROW_LAYOUT: RowLayout = {
        let username_offset = ID_OFFSET.saturating_add(ROW_SIZE.id_size);
        let email_offset = username_offset.saturating_add(ROW_SIZE.username_size);
        RowLayout{
            id_offset: ID_OFFSET,
            username_offset: username_offset,
            email_offset: email_offset,
        }
    };
}

struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<Row>,  // only used by insert statement
}

fn prepare_statement(cmd: &str) -> Result<Statement, String> {
    match &cmd[..6] {
        "insert" => {
            let it = cmd.split(char::is_whitespace);
            // let args = &mut it.skip(1);
            let args = it.skip(1).collect::<Vec<&str>>();

            if args.len() != 3 {
                return Err(String::from("Syntax Error"));
            }

            let id = args[0];
            let username = format!("{: <1$}", args[1], COLUMN_USERNAME_SIZE);
            let email = format!("{: <1$}", args[2], COLUMN_EMAIL_SIZE);

            let mut u: [u8; COLUMN_USERNAME_SIZE] = Default::default();
            u.copy_from_slice(username[..COLUMN_EMAIL_SIZE].as_bytes());

            let mut e: [u8; COLUMN_EMAIL_SIZE] = Default::default();
            e.copy_from_slice(email[..COLUMN_EMAIL_SIZE].as_bytes());

            match id.parse::<u32>() {
                Ok(n) => {
                    let row = Row{id: n, username: u, email: e};
                    let st = Statement{statement_type: StatementType::Insert, row_to_insert: Some(row)};
                    Ok(st)
                },
                Err(_) => {
                    Err(format!("Invalid ID"))
                }
            }
        },
        "select" => Ok(Statement{statement_type: StatementType::Select, row_to_insert: None}),
        _ => {
            Err(format!("Unrecognized keyword at start of {}", cmd))
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

fn serialize_row(ref row : Row, ref mut dest: [u8]) {

}

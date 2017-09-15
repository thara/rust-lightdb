extern crate itertools;
extern crate byteorder;

use std::slice;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::mem;
use std::process;

use itertools::Itertools;
use itertools::Tuples;

use byteorder::{LittleEndian, WriteBytesExt};

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

struct Row {
    id: u32,
    username: [u8; COLUMN_USERNAME_SIZE],
    email: [u8; COLUMN_EMAIL_SIZE],
}

const ID_SIZE : usize = 4;  // Bytes
const USERNAME_SIZE : usize = 4 * COLUMN_USERNAME_SIZE;
const EMAIL_SIZE : usize = 4 * COLUMN_EMAIL_SIZE;

const ID_OFFSET : usize = 0;
const USERNAME_OFFSET : usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET : usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE : usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: u32 = 4096;
const TABLE_MAX_PAGES: u32 = 100;
const ROWS_PER_PAGE: u32 = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: u32 = PAGE_SIZE / ROW_SIZE;

struct Table {
    pages: [u8; TABLE_MAX_PAGES],
    num_rows: u32,
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

fn serialize_row(row : &Row, dest: &mut Vec<u8>) {
    dest.write_u8::<LittleEndian>(row.id).unwrap();
    dest.write_u8::<LittleEndian>(row.username).unwrap();
    dest.write_u8::<LittleEndian>(row.email).unwrap();
}

fn deserialize_row(src: &Vec<u8>, row : &mut Row) {
    let mut rdr = Cursor::new(src);
    row.id = rdr.read_u8::<LittleEndian>().unwrap();
    row.username = rdr.read_u8::<LittleEndian>().unwrap();
    row.email = rdr.read_u8::<LittleEndian>().unwrap();
}

fn row_slot(table: &Table, row_num: u32) {
    let page_num = row_num / ROWS_PER_PAGE;
    let page = table.pages[page_num];
}

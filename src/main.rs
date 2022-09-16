use std::{fs::File, io::Read};

use rusqlite::{Connection, Result as RusqliteResult};

fn main() {
    let file_content = open_file(String::from("test.json")).unwrap();
    database_handler(file_content).unwrap();
}

fn open_file(file: String) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn database_handler(file_content: String) -> RusqliteResult<()> {
    let conn = Connection::open("sample.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Doctor (
            name text, 
            qualification text 
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Patient (
            id integer primary key,
            data text
        )",
        [],
    )?;

    for i in 1..=500 {
        match conn.execute("INSERT INTO Patient (data) VALUES (?1)", &[&file_content]) {
            Ok(_) => {
                println!("[+] Patient added {}", i);
            }
            Err(err) => {
                println!("[-] Failed to inserter {}", i);
                println!("{:?}", err);
            }
        }
    }

    Ok(())
}

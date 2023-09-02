use std::io::{Write, Read, Result, Error, ErrorKind, stdin};
use std::fs::{OpenOptions, create_dir_all};
use std::env::current_exe;


fn get_user_input(prompt: &str) -> Result<String> {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}


fn db_write(out: &mut dyn Write, data_in: &str) -> Result<()> {
    let data_bytes = data_in.as_bytes();
    out.write_all(data_bytes)?;
    out.flush()
}


pub fn alter_schema(schema_in: &str, data_in: &str) -> Result<()> {
    let get_schema = if schema_in.is_empty() {
        get_user_input("Please enter schema name: ")?
    } else {
        schema_in.to_string()
    };

    let mut schema_data = if data_in.is_empty() {
        get_user_input("Please input data: ")?
    } else {
        data_in.to_string()
    };

    schema_data.push('\n');
    let db_path = format!("./db/{}.ndb", get_schema);

    let mut schema_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(db_path)?;

    db_write(&mut schema_file, &schema_data).expect("Failed to write to database");

    Ok(())
}


pub fn create_schema(schema_in: &str) -> Result<()> {
    let new_schema = if schema_in.is_empty() {
        get_user_input("Please enter schema name: ")?
    } else {
        schema_in.to_string()
    };

    let current_path = current_exe()?;
    let parent_dir = current_path.parent().ok_or(Error::new(
            ErrorKind::Other,
            "Failed to get parent directory",
            ))?;

    let db_dir = parent_dir.join("db");
    println!("Attempting to create directory at: {:?}", db_dir);
    create_dir_all(db_dir)?;
    let db_path = format!("./db/{}.ndb", new_schema);

    let mut write_schema = OpenOptions::new()
        .write(true)
        .create(true)
        .open(db_path)?;

    db_write(&mut write_schema, "")?;

    Ok(())
}


pub fn select_schema(schema_in: &str) -> Result<()> {
    let get_schema = if schema_in.is_empty() {
        get_user_input("Please enter schema name: ")?
    } else {
        schema_in.to_string()
    };

    let ndb_path = format!("./db/{}.ndb", get_schema);

    let mut schema_file = OpenOptions::new()
        .read(true)
        .open(ndb_path)?;

    let mut contents = String::new();
    schema_file.read_to_string(&mut contents)?;
    println!("{}", contents);

    Ok(())
}


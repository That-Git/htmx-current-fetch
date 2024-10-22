use std::io::{stdout, Write};
use curl::easy::Easy;

fn main() {
    let mut easy = Easy::new();
    easy.url("https://htmx.org/docs").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

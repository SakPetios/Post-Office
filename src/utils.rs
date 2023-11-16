/*
^ Utilities
*/

use std::{fs, io::Error, path::Path};

use cursive::{
    views::{Dialog, TextView},
    Cursive,
};

pub fn listdir(path: &Path, only_files: bool) -> Vec<String> {
    let entries = fs::read_dir(path);
    let mut files: Vec<String> = vec![];
    for entry in entries.unwrap() {
        let direntry = entry.as_ref().unwrap();
        let name: String;

        if only_files && !direntry.path().is_file() {
            continue;
        }

        name = direntry.file_name().to_str().unwrap().to_string();

        files.push(name)
    }
    files
}

pub fn show_error(cur: &mut Cursive, error: Error, extra: Option<&str>) {
    cur.add_layer(
        Dialog::around(TextView::new(format!(
            "An Error Occured!\n{} error kind: {}{}",
            error,
            error.kind(),
            if extra.is_some() {
                format!("\nextra: {}", extra.unwrap())
            } else {
                String::from("\nNo Extra Data")
            }
        )))
        .button("Quit", close)
        .button("Close", |c| {
            c.pop_layer();
        })
        .title("Error!"),
    )
}

pub fn unimpl(cur: &mut Cursive) {
    cur.add_layer(
        Dialog::around(TextView::new(
            "Sorry This Feature Is Not Yet Implemented :(",
        ))
        .button("Close", |c| {
            c.pop_layer();
        })
        .button("Quit", close),
    )
}
pub fn close(cur: &mut Cursive) {
    cur.add_layer(
        Dialog::around(TextView::new("Are you sure you want to quit?"))
            .button("No", |c| {
                c.pop_layer();
            })
            .button("Yes", |c| c.quit())
            .h_align(cursive::align::HAlign::Center),
    )
}

pub fn info(cur: &mut Cursive,content:&str) {
    cur.add_layer(
        Dialog::around(TextView::new(content)).button("Close", |c| {c.pop_layer();}).button("Quit", close)
    )
}



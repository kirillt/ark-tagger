mod app;
mod model;
mod query;
mod action;
mod message;

#[macro_use]
extern crate lazy_static;

use iced::{Settings, Application};

use std::path::PathBuf;
use std::env;

lazy_static! {
    static ref ROOT: PathBuf =
        env::current_dir().unwrap()
            .canonicalize().unwrap();

    static ref DATA: PathBuf = {
        let mut path = ROOT.clone();
        path.push(DATA_NAME.to_owned());
        path
    };

    static ref DATA_NAME: &'static str = ".ark-tags";
}

fn main() {
    println!("Root: {:?}", *ROOT);
    app::RootWidget::run(Settings::default())
}

#![feature(try_trait)]

mod app;
mod file;
mod model;
mod database;
mod index;
mod utils;

#[macro_use]
extern crate lazy_static;

use iced::{Settings, Application};
use iced::window;

use std::path::PathBuf;
use std::str::FromStr;
use std::env;

lazy_static! {
    static ref DATA_NAME: &'static str = ".ark-tags.data";
    static ref INDEX_NAME: &'static str = ".ark-tags.index";
    //todo: it is assumed that the Database can be persisted
    // separately from the Index; this way it is possible
    // to have an Index for every root and
    // one Database across all roots
}

fn main() {
    let mut args = env::args();
    args.next();

    let root = args.next();
    let root = root
        .map(|path|
            PathBuf::from_str(&path)
                .map_err(|err| println!("WARNING: {}", err))
                .ok())
        .unwrap_or_else(||
            env::current_dir()
                .map_err(|err| println!("WARNING: {}", err))
                .ok());

    let root = root.unwrap()
        .canonicalize().unwrap();

    println!("Root: {:?}", root);

    app::RootWidget::run(Settings {
        window: window::Settings {
            size: (480,480),
            resizable: true,
            decorations: true
        },
        flags: root,

        ..Default::default()
    })
}

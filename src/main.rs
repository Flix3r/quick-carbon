#![windows_subsystem = "windows"]

use enigo::{Enigo, Settings, Direction::*, Key, Keyboard};
use urlencoding::encode;
use clipboard_win::{get_clipboard, formats};
use std::{thread::sleep, time::Duration};
use clap::Parser;

#[derive(Parser)]
struct Args {
    url: Option<String>,

    #[arg(short, long, default_value_t = 10)]
    wait: u64,
}

fn main() {
    let mut keyman = Enigo::new(&Settings::default()).unwrap();

    let args = Args::parse();
    
    // Copy
    keyman.key(Key::Control, Press).unwrap();
    keyman.key(Key::C, Click).unwrap();
    keyman.key(Key::Control, Release).unwrap();

    // Wait for Windows to copy
    sleep(Duration::from_millis(args.wait));

    let carbon_url = args.url.unwrap_or(
        "https://carbon.now.sh/?bg=rgba%28171%2C+184%2C+195%2C+1%29&t=seti&wt=none&l=auto&width=680&ds=true&dsyoff=20px&dsblur=68px&wc=true&wa=true&pv=56px&ph=56px&ln=false&fl=1&fm=Hack&fs=14px&lh=133%25&si=false&es=2x&wm=false"
        .to_string()
    );

    // Open in browser
    open::that(format!("{}&code={}", carbon_url, encode(
        get_clipboard::<String, _>(formats::Unicode)
            .unwrap()
            .as_str()
    ))).unwrap();
}
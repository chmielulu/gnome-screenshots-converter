use notify::{RecursiveMode, Watcher, recommended_watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use serde::Deserialize;
use std::{fs, thread};

#[derive(Deserialize)]
struct Config {
    pathname: String,
}

fn main() {
    let config: Config = toml::from_str(
        &fs::read_to_string("/home/jakub/.config/gnome-screenshots-converter/Config.toml").expect("Something went wrong reading the file"),
    ).expect("Failed to parse config.toml");

    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx).expect("Failed to create watcher");
    watcher.watch((&config.pathname).as_ref(), RecursiveMode::Recursive).expect("Failed to watch directory");

    for event in rx {
        match event {
            Ok(event) => {
                println!("{:?}", event);
                if let Some(path) = event.paths.get(0) {
                    if path.extension().unwrap_or_default() == "png" {
                        if path.exists() {
                            convert_image(&path);
                        }
                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn convert_image(input: &Path) {
    let delay = std::time::Duration::from_millis(50);
    thread::sleep(delay);

    let img = image::open(input).expect("Failed to open image");
    let output_path = input.with_extension("jpg");
    img.save(&output_path).expect("Failed to save image");
    fs::remove_file(input).expect("Failed to remove image");
}

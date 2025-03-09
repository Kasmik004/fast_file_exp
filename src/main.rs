// use std::fs;
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
};
use std::path::Path;

use open;
use std::{fs, io};
use walkdir::WalkDir;
// fn list_files(dir: &str) {
//     let paths = fs::read_dir(Path::new(dir)).unwrap(); // read_dir returns a Result<ReadDir> which contains an iterator of DirEntry, the DirEntry contains the metadata of the file which can be used to get the file name
//     //this is the example of output of read_dir:
//     //Ok(ReadDir { path: "C:\\Users\\user\\Desktop\\Rust\\file_system\\src", inner: Inner { inner: 0x0000000000000000 } })
//     for path in paths {
//         println!("{:?}", path.unwrap().path().display()); //unwrap the result of the iterator and get the path of the file
//     }
// }

fn list_files(path: &str) -> Vec<String> {
    let depth = 1;
    WalkDir::new(path)
        .min_depth(depth)
        .max_depth(depth)
        .into_iter()
        .filter_map(|e| e.ok().map(|entry| entry.path().display().to_string()))
        .collect()
}

fn test() {
    let path = ".";
    let path = Path::new(path);
    println!("{:?}", path.parent().unwrap());
    let paths = fs::read_dir(path).unwrap();
    println!("{:?}", paths);
    for path in paths {
        println!("{:?}", path.unwrap().path().display());
    }
}

fn main() -> io::Result<()> {
    // let dir = "."; //directory path
    // // list_files(dir);
    // test();
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut files = list_files(".");
    let mut selected = 0;

    loop {
        terminal.draw(|f| {
            let size: ratatui::prelude::Rect = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);
            let items: Vec<ListItem> = files
                .iter()
                .enumerate()
                .map(|(i, file)| {
                    if i == selected {
                        ListItem::new(format!("> {}", file))
                    } else {
                        ListItem::new(format!("  {}", file))
                    }
                })
                .collect();

            let list =
                List::new(items).block(Block::default().title("Files").borders(Borders::ALL));
            f.render_widget(list, chunks[0]);
            f.set_cursor(chunks[0].x + 1, chunks[0].y + selected as u16 + 1);
        });

        if let event::Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Down => {
                    if selected < files.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Enter => {
                    if let Some(file) = files.get(selected) {
                        if let Err(err) = open::that(file) {
                            eprintln!("Failed to open file: {}", err);
                        }
                    }
                }
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }
    disable_raw_mode().unwrap(); // Restore terminal
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    Ok(())
}

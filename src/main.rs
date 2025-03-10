// use std::fs;
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::widgets::Block;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Borders, List, ListItem},
};

use std::{
    error::Error,
    path::{Path, PathBuf},
};

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

struct App {
    folders: Vec<PathBuf>,
    files: Vec<PathBuf>,
    selected_folder: usize,
    selected_file: usize,
    current_dir: PathBuf,
}

impl App {
    fn new(initial_path: &str) -> Self {
        let path = PathBuf::from(initial_path);
        let mut folders = list_folders(&path);
        let mut files = list_files(&path);
        App {
            folders,
            files,
            selected_folder: 0,
            selected_file: 0,
            current_dir: path,
        }
    }

    fn change_dir(&mut self, dir: PathBuf) {
        if dir.is_dir() {
            self.current_dir = dir;
            self.folders = list_folders(&self.current_dir);
            self.files = list_files(&self.current_dir);
            self.selected_folder = 0;
            self.selected_file = 0;
        }
    }
}

fn list_folders(path: &Path) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap().path();
            if entry.is_dir() { Some(entry) } else { None }
        })
        .collect()
}

fn list_files(path: &Path) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap().path();
            if entry.is_file() { Some(entry) } else { None }
        })
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

fn main() -> io::Result<(), Box<dyn Error>> {
    // let dir = "."; //directory path
    // // list_files(dir);
    // test();
    enable_raw_mode().unwrap_or_else(|err| {
        eprintln!("Failed to enable raw mode: {}", err);
        std::process::exit(1);
    });
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new(".");

    // let mut files = list_files(".");
    // let mut selected = 0;

    // if files.is_empty() {
    //     eprintln!("No files found");
    //     disable_raw_mode().unwrap(); // Restore terminal
    //     execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    //     return Ok(());
    // }

    loop {
        terminal
            .draw(|f| {
                let size: ratatui::prelude::Rect = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                    .split(size);
                // let items: Vec<ListItem> = files
                //     .iter()
                //     .enumerate()
                //     .map(|(i, file)| {
                //         if i == selected {
                //             ListItem::new(format!("> {}", file))
                //         } else {
                //             ListItem::new(format!("  {}", file))
                //         }
                //     })
                //     .collect();

                let folder_items: Vec<ListItem> = app
                    .folders
                    .iter()
                    .enumerate()
                    .map(|(i, folder)| {
                        let folder_name = folder.file_name().unwrap().to_string_lossy();
                        if i == app.selected_folder {
                            ListItem::new(format!("> {}", folder_name))
                        } else {
                            ListItem::new(format!("  {}", folder_name))
                        }
                    })
                    .collect();

                let folder_list = List::new(folder_items)
                    .block(block::default().title("Folders").borders(Borders::ALL));
                f.render_widget(folder_list, chunks[0]);

                let files_list: Vec<ListItem> = app
                    .files
                    .iter()
                    .enumerate()
                    .map(|(i, file)| {
                        let file_name: std::borrow::Cow<'_, str> =
                            file.file_name().unwrap().to_string_lossy();
                        if i == app.selected_file {
                            ListItem::new(format!("> {}", file_name))
                        } else {
                            ListItem::new(format!("  {}", file_name))
                        }
                    })
                    .collect();

                let file_list = List::new(files_list)
                    .block(block::default().title("Files").borders(Borders::ALL));
                f.render_widget(file_list, chunks[1]);
                f.set_cursor(chunks[1].x + 1, chunks[1].y + app.selected_file as u16 + 1);

                // let list =
                //     List::new(items).block(Block::default().title("Files").borders(Borders::ALL));
                // f.render_widget(list, chunks[0]);
                // f.set_cursor(chunks[0].x + 1, chunks[0].y + selected as u16 + 1);
            })
            .unwrap();

        if let event::Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Down => {
                    if app.selected_file < app.files.len() - 1 {
                        app.selected_file += 1;
                        // selected += 1;
                    }
                }
                KeyCode::Up => {
                    if app.selected_file > 0 {
                        app.selected_file -= 1;
                    }
                }
                KeyCode::Right => {
                    if app.selected_folder < app.folders.len() - 1 {
                        app.selected_folder += 1;
                    }
                }
                KeyCode::Left => {
                    if app.selected_folder > 0 {
                        app.selected_folder -= 1;
                    }
                }
                KeyCode::Enter => {
                    if let Some(file) = app.files.get(app.selected_file) {
                        if let Err(e) = open::that(file) {
                            eprintln!("Failed to open file: {}", e);
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

use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use log::{info, warn};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
};
use rayon::iter::ParallelBridge;
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use open;
use rayon::iter::ParallelIterator;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

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
        let folders = list_folders(&path);
        let files = list_files(&path);
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
        .par_bridge() // Converts the iterator to a parallel iterator
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|entry| entry.is_dir())
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

fn main() -> Result<(), Box<dyn Error>> {
    // env_logger::init();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    enable_raw_mode().unwrap_or_else(|err| {
        eprintln!("Failed to enable raw mode: {}", err);
        std::process::exit(1);
    });
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(".");
    info!("Starting application at: {:?}", app.current_dir);

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(size);

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
                .block(Block::default().title("Folders").borders(Borders::ALL));
            f.render_widget(folder_list, chunks[0]);

            let files_list: Vec<ListItem> = app
                .files
                .iter()
                .enumerate()
                .map(|(i, file)| {
                    let file_name = file.file_name().unwrap().to_string_lossy();
                    if i == app.selected_file {
                        ListItem::new(format!("> {}", file_name))
                    } else {
                        ListItem::new(format!("  {}", file_name))
                    }
                })
                .collect();

            let file_list =
                List::new(files_list).block(Block::default().title("Files").borders(Borders::ALL));
            f.render_widget(file_list, chunks[1]);
            f.set_cursor(chunks[1].x + 1, chunks[1].y + app.selected_file as u16 + 1);
        })?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down => {
                    if app.selected_file < app.files.len() - 1 {
                        app.selected_file += 1;
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
                        info!("Opening file: {:?}", file);
                        if let Err(e) = open::that(file) {
                            eprintln!("Failed to open file: {}", e);
                            warn!("Failed to open file: {}", e);
                        }
                    }
                }
                KeyCode::Tab => {
                    if let Some(folder) = app.folders.get(app.selected_folder) {
                        info!("Changing directory to: {:?}", folder);
                        app.change_dir(folder.clone());
                    }
                }
                KeyCode::Backspace => {
                    if let Some(parent_dir) = app.current_dir.parent() {
                        let parent_dir = parent_dir.to_path_buf();
                        app.change_dir(parent_dir.clone());
                        info!("Changing directory to: {:?}", &parent_dir);
                    }
                }
                KeyCode::Char('q') => {
                    warn!("Quitting application");
                    break;
                }
                _ => {}
            }
        }
    }
    disable_raw_mode()?; // Restore terminal
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

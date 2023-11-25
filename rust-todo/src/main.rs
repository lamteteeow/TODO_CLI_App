// This script using "ncurses" is to compare with python-todo app using "click" in CLI
use ncurses::*;
use std::cmp;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::ops::{Add, Mul};
use std::process;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

// type Id = usize;

enum LayoutKind {
    Vert,
    Hori,
}

struct Layout {
    kind: LayoutKind,
    pos: Vec2,
    size: Vec2,
}

impl Layout {
    fn available_pos(&self) -> Vec2 {
        use LayoutKind::*;
        match self.kind {
            Hori => self.pos + self.size * Vec2::new(1, 0),
            Vert => self.pos + self.size * Vec2::new(0, 1),
        }
    }

    fn add_widget(&mut self, size: Vec2) {
        use LayoutKind::*;
        match self.kind {
            Hori => {
                self.size.x += size.x;
                self.size.y = cmp::max(self.size.y, size.y);
            }
            Vert => {
                self.size.x = cmp::max(self.size.x, size.x);
                self.size.y += size.y;
            }
        }
    }
}

#[derive(Default, Copy, Clone)]
struct Vec2 {
    y: i32,
    x: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

#[derive(Default)]
struct Ui {
    // list_curr: Option<Id>,
    layouts_stack: Vec<Layout>,
}

impl Ui {
    fn begin(&mut self, pos: Vec2, kind: LayoutKind) {
        assert!(self.layouts_stack.is_empty());
        self.layouts_stack.push(Layout {
            kind,
            pos,
            size: Vec2::new(0, 0),
        });
    }

    fn begin_layout(&mut self, kind: LayoutKind) {
        let layout = self
            .layouts_stack
            .last()
            .expect("Can not create a layout outside of UI::begin() and UI::end()");
        let pos = layout.available_pos();
        self.layouts_stack.push(Layout {
            kind,
            pos,
            size: Vec2::new(0, 0),
        });
    }

    fn label(&mut self, text: &str, pair: i16) {
        let layout = self
            .layouts_stack
            .last_mut()
            .expect("Trying to render label outside of any layout");
        let pos = layout.available_pos();

        mv(pos.y, pos.x);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));

        layout.add_widget(Vec2::new(text.len() as i32, 1));
    }

    fn end_layout(&mut self) {
        let layout = self
            .layouts_stack
            .pop()
            .expect("Unbalanced UI::begin_layout() and UI::end_layout()");
        self.layouts_stack
            .last_mut()
            .expect("Unbalanced UI::begin() and UI::end() calls.")
            .add_widget(layout.size);
    }

    fn end(&mut self) {
        self.layouts_stack
            .pop()
            .expect("Unbalanced UI::begin() and UI::end() calls.");
    }

    // fn begin_list(&mut self, id: Id) {
    //     assert!(self.list_curr.is_none(), "Nested lists are not allowed!");
    //     self.list_curr = Some(id);
    // }

    // fn list_element(&mut self, label: &str, id: Id) -> bool {
    //     let id_curr = self
    //         .list_curr
    //         .expect("Not allowed to create list elements outside of lists");
    //     self.label(label, {
    //         if id_curr == id {
    //             HIGHLIGHT_PAIR
    //         } else {
    //             REGULAR_PAIR
    //         }
    //     });
    //     return false;
    // }

    // fn end_list(&mut self) {
    //     self.list_curr = None;
    // }
}

#[derive(Debug, PartialEq)]
enum Status {
    Done,
    Todo,
}

impl Status {
    fn toggle(&self) -> Self {
        match self {
            Status::Done => Status::Todo,
            Status::Todo => Status::Done,
        }
    }
}

fn parse_item(line: &str) -> Option<(Status, &str)> {
    // let todo_prefix = "TODO: ";
    // let done_prefix = "DONE: ";

    // if line.starts_with(todo_prefix) {
    //     return Some((Status::Todo, &line[todo_prefix.len()..]));
    // }

    // if line.starts_with(done_prefix) {
    //     return Some((Status::Done, &line[done_prefix.len()..]));
    // }
    // return None;
    let todo_item = line
        .strip_prefix("TODO: ")
        .map(|title| (Status::Todo, title));
    let done_item = line
        .strip_prefix("DONE: ")
        .map(|title| (Status::Done, title));
    todo_item.or(done_item)
}

fn list_up(list_curr: &mut usize) {
    if *list_curr > 0 {
        *list_curr -= 1;
    }
}

fn list_down(list: &Vec<String>, list_curr: &mut usize) {
    if *list_curr + 1 < list.len() {
        *list_curr += 1
    }
}

fn list_transfer(
    list_dst: &mut Vec<String>,
    list_src: &mut Vec<String>,
    list_src_curr: &mut usize,
) {
    if *list_src_curr < list_src.len() {
        list_dst.push(list_src.remove(*list_src_curr));
        if *list_src_curr >= list_src.len() && !list_src.is_empty() {
            *list_src_curr = list_src.len() - 1;
        }
    }
}

fn load_state(todos: &mut Vec<String>, dones: &mut Vec<String>, file_path: &str) -> io::Result<()> {
    let file = File::open(file_path).unwrap(); //open() by default borrows file_path already
    for (index, line) in BufReader::new(file).lines().enumerate() {
        match parse_item(&line.unwrap()) {
            Some((Status::Todo, title)) => todos.push(title.to_string()),
            Some((Status::Done, title)) => dones.push(title.to_string()),
            None => {
                eprint!("{}:{}: ERROR: ill-formed item line", file_path, index + 1);
                process::exit(1);
            }
        }
    }
    Ok(())
}

fn save_state(todos: &Vec<String>, dones: &Vec<String>, file_path: &str) {
    let mut file = File::create(&file_path).unwrap();
    for todo in todos.iter() {
        writeln!(file, "TODO: {}", todo).unwrap();
    }
    for done in dones.iter() {
        writeln!(file, "DONE: {}", done).unwrap();
    }
}

// TODO: add new items
// TODO: edit items
// TODO: delete items
// TODO: highlight importance base on keyinput 1-5
// TODO: keep state on SIGINT (Ctrl C)
// TODO: undo system
// TODO: track date when moved to DONE

fn main() {
    let mut args = env::args();
    args.next().unwrap();

    let file_path = {
        match args.next() {
            Some(file_path) => file_path,
            None => {
                eprint!("Usage: todo-rs <file-path>");
                eprint!("ERROR: FILE PATH NOT PROVIDED!");
                process::exit(1);
            }
        }
    };

    let mut todos = Vec::<String>::new();
    let mut todo_curr: usize = 0;
    let mut dones = Vec::<String>::new();
    let mut done_curr: usize = 0;

    let mut notification: String;

    // load_state(&mut todos, &mut dones, &file_path);
    match load_state(&mut todos, &mut dones, &file_path) {
        Ok(()) => notification = format!("Loaded file {}", file_path),
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                notification = format!("New file {}", file_path)
            } else {
                panic!(
                    "Could not load state from file `{}`: {:?}",
                    file_path, error
                );
            }
        }
    };

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;

    // let mut todos: Vec<String> = vec![
    //     "first".to_string(),
    //     "second".to_string(),
    //     "third".to_string(),
    // ];
    // let mut dones: Vec<String> = vec![
    //     "fourth".to_string(),
    //     "fifth".to_string(),
    //     "sixth".to_string(),
    // ];

    let mut ui = Ui::default();

    let mut tab = Status::Todo;

    while !quit {
        erase();

        ui.begin(Vec2::new(0, 0), LayoutKind::Hori);
        {
            ui.begin_layout(LayoutKind::Vert);
            {
                ui.label("TODO", REGULAR_PAIR);
                // ui.label("----------------------------", REGULAR_PAIR);
                // ui.begin_list(todo_curr); //& borrow is fine
                for (index, todo) in todos.iter().enumerate() {
                    ui.label(
                        &format!("- [ ] {}", todo),
                        if index == todo_curr && tab == Status::Todo {
                            HIGHLIGHT_PAIR
                        } else {
                            REGULAR_PAIR
                        },
                    );
                }
            }
            ui.end_layout();

            // ui.end_list();

            // ui.label("----------------------------", REGULAR_PAIR);
            ui.begin_layout(LayoutKind::Vert);
            {
                ui.label("DONE", REGULAR_PAIR);
                // ui.label("----------------------------", REGULAR_PAIR);
                // ui.begin_list(done_curr);
                for (index, done) in dones.iter().enumerate() {
                    ui.label(
                        &format!("- [x] {}", done),
                        if index == todo_curr && tab == Status::Done {
                            HIGHLIGHT_PAIR
                        } else {
                            REGULAR_PAIR
                        },
                    );
                }
                // ui.end_list();
            }
            ui.end_layout();
            // ui.label("----------------------------", REGULAR_PAIR);
        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            // 'e' => {
            //     // Will not create and override existed file => No updates
            //     let mut file = File::create("TODO").unwrap();
            //     for todo in todos.iter() {
            //         writeln!(file, "TODO: {}", todo);
            //     }
            //     for done in todos.iter() {
            //         writeln!(file, "DONE: {}", done);
            //     }
            // }
            'w' => match tab {
                Status::Todo => list_up(&mut todo_curr),
                Status::Done => list_up(&mut done_curr),
            },
            's' => match tab {
                Status::Todo => list_down(&todos, &mut todo_curr),
                Status::Done => list_down(&dones, &mut done_curr),
            },
            '\n' => match tab {
                Status::Todo => list_transfer(&mut dones, &mut todos, &mut todo_curr),
                Status::Done => list_transfer(&mut todos, &mut dones, &mut done_curr),
            },
            '\t' => {
                tab = tab.toggle();
            }
            _ => {}
        }
    }

    save_state(&mut todos, &mut dones, &file_path);

    endwin();
}

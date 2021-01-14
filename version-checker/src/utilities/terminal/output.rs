use crate::utilities::terminal::output::OutputDisplayMode::{Table, Tree};
use crate::utilities::errors::{Errors, VerificationError};
use std::process::exit;
use crate::management::crates_io::Dependency;

pub enum OutputDisplayType {
    Blank,
    Entry,
    Title,
    Header,
    Guide,
    End,
}

pub struct DisplayLine {
    pub display_type: OutputDisplayType,
    pub cells: Vec<DisplayCell>,
}

pub struct DisplayCell {
    pub text: String,
    pub width: usize,
    pub color: String,
}

pub struct OutputManager {
    pub display_mode: OutputDisplayMode,
    pub display_width: usize,
}

pub enum OutputDisplayMode {
    Tree,
    Table,
}

impl OutputManager {
    pub fn new(mode: u8, width: usize) -> OutputManager {
        let mut man = OutputManager {
            display_mode: OutputDisplayMode::Table,
            display_width: width,
        };

        match mode {
            0 => {
                man.display_mode = Table;
            }
            1 => {
                man.display_mode = Tree;
            }
            _ => {
                man.display_mode = Table;
            }
        };

        man
    }

    pub fn render(&self, content: DisplayLine) {
        match content.display_type {
            OutputDisplayType::Blank => {
                println!();
            }
            OutputDisplayType::Entry => {
                let mut index = 0;
                for mut cell in content.cells {
                    match index {
                        0 => {
                            while cell.text.len() < cell.width {
                                cell.text = format!(" {}", cell.text);
                            }
                            print!(" \x1b[90;1m║\x1b[0m {}{}\x1b[0m \x1b[90;1m│\x1b[0m ", cell.color, cell.text)
                        }
                        1 => {
                            let mut border = "".to_string();

                            while (border.len() + (cell.text.len() + 4)) < cell.width {
                                border = format!("{} ", border);
                            }

                            print!("{}{}\x1b[0m{} \x1b[90;1m│\x1b[0m ", cell.color, cell.text, border);
                        }
                        2 => {
                            let mut border = "".to_string();

                            while (border.len() + (cell.text.len() + 4)) < cell.width {
                                border = format!("{} ", border);
                            }

                            print!("{}{}\x1b[0m{} \x1b[90;1m│\x1b[0m ", cell.color, cell.text, border);
                        }
                        3 => {
                            let mut border = "".to_string();

                            while (border.len() + (cell.text.len() + 4)) < cell.width {
                                border = format!("{} ", border);
                            }

                            print!("{}{}\x1b[0m{} \x1b[90;1m║\x1b[0m ", cell.color, cell.text, border);
                        }
                        _ => {}
                    }
                    index += 1;
                }
                println!()
            }
            OutputDisplayType::Title => {
                print!(" \x1b[90;1m╔");
                for _ in 0..((self.display_width - (content.cells[0].text.len() + 4)) / 2) - 1 {
                    print!("═");
                }

                print!("╡ {} ╞", content.cells[0].text);

                for _ in 0..((self.display_width - (content.cells[0].text.len() + 4)) / 2) - 1 {
                    print!("═");
                }
                println!("╗\x1b[0m");
            }
            OutputDisplayType::Guide => {
                print!(" \x1b[90;1m╟");
                for index in 0..self.display_width - 2 {
                    if index == 13 || index == 62 || index == 86 {
                        print!("┼");
                    }  else {
                        print!("─");
                    }
                }
                println!("╢\x1b[0m");
            }
            OutputDisplayType::Header => {}
            OutputDisplayType::End => {
                print!(" \x1b[90;1m╚");
                for _ in 0..self.display_width - 2 {
                    print!("═");
                }
                println!("╝\x1b[0m");
            }
        }
    }

    pub fn debug_error(&self, content: VerificationError) {
        println!("{:?}", content);
    }

    pub fn error(&self, content: VerificationError) {
        println!("{:?}", content);
        exit(1)
    }
}


impl DisplayLine {
    pub fn new_guide() -> DisplayLine {
        DisplayLine {
            display_type: OutputDisplayType::Guide,
            cells: vec![],
        }
    }

    pub fn new_table_end() -> DisplayLine {
        DisplayLine {
            display_type: OutputDisplayType::End,
            cells: vec![],
        }
    }

    pub fn new_title(title: &str) -> DisplayLine {
        DisplayLine {
            display_type: OutputDisplayType::Title,
            cells: vec![DisplayCell {
                text: title.to_string(),
                width: 0,
                color: "\x1b[37m".to_string(),
            }],
        }
    }

    pub fn new_crate(dep: Dependency) -> DisplayLine {
        DisplayLine {
            display_type: OutputDisplayType::Entry,
            cells: vec![
                DisplayCell {
                    text: "0".to_string(),
                    width: 11,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: dep.name,
                    width: 50,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: dep.version.to_string(),
                    width: 25,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: dep.remote.to_string(),
                    width: 25,
                    color: "\x1b[37m".to_string(),
                }
            ],
        }
    }

    pub fn new_header() -> DisplayLine {
        DisplayLine {
            display_type: OutputDisplayType::Entry,
            cells: vec![
                DisplayCell {
                    text: "Advisories".to_string(),
                    width: 11,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: "Dependency".to_string(),
                    width: 50,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: "Version".to_string(),
                    width: 25,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: "Latest".to_string(),
                    width: 25,
                    color: "\x1b[37m".to_string(),
                }
            ],
        }
    }

    pub fn new_footer() -> DisplayLine {
        DisplayLine {
            display_type: OutputDisplayType::Entry,
            cells: vec![
                DisplayCell {
                    text: "Advisories".to_string(),
                    width: 11,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: "Total Dependencies".to_string(),
                    width: 50,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: "Up To Date".to_string(),
                    width: 25,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: "Out Of Date".to_string(),
                    width: 25,
                    color: "\x1b[37m".to_string(),
                }
            ],
        }
    }

    pub fn new_footer_content(advisories: u16, utd: u16, ood: u16, warn: u16) -> DisplayLine {
        DisplayLine {
            display_type: OutputDisplayType::Entry,
            cells: vec![
                DisplayCell {
                    text: format!("{}", advisories),
                    width: 11,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: format!("{}", utd + ood + warn),
                    width: 50,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: format!("{}", utd),
                    width: 25,
                    color: "\x1b[37m".to_string(),
                },
                DisplayCell {
                    text: format!("{}", ood),
                    width: 25,
                    color: "\x1b[37m".to_string(),
                }
            ],
        }
    }
}
use crate::utilities::terminal::output::OutputDisplayMode::{Table, Tree};
use crate::utilities::errors::{Errors, VerificationError};
use std::process::exit;
use crate::management::crates_io::{Dependency, Version};

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

    pub fn warn_update(&self, current: Version, latest: Version) {
        let message = format!("A new update is available to install\n{} -> {}\nUse cargo update to install it", current, latest);
        print!("\x1b[90;1m╔");
        for p in 0..50 {
            print!("═")
        }
        println!("╗");

        for line in message.split("\n") {
            print!("║");
            for x in 0..(25 - line.len()/2) {
                print!(" ");
            }
            if line.contains(" -> ") {
                let halves: Vec<&str> = line.split(" -> ").collect();
                print!("\x1b[31m{}\x1b[35m -> \x1b[32m{}\x1b[90;1m", halves[0], halves[1]);
            } else if line.contains("cargo update") {
                let halves: Vec<&str> = line.split(" cargo update ").collect();
                print!("\x1b[35m{}\x1b[33m cargo update \x1b[35m{}\x1b[90;1m", halves[0], halves[1]);
            }else {
                print!("\x1b[35m{}\x1b[90;1m", line);
            }
            for x in 0..(25 - line.len()/2) {
                print!(" ");
            }
            println!("║");
        }

        print!("╚");
        for p in 0..50 {
            print!("═")
        }
        println!("╝\x1b[0m");
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
                    } else {
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
                color: "\x1b[36m".to_string(),
            }],
        }
    }

    pub fn new_crate(dep: Dependency, advisories: &u16) -> DisplayLine {
        DisplayLine {
            display_type: OutputDisplayType::Entry,
            cells: vec![
                DisplayCell {
                    text: format!("{}", advisories),
                    width: 11,
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: dep.name,
                    width: 50,
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: dep.version.to_string(),
                    width: 25,
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: dep.remote.to_string(),
                    width: 25,
                    color: "\x1b[36m".to_string(),
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
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: "Dependency".to_string(),
                    width: 50,
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: "Version".to_string(),
                    width: 25,
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: "Latest".to_string(),
                    width: 25,
                    color: "\x1b[36m".to_string(),
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
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: "Total Dependencies".to_string(),
                    width: 50,
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: "Up To Date".to_string(),
                    width: 25,
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: "Out Of Date".to_string(),
                    width: 25,
                    color: "\x1b[36m".to_string(),
                }
            ],
        }
    }

    pub fn new_footer_content(utd: u16, ood: u16, advisories: u16, warn: u16) -> DisplayLine {
        let mut d = DisplayLine {
            display_type: OutputDisplayType::Entry,
            cells: vec![
                DisplayCell {
                    text: format!("{}", advisories),
                    width: 11,
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: format!("{}", utd + ood + warn),
                    width: 50,
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: format!("{}", utd),
                    width: 25,
                    color: "\x1b[36m".to_string(),
                },
                DisplayCell {
                    text: format!("{}", ood),
                    width: 25,
                    color: "\x1b[36m".to_string(),
                }
            ],
        };

        if advisories > 0 {
            d.cells[0].color = "\x1b[31m".to_string();
        } else {
            d.cells[0].color = "\x1b[32m".to_string();
        }

        if utd > 0 {
            d.cells[2].color = "\x1b[32m".to_string();
        }

        if ood > 0 {
            d.cells[3].color = "\x1b[31m".to_string();
        }

        d
    }
}
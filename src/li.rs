// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

// pub enum Types {
//     Elitefile,
//     CMake,
//     ScriftLog,
//     ScriftAscii,
//     ScriftSettings,
//     ScriftHistory,
//     Scrift,
//     Cpp,
//     C,
//     Bash,
//     Python,
//     FlaScript,
//     Markdown,
//     FreeBrain,
//     Png,
//     Jpeg,
//     Gif,
//     Html,
//     Rust,
//     Lua,
//     includeLink,
//     File,
//     Directory
// }

use colodot::dot::{DotColors, DotTypes};
use crate::parser;

// TODO: Create 'match file' to
// support another file extension.
//
pub fn match_content(directory: &String, data: &mut parser::Data) {
    if std::fs::metadata(directory).unwrap().is_dir() {
        colodot!(DotColors::LightBlue, DotTypes::Bold,
                "[Direc]:     ");

        colodot!(DotColors::LightWhite, DotTypes::Bold,
            directory);

        println!();

        return;
    }

    match directory.as_str() {
        "Elitefile" => {
            colodot!(DotColors::Blue, DotTypes::Bold,
                "[Elitebuild]:");
        },
        "CMakeLists.txt" => {
            colodot!(DotColors::Blue, DotTypes::Bold,
                "[CMake]:     ");
        },
        _ => {
            if std::path::Path::new(directory).extension() == None {
                return;
            }

            let extension = std::ffi::OsStr::to_str(
                std::path::Path::new(directory)
                    .extension()
                    .unwrap())
                .unwrap();

            match extension {
                "scrift_log"      => {
                    colodot!(DotColors::Yellow, DotTypes::Bold,
                        "FeLog*:      ");
                },
                "scrift_ascii"    => {
                    colodot!(DotColors::Yellow, DotTypes::Bold,
                        "Ascii Art*:  ");
                },
                "scrift_settings" => {
                    colodot!(DotColors::Yellow, DotTypes::Bold,
                        "Settings*:   ");
                },
                "scrift_history" => {
                    colodot!(DotColors::Yellow, DotTypes::Bold,
                        "History*:    ");
                },
                "scr"            => {
                    colodot!(DotColors::Green, DotTypes::Bold,
                        "[Scrift]:    ");
                },
                "cpp" |
                "hpp" |
                "cxx" |
                "hxx" |
                "cc"  |
                "hh"             => {
                    colodot!(DotColors::Cyan, DotTypes::Bold,
                        "[C++]:       ");
                },
                "c"   |
                "h"              => {
                    colodot!(DotColors::Blue, DotTypes::Bold,
                        "[C]:         ");
                },
                "sh"  |
                "bash"           => {
                    colodot!(DotColors::Green, DotTypes::Bold,
                        "[Bash]:      ");
                },
                "py"  |
                "pyc"            => {
                    colodot!(DotColors::Blue, DotTypes::Bold,
                        "[Python]:    ");
                },
                "fls" |
                "flsh"           => {
                    colodot!(DotColors::Yellow, DotTypes::Bold,
                        "[FlaScript]: ");
                },
                "md"             => {
                    colodot!(DotColors::Yellow, DotTypes::Bold,
                        "[Markdown]:  ");
                },
                "frbr"           => {
                    colodot!(DotColors::Magenta, DotTypes::Bold,
                        "[FreeBr]:    ");
                },
                "png"            => {
                    colodot!(DotColors::Blue, DotTypes::Bold,
                        "[Png]:       ");
                },
                "jpg" |
                "jpeg"           => {
                    colodot!(DotColors::Blue, DotTypes::Bold,
                        "[Jpg]:       ");
                },
                "gif"            => {
                    colodot!(DotColors::Blue, DotTypes::Bold,
                        "[Gif]:       ");
                },
                "htm" |
                "html"           => {
                    colodot!(DotColors::Magenta, DotTypes::Bold,
                        "[Html]:      ");
                },
                "rs"  |
                "rslib"          => {
                    colodot!(DotColors::Yellow, DotTypes::Bold,
                        "[Rust]:      ");
                },
                "lua"            => {
                    colodot!(DotColors::Black, DotTypes::Bold,
                        "[Lua]:       ");
                },
                "inclink"        => {
                    colodot!(DotColors::Yellow, DotTypes::Bold,
                        "[incLink]:   ");
                },
                _ => {
                    let info = data.get(extension.to_string());

                    colodot!(info.1, DotTypes::Bold,
                        &info.0);
                }
            }
        }
    }

    colodot!(DotColors::White, DotTypes::Bold,
        directory);

    println!();
}

pub fn li() {
    let paths = std::fs::read_dir(".").unwrap();
    let mut data: parser::Data = parser::Data{everything: Vec::new()};

    data.read_data();

    for path in paths {
        let lol = path.unwrap().file_name().to_str().unwrap().to_string();

        match_content(&lol, &mut data);
    }
}
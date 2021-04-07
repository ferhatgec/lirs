// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

// Parser for Li[dot]rs
//
// Syntax:
//
// >'[Kalem]:     '< >"kalem"< >{95}<
// ^^^^^^^^^      ^^^^^^^^^^^  ^^^^^
// >'name'<       >"extension"< >{ansi_color}<

use std::str::FromStr;

#[derive(Clone)]
pub struct Extension {
    pub name: String,
    pub extension: String,
    pub color: i32
}

#[derive(Clone)]
pub struct Data {
    pub everything: Vec<Extension>
}

impl Data {
    pub fn between(data: &String, substring: &str, substring2: &str) -> String {
        data[
            data.find(substring)
                .unwrap_or(0)
            ..
            data.find(substring2)
                .unwrap_or(data.len())
        ].to_string().replace(substring, "")
    }

    pub fn get(&mut self, extension: String) -> (String, i32) {
        let data: Vec<Extension> = self.everything.clone();

        for element in data {
            if element.extension == extension {
                return (element.name, element.color);
            }
        }

        ("[File]:      ".to_string(), 34)
    }

    pub fn parse(&mut self, data: String) {
        let color: i32 = FromStr::from_str(
            Data::between(&data, ">{", "}<")
                .as_str()
        ).unwrap_or(34);

        let ext = Extension {
            name: Data::between(&data, ">'", "'<"),
            extension: Data::between(&data, ">\"", "\"<"),
            color: color
        };


        self.everything.push(ext);
    }

    pub fn read_data(&mut self) {
        if std::path::Path::new(
            format!("{}/{}",
                    std::env::var("HOME").unwrap(),
                    ".lirs_data").as_str()).exists() {
            let data = std::fs::read_to_string(
                format!("{}/{}",
                        std::env::var("HOME").unwrap(),
                        ".lirs_data")
            ).expect("Unable to read file");

            for line in data.split('\n') {
                self.parse(line.to_string());
            }
        } else {
            match std::fs::File::create(format!("{}/{}",
                                          std::env::var("HOME").unwrap(),
                                          ".lirs_data").as_str()) {
                Err(why) => panic!("Couldn't create .lirs_data: {}", why),
                _ => {
                    self.read_data();
                }
            }
        }
    }
}
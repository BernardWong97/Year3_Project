//!
//! AppConfig responsible for App configuration (settings).
//! Then created it reads settings from a given file and loads them to
//! `AppConfig.settings` field.
//!
//! # author: Mindaugas Sharskus
//! # date: 18-04-2019
//!


//#![allow(unused_imports)]

#[allow(unused_imports)]
use std::io::Write;
use std::io::{Error, BufRead};
use std::io;
use std::fs;
use std::collections;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug)]
pub struct Config<'a>{
    #[serde(skip)]
    settings_file_name: &'a str,
    settings: collections::HashMap<String, String>,
}

#[allow(dead_code)]
impl <'a>Config<'a>{
    ///
    /// Creates new `AppConfig` from a given file.
    /// ToDo:
    /// - refactor
    ///
    pub fn from(settings_file: &'a str) -> Result<Self, Error>{
        let mut settings = collections::HashMap::new();
        let file = fs::File::open(settings_file)?;
        let buffered = io::BufReader::new(file);

        for line in buffered.lines(){
            let value = line?;
            let semi_pos = value.chars().position(|ch| ch == ':').unwrap_or_else(||{
                panic!("Semicolon not found");
            });

            let (key, value) = value.split_at(semi_pos);
            settings.insert(
                key.trim().to_string(),
                value[1..].trim().to_string()
            );
        }

        let this = Self{
            settings_file_name: settings_file,
            settings,
        };

        Ok(this)
    }

    ///
    /// Gets requested setting value
    ///
    pub fn get_value(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }

    ///
    /// Sets setting key to a given value
    ///
    pub fn set_to(&mut self, key: &str, value: &str ) -> Result<(), Error> {
        self.settings.entry(key.to_string()).and_modify(|e| {
            e.clear();
            e.push_str(value);
        });

        self.save_to_file()
    }

    ///
    /// Saves settings to file
    ///
    #[allow(unused_must_use)]
    fn save_to_file(&self) -> Result<(), Error> {
        let mut file = fs::OpenOptions::new()
            .append(false)
            .write(true)
            .open(self.settings_file_name)?;

        self.settings.iter()
            .for_each(|(k, v)|{
                writeln!(file, "{}:{}", k, v);
            });

        Ok(())
    }

}


/////////////////////////// TESTs ////////////////////////////

#[test]
#[allow(unused_must_use)]
fn test_read_file() -> Result<(), Error> {
    let test_file = "test_file.tmp";
    fs::File::create(test_file)?;
    let mut file = fs::OpenOptions::new().append(true).open(test_file)?;

    let option1 = "key1:value1";
    let option2 = " key2  : value2 ";
    writeln!(file, "{}", option1)?;
    writeln!(file, "{}", option2)?;

    let config = Config::from(test_file)?;
    fs::remove_file(test_file)?;    // house keeping remove temporary file

    println!("{:#?}", config.settings);

    let op1 = option1.split(':')
        .map(|l| l.trim())
        .collect::<Vec<_>>();
    assert!(config.settings.contains_key(op1[0]));
    assert_eq!(Some(&op1[1].to_string()), config.settings.get(op1[0]));

    let op2 = option2.split(':')
        .map(|l| l.trim())
        .collect::<Vec<_>>();
    assert!(config.settings.contains_key(op2[0]));
    assert_eq!(Some(&op2[1].to_string()), config.settings.get(op2[0]));

//    assert!(false);
    Ok(())
}

#[test]
#[allow(unused_must_use)]
fn test_config_get_settings_value() -> Result<(), Error> {
    let test_file = "test_file.tmp";
    let mut file = fs::File::create(test_file)?;

    let test = vec!["key1", "val1", "key2", "val2"];
    writeln!(file, "{}:{}",      test[0], test[1])?;
    writeln!(file, "  {}  : {}", test[2], test[3])?;

    let config = Config::from(test_file)?;
    fs::remove_file(test_file)?;    // house keeping remove temporary file

    assert_eq!(Some(&test[1].to_string()), config.get_value(test[0]));
    assert_eq!(Some(&test[3].to_string()), config.get_value(test[2]));

    println!("{:#?}", config.settings);
//    assert!(false);
    Ok(())
}

#[test]
#[allow(unused_must_use)]
fn test_config_set_settings_value() -> Result<(), Error> {
    let test_file = "test_file.tmp";
    let mut file = fs::File::create(test_file)?;

    let test = vec!["key1", "val1", "key2", "val2"];
    writeln!(file, "{}:{}",      test[0], test[1]);
    writeln!(file, "  {}  : {}", test[2], test[3]);

    let mut config = Config::from(test_file)?;
    config.set_to("key1", "value-1")?;
    config.set_to("key2", "value-2")?;

    let config2 = Config::from(test_file)?;

    assert_eq!(Some(&"value-1".to_string()), config2.get_value(test[0]));
    assert_eq!(Some(&"value-2".to_string()), config2.get_value(test[2]));

    println!("{:#?}", config.settings);
    fs::remove_file(test_file)?;    // house keeping remove temporary file
//    assert!(false);
    Ok(())
}
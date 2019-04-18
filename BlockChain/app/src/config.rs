//!
//! AppConfig responsible for App configuration (settings).
//! Then created it reads settings from a given file and loads them to
//! `AppConfig.settings` field.
//!
//! Author: Mindaugas Sharskus
//! Date: 18-04-2019
//!


//use std::error::Error;
use std::io;
use std::io::{Write, Error, BufRead};
use std::fs;
use std::collections;

use serde::{Deserialize, Serialize, Serializer};
use std::collections::hash_map::Entry;



#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig<'a>{
    settings_file_name: &'a str,
    settings: collections::HashMap<String, String>,
}

#[allow(dead_code)]
impl <'a>AppConfig<'a>{
    ///
    /// Creates new `AppConfig` from a given file.
    ///
    pub fn from(settings_file: &'a str) -> Result<Self, Error>{
        let mut settings = collections::HashMap::new();
        let file = fs::File::open(settings_file)?;
        let buffered = io::BufReader::new(file);

        for line in buffered.lines(){
            let value = line?;
            let entry = value.split(':')
                .map(|l| l.trim())
                .collect::<Vec<_>>();

            if entry.len() > 2 { panic!(format!("more than 1 setting value is found {:#?}", entry)); }

            settings.insert(entry[0].to_string(), entry[1].to_string());
        }

//        settings = buffered.lines()
//            .map(|line| line.unwrap())
//            .map(|l| l.split(':').collect::<Vec<_>>())
//            .map(|g| collections::hash_map::Entry(g[0], g[1]))
//            .collect();

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

}


/////////////////////////// TESTs ////////////////////////////

#[test]
fn test_read_file() -> Result<(), Error> {
    let test_file = "test_file.tmp";
    fs::File::create(test_file)?;
    let mut file = fs::OpenOptions::new().append(true).open(test_file)?;

    let option1 = "key1:value1";
    let option2 = " key2  : value2 ";
    writeln!(file, "{}", option1);
    writeln!(file, "{}", option2);

    let config = AppConfig::from(test_file)?;
    fs::remove_file(test_file)?;

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
fn test_config_get_settings_value() -> Result<(), Error> {
    let test_file = "test_file.tmp";
    let mut file = fs::File::create(test_file)?;

    let test = vec!["key1", "val1", "key2", "val2"];
    writeln!(file, "{}:{}",      test[0], test[1]);
    writeln!(file, "  {}  : {}", test[2], test[3]);

    let config = AppConfig::from(test_file)?;
    fs::remove_file(test_file)?;    // house keeping remove temporary file


    assert_eq!(Some(&test[1].to_string()), config.get_value(test[0]));
    assert_eq!(Some(&test[3].to_string()), config.get_value(test[2]));

    println!("{:#?}", config.settings);
//    assert!(false);
    Ok(())
}
//!
//! AppConfig responsible for App configuration (settings).
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

const APP_FILE:&'static str = "settings_file.txt";


#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig<'a>{
    settings_file_name: &'a str,
    settings: collections::HashMap<String, String>,
}

#[allow(dead_code)]
impl <'a>AppConfig<'a>{

    pub fn get_settings(settings_file: &str) -> Result<Self, Error>{
        let mut settings = collections::HashMap::new();
        let file = fs::File::open(settings_file)?;
        let buffered = io::BufReader::new(file);

        for line in buffered.lines(){
            let value = line?;
            let entry = value.split(':').map(|l| l.trim()).collect::<Vec<_>>();
            settings.insert(entry[0].to_string(), entry[1].to_string());
        }

//        settings = buffered.lines()
//            .map(|line| line.unwrap())
//            .map(|l| l.split(':').collect())
//            .map(|g| g)
//            .collect();

        let this = Self{
            settings_file_name: APP_FILE,
            settings,
        };

        Ok(this)
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

    let config = AppConfig::get_settings(test_file)?;

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
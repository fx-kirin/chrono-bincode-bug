#![feature(map_first_last)]
#![allow(dead_code)]
use std::collections::BTreeMap;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::io::{Error, ErrorKind};
use chrono::serde::ts_nanoseconds;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use chrono::serde::ts_nanoseconds_option;
use anyhow::Result;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumStruct {
    pub value1: Option<i64>,
    pub value2: Option<i64>,
    #[serde(with = "ts_nanoseconds_option")]
    pub time1: Option<DateTime<Utc>>,
    pub status1: Status1,
    pub status2: Status2,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status1 {
    V1 = 0,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status2{
    V1 = 0,
}


fn main() -> Result<()> {
    let mut tmp_struct = EnumStruct{
        value1: None,
        value2: None,
        time1: None,
        status1: Status1::V1,
        status2: Status2::V1,
    };
    
    tmp_struct.time1 = Some(Utc.ymd(1970, 1, 1).and_hms(0, 1, 1));
    tmp_struct.value1 = Some(999);
    tmp_struct.value2 = Some(100);
    let bytes: Vec<u8> = bincode::serialize(&tmp_struct).unwrap();
    
    println!("bytes:{:?}", bytes);
    
    let result = bincode::deserialize::<EnumStruct>(&(bytes))?;
    Ok(())
}

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use ron;
use crate::operations::*;


#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct MidiQuery {
    pub channel: i32,
    pub control: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BindingCommand {
    pub cmd: String,
    pub operations: Vec<Operation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub obs_host: String,
    pub obs_port: String,
    pub bindings: HashMap<MidiQuery, BindingCommand>
}

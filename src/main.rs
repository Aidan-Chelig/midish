mod config;
mod operations;
use std::collections::HashMap;
use std::process::Command;
use anyhow::Result;
use config::*;
use midir::os::unix::VirtualInput;
use midir::{Ignore, MidiInput};
use obws::Client;
use ron;
use std::thread::{sleep, Thread};
use std::time::Duration;
use std::{env, fs, string};
use tokio;

use url::Url;

use std::io::{stdin, stdout, Write};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<()> {
    let file = fs::read_to_string("config.ron").expect("Failed to open config.rs");
    let config: Config = ron::from_str(&file).unwrap_or_else(|err| {
        panic!("Error reading config {:?}", err);
    });
    drop(file);

    println!("{:?}", config);

    let _ = Url::parse(&config.obs_host.as_str()).expect("Bad obs_host Url!");

    env::set_var("RUST_LOG", "obws=debug");

    //    let client = Client::connect("localhost", 4444).await?;
    //
    //    let version = client.general().get_version().await?;
    //    println!("{:#?}", version);

    //    while client.is_err() {
    //        println!("failed to connect: {:?}", client.err());
    //    client = Client::connect("127.0.0.1", 4444).await;
    //        sleep(Duration::from_millis(3000))
    //    }

    match run(config) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    };

    return Ok(());
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("mid2obs")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    println!("\nOpening connection");

    let in_port = midi_in.create_virtual(
        "input",
        move |stamp, message, _| {
            let channel = message[0] & 0xf;
            let msg = message[0] & 0xf0;
            let control = message[1];
            let value = message[2];

            println!(
                "(channel = {}) (msg = {}) (control = {}) (value = {})",
                channel, msg, control, value
            );

            let defualt: &BindingCommand = &BindingCommand{
                cmd: "".to_string(),
                operations: Vec::new()
            };
            let defaultString = "".to_string();

            let binding = config.bindings.get(&MidiQuery {
                channel: channel as i32,
                control: control as i32
            }).unwrap_or(defualt);

            if let Some(operation) = binding.operations.get(0){
                let processedValue = operation.run(value.into());
            let mapping2: &str = &processedValue.to_string();

            let newsh = binding.cmd.replace('_', mapping2);

                println!("{}", newsh);

            Command::new("sh")
            .arg("-c")
            .arg(newsh)
            .spawn()
            .expect("failed to execute process");


            }

//            let mut sh = binding.get("sh").unwrap_or(&defaultString);
//            let map: Vec<&str> = binding.get("map").unwrap_or(&defaultString).split('-').collect();
//
//            let numbermapping: f32 = (value as f32) / 127.;
//

//
//            let minimum: f32 = map[0].parse::<f32>().unwrap();
//            let maximum: f32 = map[1].parse::<f32>().unwrap();
//
//            println!("{}-{}", minimum, maximum);
//
//            println!("mapping {}->{}", value, ((value as f32) / 127.));
//
//
 //          println!("binding = {}", sh);

        },
        (),
    )?;
    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}

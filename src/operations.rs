use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use url::form_urlencoded::parse;

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    Map(
        f32, // min
        f32, // max
    ),
    GreaterThan(f32, Output),
    LessThan(f32, Output),
    EqualTo(f32, Output),
    Index(Vec<Output>),
    Compare(f32, Output, Output, Output)

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Output {
    String(String),
    Number(f32),
    None
}

impl Output {
    pub fn to_string(&self) -> String {
        match self {
            Output::String(out) => out.to_owned(),
            Output::Number(out) => out.to_string(),
            Output::None => "".to_owned(),
        }
    }
}

impl Operation {
    pub fn run(&self, value: f32) -> Output{
        const MAX: f32 = 127.;
        match self {
            Operation::Map(min, max) => {
                Output::Number(min + (max - min) * (value/MAX))
            }

            Operation::GreaterThan(compare, out) => {
                if &value > compare {
                    out.clone()
                } else {
                    Output::None
                }
            }

            Operation::LessThan(compare, out) => {
                if &value < compare {
                    out.clone()
                } else {
                    Output::None
                }
            }

            Operation::EqualTo(compare, out) => {
                if &value == compare {
                    out.clone()
                } else {
                    Output::None
                }
            }

            Operation::Index(array) => {
                let index = ((array.len() - 1) as f32 * value/(MAX - MAX / array.len() as f32)).floor() as usize;

                array[index].clone()
            }

            Operation::Compare(compare, lessthan, greaterthan, equal) => {
                if &value > compare {
                    greaterthan.clone()
                } else if &value < compare {
                    lessthan.clone()
                } else {
                    equal.clone()
                }
            }

            _ => {
                Output::Number(0.)
            }
        }
    }
}

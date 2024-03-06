use std::{fmt::Display, process::Command};

use anyhow::{bail, Ok, Result};
use tracing::{event, Level};

const PROGRAM: &str = "./mock";

#[derive(Debug)]
enum Operation {
    Encode,
    Decode,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::Encode => "encode",
                Operation::Decode => "decode",
            }
        )
    }
}

#[derive(Clone)]
pub(crate) struct AnonyMuseClient;

impl AnonyMuseClient {
    fn program() -> String {
        std::env::var("PROGRAM").unwrap_or(PROGRAM.to_owned())
    }

    fn run(&self, operation: Operation, body: &str) -> Result<String> {
        event!(Level::INFO, "Spawn");
        let output = Command::new(Self::program())
            .arg(operation.to_string())
            .arg("--json")
            .arg(body)
            .output()?;

        if !output.status.success() {
            bail!("Failed to run process: {output:?}");
        }

        Ok(String::from_utf8(output.stdout)?)
    }

    pub fn encode(&self, body: &str) -> Result<String> {
        event!(Level::INFO, "Encoding {body}");

        self.run(Operation::Encode, body)
    }

    pub fn decode(&self, body: &str) -> Result<String> {
        event!(Level::INFO, "Decoding {body}");
        self.run(Operation::Decode, body)
    }
}

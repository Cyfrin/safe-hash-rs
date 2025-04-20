use std::{
    io::{BufWriter, Write},
    process::{Command, Stdio},
};

use crate::Result;

pub struct Eip712Hasher {
    typed_message_string: String,
}

impl Eip712Hasher {
    pub fn new(typed_message_string: String) -> Self {
        Self { typed_message_string }
    }

    pub fn hash(&self) -> Result<String> {
        let mut cmd = Command::new("ts-eel/dist/ts-eel");
        cmd.stdin(Stdio::piped()).stderr(Stdio::piped()).stdout(Stdio::piped());

        let mut child = cmd.spawn()?;

        {
            let mut stdin = BufWriter::new(child.stdin.take().unwrap());
            writeln!(&mut stdin, "{}", &self.typed_message_string)?;
            stdin.flush()?;
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(output.stderr.as_ref()).into());
        }

        let output_str = String::from_utf8_lossy(output.stdout.as_ref());
        Ok(output_str.to_string())
    }
}

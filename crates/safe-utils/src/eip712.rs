use std::{
    io::{BufWriter, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use serde::Deserialize;

use crate::Result;

pub struct Eip712Hasher {
    typed_message_string: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EIP7127HashDetails {
    pub eip_712_hash: String,
    pub domain_hash: String,
    pub message_hash: String,
}

impl Eip712Hasher {
    pub fn new(typed_message_string: String) -> Self {
        Self { typed_message_string }
    }

    pub fn hash(&self) -> Result<EIP7127HashDetails> {
        let mut cmd = Command::new(ts_eel_path());
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
        let eip_712_details: EIP7127HashDetails = serde_json::from_str(&output_str)?;
        Ok(eip_712_details)
    }
}

#[cfg(not(debug_assertions))]
fn ts_eel_path() -> PathBuf {
    use std::{fs::OpenOptions, os::unix::fs::PermissionsExt, path::Path, time::Duration};

    let release = env!("CARGO_PKG_VERSION");
    let tt = target_triple::TARGET;

    let eels_dir = dirs::home_dir().unwrap().join(".cyfrin").join("eels").join(release);
    let eel_tar_file = eels_dir.join(format!("ts-eel-{}.tar.gz", tt));
    let eel_temp_file = eels_dir.join(tt);
    let eel_file = eels_dir.join("ts-eel");

    // If we already downloaded the eel for this version, don't have to re-download
    if eel_file.exists() {
        return eel_file;
    }

    let eel_code = {
        // Github will setup permanant redirect on repo renames. so thos should be fine..
        let url = format!(
            "https://github.com/Cyfrin/safe-hash-rs/releases/download/safe-hash-v{}/ts-eel-{}.tar.gz",
            release, tt
        );
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("unable to create eel client");
        println!("Downloading eel {}", url);
        let response = client.get(url).send();

        if let Ok(response) = response {
            response.bytes().unwrap()
        } else {
            eprintln!("Error while downloading eel: {:#?}", response.err());
            std::process::exit(1);
        }
    };

    std::fs::create_dir_all(&eels_dir).unwrap();

    {
        println!("Writing to tar file {}", eel_tar_file.display());
        // Write the downloaded contents to a local tarball
        let mut file = OpenOptions::new().write(true).create_new(true).open(&eel_tar_file).unwrap();

        file.write(&eel_code).expect("failed to commit eel");

        // Unzip the tarball
        let mut tar = Command::new("tar");
        tar.stdout(Stdio::piped()).stderr(Stdio::piped());

        // Creates eel_temp_file
        tar.arg("-xvzf")
            .arg(format!("ts-eel-{}.tar.gz", tt))
            .current_dir(eels_dir)
            .status()
            .expect("failed to unzip eel");

        // Delete the tarball
        std::fs::remove_file(&eel_tar_file).expect("deleting tarball failed");

        // Rename binary to ts-eel
        println!("Cleaning {}", eel_temp_file.display());
        std::fs::rename(eel_temp_file, &eel_file).expect("unable to rename eel");

        // Make the binary executable
        let make_executable = |path: &Path| -> std::io::Result<()> {
            let mut perms = std::fs::metadata(path)?.permissions();
            perms.set_mode(0o755); // rwxr-xr-x
            std::fs::set_permissions(path, perms)?;
            Ok(())
        };

        _ = make_executable(&eel_file); // don't complain if it fails
    }

    assert!(eel_file.exists());
    eel_file
}

#[cfg(debug_assertions)]
fn ts_eel_path() -> PathBuf {
    use std::str::FromStr;
    let filepath =
        PathBuf::from_str(concat!(env!("CARGO_MANIFEST_DIR"), "/../../ts-eel/dist/ts-eel"))
            .unwrap();
    assert!(filepath.exists());
    filepath
}

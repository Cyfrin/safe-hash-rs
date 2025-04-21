use std::{
    io::{BufWriter, Write},
    path::PathBuf,
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
        Ok(output_str.to_string())
    }
}

#[cfg(not(debug_assertions))]
fn ts_eel_path() -> PathBuf {
    use std::{fs::OpenOptions, os::unix::fs::PermissionsExt, path::Path, time::Duration};

    let release = env!("CARGO_PKG_VERSION");
    let tt = target_triple::TARGET;
    let th = target_triple::HOST;

    let eels_dir = dirs::home_dir().unwrap().join(".cyfrin").join("eels").join(release);
    let eel_tar_file = eels_dir.join(format!("ts-eel-{}.tar.gz", tt));
    let eel_temp_file = eels_dir.join(th);
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
        // Write the downloaded contents to a local tarball
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(true)
            .open(&eel_tar_file)
            .unwrap();

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
        assert!(eel_temp_file.exists());
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
    PathBuf::from_str(concat!("ts-eel/dist/ts-eel")).expect("build failed")
}

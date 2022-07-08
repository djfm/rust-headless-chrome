use std::env;

use headless_chrome::{
    browser::default_executable,
    Browser,
};

use anyhow::Result;

#[test]
fn connect_to_url() -> Result<()> {
    let mut child = None;

    // let debug_ws_url = env::args().nth(1).expect("Must provide debug_ws_url");
    let debug_ws_url = env::var("DEBUG_WS_URL").unwrap_or_else(|_| {
        let debug_port = portpicker::pick_unused_port().unwrap();
        let executable_path = default_executable().unwrap();
        let port_option = format!("--remote-debugging-port={}", debug_port);
        let mut command = std::process::Command::new(&executable_path);
        child = Some(command.args(&[port_option, "--headless".to_owned()]).stderr(std::process::Stdio::piped()).spawn().unwrap());
        let re = regex::Regex::new(r"\sws://(.*):(\d+)").unwrap();

        let mut tries = 0;
        loop {
            tries += 1;
            let mut output = String::new();
            child.unwrap().stdout.unwrap().(&output).unwrap();

            match re.captures(&output) {
                Some(captures) => {
                    return captures.get(1).unwrap().as_str().to_owned();
                }
                None => {
                    if tries > 4 {
                        panic!("Could not find debug_ws_url");
                    }
                    continue;
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    let browser = Browser::connect(debug_ws_url.to_string());

    assert!(browser.is_ok());

    match child {
        Some(mut child) => {
            child.kill().unwrap();
        }
        None => {}
    }

    Ok(())
}

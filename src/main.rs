use std::time::Duration;

fn parse_terminal_input(input: String) -> Result<(u64, String), String> {
    let input = &input[..].split(",").collect::<Vec<_>>();
    let interval = input.get(0);
    let url = input.get(1);

    match (interval, url) {
        (Some(interval), Some(url)) => {
            if let Ok(interval) = interval.parse::<u64>() {
                Ok((interval, url.trim().to_owned().to_owned()))
            } else {
                Err(format!("Interval {interval} and url {url} are invalid"))
            }
        }
        _ => Err(format!("Invalid input: {:?}", input)),
    }
}

async fn get_info_from_terminal() -> Result<(), String> {
    let mut string = String::new();
    let stdin = std::io::stdin();
    let _ = stdin.read_line(&mut string).expect("Failed to read");

    loop {
        let (interval, url) = parse_terminal_input(string.clone())?;
        let response = check_health(interval, url.clone()).await;

        println!("Checking '{url}'. Result: {response:#?}");
        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}

async fn check_health(interval: u64, url: String) -> String {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(interval))
        .build()
        .unwrap();

    match client.get(url).send().await {
        Ok(ok) => {
            let status = ok.status();

            if status.is_success() {
                return format!("OK(200)");
            } else {
                return format!("ERR({})", status.to_string());
            }
        }
        Err(err) => {
            format!("ERR({:#?})", err.status())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parse_error = get_info_from_terminal().await;

    if parse_error.is_err() {
        print!("{parse_error:?}");
    }

    Ok(())
}

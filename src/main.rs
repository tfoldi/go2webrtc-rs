// Copyright (c) 2024, RoboVerse community
// 
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// 
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
// 
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
// 
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use anyhow::Result;
use clap::{AppSettings, Arg, Command};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let robot_ip_env = env::var("GO2_IP").unwrap_or_else(|_| "".to_string());
    let robot_token_default = env::var("GO2_TOKEN").unwrap_or_else(|_| "".to_string());

    let mut app = Command::new("go2webrtc-rc")
        .version("0.1.0")
        .about("A Go2 WebRTC to udp broadcaster.")
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand_negates_reqs(true)
        .arg(
            Arg::new("FULLHELP")
                .help("Prints more detailed help information")
                .long("fullhelp"),
        )
        .arg(
            Arg::new("video_port")
                .takes_value(true)
                .short('v')
                .long("video")
                .value_parser(clap::value_parser!(u16))
                .default_value("4002")
                .help("UDP port for video streaming (default 4002)."),
        )
        .arg(
            Arg::new("audio_port")
                .takes_value(true)
                .short('a')
                .long("audio")
                .value_parser(clap::value_parser!(u16)) // Ensure the value is parsed as u16
                .default_value("4000") // Set the default value
                .help("UDP port for audio streaming (default 4000)."),
        )
        .arg(
            Arg::new("robot_ip")
                .takes_value(true)
                .short('r')
                .long("robot")
                .help("IP address of your GO2 robot.")
                .default_value(&robot_ip_env),
        )
        .arg(
            Arg::new("robot_token")
                .takes_value(true)
                .short('t')
                .long("token")
                .default_value(&robot_token_default)
                .help("Authentication token for your GO2 robot."),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .help("Prints debug log information"),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("FULLHELP") {
        app.print_long_help().unwrap();
        std::process::exit(0);
    }

    let video_port: u16 = *matches.get_one("video_port").expect("default not provided");
    let audio_port: u16 = *matches.get_one("audio_port").expect("default not provided");

    let robot_ip: &str = matches
        .get_one::<String>("robot_ip")
        .expect("required unless FULLHELP is present");
    let robot_token: &str = matches
        .get_one::<String>("robot_token")
        .expect("default not provided");
    let debug = matches.is_present("debug");

    return go2webrtc_rs::run(video_port, audio_port, robot_ip, robot_token, debug).await;
}

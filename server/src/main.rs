// SPDX-License-Identifier: GPL-2.0-or-later

use clap::Parser;

/// Server for SIMS protocol clients
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Separate configuration file
    #[clap(short, long)]
    config_file: Option<String>,

    /// Separate port number
    #[clap(short, long)]
    port: Option<u16>,
}

fn main() {
    let args: Args = Args::parse();
}

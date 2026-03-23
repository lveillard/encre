#![doc(html_logo_url = "http://gitlab.com/encre-css/encre-css/raw/main/.assets/logo.svg")]
#![forbid(unsafe_code)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    rustdoc::private_doc_tests,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    clippy::unnecessary_wraps,
    clippy::too_many_lines,
    clippy::implicit_clone,
    clippy::explicit_iter_loop,
    clippy::unnecessary_cast,
    clippy::missing_errors_doc,
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::non_ascii_literal,
    clippy::dbg_macro,
    clippy::use_debug,
    clippy::pattern_type_mismatch,
    clippy::map_err_ignore,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::indexing_slicing
)]

use std::{env, path::PathBuf};

use clap::{Parser, Subcommand};
use color_eyre::Report;

mod build;
mod playground;
mod utils;

use build::build;
use playground::launch;
use utils::generate_config;

pub const DEFAULT_CONFIG_FILE: &str = "encre.toml";

#[derive(Debug, Subcommand)]
enum Commands {
    /// Launches a playground environment for rapidly prototyping new ideas
    Playground {
        /// The name of the playground (if not provided a randomly named directory will be created)
        name: Option<String>,
    },

    /// Generates the CSS styles needed
    Build {
        /// The path to a custom configuration file
        #[clap(short)]
        config: Option<String>,

        /// An extra input path not specified in the configuration file
        #[clap(short)]
        input: Option<PathBuf>,

        /// Output file which will contains the generated CSS styles
        /// (will be printed to the standard output by default)
        #[clap(short)]
        output: Option<String>,

        /// Watch for changes
        #[clap(short, long)]
        watch: bool,
    },
    /// Generates default configuration, written to standard output.
    GenerateConfig,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Report> {
    // Enable nice panic reports with a backtrace
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "1");
    }

    color_eyre::install()?;

    let args = Cli::parse();

    match args.command {
        Commands::Playground { name } => launch(name),
        Commands::Build {
            config,
            input: extra_input,
            output,
            watch,
        } => build(config.as_ref(), extra_input, output, watch),
        Commands::GenerateConfig => generate_config(),
    }
}

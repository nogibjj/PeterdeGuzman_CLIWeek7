
use clap::Parser;
use votesqlite::{extract_zip};


/// A simple CLI tool to download and extract ZIP files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
//This struct will generate an object from our CLI inputs
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
// I separate the commands as enum types 
enum Commands{
    //Download a file from a link, unzip, and save to directory
    #[command(alias = "extract_zip", short_flag = 'z')]
    ExtractZipped { directory : String },
}

fn main() -> Result<()> {
    //Parse CLI arguments and store them in the args object
    let args = Cli::parse();
    
    //Match the behavior on the subcommand and call lib functions
    match args.command {
        Commands::ExtractZipped {directory } => {
            println!("Downloading and unzipping file to {}", directory);
            extract_zip()
        }

    }
    Ok(())
}




fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match extract_zip(&cli.url, &cli.directory) {
        Ok(path) => println!("Files extracted to: {}", path),
        Err(e) => eprintln!("An error occurred: {}", e),
    }

    Ok(())
}

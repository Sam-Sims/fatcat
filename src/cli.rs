use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Pretty print a FASTQ file",
)]
pub struct Cli {
    #[arg(required = true, value_parser(check_input_exists))]
    pub input: String,
}

pub

fn check_input_exists(s: &str) -> Result<String, String> {
    if std::path::Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err(format!("File does not exist: {}", s))
    }
}

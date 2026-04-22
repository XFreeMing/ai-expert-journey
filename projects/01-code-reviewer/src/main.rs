use clap::Parser;
use std::path::PathBuf;

/// Intelligent Code Review Assistant — see docs/adr/001-code-reviewer.md
#[derive(Parser)]
#[command(name = "code-reviewer", version = "0.1.0", about = "Intelligent Code Review Assistant")]
struct Cli {
    /// Path to code file or directory
    path: PathBuf,

    /// Programming language
    #[arg(short, long, default_value = "python")]
    lang: String,

    /// Output format
    #[arg(short, long, default_value = "markdown")]
    output: String,

    /// Custom rules config file
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    println!("Code Review Assistant v{}", env!("CARGO_PKG_VERSION"));
    println!("Scanning: {}", cli.path.display());
    println!("Language: {}", cli.lang);

    // TODO: Implementation pending — see docs/adr/001-code-reviewer.md
    eprintln!("[TODO] Full implementation pending");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}

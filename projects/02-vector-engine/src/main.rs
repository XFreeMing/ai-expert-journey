use clap::Parser;

/// High-Performance Vector Retrieval Engine — see docs/adr/002-vector-engine.md
#[derive(Parser)]
#[command(name = "vector-engine", version = "0.1.0", about = "High-Performance Vector Retrieval Engine")]
struct Cli {
    /// Engine mode (index, search, bench)
    #[arg(short, long, default_value = "search")]
    mode: String,

    /// Index file path
    #[arg(short, long)]
    index: Option<String>,

    /// Number of nearest neighbors to return
    #[arg(short = 'k', long, default_value_t = 10)]
    top_k: usize,
}

fn main() {
    let cli = Cli::parse();

    println!("Vector Retrieval Engine v{}", env!("CARGO_PKG_VERSION"));
    println!("Mode: {}", cli.mode);

    // TODO: Implementation pending — see docs/adr/002-vector-engine.md
    eprintln!("[TODO] HNSW implementation pending");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}

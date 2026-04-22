use clap::Parser;
use std::path::PathBuf;

/// LLM Inference Tools (GGUF parsing + inference) — see docs/adr/006-llm-inference.md
#[derive(Parser)]
#[command(name = "llm-inference-tools", version = "0.1.0", about = "LLM Inference Tools")]
struct Cli {
    /// GGUF model file path
    #[arg(short, long)]
    model: PathBuf,

    /// Input prompt
    #[arg(short, long, default_value = "")]
    prompt: String,

    /// Max tokens to generate
    #[arg(long, default_value_t = 256)]
    max_tokens: usize,

    /// Temperature for sampling
    #[arg(short, long, default_value_t = 0.7)]
    temperature: f32,
}

fn main() {
    let cli = Cli::parse();

    println!("LLM Inference Tools v{}", env!("CARGO_PKG_VERSION"));
    println!("Model: {}", cli.model.display());

    // TODO: Implementation pending — see docs/adr/006-llm-inference.md
    eprintln!("[TODO] GGUF parsing + inference implementation pending");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}

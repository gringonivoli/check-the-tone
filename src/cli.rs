use clap::Parser;
use ollama_rs::{Ollama, error::OllamaError, generation::completion::request::GenerationRequest};

use crate::msg::Msg;

#[derive(Parser)]
#[command(name = "ctt", version, about)]
pub struct Cli {
    #[arg(short, long)]
    msg: String,

    #[arg(short = 'M', long, default_value = "llama3.1:latest")]
    model: String,

    #[arg(short, long, default_value = "http://localhost")]
    url: String,

    #[arg(short, long, default_value = "11434")]
    port: u16,
}

impl Cli {
    fn msg(&self) -> Msg {
        self.msg.as_str().into()
    }

    fn model(&self) -> String {
        self.model.clone()
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn url(&self) -> String {
        self.url.clone()
    }

    pub async fn run(&self) -> Result<(), OllamaError> {
        println!(
            "{}",
            Ollama::new(self.url(), self.port())
                .generate(GenerationRequest::new(self.model(), self.msg().prompt()))
                .await?
                .response
        );
        Ok(())
    }
}

use std::io::{stdin, stdout, Write};
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();

    let model = "deepseek-r1:1.5b".to_string();
    let mut s = String::new();
    let mut prompt;
    let mut res; 

    while s != "exit" {    
        print!("\nEnter your prompt (type 'exit' to quit): ");
        
        let _ = stdout().flush();
        
        stdin().read_line(&mut s).expect("Failed to read line");

        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }

        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        prompt = s.to_string();
        res = ollama.generate(GenerationRequest::new(model.clone(), prompt.clone())).await;

        if let Ok(res) = res {
            println!("{}", res.response);
        }
    }

}
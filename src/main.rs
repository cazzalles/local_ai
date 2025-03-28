use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();

    let model = "deepseek-r1:1.5b".to_string();
    let prompt = "Why is the sky blue?".to_string();

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        println!("{}", res.response);
    }

}


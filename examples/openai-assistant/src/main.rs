use futures::StreamExt;

use lib::{
    openai::{assistants::Assistant, streams::TokenStream},
    prompt_read_line,
};

/// Expects the OPENAI_API_KEY environment variable to be set
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let assistant = Assistant::new("gpt-4o-mini", "Be a helpful assistant").await?;
    let thread = assistant.create_thread().await?;

    while let Some(input) = get_user_input() {
        let mut stream: TokenStream = thread.run_stream(&input).await?.into();
        while let Some(result) = stream.next().await {
            if let Ok(text) = result {
                print!("{}", text);
            }
        }
        println!();
    }
    assistant.delete().await?;

    Ok(())
}

fn get_user_input() -> Option<String> {
    prompt_read_line!("> ")
        .ok()
        .take_if(|input| !input.is_empty())
}

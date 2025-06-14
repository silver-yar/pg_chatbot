use agentai::Agent;
use anyhow::Result;
use genai::Client;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the GenAI client
    let client = Client::default();

    // Create an agent with system prompt and client as context
    let mut agent = Agent::new(
        "You are a helpful AI assistant answering discussion board questions on Purdue Global.",
        &client,
    );

    let model = "gemini-1.5-flash";

    println!("🤖 AI Chatbot initialized! Type 'quit' to exit.");
    println!("Model: {}", model);
    println!("{}", "-".repeat(50));

    // Chat loop
    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "quit" || input == "exit" {
            println!("Goodbye! 👋");
            break;
        }

        if input == "/info" {
            println!("Current model: {model}");
            println!("Provider: Google Gemini (Free Tier)");
            println!("Limits: 15 req/min, 1,500 req/day, 1M tokens/min");
            println!();
            continue;
        }

        // Use the agent to get a response
        match agent.run::<String>(model, input).await {
            Ok(response) => {
                println!("Bot: {response}");
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }

        println!(); // Add spacing
    }

    Ok(())
}

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Default, Debug, Clone)]
#[command(
    name = "GPT-CLI",
    author = "tsorabel",
    version = "1.0",
    about = "GPT-CLI made by tsorabel"
)]
pub struct Arguments {
    pub arg: Vec<String>,

    #[clap(short, long, default_value_t = false)]
    pub info: bool,

    #[clap(long, default_value_t = false)]
    pub config: bool,

    #[clap(long, about, value_name = "COLOR")]
    pub color: Option<String>,

    #[clap(long, value_name = "COLOR")]
    pub code_color: Option<String>,

    #[clap(long, value_name = "0 - 16000")]
    pub max_tokens: Option<i16>,

    #[clap(long, value_name = "0 - 2.0")]
    pub temperature: Option<f32>,

    #[clap(long, value_name = "0 - 1.0")]
    pub top_p: Option<f32>,

    #[clap(long, value_name = "0 - 2.0")]
    pub frequency_penalty: Option<f32>,

    #[clap(long, value_name = "0 - 2.0")]
    pub presence_penalty: Option<f32>,

    #[clap(short, long, value_name = "LANGUAGE")]
    pub translate: Option<String>,

    #[clap(short, long, value_name = "LANGUAGE")]
    pub correct: Option<String>,

    #[clap(long, default_value_t = false)]
    pub gpt_4: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i16,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choices {
    pub index: i32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub model: String,
    pub choices: Vec<Choices>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub color: String,
    pub code_color: String,
    pub total_tokens: i32,
    pub max_tokens: i16,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

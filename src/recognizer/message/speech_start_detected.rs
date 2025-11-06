use crate::recognizer::Offset;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub(crate) struct SpeechStartDetected {
    #[serde(rename = "Offset")]
    pub(crate) offset: Offset,
}
#[derive(Deserialize, Debug, Clone, Default)]
pub async fn KEYWORDMANAGER(prompt: &str, data: &str) -> String {
    if data[0..100].contains(prompt) {
        prompt.to_string() + &data[0..100];
    }
    return prompt.to_string();  
}

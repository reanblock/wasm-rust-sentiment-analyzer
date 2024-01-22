use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn analyze_sentiment(text: &str) -> String {
    let positive_words = vec!["happy", "good", "great", "awesome", "positive"];
    let negative_words = vec!["sad", "bad", "terrible", "awful", "negative"];

    let mut score = 0;
    for word in text.split_whitespace() {
        if positive_words.contains(&word) {
            score += 1;
        } else if negative_words.contains(&word) {
            score -= 1;
        }
    }

    match score {
        s if s > 0 => "Positive".to_string(),
        s if s < 0 => "Negative".to_string(),
        _ => "Neutral".to_string(),
    }
}
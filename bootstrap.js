import init, { analyze_sentiment } from './pkg/sentiment_analyzer.js';

async function run() {
    await init();

    document.getElementById('analyzeButton').addEventListener('click', () => {
        const text = document.getElementById('textInput').value;
        const result = analyze_sentiment(text);
        document.getElementById('result').textContent = result;
    });
}

run();
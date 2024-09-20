import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [japaneseText, setJapaneseText] = useState('');
  const [englishText, setEnglishText] = useState('');

  const handleTranslateJpToEn = async () => {
    const result = await invoke<string>('translate_jp_to_en', { text: japaneseText });
    setEnglishText(result);
  };

  const handleTranslateEnToJp = async () => {
    const result = await invoke<string>('translate_en_to_jp', { text: englishText });
    setJapaneseText(result);
  };

  return (
    <div className="app-container">
      <div className="input-container">
        <textarea
          value={japaneseText}
          onChange={(e) => setJapaneseText(e.target.value)}
          placeholder="日本語のテキストを入力してください"
        />
        <textarea
          value={englishText}
          onChange={(e) => setEnglishText(e.target.value)}
          placeholder="English text will appear here"
        />
      </div>
      <div className="button-container">
        <button onClick={handleTranslateJpToEn}>日→英</button>
        <button onClick={handleTranslateEnToJp}>英→日</button>
      </div>
    </div>
  );
}

export default App;

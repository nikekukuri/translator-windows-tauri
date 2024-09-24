Simple translator GUI app in Windows
Japanise to English and English to Japanese.

# Getting Started
binフォルダに以下のファイルを配置してください。
* [C3TR-Adapter](https://huggingface.co/webbigdata/C3TR-Adapter_gguf)
* [llama.cpp](https://github.com/ggerganov/llama.cpp)
* [c3tr-client](https://github.com/koron/c3tr-client)

コンパイル済バイナリでも大丈夫なはずです。

配置したら、model_config.jsonのadaptorを修正してください。
同じモデルを使用している場合は変更の必要はありません。

```model_config.json
{
  "model": "C3TR-Adapter-Q4_k_m.gguf",
  "server": "llama-server.exe",
  "client": "c3tr-client.exe"
}
```

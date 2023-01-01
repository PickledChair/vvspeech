# vvspeech

与えられた日本語テキストから合成音声を出力するコマンドです。[VOICEVOX エンジン](https://github.com/VOICEVOX/voicevox_engine)、またはその互換エンジンと通信して音声データを取得するため、コマンド利用時はそれらのエンジンが起動している必要があります。



## 動作環境

- Windows
- macOS
- Linux



## インストール

Rust の開発環境が必要です。このリポジトリを `git clone` したのちプロジェクトのディレクトリに移動し、以下のコマンドを実行してください：

```
cargo install --path .
```

更新時は `git pull` したあと、以下のコマンドを実行してください：

```
cargo install --path . -f
```



## 使い方

４つのサブコマンド (`info`, `kana`, `play`, `save`) を利用することができます：

```
$ vvspeech
A tool to communicate with the VOICEVOX (or other) engine to retrieve audio queries and audio files for playback and save

Usage: vvspeech [OPTIONS] <COMMAND>

Commands:
  info  Show the speakers information
  kana  Convert the given text to AquesTalk-like notation
  play  Speak the given text
  save  Generate an audio file from the given text and save it
  help  Print this message or the help of the given subcommand(s)

Options:
  -e, --engine-url <URL>  Set the TTS engine URL (default = "http://127.0.0.1:50021". shorthands: ["voicevox", "coeiroink", "sharevox", "lmroid", "itvoice"])
  -h, --help              Print help information
  -V, --version           Print version information
```

### `vvspeech info`

```
$ vvspeech info -h
Show the speakers information

Usage: vvspeech info [OPTIONS]

Options:
  -n, --name <SPEAKER NAME>  speaker name
  -j, --json                 JSON output
  -p, --pretty-json          pretty JSON output
  -h, --help                 Print help information
```

### `vvspeech kana`

```
$ vvspeech kana -h
Convert the given text to AquesTalk-like notation

Usage: vvspeech kana <TEXT>

Arguments:
  <TEXT>  input text

Options:
  -h, --help  Print help information
```

### `vvspeech play`

```
$ vvspeech play -h
Speak the given text

Usage: vvspeech play [OPTIONS] <TEXT>

Arguments:
  <TEXT>  input text (if '--kana' option is specified, AquesTalk-like notation required)

Options:
  -i, --id <SPEAKER ID>              speaker ID (default = 0)
  -n, --name <SPEAKER NAME>          speaker name
  -k, --kana                         AquesTalk-like notation flag
      --speed <SPEED>                speed of speech [default: 1]
      --pitch <PITCH>                pitch of speech [default: 0]
      --intonation <INTONATION>      intonation of speech [default: 1]
      --volume <VOLUME>              volume of speech [default: 1]
      --pre-phoneme <PRE_PHONEME>    pre phoneme length [default: 0.1]
      --post-phoneme <POST_PHONEME>  post phoneme length [default: 0.1]
  -h, --help                         Print help information
```

### `vvspeech save`

```
$ vvspeech save -h
Generate an audio file from the given text and save it

Usage: vvspeech save [OPTIONS] --output <OUTPUT FILE> <TEXT>

Arguments:
  <TEXT>  input text (if '--kana' option is specified, AquesTalk-like notation required)

Options:
  -i, --id <SPEAKER ID>              speaker ID (default = 0)
  -n, --name <SPEAKER NAME>          speaker name
  -o, --output <OUTPUT FILE>         output file name
  -k, --kana                         AquesTalk-like notation flag
      --speed <SPEED>                speed of speech [default: 1]
      --pitch <PITCH>                pitch of speech [default: 0]
      --intonation <INTONATION>      intonation of speech [default: 1]
      --volume <VOLUME>              volume of speech [default: 1]
      --pre-phoneme <PRE_PHONEME>    pre phoneme length [default: 0.1]
      --post-phoneme <POST_PHONEME>  post phoneme length [default: 0.1]
  -h, --help                         Print help information
```



## ライセンス

MIT License

# VoiceTranscriber
[Elite35P-Server](https://huggingface.co/Elite35P-Server/)の[EliteVoiceProject](https://huggingface.co/datasets/Elite35P-Server/EliteVoiceProject)で使用する、学習用音声データセットの作成を効率的に行う為のGUIアプリ。

## Usage
```bash
git clone https://github.com/minetake01/VoiceTranscriber
cd VoiceTranscriber
pnpm install
pnpm tauri dev
```

## TODO
- FileSelectページ
  - ファイルを開く
    - [x] ボタンクリックでファイル選択ダイアログを表示する。
    - [x] ファイルが選択されたらデコードを開始し、終了したらAudioSplitページに遷移する。
    - [x] デコード処理中、ボタンを操作不可能にする。
  - プロジェクトを開く
    - [ ] ボタンクリックでファイル選択ダイアログを表示する。
    - [ ] ファイルが選択されたらデコードを開始し、終了したらLabelingページに遷移する。
    - [ ] 音声ファイルが見つからなければ参照し直すダイアログを表示する。
    - [x] デコード処理中、ボタンを操作不可能にする。
- AudioSplitページ
  - 振幅グラフ
    - [x] グラフを描画する。
    - [x] しきい値の線を描画する。
    - [ ] 時間軸を表示する。
    - [ ] 分割された場所に縦線を描画する。
    - [ ] 分割された範囲を１クリックで選択、コンテキストメニューからその範囲だけSpleeterを適用できるようにする。
      - [ ] Spleeter適用済みの範囲は別色で表示する。
    - [ ] 範囲をダブルクリックで再生できるようにする。
      - [ ] 再生中、範囲の中心に停止ボタンを表示。
    - [ ] `ctrl+スクロール`で拡大縮小、`shift+スクロール`で横移動できるようにする。
    - ショートカット
      - [ ] `p` 範囲選択状態の時Spleeterを適用
  - 分割パラメータ入力欄
    - [x] しきい値、最短無音時間(秒)、最短分割範囲(秒)、拡張時間(秒)
  - 分割結果の情報
    - [x] 分割結果の個数を表示。
    - [ ] 特徴毎に分割結果を再生できるようにする。
      - [ ] 声(最小の平均音量)
      - [ ] 無音(最大の瞬間音量)
      - [ ] 声(最長)
      - [ ] 声(最短)
    - [ ] 分割処理の待機中、分割結果の個数の代わりに`処理中…`と表示し、再生ボタンを操作不可能にする。
    - ショートカット
      - [ ] `1` 声(最小の平均音量)を再生
      - [ ] `2` 無音(最大の瞬間音量)を再生
      - [ ] `3` 声(最長)を再生
      - [ ] `4` 声(最短)を再生
  - 分割完了
    - [ ] ボタンクリックでファイル保存ダイアログを表示する。
    - [ ] 保存先が選択されたら、プロジェクトフォルダに音声ファイルをコピーするか問うモーダルを表示する。
    - [ ] プロジェクトフォルダを作成し、中にプロジェクトファイルとコピーした音声データと`work`フォルダを作成する。
- Labelingページ
  - 分割結果リスト
    - [ ] すべての音声をWhisperで文字起こしし、処理済みのものにはチェックマークを表示する。
    - [ ] 再生ボタンと振幅グラフを表示し、視聴できるようにする。
  - 音声エディタ
    - [ ] 振幅グラフを表示する。
    - [ ] 時間軸を表示する。
    - [ ] 上部にループ範囲とカーソルを表示するバーを表示。
    - [ ] `ctrl+スクロール`orバーを上下ドラッグで拡大縮小、`shift+スクロール`orバーを左右ドラッグで横移動できるようにする。
    - [ ] クリックでカーソル移動、ドラッグで範囲選択、バーをクリックでセグメント全体を範囲選択できるようにする。
    - [ ] 選択範囲外をダブルクリックで音声を分割する。
    - [ ] 選択範囲内をダブルクリックで、範囲を消去して音声を分割する。
    - [ ] 再生・停止・ループ有効化ボタンを表示する。
    - [ ] ループを有効化させた時、範囲が選択されていればそこにループ範囲を設定する。
    - [ ] 再生速度入力欄を表示する。
    - ショートカット
      - [ ] `ctrl+space` 再生・停止。
      - [ ] `esc` 選択解除。
      - [ ] `ctrl+l` ループ有効化・無効化。有効化させた時、範囲が選択されていればそこにループ範囲を設定する。
      - [ ] `ctrl+w` カーソルを最初に移動。
      - [ ] `ctrl+p` Spleeterを適用。
      - [ ] `<` 再生速度を下げる。
      - [ ] `>` 再生速度を上げる。
      - [ ] `+` 音量を大きくする。
      - [ ] `-` 音量を小さくする。
  - ラベル入力欄
    - [ ] Whisperを使って起こした文章を入力しておく。
    - [ ] kuromoji.jsを使って入力された文章の形態素解析を行い、文節にまとめてショートカットで移動できるようにする。
      - 付属語(助動詞、助詞)以外の単語の後ろに付属語がある場合、結合する。付属語が連続している場合、全て結合する。
    - ショートカット
      - [ ] `tab` 次の単語の最初に移動。
      - [ ] `shift+tab` 前の単語の最初に移動。
- Exportページ
  - [ ] エクスポート先のパス入力欄を表示。参照から保存先選択ダイアログを表示する。デフォルトはプロジェクトフォルダ内の`work`フォルダ。
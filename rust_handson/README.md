# Rustハンズオン

2017/01/15(Sun.) ヒカリエ 17F 
https://rust.connpass.com/event/43893/
twitter_tag: rust_jp

## 発表資料

https://chikoski.github.io/rust-handson/?locale=ja-JP#/
構文等rustの説明
cat,grepを実装してみる

## その他

代入: 箱を用意して、そこに値を入れる
束縛: 値があって、名前を紐付ける

rustは束縛 = 再代入は基本的にはできない
再代入しないことでコンパイラが値を定めるのが早くなる

メモリの開放は記述しなくてもコンパイラが勝手に行ってくれる
（gcを入れることでも解決できるが、gcの起こるタイミングにより足を引っ張られる）
その際にsignatureは重要。

# 【Rust】Enumとパターンマッチング

列挙型についてみていく。

enumは多くの言語に存在する機能ですが、その能力は言語ごとに異なります。Rustのenumは、F#、OCaml、Haskellなどの、 関数型言語に存在する*代数的データ型*に最も酷似しています。

ここで理解することは

- enumの定義方法
- 列挙子への値の登録方法
- Option Enum
- match式

## Enumを定義する

enumのいいところ：取りうる値をすべて列挙できる。

例えばIPアドレスを扱う必要がある場合、IPアドレスの企画はv4とv6がある。

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

### Enumの値

以下のようにしてインスタンスを生成できる

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

また次のようにしてEnumを引数にとる関数も定義できます

```rust
fn route(ip_type: IpAddrKind) { }

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

この`IpAddrKind`だけでは実際のIPアドレスを保持できない。次のように構造体と組み合わせて対処する

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

また、各列挙子に直接データを格納して、enumだけを使って同じ概念を簡単に表現することができる

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

同じ型を持つ必要もない

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```



列挙子の型が異なる場合をもう少し深く考える

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

`Message`は異なる4つの列挙子を持っている。これはそれぞれ`struct`を使って定義することと似ている

```rust
struct QuitMessage; // ユニット構造体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // タプル構造体
struct ChangeColorMessage(i32, i32, i32); // タプル構造体
```

enumがstructと似ている点がもう一つある。`impl`を使ってメソッドを定義できます。

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### Option enumとNullに勝る利点

`Option`型：値が何かかそうでないかを示す。

RustにはNullがない。何らかの理由で現在無効、存在しない値のことを示すときに`Option<T>`を使用する。

`<T>`はジェネリクス型引数

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option`は有益なので、初期化処理に含まれる。つまり、明示的にスコープに導入する必要がない。

さらに列挙子もそうなっている。つまり、`Options::`接頭辞なしに使える。

`Some`も`None`も`Options`の列挙子であることに注意する

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

`None`を使って宣言する場合型が何かを示さなければならない。

#### なぜnullよりOptionの方がいいのか

`Option<T>`と`T`は型が異なる。以下のコードはエラーになる。

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;

---
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
(エラー: `i8: std::ops::Add<std::option::Option<i8>>`というトレイト境界が満たされていません)
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
  |
```

`i8`と`Option<i8>`が型が異なるので、足し合わせることができない。と言っている。

つまり、`T`型の処理を行いたい場合`Option<T>`を`T`型に変換する必要がある。

どのように返還するのか。[ドキュメント](https://doc.rust-lang.org/std/option/enum.Option.html)で確認できます。

```Rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y.unwrap();
```

一般的に`Option<T>`を使う場合、各列挙子を処理するコードが欲しい。例えば、Someの時だけ走るコードや、Noneの場合に欲しいコードがある。

次に説明する`match`式がenumとともに使用したときに、列挙子に応じて処理を分岐させる。

## `match`制御フロー演算子

`match`:一連のパターンに対して値を比較してマッチしたパターンに応じてコードを実行する

一連のパターンとはリテラル値、変数名、ワイルドカードなどがある。

`match`式をコイン並べ替え装置のようなものと考えてください: コインは、様々なサイズの穴が空いた通路を流れ落ち、 各コインは、サイズのあった最初の穴に落ちます。

たとえに従って、`Coin`enumを使ってコインに応じて価値を数え上げる例をしめす。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        `match` => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

まず`match`キーワードを使う。続けて式書くこの例だと`coin`である。

次に**`match`アーム**が並ぶアームはパターン(`Coin::Nickel`)と動作するコード`1`と、`=>`で構成される。

各アームに紐づけられる動作するコードは式で、match全体の戻り値になる

アームのコードが長ければ`{}`で囲んで、複数行かける

```rust
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 値に束縛されるパターン

列挙子が値を保持している場合を考える。

Quaretrコインはその昔、州によってデザインが異なったので、州を保持させる。

```rust
#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

matchでこの値を使用する場合

```rust
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

`value_in_cents(Coin::Quarter(UsState::Alaska))`を呼び出すとき、

`Coin::Quarter(state)`に到達すると、`state`に`UsState::Alaska`が束縛され、値を取得できる。

### `Option<T>`とのマッチ

`Option<T>`もパターンマッチで処理をかける。

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### マッチは包括的

次のようなものはコンパイルできない。

Noneを扱っていないからだ。

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}

---
error[E0004]: non-exhaustive patterns: `None` not covered
(エラー: 包括的でないパターン: `None`がカバーされてません)
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered
```

**すべての可能性を網羅していないことをコンパイラは検知する。どのパターンを忘れているかも知っている。**

### `_`というプレースホルダー

すべての可能性を列挙したくないときもある。そんな時は`_`をつかう。

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```



## `if let`で簡潔な制御フロー

以下のように一つのパターンのときにmatch式と`_`を使って書くのは少し冗長だ

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

`if let`を使用するともっと短くかける

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```

`if let`という記法は等号記号で区切られたパターンと式を取り、式が`match`に与えられ、パターンが最初のアームになった`match`と、 同じ動作をします。

`if let`を使うとmatchの包括チェックがなくなることに注意する。

また。elseが使用できるので以下の2つの例は等価である

```rust
let mut count = 0;
match coin {
    // {:?}州のクォーターコイン
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```


構造体

## 構造体を使用して関係のあるデータを構造化する

`struct`または構造体は意味のあるグループを形成する複数の関連した値をまとめ、名前つけをできる。

#　構造体を定義し、インスタンス化する

構造体はタプルと似ている。データに名前をつけるので、値の意味が明確になる。

`struct`キーワードを使う。名前とデータ型を定義する。これを**フィールド**と呼ぶ

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

インスタンス化は、`key: value`で行う。

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

フィールドの値を取得するときはドット`.`を使う

フィールドに値を代入するときも`.`を使う。`mut`も必要。

```rust
let email = user1.email

let mut user2 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
user2.email = String::from("hoge@test.com")
```

### フィールドと変数が同名ならフィールド初期化省略記法を使う

Rustの構造体は一部のフィールドのみを可変にすることはできないので、インスタンス全体が可変でなければならない。

次のような関数を考える。インスタンスを生成する関数だ。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

この時、変数名とフィールド名が同じなら**フィールド初期化省略記法**が使える


```rust
fn build_user(email: String, username: String) -> User {
    User {
        email, // 省略！！
        username,　// 省略！！
        active: true,
        sign_in_count: 1,
    }
}
```

### 構造体更新記法でほかのインスタンスからインスタンスを生成する

**構造体更新記法**: あるインスタンスの値を使用しつつ、変更する箇所もあるインスタンスを生成するときに有用。

user1をもとにuser2を作ります

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active, // user1の値を使う
    sign_in_count: user1.sign_in_count,　// user1の値を使う
};
```

構造体更新記法をつかうとこのようになる。`..user1`で明示されてないフィールドがセットされる。

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1 
};
```

### 異なる型を生成する名前付きフィールドのないタプル構造体を使用する

**タプル構造体**: 構造体名を持つタプル。フィールドに名前のない、型情報だけのもの・

`struct`キーワードを使う。`.`と添え字でフィールドにアクセスできる。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32); // Colorと中身の型は同じだけど、名前が違う。

// インスタンス化
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### 構造体データの所有権

`User`でStringをフィールドに持っているが、これは意図的な選択である。

構造体のインスタンスはすべてのフィールドの所有権を所有する必要がある。
→構造体が有効な間フィールドは有効である必要があるから。

構造体にデータの参照を渡すことはできるが、**ライフタイム**というものを使用しなければならない。ライフタイムのおかげで構造体が有効な間参照されたデータが有効なことを保証してくれる。



## 構造体を使ったプログラムの例

構造体を理解するために長方形の面積を導くプログラムを作ります。

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

`fn area(width: u32, height: u32) -> u32`で二つの引数が関連性があるのにどこにもそのことが書いていない。幅と高さをグループ化した方がいい。

タプルを使ってリファクタしました。

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

添え字で幅と高さをしめしているのが苦しいですね。

構造体を使いましょう

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

これまで習ったことがちょこちょこ出てますね

かなりよくなりました。



### トレイトの導出で有用な機能を追加する

かなり話が変わるが、デバックで構造体のインスタンスを以下のように確認したいことはよくある。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // rect1は{}です
    println!("rect1 is {}", rect1);
    // できればこんな感じで見たいが、エラーになる
    // rect is Rectangle {
    //     width: 30,
    //     height: 50
    // }
}
```

これはエラーになります。

これまで使ってきた`println!`はマクロで、様々な整形がある。(マクロはいつかどこかで触れるはず)

標準では波括弧は`Display`として知られる生計をするように指示している。これまでの型は標準で`Displey`を実装しているが、構造体では成型する方法が自明ではなくなる。

 `Debug`トレイトは、開発者にとって有用な方法で構造体を出力させてくれるので、 コードをデバッグしている最中に、値を確認することができます。

トレイトとはなんですか？
→ 共通の振る舞いを提供するためのメカニズム

`:?`という指定子を書くと`println!`に`Debug`と呼ばれる出力整形を使いたいと支持する。

これを使うために構造体に`[derive(Debug)]`という注釈を追加する

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1); // :#?で改行あり
}

---
rect1 is Rectangle { width: 30, height: 50 }
rect1 is Rectangle {
    width: 30,
    height: 50
}
```

Rustには、`derive`注釈で使えるトレイトが多く提供されており、独自の型に有用な振る舞いを追加することができます。



## メソッド記法

話を戻すと`area`関数は`Rectangle`構造体と密接に結びついている。`area`メソッドに変形しよう！

関数とメソッドの違い
関数は処理のまとまり(main関数)。メソッドはオブジェクトの操作(getter, setter)

### メソッドを定義する

`impl`キーワードを使って`impl`ブロックを始める。関数と同じように`fn`キーワードを使うが最初の引数が必ず`&self`になる。

area関数を移動させて、引数を`&self`にする。呼び出しは`.`キーワードを使う。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

引数に`&`キーワードが使われていることに注目する。もしメソッド内でインスタンスを変更したい場合第一引数は`&mut self`を使用する、

まれに`self`を所有権ごとムーブする場合もある。selfを変形して、変形後に呼び出し元が元のインスタンスを使用できないようにしたい場合に使用されます。

### 引数の多いメソッド

`Rectrangle`インスタンスで、別の`Rectangle`を引数にとって、はめ込むことができるなら`ture`を返すメソッド`can_hold`を考える

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

### 関連関数

`impl`ブロックの別な有益な機能で`self`を引数に取ら**ない**関数を定義できる。これを構造体に関連付けられているので**関連関数**と呼ぶ。

関連関数はメソッドではない。対象となるインスタンスがないからである。例(`String::from`)

関連関数は新規インスタンスを返すコンストラクタによく使われる。

```rust
// 引数を1辺とする正方形を返す
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
// 使い方
Rectangle::sqyare(4);
```

### 複数のimpleブロック

implブロックは複数に分けることができる。特に意味はないが、場合によっては有用である。

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```


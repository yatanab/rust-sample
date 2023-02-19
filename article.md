# 基本編

## 変数と定数

letキーワードを使って宣言する。

```rust
let x = 5;
x= 50; // コンパイルエラー
```

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:7:5
  |
5 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
6 |     println!("x={}", x);
7 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
```

このままでは再代入できない。
mutキーワードを付与すると再代入可能になる。

```rust
let mut x = 5;
x = 500; // OK!
```

定数はconstキーワード
すべて大文字,_区切りで定義する

```rust
const TAX = 0.1;
const MAX_HEIGHT = 1100;
```

シャドーイング

同名の変数を再定義できる
型も変えられる

かっこ内を抜けると、元の値にもどる

```rust
let x = 5;
let x = 20;
println!("x={}", x); // x=20
{
    let x = "ABCD";
    println!("x={}", x); // x=ABCD
}
println!("x={}", x); // x=20
```
## データ型

スカラー型



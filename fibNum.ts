const fibNumJs = (n: number): number => {
    if (n == 0 || n == 1) {
        return n
    } else {
        return fibNumJs(n - 2) + fibNumJs(n - 1)
    }
}

const startTime = performance.now(); // 開始時間
fibNumJs(10); // 計測する処理
const endTime = performance.now(); // 終了時間

console.log("js score fibnum(10)", endTime - startTime); // 何ミリ秒かかったかを表示する

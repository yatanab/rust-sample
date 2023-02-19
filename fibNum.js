var fibNumJs = function (n) {
    if (n == 0 || n == 1) {
        return n;
    }
    else {
        return fibNumJs(n - 2) + fibNumJs(n - 1);
    }
};
var startTime = performance.now(); // 開始時間
const fib_js = fibNumJs(30); // 計測する処理
var endTime = performance.now(); // 終了時間
console.log("js score fibnum(30)", endTime - startTime); // 何ミリ秒かかったかを表示する
console.log(fib_js)

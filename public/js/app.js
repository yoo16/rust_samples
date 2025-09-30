import init, { greet, add } from "../../pkg/rust_samples.js";

// 初期化（WASMのロード）
await init();

// DOM操作
const nameInput = document.getElementById('user-name');
const num1Input = document.getElementById('number-1');
const num2Input = document.getElementById('number-2');
const resultDiv = document.getElementById('result');
const greetBtn = document.getElementById('greetBtn');
const calculateBtn = document.getElementById('calculateBtn');

// クリックイベント
greetBtn.addEventListener('click', () => {
    const name = nameInput.value;
    if (!name) {
        resultDiv.innerText = "名前をいれてね！";
        return;
    }
    const result = greet(name);
    resultDiv.innerText = result;
});
calculateBtn.addEventListener('click', () => {
    const num1 = parseInt(num1Input.value) || 0;
    const num2 = parseInt(num2Input.value) || 0;

    const result = add(num1, num2);
    resultDiv.innerText = result;
});
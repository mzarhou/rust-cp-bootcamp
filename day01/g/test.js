const n = 2 * 1e5;
const random = () => Math.floor(Math.random() * 1e9);
let numbers = Array(n).fill().map((_, _idx) => n - _idx)
console.log(`${n}\n${numbers.join(' ')}\n`)

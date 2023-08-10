const n = 1000;
const dots = Array(n).fill().map((_, i) => Array(n).fill().map((_, i) => '.'))
console.log(`${n}`)
dots.forEach((d) => {
    console.log(d.join(''))
})

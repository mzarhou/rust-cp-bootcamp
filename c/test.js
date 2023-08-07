const n = 1e6;

let output = [];

output.push(n);

Array(n).fill().map((_, index) => {
    output.push(`1 ${index}`);
    output.push(`3`);
    output.push(`2`);
})
console.log(output.join('\n'))

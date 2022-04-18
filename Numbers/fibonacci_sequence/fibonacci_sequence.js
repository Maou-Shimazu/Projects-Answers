//fibonacci sequence in javascript 
let readlineSync = require("readline-sync");

let loc = parseInt(readlineSync.question("Enter number you want to go up to: "));
let fibonacci = [0, 1];

for (let i = 2; i <= loc; i++) {
    fibonacci[i] = fibonacci[i - 1] + fibonacci[i - 2];
}

console.log(fibonacci);

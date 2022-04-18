let readlineSync = require("readline-sync"); // npm i readline-sync
let string: String = readlineSync.question("String to be reversed: ");
console.log(string.split("").reverse().join(""));
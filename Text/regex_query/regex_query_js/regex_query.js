//regex tester for javascript

const readlineSync = require("readline-sync"); 

let string = readlineSync.question("Enter the string you want to test: ");
let expression = readlineSync.question("Enter the expression you want to test: "); 


console.log(`The expression ${expression} is ${string.match(expression) ? "true" : "false"}`);


let readlineSync = require("readline-sync");

// get numbers from user
let num1 = parseInt(readlineSync.question("Enter first number: "));
let num2 = parseInt(readlineSync.question("Enter second number: "));


//get operator from user 
let operator = readlineSync.question("Enter operator: ");



//calculate result based on operator
switch (operator) { 
    case "+":
        console.log(num1 + num2);  
        break;
    case "-":
        console.log(num1 - num2);
        break;
    case "*":
        console.log(num1 * num2);
        break;
    case "/":
        console.log(num1 / num2);
        break;
    default:
        console.log("Invalid operator");
}

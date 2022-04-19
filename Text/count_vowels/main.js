const readlineSync = require("readline-sync"); 

let string = readlineSync.question("Enter a word: ");

//count the vowels in the string 
let vowels = 0;

for (let i = 0; i < string.length; i++) {
    if (string[i] === "a" || string[i] === "e" || string[i] === "i" || string[i] === "o" || string[i] === "u") {
        vowels++;
    }
}

console.log(`The number of vowels in the string is ${vowels}`);

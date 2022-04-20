//Cease cipher in js 
// Language: javascript

const readlineSync = require("readline-sync"); 

console.clear();



let text = readlineSync.question("Enter the text you want to encrypt: ");

let shift = parseInt(readlineSync.question("Enter the shift you want to use: "));



//create the cipher
let cipher = "";

//loop through the text
for (let i = 0; i < text.length; i++) {
    //get the character code
    let charCode = text.charCodeAt(i);

    //check if the character is a letter
    if (charCode >= 65 && charCode <= 90) {
        //add the shift to the character code
        charCode = charCode + shift;

        //check if the character code is greater than 90
        if (charCode > 90) {
            //subtract 26 from the character code
            charCode = charCode - 26;
        }
    }

    //check if the character is a letter
    if (charCode >= 97 && charCode <= 122) {
        //add the shift to the character code
        charCode = charCode + shift;

        //check if the character code is greater than 122
        if (charCode > 122) {
            //subtract 26 from the character code
            charCode = charCode - 26;
        }
    }

    //add the character to the cipher
    cipher += String.fromCharCode(charCode);
}

console.log(`The cipher of ${text}is: ${cipher}`);


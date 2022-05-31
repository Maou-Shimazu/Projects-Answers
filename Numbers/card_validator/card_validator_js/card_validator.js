const readlineSync = require("readline-sync"); 

const cardNumber = readlineSync.question("Enter your card number: "); 

// check if the card number is valid using Luhn_algorithm
function isValidCardNumber(cardNumber) {
    let cardNumberArray = cardNumber.split("");
    let cardNumberArrayLength = cardNumberArray.length;
    let sum = 0;
    let checkDigit = cardNumberArray[cardNumberArrayLength - 1];
    cardNumberArray.pop();
    cardNumberArrayLength = cardNumberArray.length;
    for (let i = 0; i < cardNumberArrayLength; i++) {
        if (i % 2 === 0) {
            cardNumberArray[i] = cardNumberArray[i] * 2;
            if (cardNumberArray[i] > 9) {
                cardNumberArray[i] = cardNumberArray[i] - 9;
            }
        }
        sum += parseInt(cardNumberArray[i]);
    }
    if (sum % 10 === 0) {
        return true;
    } else {
        return false;
    }
}

isValidCardNumber(cardNumber);

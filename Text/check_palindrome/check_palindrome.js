// check if the word is a palindrome 

function Palindrome(word){ 
    let reverse = word.split('').reverse().join('');
    if (word == reverse){
        console.log(word + " is a palindrome");
    }
    else{
        console.log(word + " is not a palindrome");
    }
}

// is a palindrome
Palindrome("mom");

// is not a palindrome
Palindrome("hello");



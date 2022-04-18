// Binary to decimal conversion

function binary_to_decimal(binary){
    var decimal = 0;
    for(var i = 0; i < binary.length; i++){
        decimal += binary[i] * Math.pow(2, binary.length - i - 1);
    }
    return decimal;
}

console.log(binary_to_decimal("11111111"));

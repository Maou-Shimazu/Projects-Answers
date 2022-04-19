// coinflip in javascript 

function coinflip(times) {
    var options = ["heads", "tails"]; 
    var results = [];

    for (var i = 0; i < times; i++) { 
        var result = options[Math.floor(Math.random() * 2)];
        results.push(result);
    }

    return results;
}

console.log(coinflip(10));

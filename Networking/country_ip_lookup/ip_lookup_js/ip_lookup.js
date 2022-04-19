// looks up the country of an IP address 
// this app requires a package please install it useing NPM I 

const https = require('https');
const readlineSync = require("readline-sync"); 

let ip = readlineSync.question("Enter the IP address you want to look up: ");

https.get(`https://ipapi.co/${ip}/json/`, (res) => {
    res.setEncoding('utf8');
    let body = '';
    res.on('data', (data) => {
        body += data;
    });
    res.on('end', () => {
        let parsed = JSON.parse(body);
        console.log(`The country of ${ip} is ${parsed.country_name}`);
    });
}).on('error', (err) => {
    console.log(`Error: ${err.message}`);
});
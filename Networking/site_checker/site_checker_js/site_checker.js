// Site checker in js
// made by: @mini51


//! Warning:  if you use gmail you are going to get warnings since gmail is blocking nodeMailer


const readlineSync = require("readline-sync"); 
const fetch = require("node-fetch");
const nodemailer = require("nodemailer");


//get the url from the user 
let url = readlineSync.question("Enter the url you want to check: ");
let alerEmail = readlineSync.question("Enter the email you want to send the alert to: ");
let checkInterval = parseInt(readlineSync.question("Enter the interval in seconds you want to check the site: "));

checkInterval = checkInterval * 1000;


// create email transporter
let transporter = nodemailer.createTransport({
    service: "Service", // Gmail, etc.
    auth: {
        user: "Email",
        pass: "password"
    }
});



setInterval(() => {
    fetch(url)
        .then(res => res.text())
        .then(body => {
            if (body.includes("404")) {
                console.log("Site is down");
                transporter.sendMail({
                    from: "Email",
                    to: alerEmail,
                    subject: "Site is down",
                    text: "Site is down"
                });
            } else {
                console.log("Site is up");
            }
        })
        .catch(err => {
            console.log('Error: ' + err);
        });
}, checkInterval);


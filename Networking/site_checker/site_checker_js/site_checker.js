// Site checker in js
// made by: @mini51

const readlineSync = require("readline-sync"); 
const http = require("http");
const nodemailer = require("nodemailer");


//get the url from the user 
let url = readlineSync.question("Enter the url you want to check: ");
let alerEmail = readlineSync.question("Enter the email you want to send the alert to: ");
let checkInterval = parseInt(readlineSync.question("Enter the interval in seconds you want to check the site: "));

checkInterval = checkInterval * 1000;


// create email transporter
let transporter = nodemailer.createTransport({
    service: "gmail",
    auth: {
        user: "Email",
        pass: "Email-Password"
    }
});



setInterval(() => {
    http.get(url, (res) => {
        console.log(`The server is up and running on ${url}`);
    }).on("error", (err) => {
        //send an email to the user
        console.log(`The server is down on ${url}`);
        console.log(`Sending an email to ${alerEmail}`);

        transporter.MailMessage({
            from: "",
            to: alerEmail,
            subject: "The server is down",
            text: `The server you are listening to is down on ${url}`
            
        }).send((err, info) => {
            if (err) {
                console.log(err);
            } else {
                console.log(`Email sent to ${alerEmail}`);
            }
        });
    });
}, checkInterval);


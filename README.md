# Meetinizer

* [backend/README.md](backend/README.md)
* [frontend/README.md](frontend/README.md)

## Security

Should I trust you/this project? -- Of course not. 

...but I do my best to develop things in a secure manner.

### GDPR

Sorta...

## Goal

A self hosted service that anyone can use to get a bunch of people to decide what date/time suits the most the best, 
without giving away their <del>souls</del> personal data.

Login via email, to verify that the emails are correct and up to date.

Admin login in a secure way, create meetings etc.

### Auth

For normal users:

* One admin password 
* Via mail - code / HMAC thingie

For admin user:

* Store argon2 hash somewhere
* Onetime passwords in log?

### Options

* Day
    * Multiday choise over multiple month calendar
* Time
    * Multiple days
* Location
* Activity

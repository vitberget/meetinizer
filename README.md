# Meetinizer

A self hosted service that anyone can use to get a bunch of people to decide what date/time suits the most the best, 
without giving away their <del>souls</del> personal data.

## Goal

* Easy to deploy
    * Helm/Kubernetes
    * Docker compose (to be done)
* No user passwords, login via mailed links
    * Verifying mails
    * No DB with passwords to hack
    * Separate login per meeting/event
* No default admin password
    * Stored safely / Secure algoritm / Argon2
    * Use backend server to create Argon2 hash 
* No external dependencies
    * sqlite database
    * Still needs mail though

## Security

Should I trust you/this project? -- Of course not. 

...but I do my best to develop things in a secure manner.

### GDPR

TODO

### Cookies

TODO

## How to administrate

How to administrate, install, host somewhere etc.

[administrator.md](administrator.md)

## For developers

* [developer.md](developer.md)
* [backend/README.md](backend/README.md)
* [frontend/README.md](frontend/README.md)

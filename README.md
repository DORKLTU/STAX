# STAX

STAX is our server for hosting boardgame votes. Because google forms is not good enough.

STAX will possibly be used for more in the future.

## Running instructions:

Get rust, duh.

Install `cargo-leptos` with

`cargo install cargo-leptos`

Then run

`cargo leptos watch` 

to run the site.

## TODO:

- [ ] Host multible votes simultaneously.
- [ ] Display a piechart for the votes. (Colourful one)
- [ ] Each poll should have a unique url.
- [ ] Each alternative in the poll should extend for more info, in boardgames.
- [ ] An admin should be able to create new polls.
- [ ] An admin should be able to add items to a poll.
- [ ] Members of DORK should have their own account, which is used for voting. (The username is their ltu ideal)
- [ ] Members should be able to change their password, and request a recovery code.
- [ ] If a membership runs out the member should not be able to vote.
- [ ] Write a usable frontend using Leptos.
- [ ] If a user has voted before the option to change their vote should be made.
- [ ] Save results to a database. (diesel?)
- [ ] Save user info to a database. 

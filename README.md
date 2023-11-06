# Cos Backend

This is the backend fo a hobby project named "Cos" (short for cosplay) which is a social media platform similiar to instagram but aimed at cosplayers and anime/manga enthusiasts in general. The iOS Cos app ( [`DragonCat4012/Cos`](https://github.com/DragonCat4012/Cos) ) is currently the only mobile app that is being developed.

What makes this social media platform special is the focus on events and connecting individuals through them. While you might be able to search for a place on instagram for example, it is not possible to search for certain events in a good way. HashTags have been used to solve this problem in the past but they come with their own issues. The cos project aims to solve issues like this for the cosplaying community by providing additional attributes for posts, like the character that you are cosplaying, the event that you are attending or even the photographer. This will open up many new possibilities for the cosplaying community as it allows for a much better search in todays seemingly endless amount of media posted online.

By mainting a curated list of public or even private events with location, dates and other metadata, this will make connecting with other cosplayers so much easier.

### Current State

Initially this backend used `rocket` for the HTTP Server and `diesel` as an ORM but is currently undegoing a rewrite to replace those dependencies with `axum` and `sqlx` as they seem better suited as a long term solution.

## Structure

The project is divided into 5 individual crates to improve compilation times.

- api
  - Handles incoming HTTP requests 
- application
  - Business logic to access the database 
- domain
  - Contains all structures and types
- infrastructure
  - Contains the `migrations` folder for all the migrations and implements the migrator
- shared 
  - Types that get shared accross all crates


## Setup

1. First you need a reachable postgres database with valid credentials. Then create a `.env` file with a single key called `DATABASE_URL` which contains a url to your database with the credentials filled in like shown below:
```env
DATABASE_URL=postgres://<username>:<password>@you.db.url/cos
```

> All of the migrations will automatically run at start up. Every time the server starts it will check for new migrations in the `infrastructure/migrations` directory and run those automatically


### Deployment
The sever can be run with `cargo run --release` like every rust application or you can build it with `cargo build --release` and then directly execute the `target/release/main` file.


You can run the server however you want (as a systemd service for example). The server only needs to be able to establish a connection to the database and write files to the `images` folder. 

<details>
<summary>This is how a .service file might look like </summary>

```toml
[Unit]
Description=Backend for Cos App
After=docker.service
BindsTo=docker.service
Documentation=

Wants=network.target
After=network.target

[Service]
User=server
Group=server

ProtectHome=true
ProtectSystem=full
PrivateDevices=true
NoNewPrivileges=true
PrivateTmp=true
InaccessibleDirectories=/root /sys /srv -/opt /media -/lost+found
WorkingDirectory=/path/to/cos_backend
# Wait for the postgres DB
# ExecStartPre=/bin/sleep 30
ExecStart=/path/to/cos_backend/target/release/main
ExecStop=/bin/kill -s QUIT $MAINPID

[Install]
WantedBy=multi-user.target
```

</details>

</br>

This repository also contains a `Dockerfile` that you can use to run the server. Just change the `DATABASE_URL` in the `Dockerfile` and everything should work fine. The main problem of this approach is that it can take a long time to compile the project as incremental compilation for rust projects is a bit tricky to do with docker ( if you know a better way please open a issue ).

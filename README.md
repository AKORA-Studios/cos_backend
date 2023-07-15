# Cos Backend

This is the backend fo a hobby project named "Cos" (short for cosplay) which is a social media platform similiar to instagram but aimed at cosplayers and anime/manga enthusiasts in general. The iOS Cos app ( [`DragonCat4012/Cos`](https://github.com/DragonCat4012/Cos) ) is currently the only mobile app that is being developed.

What makes this social media platform different is the focus on events and connecting individuals through them. While you might be able to search for a place on instagram for example, it is not possible to search for certain events in a good way. HashTags have been used to solve this problem in the past but they come with their own issues. The cos project aims to solve issues like this for the cosplaying community by providing additional attributes for posts, like the character that you are cosplaying, the event that you are attending or even the photographer. This will open up many new possibilities for the cosplaying community as it allows for a much better search in todays seemingly endless amount of media posted online.

By mainting a curated list of public or even private events with location, dates and other metadata, this will make connecting with other cosplayers so much easier.

### Current State

As of right now the development of this project is paused because of personal reasons.

## Structure

The project is divided into 5 individual crates to improve compilation times. The [rocket](https://rocket.rs/) framework is used for the webserver and the [diesel](https://diesel.rs/) crate is used for the database connection, in this case a postgres database.

## Setup

1. First you need a reachable postgres database with valid credentials. Then create a `.env` file with a single key called `DATABASE_URL` which contains a url to your database with the credentials filled in like shown below:

```env
DATABASE_URL=postgres://<username>:<password>@you.db.url/cos
```

2. To run all the migrations on the database, you will have to install the diesel cli and then run `diesel migration run` in the infrastructure folder.

```bash
$ cargo install diesel_cli --no-default-features --features postgres
$ cd infrastructure
$ diesel migration run
```

3. After you succefully set up the database the only thing left is to deploy the server. There are two currently supported ways to run this server but feel free to try your own.

Method a) The current recommend way of running the server is by running it as a systemd service. For this you just have to run `cargo build --release` once and then create a systemd service running the release executable.

<details>
<summary>Example .service file </summary>

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

Method b) You can also run ther server as a docker container with the provided `Dockerfile`, just change the `DATABASE_URL` in the `Dockerfile` and everything should work fine. The main problem of this approach is that it can take a long time to compile the project as incremental compilation for rust projects is a bit tricky to do with docker ( if you know a better way please open a issue ).

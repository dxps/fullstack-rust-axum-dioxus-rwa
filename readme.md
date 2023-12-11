# ![RealWorld Example App](logo.png)

> ### [Dioxus](https://dioxuslabs.com/) and [Axum](https://github.com/tokio-rs/axum) codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

### [Demo](https://demo.realworld.io/)&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)

This codebase was created to demonstrate a fully fledged fullstack application built with **[Dioxus](https://dioxuslabs.com/)** (as front-end) and **[Axum](https://github.com/tokio-rs/axum)** (as back-end), including routing, authentication, CRUD operations, authentication, and more.

We've gone to great lengths to adhere to the styleguides & best practices promoted by these frameworks. For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

<br/>

# How it works

This project uses the classic three-tier architecture:

```
  ╭───────────────╮       ╭────────────────╮       ╭────────────────╮
  │   Front-end   │       │    Back-end    │       │     Database   │
  │    (Dioxus)   ├──────►│     (Axum)     ├──────►│   (PostgreSQL) │
  ╰───────────────╯       ╰────────────────╯       ╰────────────────╯
```

<br/>

# Getting started

First, have the database available as described in backend's [readme](./backend/readme.md) by starting it and run the database migration (that populates it with the required objects).

Next, for local development and usage, just run `./run_dev.sh` script that starts both the front-end and the back-end.

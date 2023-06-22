# The Front-End Side

This is the Web UI side of the project.

state: `work-in-progress`

<br/>

## Setup

1. Install [Dioxus CLI](https://github.com/DioxusLabs/cli) to be able to use `dioxus` utility command.<br/>
   Needed for doing the build and running in _dev mode_.
   Install the git version of it (using `cargo install --git https://github.com/DioxusLabs/cli`).
2. Run the initial build using `dioxus build`.

<br/>

## Usage

### Start

To start this project in _dev mode_ (detect changes and reload the page), use `./run_dev.sh`.

<br/>

## Build

To package this project use `dioxus build --release`.

<br/>

## Project Structure

```
public         # save the assets you want include and refer in the project.
src            # put your code
 ╰──
 ╰── comps      # where custom components are defined
 ╰── pages      # where "pages" (or views, considering SPA terminology)
 ╰── utils      # save some public function (tbd)
```

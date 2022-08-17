# The Front-end Part

`work-in-progress` This file will be updated accordingly.

<br/>

## Setup

1. Install [Dioxus CLI](https://github.com/DioxusLabs/cli).
2. Run the initial build using `dioxus build`.

<br/>

## Usage

### Start

To start this project in _dev mode_ (detect changes and reload the page) use provided `run_dev.sh` script.


### Build

To package this project use `dioxus build --release`.

<br/>

## Project Structure

```
.project
- public         # save the assets you want include and refer in the project.
- src            # put your code
- - comps        # where custom components are defined
- - pages        # where "pages" (or views, considering SPA terminology)
- - utils        # save some public function
```

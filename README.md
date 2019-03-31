# Seed Quickstart Webpack

> I want to write fast, reliable and efficient web apps. Quickly.  
> And I like documentation.

The core is Rust framework [Seed](https://seed-rs.org).

**LIVE DEMO**: [seed-quickstart-webpack.netlify.com](https://seed-quickstart-webpack.netlify.com)

@TODO gif with auto-reload

# I need to install:

1. [Rust](https://rust-lang.org/tools/install)
1. [NodeJs](https://nodejs.org/en/download/)
1. [Yarn](https://yarnpkg.com/lang/en/docs/install)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

# How to create my new app:

1. Clone or download this repo.
1. Change `name` and `author` in `/package.json`
1. Change `name` and `authors` in `/crate/Cargo.toml`
1. Change `title` in `/index.html`
1. Modify `README.md` and `LICENCE`
1. Run command `yarn` in the project root.
1. _[Optional]_ Push new app into my repository. (GitHub [guide](https://help.github.com/en/articles/adding-an-existing-project-to-github-using-the-command-line))
1. _[Optional]_ Setup auto-deploy into [Netlify](https://www.netlify.com) through [Travis CI](https://travis-ci.org). (See chapter [Continous integration](#continous-integration))

# Basic commands

- **`yarn start`**

  - Start developer server on `localhost:3000`.
  - Server auto-reloads browser tabs on changes.
  - It doesn't destroy state if you change only styles (hot reload).
  - You can connect mobile devices to dev server:
    1. [Find out](https://www.whatismybrowser.com/detect/what-is-my-local-ip-address) local IP of the device when the dev server is running.
    1. _Note:_ Both devices have to be connected to the same network.
    1. Connect mobile device with that IP and server port. E.g.: `192.168.1.5:3000`

- **`yarn build`**

  - Build app for production in `/dist`.
  - _Note:_ You'll find out how to shrink code size in chapter [Optimizing for size](#optimizing-for-size).

- **`yarn test`**

  - Run tests in NodeJs.

- **`yarn test:{browser}`**
  - Run tests in headless browser.
  - `{browser}` should be `firefox`, `chrome` or `safari`.

# Project file structure

@TODO (maybe add links to Typescript, Tailwind, webpack config chapters...)
(dont forget tests, static..)

# Continous integration

> How to setup Travis CI pipeline with deploy into Netlify

@TODO (don't forget prerender)

# TailwindCSS

@TODO - settings, what is it..., hot reload(?), postcss config(?)

# Typescript

@TODO - settings, what is it..., why.., how to use it

# Webpack

@TODO - settings, what is it..., why..

# Optimizing for size

@TODO (panic hook, compiler settings?, tailwindcss how to shrink, ..., allocator)

# Browser and platform support

@TODO (Edge + IE help wanted (=> anaheim), not tested on android mobiles, mac and linux (help wanted))

# Recommended development tools

@TODO (rust-analyzer, prettier, vscode...)

# Contributing

@TODO (https://help.github.com/en/articles/setting-guidelines-for-repository-contributors)

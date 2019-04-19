# Seed Quickstart Webpack

> I want to write fast, reliable and efficient web apps. Quickly.  
> And I like documentation and WebAssembly.

Main parts:
- **[Seed](https://seed-rs.org)** - [Rust](https://www.rust-lang.org/) framework, inspired by [Elm](https://elm-lang.org/) 
- **[Tailwind CSS](https://tailwindcss.com/)** - CSS framework. All CSS classes are typed for safe use in Rust code. Unused classes are automatically deleted for much smaller bundle size.
- **[Typescript](https://www.typescriptlang.org/)** - When you need to go to the dark world of Javascript. You can generate Typescript types from Rust code for safer communication.
- **[Webpack](https://webpack.js.org/)** - It needs a little bit of magic to setup, but it's flexible and fast enough. Also it has many  useful loaders and plugins and dev-server that is accessible from mobile devices.

[**LIVE DEMO**: seed-quickstart-webpack.netlify.com](https://seed-quickstart-webpack.netlify.com)
  - Automatically deployed to [Netlify](https://www.netlify.com/) from `master` branch by [Travis CI](https://travis-ci.org/)
  - Compressed app size is about **90 KB** (look at _Developer Tools_ on demo page)

# Basic workflow
![](readme_video.gif)
_Recorded in [ScreenToGif](https://github.com/NickeManarin/ScreenToGif/)_
1. Start dev-server with `yarn start`.
1. Open `127.0.0.1:3000` in my browser. Or something like `192.168.0.5:3000` on my phone.
1. Change code & save it.
1. Check changes in browsers.
1. Run tests on NodeJS with `yarn test`. Or for specific browser e.g. `yarn test:firefox`.
1. If I haven't configured `.travis.yml`, I have to run `yarn build:release` and upload `/dist` folder into my server.

# I need to install:

1. [Rust](https://rust-lang.org/tools/install)
1. [NodeJs](https://nodejs.org/en/download/)
1. [Yarn](https://yarnpkg.com/lang/en/docs/install)
1. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

# How to create my new app:

1. Clone or download this repo.
1. Change `name` and `author` in `/package.json`
1. Change `name` and `authors` in `/crate/Cargo.toml`
1. Change `appname` in imported url to the same _name_ as in the previous steps (`/index.ts`)
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

@TODO (not tested on android mobiles, mac and linux (help wanted))

# Recommended development tools

@TODO (rust-analyzer, prettier, vscode..., webstorm + rust plugin)

# Contributing

@TODO (https://help.github.com/en/articles/setting-guidelines-for-repository-contributors)

# Seed Quickstart Webpack

> I want to write fast, reliable and efficient web apps. Quickly.
> And I like documentation and WebAssembly.

Main parts:

- **[Seed](https://seed-rs.org)** - [Rust](https://www.rust-lang.org/) framework, inspired by [Elm](https://elm-lang.org/)
- **[Tailwind CSS](https://tailwindcss.com/)** - CSS framework. All CSS classes (not only Tailwind's(!)) are typed for safe use in Rust code. Unused classes are automatically deleted for much smaller bundle size.
- **[Typescript](https://www.typescriptlang.org/)** - When you need to go to the dark world of Javascript. You can generate Typescript types from Rust code for safer communication.
- **[Webpack](https://webpack.js.org/)** - It needs a little bit of magic to setup, but it's flexible and fast enough. Also it has many useful loaders and plugins and dev-server that is accessible from mobile devices.

[**LIVE DEMO**: seed-quickstart-webpack.netlify.com](https://seed-quickstart-webpack.netlify.com)

- Automatically deployed to [Netlify](https://www.netlify.com/) from `master` branch by [Travis CI](https://travis-ci.org/)
- Compressed app size is about **90 KB** (look at _Developer Tools_ on demo page)

# Basic workflow

![](readme_video.gif)

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

# How to create my new app:

1. Clone or download this repo.
1. Choose name for my app. E.g. "iamgroot"
1. `/package.json` - Change `author` and `name` set to "iamgroot".
1. `/crate/Cargo.toml` - Change `authors` and `description`, `name` set to "iamgroot".
1. _[Optional]_ `/crate/Cargo.toml` - Comment out last two lines (see comments in the file for more info)
1. `/entries/index.ts` - Change word "appname" to "iamgroot" everywhere.
1. `/entries/index.html` - Change `title`.
1. Modify `/README.md` and `/LICENCE`.
1. Run command `yarn` in the project root.
1. _[Optional]_ Push new app into my repository. (GitHub [guide](https://help.github.com/en/articles/adding-an-existing-project-to-github-using-the-command-line))
1. _[Optional]_ Setup auto-deploy into [Netlify](https://www.netlify.com) through [Travis CI](https://travis-ci.org). (See chapter [Continous integration](#continous-integration))

# Commands

- **`yarn build`** and **`yarn build:release`**

  - Bundle app and save it into `/dist`.
  - _Note:_ Make sure that last two lines in `/crate/Cargo.toml` are uncommented when I want to run `yarn build:release` (more info in that file)

  Build pipeline:

  - Remove previous build from `/dist/`
  - Generate styles from `/css/tailwind.js` and `/css/styles.css` (see `/configs/postcss.config.js`)
  - Generate `/crate/src/generated/css_classes.rs` from styles
  - _[only release]_ Filter out unused CSS classes from styles
  - Process styles with [autoprefixer](https://github.com/postcss/autoprefixer)
  - Compile Rust
    - `/crate/` will be built into `/create/pkg/`
    - Typescript files included in Rust code (see `/crate/src/ts_apis.rs`) will be copied to `/crate/pkg/snippets/[id]/src/ts/`
  - Bundle assets (minify/compile/uglify css, images, ts, js,...) (see `/configs/webpack.config.js` and `/configs/tsconfig.json`)
  - Compile template `/entries/index.html` (i.e. Add bundle link into template and save result into `/dist/`)
  - Copy files from `/static/` into `/dist/static/`
  - _[only release]_ Optimize `/dist/[name].wasm` for size (see `/scripts/optimize_wasm.js`)

* **`yarn start`**

  - Build project and start developer server on `127.0.0.1:3000`.
  - _Note:_ Make sure that last two lines in `/crate/Cargo.toml` are commented out (more info in that file)
  - Server auto-reloads browser tabs on changes.
  - It doesn't destroy state if you change only styles (hot reload).
  - You can connect mobile devices to dev server:
    1. _Note:_ Devices and server have to be connected to the same network.
    1. [Find out](https://www.whatismybrowser.com/detect/what-is-my-local-ip-address) local IP of the device where the dev server is running.
    1. Connect mobile device with that IP and server port. E.g.: `192.168.1.5:3000`

* **`yarn test`**

  - Run tests in NodeJs.

* **`yarn test:{browser}`**

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

# Browser and platform support

@TODO (not tested on android mobiles, mac and linux (help wanted))

# Recommended development tools

@TODO (rust-analyzer, prettier, vscode..., webstorm + rust plugin)

# Contributing

@TODO (https://help.github.com/en/articles/setting-guidelines-for-repository-contributors)

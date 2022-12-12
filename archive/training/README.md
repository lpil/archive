# Training

[![Build Status](https://travis-ci.org/honeycomb-tv/training.svg?branch=v2/master)](https://travis-ci.org/honeycomb-tv/training)

Tracking conferences and other training fun :)


## About

A single page app written in Elm and a little Javascript. The app uses the Elm
architecture and largely uses Kris Jenkin's Elm app [pattern][pattern].

[Auth0][auth0] provides user authentication service.

[GraphCool][graphcool] provides a GraphQL based backend service.

Webpack is used to compile the application, and SCSS is used for styling.

[pattern]: http://blog.jenkster.com/2016/04/how-i-structure-elm-apps.html
[auth0]: https://auth0.com/
[graphcool]: https://www.graph.cool


## Usage

```sh
# Install deps
npm install --global yarn
make install

# Run the dev server
make start

# Run the tests
make test
make test-watch
```


## Contributing

Run [Elm format](https://github.com/avh4/elm-format) on Elm code. Having your
editor do this automatically when you hit save is nice and saves you a lot of
typing :)

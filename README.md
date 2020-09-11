<p align="center"><img src="./resources/logo.png" /></p>

<h1 align="center">Nox</h1>
<p align="center">A hobby web browser developed from scratch</p>

<p align="center">
  <img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT License" />
  <a href="https://webuild.community">
    <img src="https://raw.githubusercontent.com/webuild-community/badge/master/svg/by.svg" alt="By Vietnamese" />
  </a>
</p>

## About

I started this project to learn more about how the browser works and also use this as a oppotunity to work on my Rust skill.

To keep the "do it from scratch" spirit, I'll try to limit the use of dependencies to minimum.

## Features

- [x] :electric_plug: DOM API
- [ ] :memo: HTML Parsing
  - [x] HTML tokenizer
  - [ ] HTML dom tree builder
- [ ] :art: Rendering
  - [ ] GPU rendering
  - [ ] Font rendering
  - [ ] Box model
  - [ ] CSS selectors
    - [ ] Tag selectors
    - [ ] Id selectors
    - [ ] Class selectors
- [ ] :earth_americas: Networking
  - [ ] URL parsing
  - [ ] DNS resolving
  - [ ] DNS caching
  - [ ] HTTP/HTTPS
- [ ] :framed_picture: Media
  - [ ] :framed_picture: Image rendering
    - [ ] JPG
    - [ ] PNG
    - [ ] GIF
  - [ ] :clapper: Video playing
    - [ ] MP4
    - [ ] WebM
  - [ ] :speaker: Audio playing
    - [ ] MP3
    - [ ] WAV
- [ ] JavaScript

## Blog posts

I write about what I learn from this journey on my blog (order by latest):

### Browser from Scratch: DOM API

One of the main building blocks of the HTML rendering process is the DOM API. Before a browser can render the HTML document, it needs to parse the document content into a tree structure called the DOM tree. In this post, I'll break down my experimentation in building a DOM API with Rust. - [**Read more**][2]

### Browser from Scratch: Introduction

This is the start of Browser from Scratch series, created to help me (and probably you too) to learn more about how a browser works by building one! - [**Read more**][1]

[1]: https://zerox-dg.github.io/blog/2020/05/29/Browser-from-Scratch-Introduction/
[2]: https://zerox-dg.github.io/blog/2020/09/01/Browser-from-Scratch-DOM-API/

<div align="center">
  <img src="./src/assets/remitm_logo.png" width="25%"/>
  <br /><br />
  <div align="center">
     <img src="https://img.shields.io/github/actions/workflow/status/SpikeHD/reMITM/build.yml" />
     <img src="https://img.shields.io/github/package-json/v/SpikeHD/reMITM" />
     <img src="https://img.shields.io/github/repo-size/SpikeHD/reMITM" />
  </div>
  <div align="center">
    <img src="https://img.shields.io/github/issues-raw/SpikeHD/reMITM.svg?maxAge=25000" />
    <img src="https://img.shields.io/github/contributors/SpikeHD/reMITM.svg" />
    <img src="https://img.shields.io/github/commit-activity/m/SpikeHD/reMITM.svg" />
  </div>
  
  <div align="center">Dead-simple MITM redirection program.</div>
</div>

# Table of Contents

- [Introduction](#introduction)
- [Requirements](#requirements)
  - [Windows](#windows)
  - [Linux](#linux)
  - [MacOS](#macos)
- [Setup](#setup)
- [TODO](#todo)
- [Contributing](#contributing)
- [Translating](#translating)

# Introduction

[View an overview here!](https://spikehd.github.io/projects/reMITM)

reMITM was made for one purpose: to redirect requests to other places. While it allows you to log network traffic, this isn't really intended to be a debugger. Rather, it's use comes from it's ease of configuration for simply redirecting network traffic to other places (such as `localhost`, if you are intending on consuming/monitoring the traffic).

<div align="center">
  <img height="220px" alt="reMITM on Windows" src="https://github.com/SpikeHD/reMITM/assets/25207995/47166174-823c-4bf2-8b5c-1b1b29beff11" />
  <img height="220px" alt="reMITM on MacOS" src="https://github.com/SpikeHD/reMITM/assets/25207995/a4d7b299-abc2-4413-ada7-a9ea9e04da5e">
</div>

# Requirements

### Windows

- Nothing!

### Linux

- `libnss3-tools`
- `gsettings` (optional)

### MacOS

- Nothing!

# Setup

Download a release build [from here](https://github.com/SpikeHD/reMITM/releases). Open the executable, and use!

# TODO

- [ ] Make setting env variables in terminal work (add setting to set whatever `.bashrc`-type file is used?)
- [ ] Make UI a little fancier
- [ ] Make Linux and MacOS more stable
- [x] Import URIs to redirect from a file
- [ ] Seperate(?) window for showing each request and it's data
- [ ] Button to set via `adb` (eg. `adb shell settings put global http_proxy <ADDRESS>`), for things like Anbox.

# Contributing

Issues, pull requests, etc. are all welcome!

# Translating

Same as above, you can find language files in `src-tauri/lang`.

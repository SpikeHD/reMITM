<div align="center">
  <img src="./src/assets/remitm_logo.png" width="25%"/>
  <br /><br />
  <div align="center">Dead-simple MITM redirection program.</div>

  <div align="center">
    <img src="https://img.shields.io/github/issues-raw/SpikeHD/reMITM.svg?maxAge=25000" />
    <img src="https://img.shields.io/github/contributors/SpikeHD/reMITM.svg" />
    <img src="https://img.shields.io/github/commit-activity/y/SpikeHD/reMITM.svg" />
  </div>
</div>

# Introduction

[View an overview here!](https://spikehd.github.io/projects/reMITM)

reMITM was made for one purpose: to redirect requests to other places. While it allows you to log network traffic, this isn't really intended to be a debugger. Rather, it's use comes from it's ease of configuration for simply redirecting network traffic to other places.

# Requirements

### Windows

- Nothing!

### Linux

- Requires `libnss3-tools`
  - Ubuntu: `sudo apt install libnss3-tools`

### MacOS

- Nothing!

# TODO

- Settings
  - `Set in Wine`
  - `Launch on startup`
  - `Proxy Port`
- Finish UI
- "Open Terminal" button, opens with proxy env variables set (platform agnostic)

<div align="center">
  <img src="./src/assets/remitm_logo.png" width="25%"/>
  <br /><br />
  <div align="center">Dead-simple MITM redirection program.</div>

  <div align="center">
    <img src="https://img.shields.io/github/issues-raw/SpikeHD/reMITM.svg?maxAge=25000" />
    <img src="https://img.shields.io/github/contributors/SpikeHD/reMITM.svg" />
    <img src="https://img.shields.io/github/commit-activity/m/SpikeHD/reMITM.svg" />
  </div>
</div>

# Introduction

[View an overview here!](https://spikehd.github.io/projects/reMITM)

reMITM was made for one purpose: to redirect requests to other places. While ~~it allows you to log network traffic~~, this isn't really intended to be a debugger. Rather, it's use comes from it's ease of configuration for simply redirecting network traffic to other places (such as `localhost`, if you are intending on modifying/monitoring the traffic).

<div align="center">
  <img src="https://github.com/SpikeHD/reMITM/assets/25207995/39dbf9ad-eec7-4379-88b5-068bc003bef3" />
</div>

# Requirements

### Windows
- Nothing!

### Linux
- Requires `libnss3-tools` for certificate generation.
  - Ubuntu: `sudo apt install libnss3-tools`

### MacOS
- Nothing!

# Setup

Download a release build [from here](https://github.com/SpikeHD/reMITM/releases). Open the executable, and use!

# TODO

- Settings
  - `Set in Wine` (this is handle by environment variables, so the shell opening button should be enough)
- Make UI a little fancier
- Make all settings actually do something

# Contributing
Issues, pull requests, etc. are all welcome!

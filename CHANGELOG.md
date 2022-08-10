# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0] - 2022-08-10

## Changed

- [breaking-change] Minimum Supported Rust Version (MSRV) guarantee has been removed

## Fixed

- fixed miri errors around use of `mem::uninitialized`; updated code to use `MaybeUninit`

## [v0.1.2] - 2022-08-10

## Added

- added support for hexadecimal formatting of integers using the `{:x}` format string

## [v0.1.1] - 2022-08-09

### Fixed

- fixed link in crate level documentation
- fixed `uDebug` implementation for `i32` / `u32` on 16-bit platforms (like AVR and MSP430)

## v0.1.0 - 2019-11-17

Initial release

[Unreleased]: https://github.com/japaric/ufmt/compare/ufmt-v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/ufmt/compare/ufmt-v0.1.2...ufmt-v0.2.0
[v0.1.2]: https://github.com/japaric/ufmt/compare/ufmt-v0.1.1...ufmt-v0.1.2
[v0.1.1]: https://github.com/japaric/ufmt/compare/ufmt-v0.1.0...ufmt-v0.1.1

# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0] - 2022-08-10

### Changed

- [breaking-change] Minimum Supported Rust Version (MSRV) guarantee has been removed
- [breaking-change] the capacity type parameter `N` of `LineBuffered` is now a constant, rather than a type

### Removed

- [breaking-change] the `consts` module has been removed

## [v0.1.1] - 2020-02-11

### Added

- a `WriteAdapter` that lets one `uwrite!` types that implement `core::fmt::Write`

## v0.1.0 - 2019-11-17

Initial release

[Unreleased]: https://github.com/japaric/ufmt/compare/ufmt-utils-v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/ufmt/compare/ufmt-utils-v0.1.1...ufmt-utils-v0.2.0
[v0.1.1]: https://github.com/japaric/ufmt/compare/ufmt-utils-v0.1.0...ufmt-utils-v0.1.1

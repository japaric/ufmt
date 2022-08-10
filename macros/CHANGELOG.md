# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.1.2] - 2022-08-09

### Fixed

- `derive(uDebug)` on enums that have no variants

## [v0.1.1] - 2020-02-11

### Fixed

- fully qualify internal uses of `core::result::Result` to avoid problems when derive in presence of an imported `Result` type that's not libcore's

## v0.1.0 - 2019-11-17

Initial release

[Unreleased]: https://github.com/japaric/ufmt/compare/ufmt-macros-v0.1.2...HEAD
[v0.1.2]: https://github.com/japaric/ufmt/compare/ufmt-macros-v0.1.1...ufmt-macros-v0.1.2
[v0.1.1]: https://github.com/japaric/ufmt/compare/ufmt-macros-v0.1.0...ufmt-macros-v0.1.1

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/RustyNova016/api_bindium/compare/v0.4.0...v0.4.1) - 2026-04-17

### Other

- fmt
- fix clippy hack
- more lints
- *(lint)* unused_trait_names
- *(lint)* return_self_not_must_use
- *(lint)* redundant_else
- *(lint)* cast_lossless
- *(lint)* std_instead_of_core

## [0.4.0](https://github.com/RustyNova016/api_bindium/compare/v0.3.1...v0.4.0) - 2026-04-16

### Added

- added headers_mut
- addclient response
- use self on parse trait method

### Other

- *(deps)* update hotpath requirement from 0.14.0 to 0.15.0
- lints
- [**breaking**] moved parsers
- move the error

## [0.3.1](https://github.com/RustyNova016/api_bindium/compare/v0.3.0...v0.3.1) - 2026-03-04

### Other

- update deps

## [0.3.0](https://github.com/RustyNova016/api_bindium/compare/v0.2.2...v0.3.0) - 2026-01-21

### Added

- generic body parsers

## [0.2.2](https://github.com/RustyNova016/api_bindium/compare/v0.2.1...v0.2.2) - 2026-01-19

### Other

- fix docs
- try release-plz ci
## [0.2.1] - 2026-01-15

### 🚀 Features

- Set send_with_retries_async publiic like the sync version
- Docs.rs doesn't scrap exemples
- Allow changing the parser of a api request

### 💼 Other

- *(deps)* Bump actions/checkout from 5 to 6

### ⚙️ Miscellaneous Tasks

- Reexport governor as a module
## [0.2.0] - 2026-01-15

### 🚀 Features

- Image parser
- Set max body size in API request

### ⚙️ Miscellaneous Tasks

- Add semver-checks
## [0.1.1] - 2026-01-07

### 🚀 Features

- Make send_with_retries public

### 📚 Documentation

- Documentation improvements

### ⚙️ Miscellaneous Tasks

- Bump version
- Add trusted publishing
- Clippy fixes
## [0.1.0] - 2026-01-07

### 🚀 Features

- Initial crate
- Post requests
- Rustls_feature
- Add location to errors
- Add parameter encoding
- UriBuilderError
- Added other parsers
- Passthrough cookies feature
- Request headers
- Retry on peer disconect

### 🐛 Bug Fixes

- Clippy lints

### 💼 Other

- *(deps)* Bump actions/checkout from 5 to 6
- *(deps)* Update hotpath requirement from 0.7.3 to 0.9.0

### 🧪 Testing

- Added get request testing

### ⚙️ Miscellaneous Tasks

- Rename crate
- Use each-feature instead of powerset
- Release v0.1.0
- Crates.io requirements

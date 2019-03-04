# tao-log

[![Crates.io](https://img.shields.io/crates/v/tao-log.svg?maxAge=3600)](https://crates.io/crates/tao-log)
[![Rustdoc](https://docs.rs/tao-log/badge.svg)](https://docs.rs/tao-log)
[![Travis CI Build](https://travis-ci.org/dekellum/tao-log.svg?branch=master)](https://travis-ci.org/dekellum/tao-log)
<!-- [![Appveyor CI Build](https://ci.appveyor.com/api/projects/status/FIXME/branch/master?svg=true)](https://ci.appveyor.com/project/dekellum/tao-log) -->
[![deps status](https://deps.rs/repo/github/dekellum/tao-log/status.svg)](https://deps.rs/repo/github/dekellum/tao-log)

## Extension macros for output to the rust _log_ crate.

> <em>“Why write an RFC, when you can just write code?”</em>
> — 老子 (“Old Master”), 557 BCE

This unenlightened wanderer (游侠) wrote a _log_ [PR], [RFC 317], and
_only then_ published this as a standalone crate, in the hopes of
gaining insights through use or abuse in the wild.

## Minimum supported rust version (MSRV)

1.31.0

The project CI tests with this version, included both 2015 and 2018
edition external macro imports. A PR would be considered to backport
the project to earlier rust versions, possibly as far back as
1.16.0. (_log_'s current MSRV). Or consider lobbying for the
acceptance of this feature in _log_.

[log]: https://docs.rs/crate/log
[PR]: https://github.com/rust-lang-nursery/log/pull/316
[RFC 317]: https://github.com/rust-lang-nursery/log/pull/317

## License

This project is dual licensed under either of following:

* The Apache License, version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)

* The MIT License ([LICENSE-MIT](LICENSE-MIT)
  or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in body-image by you, as defined by the Apache License, shall be
dual licensed as above, without any additional terms or conditions.
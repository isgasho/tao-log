// Copyright Ⓒ 2019 David Kellum
//
// These _-v_ macros were co-developed with
// https://github.com/rust-lang-nursery/log/pull/316 (by the same author)
//
// All original _log_ source is offered under the same Apache 2.0 or MIT
// licenses, and is:
// Copyright Ⓒ 2015 The Rust Project Developers

/// Log an expression and its value at any specified level.
///
/// Logs with the optional or default (module path of use) target, specified
/// `Level`, optional prefix, and optional or default (`"{:?}"`) value format
/// string, and a single expression. The expression argument is evaluated
/// exactly once, regardless of if the logging level is enabled, and its
/// value is returned from the macro. This is normally only used through the
/// _-v_ macros like `debugv!` or `tracev!`.
///
/// Note that the value is moved and then returned. If the type does not
/// implement `Copy`, ownership may be retained by borrowing by reference
/// e.g. `debugv!(&expr)`.
///
/// # Examples
///
/// ```rust
/// use tao_log::*;
///
/// #[derive(Debug)]
/// struct Point { x: f32, y: f32 }
///
/// fn circle(center: &Point, radius: f32) { /*...*/ }
///
/// # fn main() {
/// let center = Point { x: 3.234, y: -1.223 };
///
/// circle(logv!(log::Level::Trace, &center), 7.3);
/// //     ^-- trace level message: "&center → Point { x: 3.234, y: -1.223 }"
/// circle(tracev!(&center), 8.0);
/// //     ^-- trace level message: "&center → Point { x: 3.234, y: -1.223 }"
/// # }
/// ```
#[macro_export]
macro_rules! logv {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => (
        $crate::__tao_logv!($lvl, target: $target, $($arg)+)
    );
    ($lvl:expr, $($arg:tt)+) => (
        $crate::__tao_logv!($lvl, target: module_path!(), $($arg)+)
    );
}

/// Log an expression at the error level, returning its value.
#[macro_export]
macro_rules! errorv {
    ($($arg:tt)+) => ($crate::__tao_logv!($crate::log::Level::Error, $($arg)+))
}

/// Log an expression at the warn level, returning its value.
#[macro_export]
macro_rules! warnv {
    ($($arg:tt)+) => ($crate::__tao_logv!($crate::log::Level::Warn, $($arg)+))
}

/// Log an expression at the info level, returning its value.
#[macro_export]
macro_rules! infov {
    ($($arg:tt)+) => ($crate::__tao_logv!($crate::log::Level::Info, $($arg)+))
}

/// Log an expression at the debug level, returning its value.
#[macro_export]
macro_rules! debugv {
    ($($arg:tt)+) => ($crate::__tao_logv!($crate::log::Level::Debug, $($arg)+))
}

/// Log an expression at the trace level, returning its value.
#[macro_export]
macro_rules! tracev {
    ($($arg:tt)+) => ($crate::__tao_logv!($crate::log::Level::Trace, $($arg)+))
}

// Helper macro for the -v macros, handling the permutations of optional
// parameters. Note: The required level parameter is first here for
// convenience of internal use with variable-args.
#[doc(hidden)]
#[macro_export]
macro_rules! __tao_logv {
    ($lvl:expr, target: $tgt:expr, $pre:expr, $vfmt:expr, $exp:expr) => (
        $crate::__tao_v_eval!($tgt, $lvl, concat!($pre, " {} → ", $vfmt), $exp)
    );
    ($lvl:expr, target: $tgt:expr, $pre:expr, $exp:expr) => (
        $crate::__tao_v_eval!($tgt, $lvl, concat!($pre, " {} → {:?}"), $exp)
    );
    ($lvl:expr, target: $tgt:expr, $exp:expr) => (
        $crate::__tao_v_eval!($tgt, $lvl, "{} → {:?}", $exp)
    );
    ($lvl:expr, $pre:expr, $vfmt:expr, $exp:expr) => (
        $crate::__tao_v_eval!(module_path!(), $lvl, concat!($pre, " {} → ", $vfmt), $exp)
    );
    ($lvl:expr, $pre:expr, $exp:expr) => (
        $crate::__tao_v_eval!(module_path!(), $lvl, concat!($pre, " {} → {:?}"), $exp)
    );
    ($lvl:expr, $exp:expr) => (
        $crate::__tao_v_eval!(module_path!(), $lvl, "{} → {:?}", $exp)
    );
}

// Inner helper macro for __tao_logv. Evaluates expression exactly once, moves
// value and returns it.
#[doc(hidden)]
#[macro_export]
macro_rules! __tao_v_eval {
    ($tgt:expr, $lvl:expr, $fmt:expr, $exp:expr) => (
        match $exp {
            vt => {
                $crate::log!(target: $tgt, $lvl, $fmt, stringify!($exp), &vt);
                vt
            }
        }
    )
}


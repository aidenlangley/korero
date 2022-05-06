//! Kōrero, a communication library. Contains code that I find myself copying
//! and pasting often.
//!
//! # TerminalLogger
//! Provides a [`TerminalLogger`](https://docs.rs/korero/latest/korero/output/struct.TerminalLogger.html)
//! to use out of the box. Construction requires a [`Verbosity`](https://docs.rs/korero/latest/korero/output/verbosity/enum.Verbosity.html)
//! level to make use of [`MinVerbosity`](https://docs.rs/korero/latest/korero/output/verbosity/trait.MinVerbosity.html)
//! checks.
//!
//! The source file for `TerminalLogger`, `output/mod.rs`, has an example of how
//! you would implement the various traits provided by this library. In this
//! particular instance, `TerminalLogger` is logging, but the intention is that
//! another operation would implement:
//! * [`Logs`](https://docs.rs/korero/latest/korero/output/logger/trait.Logs.html)
//!   in order to access a logger that presumably exists on a struct.
//! * [`Verbose`](https://docs.rs/korero/latest/korero/output/verbosity/trait.Verbose.html)
//!   to declare how verbose the logger will be at runtime.
//! * [`MinVerbosity`](https://docs.rs/korero/latest/korero/output/verbosity/trait.MinVerbosity.html)
//!   to declare at what level of verbosity the logger should print. So if the
//!   logger is set to `Medium` verbosity, and the `MinVerbosity` for this
//!   operation is `Medium`, then the user will see some output.
//! * [`Logger`](https://docs.rs/korero/latest/korero/output/logger/trait.Logger.html)
//!   which describes how the logger will write to the terminal (or some other
//!   destination.)
//!
//! A logger would implement [`Prints`](https://docs.rs/korero/latest/korero/output/logger/trait.Prints.html)
//! for each type it would like to output.
//!
//! More on usage [here](https://github.com/aidenlangley/nedots/tree/main/installer).

pub mod http;
pub mod output;

#[cfg(test)]
mod tests;

cargo test
   Compiling tulipchain v0.1.0 (file:///Users/michaelnoronha/Documents/workspace/rust/tulipchain)
error[E0432]: unresolved import `super::Address`
 --> src/blockchain.rs:6:5
  |
6 |     Address,
  |     ^^^^^^^ no `Address` in the root. Did you mean to use `address`?

error: internal compiler error: librustc/ich/impls_ty.rs:906: ty::TypeVariants::hash_stable() - Unexpected variant TyInfer(?1).

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (5508b2714 2018-03-18) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `tulipchain`.

To learn more, run the command again with --verbose.
make: *** [test] Error 101


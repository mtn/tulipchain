   Compiling tulipchain v0.1.0 (file:///Users/michaelnoronha/Documents/workspace/rust/tulipchain)
error: chained comparison operators require parentheses
  --> src/blockchain.rs:96:34
   |
96 |             signed_digest: Option<SignedDigest>,
   |                                  ^^^^^^^^^^^^^^^
   |
   = help: use `::<...>` instead of `<...>` if you meant to specify type arguments
   = help: or use `(...)` if you meant to specify fn arguments

error: expected expression, found `,`
  --> src/blockchain.rs:96:48
   |
92 |         let coinbase_transaction = Transaction {
   |                                    ----------- while parsing this struct
...
96 |             signed_digest: Option<SignedDigest>,
   |                                                ^ expected expression

error[E0432]: unresolved import `super::Address`
 --> src/blockchain.rs:6:5
  |
6 |     Address,
  |     ^^^^^^^ no `Address` in the root. Did you mean to use `address`?

error[E0423]: expected value, found type alias `PublicKey`
  --> src/blockchain.rs:94:29
   |
94 |             recipient_addr: PublicKey,
   |                             ^^^^^^^^^
   |
   = note: can't use a type alias as a constructor
help: possible better candidates are found in other modules, you can import them into scope
   |
1  | use sodiumoxide::crypto::box_::PublicKey;
   |
1  | use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey;
   |
1  | use sodiumoxide::crypto::sign::PublicKey;
   |
1  | use sodiumoxide::crypto::sign::ed25519::PublicKey;
   |

error[E0425]: cannot find value `Tulips` in this scope
  --> src/blockchain.rs:95:20
   |
95 |             value: Tulips,
   |                    ^^^^^^ not found in this scope

error[E0425]: cannot find value `coinbase_hash` in this scope
   --> src/blockchain.rs:105:13
    |
105 |             coinbase_hash,
    |             ^^^^^^^^^^^^^ not found in this scope

error: internal compiler error: librustc/ich/impls_ty.rs:906: ty::TypeVariants::hash_stable() - Unexpected variant TyInfer(?1).

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (5508b2714 2018-03-18) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `tulipchain`.

To learn more, run the command again with --verbose.


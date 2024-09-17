## Release profiles
Prefined configurations for compiling. Two main profiles:
1. dev - cargo build
1. release - cargo build --release

## Publishing to crates.io
### Documentation comments
Use three slashes to denote documentation comments.
You can use markdown here. This documentation is intended for consumers of your API, it should not include implementation details.

To generate the HTML documentation, run "cargo doc". This runs the rustdoc tool (part of rust) and puts the generated HTML in target/doc.
Can also run cargo doc --open to open the docs in your browser

#### Testing doc examples
The Examples header in your doc comments will be treated as tests by cargo!
This means that running cargo test will test your documentation examples.

#### //!
Use //! (instead of ///) to write doc comments that pertain to the thing which the comment is inside (instead of the thing following the comments).
This is useful for documenting the crate itself by putting such comments at the crate root

### Log in to crates.io
Create an account and save your apikey to ~/.cargo/credentials with
```
cargo login <apikey>
```

### Publishing
```
cargo publish
```
You cannot remove a crate once it is published, otherwise you could break someone using your crate.
You can prevent any future projects from adding a version of your crate by deprecating it with
```
cargo yank --vers 1.0.1
```

## Workspaces
A workspace is a set of related packages that share the same Cargo.lock and target dir. To create workspace, create the directory and a Cargo.toml file like
```toml
[workspace]
members = [
    "binary_crate",
    "library_crate_1",
    "library_crate_2",
]
```
and run "cargo new package" within your workspace directory to create these packages.
The packages can depend on the other packages in this workspace (though this must be explicitly listed via each package's Cargo.toml).
The packages can also have external dependencies, the workspace will resolve duplicate dependencies into a single version which is stored in Cargo.lock.

## cargo install
This command lets you install and use binary crates locally - as a convenient way for Rust devs to install tools off crates.io.
By default, these are installed to _$HOME/.cargo/bin_, which should be on your PATH.

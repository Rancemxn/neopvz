# neopvz

`neopvz` is a clean-room Rust reimplementation of the observable gameplay of
Plants vs. Zombies 1.0.0.1051. The simulation and rendering algorithms may be
different from the original program; the target is equivalent gameplay,
timing, animation, audio, progression, and all player-accessible content.

The repository intentionally contains no original game assets, binaries, IDA
databases, function tables, or reverse-engineering reference projects. Supply
your own legally obtained resource directory or archive at runtime:

```text
neopvz --data-dir C:\path\to\resources
neopvz --pak C:\path\to\main.pak
```

The loader auto-detects a directory containing `properties/resources.xml`, a
directory containing `main.pak`, or an explicit PAK path. Resource loading is
kept behind an interface so desktop and mobile asset providers can differ.

## Development

For implementation changes, run the local stable Cargo gate:

```text
cargo +stable fmt --all -- --check
cargo +stable clippy --workspace --all-targets --locked -- -D warnings
cargo +stable test --workspace --locked
cargo +stable build --workspace --locked
```

Use a local debug build with externally supplied 1.0.0.1051 resources for GUI,
visual, and audio validation. GitHub Actions may record supplementary results,
but they do not replace the local gate. Original resources and local comparison
evidence must never be uploaded to the repository or Actions.

Use issues and milestones for work planning and pull requests for changes. Tags
and releases will be introduced after the implementation is stable.

See [the development workflow](docs/development.md) for verification and
reference boundaries. The bounded compatibility contract is in [loop.md](loop.md),
and current progress is recorded in the [compatibility ledger](docs/compatibility.md).

## License

This project is licensed under the GNU Affero General Public License, version
3 or any later version. The license applies to the implementation only; the
Plants vs. Zombies intellectual property and user-supplied assets remain
owned by their respective rights holders.

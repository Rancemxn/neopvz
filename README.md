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

Do not run `cargo build`, `cargo test`, or other compilation commands locally.
The local checkout does not have the complete test/resource environment. Use
the GitHub Actions `full-gate` workflow from a branch or pull request:

```text
gh workflow run full-gate.yml --repo Rancemxn/neopvz --ref <branch>
```

Use issues and milestones for work planning, pull requests for changes, and
Actions for verification. Tags and releases will be introduced after the
implementation is stable.

The complete gate runs on Ubuntu. The `windows-artifact` workflow builds a
resource-free Windows executable for download and local GUI, visual, and audio
verification. Original resources and local comparison evidence must never be
uploaded to the repository or Actions.

See [the development workflow](docs/development.md) for verification and
reference boundaries. The bounded compatibility contract is in [loop.md](loop.md),
and current progress is recorded in the [compatibility ledger](docs/compatibility.md).

## License

This project is licensed under the GNU Affero General Public License, version
3 or any later version. The license applies to the implementation only; the
Plants vs. Zombies intellectual property and user-supplied assets remain
owned by their respective rights holders.

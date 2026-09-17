# arvo-plugin-yahoo

Yahoo Finance as an [Arvo](https://github.com/wjpin84/arvo-desktop) source
plugin: a process that serves `arvo.source.v1.Source` over gRPC, so Arvo lists
its two Yahoo sources (split-adjusted under venue `YF`, total return under
`YFTR`) exactly as it did when they were compiled in.

This is the first source to live outside the app, and the reference for how a
source plugin is shaped (ADR-0022). Everything Yahoo-specific stays in
`arvo-yfinance` inside `arvo-desktop`; this repository is the process boundary
and nothing else. `src/main.rs` is the whole plugin.

## Run it

```sh
cargo run --release
```

It listens on `127.0.0.1:50052`. Set `ARVO_PLUGIN_ADDR` to change that.

Then tell Arvo where it is. Arvo reads `plugins.toml` from its configuration
directory:

| OS | Path |
|---|---|
| Windows | `%APPDATA%\com.arvo.desktop\plugins.toml` |
| macOS | `~/Library/Application Support/com.arvo.desktop/plugins.toml` |
| Linux | `~/.config/com.arvo.desktop/plugins.toml` |

```toml
[[plugin]]
id = "yahoo"
address = "http://127.0.0.1:50052"
```

Or let Arvo start it: `command = "path/to/arvo-plugin-yahoo"` in place of
`address`, and Arvo launches it on a port of its own choosing, restarts it if
it dies, and stops it when Arvo quits. Either way it is the same plugin in the
same registry.

Arvo probes it at launch and every thirty seconds. The Extensions view shows it
under Providers as Reachable, and Fetch offers both Yahoo sources. When the
plugin is running, it serves in place of the compiled-in Yahoo; when it is
not, the compiled-in one still answers.

## What the boundary carries

The proto mirrors `arvo_data::source::Source` method for method: Describe,
Connected, Bars, Dividends, Search, Quotes. A description carries the id, the
label, the venue, the credential kind, and the **basis** (which feed, which
adjustment), which is the declaration the boundary exists to keep: two sources
on different bases are two datasets, and a plugin that does not declare its
basis is refused by the host rather than filed under a default.

Errors cross as one gRPC status code per `SourceError` variant, and the host
rebuilds the variant on its side, so `needs_sign_in` survives the boundary.

Yahoo needs no credential. A plugin that does (Alpaca, next) receives what one
call needs and never holds a secret (ADR-0022 point 4).

## Installing from GitHub

`arvo-extension.json` declares this repository as a provider extension with
its recipe (ADR-0025):

```json
"build": "cargo build --release",
"run": "target/release/arvo-plugin-yahoo"
```

Paste `wjpin84/arvo-plugin-yahoo` into Arvo's Extensions view. Install clones
the repository at a pinned commit and runs nothing. The extension then shows
the recipe verbatim with a Build button; confirming runs the build in the
extension's own folder with its output in the Output panel, and copies the
binary into a cache Arvo owns. The Extensions view says whether it built.

Once built, Arvo starts it itself (ADR-0023): the binary is launched from the
cache on a port of Arvo's choosing, appears under Providers, is restarted a few
times if it dies and then reported, and is stopped when the extension is
disabled or removed and when Arvo quits. Nothing goes in `plugins.toml`.

## Building this repository

The host crates are git dependencies on `arvo-desktop`, pinned to a tag.
`arvo-desktop` is private, so building needs read access to it; `.cargo/config.toml`
makes cargo fetch with the git CLI so your credential helper is used.

```sh
cargo test
```

The one test starts the plugin in-process, discovers it the way Arvo does, and
checks that every declaration of both Yahoo sources came through unchanged. It
touches no network.

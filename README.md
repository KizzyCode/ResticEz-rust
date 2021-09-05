[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/ResticEz-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/ResticEz-rust)
[![docs.rs](https://docs.rs/restic_ez/badge.svg)](https://docs.rs/restic_ez)
[![crates.io](https://img.shields.io/crates/v/restic_ez.svg)](https://crates.io/crates/restic_ez)
[![Download numbers](https://img.shields.io/crates/d/restic_ez.svg)](https://crates.io/crates/restic_ez)
[![dependency status](https://deps.rs/crate/restic_ez/0.1.0/status.svg)](https://deps.rs/crate/restic_ez/0.1.0)


# `restic-ez`
Welcome to `restic-ez` 🎉

`restic-ez` provides a simple configuration loader and API around `restic`.


## Configuration example
```toml
[restic]
dir = "/Development"
repo = "s3:https://<url to s3 bucket>"
safe_restore = true

[restic.flags]
backup = [
    "--exclude-caches",
    "--iexclude", "/Development/Code/Swift/*/.swiftpm",
    "--iexclude", "/Development/Code/Swift/*/.build"
]

[credentials.restic]
command = "set -euo pipefail; pass <path to password file> | sed -n 1p"

[credentials.s3]
id = { value = "<insert your s3 key id here>" }
secret = { command = "set -euo pipefail; pass <path to password file> | sed -n 2p" }
```

## Man page
```
Usage: restic-ez [verb]

Possible verbs are:
    create      Creates a new archive
    list        Lists the existing archives
    restore     Restores the latest archiv tagged with \"backup\"
    tmux        Opens a tmux session configured for easy manual restic invocation
    check       Checks the repository for consistency
    prune       Removes all unused chunks from the repository
    break-lock  Breaks a stale lock
    help        Displays this help

Specify the configuration using environment variables:
    RESTIC_EZ_CONFIG_TOML  Specifies the raw configuration toml string
    RESTIC_EZ_CONFIG       Specifies a custom path to a configuration file

Per default, restic-ez looks for the following config files:
    ./restic-ez
    ./restic-ez.conf
    ./restic-ez.toml
    ./.restic-ez
    ./.restic-ez.conf
    ./.restic-ez.toml
```
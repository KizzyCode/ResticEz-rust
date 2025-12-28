[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/ResticEz-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/ResticEz-rust)
[![docs.rs](https://docs.rs/restic_ez/badge.svg)](https://docs.rs/restic_ez)
[![crates.io](https://img.shields.io/crates/v/restic_ez.svg)](https://crates.io/crates/restic_ez)
[![Download numbers](https://img.shields.io/crates/d/restic_ez.svg)](https://crates.io/crates/restic_ez)
[![dependency status](https://deps.rs/crate/restic_ez/latest/status.svg)](https://deps.rs/crate/restic_ez)


# `restic-ez`
Welcome to `restic-ez` 🎉

`restic-ez` provides a simple configuration loader and CLI-API around `restic`.


## Configuration example
```toml
[restic]
dirs = "/Development"
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

[commands]
preexec = [
    "touch first-preexec-command",
    "sleep 3",
    "touch second-preexec-command",
]
postexec = [
    "touch first-postexec-command",
    "sleep 3",
    "touch second-postexec-command",
]
```

## Man page
```
restic-ez v0.5.1

Usage: restic-ez [/my/customconfig.toml] [verb]

Verbs:
    create   Creates a new archive
    list     Lists the existing archives
    restore  Restores the latest archiv tagged with \"backup\"
    shell    Opens a shell session configured for easy manual restic invocation
    check    Checks the repository for consistency
    prune    Removes all unused chunks from the repository
    unlock   Breaks a stale lock
    help     Displays this help

Config:
    Explicitly specify the configuration file by passing the path as first argument:
        /my/customconfig.toml  An explicit path to a *.toml configuration file
        /my/customconfig.conf  An explicit path to a *.conf configuration file

    Explicitly specify the configuration using environment variables:
        RESTIC_EZ_CONFIG_TOML  Specifies the raw configuration toml string
        RESTIC_EZ_CONFIG       Specifies a custom path to a configuration file

    Per default, restic-ez looks for the following configuration files:
        ./restic-ez.conf
        ./restic-ez.toml
        ./.restic-ez.conf
        ./.restic-ez.toml
```

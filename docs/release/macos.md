# macOS

The game and the dedicated server build for macOS. There is no macOS zip in a release yet, so
the two are built from source, into `dist/macos`: `skate2-pc` (the game, which also hosts with
`--server`), `skate2-server` (the server alone) and **`Skate 2.app`**, the bundle to open. Built
from the same `release-secret.txt` and version as the Windows and Linux programs, they play
together with them on one server
([auto-update.md](auto-update.md#the-release-key-and-the-build-secret)).

## Playing

**The game needs:**

- An Apple silicon Mac. The build is arm64; Intel Macs are untested.
- A GPU with Metal. wgpu has no software fallback here, so a Mac without Metal cannot run it.
- CMake, Ninja and the Xcode command line tools, which SDL3's source build uses, and a current
  stable Rust toolchain.
- **Your own copy of retail Skate 2 for Xbox 360**, extracted to a folder.

```sh
./build-release-macos.sh          # dist/macos/skate2-pc and 'dist/macos/Skate 2.app'
open "dist/macos/Skate 2.app"
```

The first start asks for your copy's `.xex`; the folder it is in is kept as `config.json`'s
`game_root`, as on the other platforms. `SKATE2_GAME_ROOT` names it without the dialog.

The app starts itself in `~/Library/Application Support/Skate 2`, so the game's files stay
together there: `config.json`, `online-config.json`, `saves/` (replays and photos in
`saves/skate_reel/`, crash reports in `saves/crash_reports`) and `game.log`, which holds
everything the game printed. Launch the **app**, not the executable inside it: winit takes
`NSApplication` as the process's own, and a program started straight from a terminal has no
application to put its dialogs in front of.

The bundle is ad-hoc signed, which is all a local build can be: it runs from where you build it.
Moved elsewhere and blocked by Gatekeeper, right-click it and choose **Open**.

**Controllers:** SDL3 reads every pad, as on Linux: Xbox, PlayStation, Switch and others.

**MP4 export:** FFmpeg with H.264 and AAC, as `ffmpeg` beside the game or on `PATH`.

## Hosting

`./skate2-server [ip:port]` runs the server alone and `./skate2-pc --server [ip:port]` the same
server from the game; the game's arm64 build is what specifies either. There is nothing to
update: a macOS build has no release zip, so `--update` prints that this build does not update
and stops.

## Building

```sh
xcode-select --install                                        # if the command line tools are missing
brew install cmake ninja                                      # or any CMake and Ninja
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
./build-release-macos.sh            # both programs and the app bundle
./build-release-macos.sh game       # the game and the bundle only
./build-release-macos.sh server     # the server only
```

The game's crates need several GB each to compile, so the script builds with two compiler jobs,
which keeps a 16 GB machine off its swap file. The build writes `~` in place of your home folder
into the binaries, which would otherwise name you. With `release-secret.txt` beside `Cargo.toml`
they are release builds ([auto-update.md](auto-update.md)).

The bundle's icon is made at build time from `apps/skate2-pc/skate2-macos.png`, the Windows
icon's art in the rounded shape macOS icons use: `sips` renders each size an `.icns` holds and
`iconutil` packs them, so nothing extra has to be installed. Both programs are ad-hoc signed
afterwards, which leaves the bundle's files as its own.

## Where macOS differs from Windows and Linux

- **The event loop is made before the first dialog.** winit must take `NSApplication` as the
  process's principal class before any other part of the application does, and the game asks for
  its folder through rfd's file dialog first. `render_window::prepare_event_loop` therefore makes
  the event loop in the player's launch, before `game_root::resolve`, and
  `render_window::run_with_options` takes that one. Without it the first launch stops at
  "`winit` requires control over the principal class".
- **The first start's messages do not appear.** rfd 0.17 shows message boxes on macOS through
  `CFUserNotificationDisplayAlert`, which recent macOS releases no longer present for an
  application that is already running: the "Welcome! Locate your copy of Skate 2" notice and a
  fatal error's box are silently passed over, and the file dialog is what the player sees. A
  fatal error is still written to `game.log` and `saves/crash_reports`. Started from a terminal
  instead of the bundle, the same call can block with no window at all, which is the second
  reason to launch the app.
- **No self-update.** `Platform::current()` is `None` where releases have no zip, so the updater
  is inert ([auto-update.md](auto-update.md#how-it-works-crates-skate2-update)).

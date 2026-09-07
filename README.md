# Barkely's Highlands Adventure

A native Rust adventure about planning a curious terrier's walk through Highlands.
Build a plan with mouse-driven command cards, predict its result, then watch or
step through Barkely's actions. Unlimited retries, no accounts, and no network
connection during play.

## Launch

Install a stable Rust toolchain (1.93 or newer), then run from this directory:

```sh
cargo run
```

Cargo needs internet access for the initial dependency download. After that,
`cargo run --offline` works. For a standalone optimized executable:

```sh
cargo build --release
./target/release/barkely-highlands
```

Artwork and sound are generated in Rust; there are no asset downloads or web runtimes.

## Play

Click **Begin the adventure**, then add command cards. **RUN** executes the plan;
**STEP** performs one action; **Pause** stops after the current action finishes.
**Reset / edit plan** restores the world and keeps your cards. Select cards to
replace, remove, or move them. Later trails introduce repeated groups, waiting,
prediction flags, fetching, switches, and sniffing for hidden balls.

All controls work with a mouse. Optional shortcuts: Space runs/pauses, Enter
steps, R resets, Escape returns to the map. The question-mark button explains
play. Sound volume, slower animation, and a grid overlay are available onscreen.

The 16 trails are:

1. A familiar whistle
2. Around the flower bed
3. Which way is forward?
4. The old oak
5. A muddled little plan
6. Two ways through
7. The long meadow
8. The winding garden
9. After you, Mudge
10. A gap in the patrol
11. Catch up with Henry
12. One very important ball
13. Over the silver creek
14. The secret garden gate
15. A nose for adventure
16. Home before the picnic

## Saves

Completed trails, best card counts, and settings save automatically. On macOS,
the file is `~/Library/Application Support/BarkelyHighlands/progress.json`.
Windows uses `%APPDATA%/BarkelyHighlands`; Linux uses
`$XDG_DATA_HOME/BarkelyHighlands` or `~/.local/share/BarkelyHighlands`.
Set `BARKELY_DATA_DIR` to choose another save directory. Unreadable saves are
preserved and a warning appears. Unfinished command plans are not saved.

## Architecture and checks

`simulation.rs` runs deterministic grid actions independently of graphics.
Every action advances moving characters by one beat. Collisions include actors
swapping positions; objectives are evaluated after the plan ends, so multiple
valid solutions work. `commands.rs` expands repeat cards while retaining their
source card for highlighting. `mission.rs` holds authored trails; `game.rs`
coordinates the editor and animation, and `art.rs`/`ui.rs` render the park.

```sh
cargo fmt --check
cargo check
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo run --features smoke-test -- --smoke
```

The last command opens a real window and solves all 16 trails through rendered
mouse buttons, testing progression and saving screenshots in the system temporary
directory under `barkely-smoke`. It uses isolated progress and does not overwrite
the player's save. It requires a desktop display. Simulation tests also check
incorrect plans, timing, multiple solutions, repeats, items, and save recovery.

## First-version scope

Includes the full 16-trail progression and all five canonical characters.
Conditions, named routines, nested repeats, and an in-game mission authoring
tool are deferred. Future improvements should start with child playtesting and
accessibility (adjustable text size and keyboard-only navigation). Desktop
packaging and testing on Windows/Linux remain separate follow-up work.

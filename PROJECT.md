# Barkely's Highlands Adventure

## Project Vision

Build a polished, fun, offline 2D educational adventure game designed primarily for approximately a **9-year-old child**.

The main character is **Barkely**, a brown-and-white Jack Russell Terrier exploring a park called **Highlands**.

The central gameplay mechanic is giving Barkely a precise sequence of commands and then watching him execute those commands exactly.

The player should develop critical-thinking and early programming skills without feeling like they are completing programming worksheets.

The game should feel like an **adventure game first and an educational application second**.

The central learning cycle is:

**PLAN → PREDICT → RUN → OBSERVE → DEBUG → IMPROVE**

The player should gradually learn to think:

> "What exactly will happen when Barkely follows these instructions?"

---

# Educational Goals

The game should develop:

* Critical thinking
* Logical reasoning
* Planning
* Sequencing
* Algorithmic thinking
* Debugging
* Pattern recognition
* Spatial reasoning
* Cause-and-effect reasoning
* Problem decomposition
* Prediction
* Optimization
* Abstraction
* Understanding repetition
* Eventually, simple conditional reasoning

Teach these ideas through gameplay rather than formal computer-science lessons.

Do not require the player to know programming terminology.

---

# Technical Requirements

Build the game entirely in **Rust**.

Use:

* Rust stable
* Macroquad for graphics, input, sound, and the game loop
* Appropriate Rust crates where genuinely useful

Do NOT use:

* JavaScript
* TypeScript
* Node.js
* npm
* Tauri
* Electron
* Phaser
* WebView
* Python

Keep dependencies reasonable.

Favor straightforward Rust solutions over unnecessary frameworks and abstractions.

The development experience is extremely important.

A fresh clone of the repository should primarily require:

```sh
cargo run
```

to build and launch the game.

A release build should primarily require:

```sh
cargo build --release
```

The game must work completely offline.

---

# Setting: Highlands

The entire adventure takes place in and around a park called **Highlands**.

Highlands should feel like a real, connected place that Barkely enjoys exploring rather than a collection of unrelated puzzle grids.

Potential areas include:

* Open grass fields
* Walking paths
* Woods
* Picnic areas
* Playground
* Creek
* Bridges
* Hills
* Sports fields
* Benches
* Trees
* Fences
* Park entrances
* Hidden trails
* Maintenance areas
* Gardens
* Small shelters

Different areas of Highlands should naturally create different reasoning problems.

The world can use an underlying logical grid for puzzle simulation while visually appearing like a smooth, attractive park rather than an obvious spreadsheet-like grid.

---

# Canonical Characters

The following character descriptions are **canonical**.

These defining visual traits must remain consistent throughout the game, including:

* Gameplay sprites
* Animations
* Portraits
* Mission screens
* Menus
* Cutscenes
* Future artwork

Do not randomly change character appearances between missions.

---

## Barkely

Barkely is the main character.

Barkely is a:

**Brown-and-white Jack Russell Terrier**

He is:

* Small
* Energetic
* Curious
* Clever
* Determined
* Adventurous
* Occasionally mischievous

Barkely should have personality.

Possible animations include:

* Walking
* Running
* Turning
* Sitting
* Sniffing
* Tail wagging
* Looking around
* Becoming excited
* Becoming surprised
* Stopping suddenly
* Celebrating
* Reacting when his path is blocked

Barkely should feel like a character the player wants to help rather than simply a game piece moving around a grid.

---

## Mudge

Mudge is a:

**Rottweiler mix**

Mudge is noticeably larger than Barkely and must be visually distinct from him.

Mudge is a recurring character who creates strategic avoidance challenges.

Barkely generally wants to **avoid Mudge rather than confront him**.

There should be no violent interactions between the dogs.

Mudge might:

* Patrol a trail
* Block a route
* Guard an area
* Move according to a predictable pattern
* Temporarily occupy an important location
* Cause Barkely to choose another route
* Force Barkely to wait before crossing somewhere

Mudge becomes especially useful for teaching prediction.

For example:

```text
Mudge moves left
Mudge moves left
Mudge waits
Mudge moves right
Mudge moves right
Mudge waits
```

The player may have to determine when Barkely can safely cross the trail.

---

## Oliver

Oliver is a:

**9-year-old boy with bright orange hair**

His bright orange hair should be one of his most immediately recognizable visual characteristics.

Oliver is generally a friendly character.

Possible missions involving Oliver include:

* Reach Oliver
* Find Oliver
* Return to Oliver
* Retrieve Barkely's ball after Oliver throws it
* Follow a clue from Oliver
* Bring something to Oliver
* Determine where Oliver is waiting

---

## Henry

Henry is an:

**18-year-old boy with blonde hair**

Henry should look clearly older and taller than Oliver.

His blonde hair should remain consistent throughout the game.

Henry can participate in more complicated missions involving movement and prediction.

For example:

* Find Henry
* Intercept Henry while he walks through Highlands
* Predict where Henry will be
* Follow Henry's route
* Reach a location before Henry leaves
* Have Henry open a gate or change something in the environment

---

## Betsey

Betsey is a:

**47-year-old woman with long brown hair**

Her long brown hair should be a consistent identifying characteristic.

Betsey is a recurring friendly character.

Possible missions include:

* Return to Betsey
* Find Betsey
* Retrieve something for Betsey
* Determine which trail leads back to Betsey
* Reach Betsey while avoiding another obstacle
* Follow Betsey along a park trail

---

# Character Visual Consistency

Always preserve these defining traits:

**Barkely:** brown-and-white Jack Russell Terrier
**Mudge:** Rottweiler mix
**Oliver:** 9-year-old boy with bright orange hair
**Henry:** 18-year-old boy with blonde hair
**Betsey:** 47-year-old woman with long brown hair

If characters are drawn procedurally or with code-generated artwork, create reusable character-rendering components instead of independently recreating characters for each scene.

Characters should remain recognizable even when rendered relatively small.

The visual style should be:

* Colorful
* Warm
* Friendly
* Expressive
* Appropriate for a 9-year-old
* Polished enough to feel like a real game

Avoid making the game look like preschool educational software.

---

# Core Gameplay

The player does NOT normally control Barkely directly during a puzzle.

Instead, the player constructs a **plan** consisting of commands.

For example:

```text
FORWARD
FORWARD
TURN RIGHT
FORWARD
FORWARD
TURN LEFT
FORWARD
```

The player then presses:

**RUN**

Barkely executes the commands exactly as specified.

This is extremely important.

Do not automatically correct the player's program.

If the program contains a mistake, allow Barkely to demonstrate the consequence.

For example:

* Barkely turns the wrong direction
* Barkely reaches the wrong trail
* Barkely stops at a fence
* Barkely walks past Oliver
* Barkely reaches a dead end
* Barkely approaches Mudge at the wrong time
* Barkely misses his ball
* Barkely reaches the wrong side of the creek

The player should observe what happened, modify the plan, and try again.

Unlimited retries should be encouraged.

Failure should be informative and sometimes humorous, but never punitive.

---

# Fundamental Command System

Begin with only:

```text
FORWARD
TURN LEFT
TURN RIGHT
```

Do not introduce too many concepts immediately.

Later commands can include:

```text
WAIT
REPEAT
PICK UP
DROP
SNIFF
IF
IF BLOCKED
IF PATH CLEAR
IF MUDGE NEARBY
CALL ROUTINE
```

The terminology shown to the child can be friendlier than formal programming terminology.

---

# Visual Command Interface

Do not require the child to type programming syntax.

Commands should primarily be created using large graphical buttons or cards.

For example:

```text
[ ↑ FORWARD ]

[ ↶ TURN LEFT ]

[ ↷ TURN RIGHT ]

[ ⏱ WAIT ]
```

Selecting a command adds it to Barkely's plan.

Display the plan visually:

```text
BARKELY'S PLAN

1   ↑ Forward
2   ↑ Forward
3   ↷ Turn Right
4   ↑ Forward
5   ⏱ Wait

        [ ▶ RUN ]
        [ ⏭ STEP ]
```

Commands should be easy to:

* Add
* Remove
* Reorder
* Clear
* Replace

Mouse interaction should be sufficient.

Keyboard shortcuts may be provided but should not be required.

---

# Step Mode

**STEP mode is a major educational feature.**

Provide both:

**RUN**

and:

**STEP**

RUN executes the entire program.

STEP executes exactly one instruction.

Highlight the currently executing instruction.

For example:

```text
1   ↑ Forward       ✓
2   ↑ Forward       ✓
3   ↷ Turn Right    ← CURRENT
4   ↑ Forward
5   ↑ Forward
```

While command 3 is highlighted, Barkely should visibly perform that action.

Also provide:

* Pause
* Reset
* Edit Plan
* Run Again

This helps the child connect an abstract instruction with its physical effect.

---

# Central Learning Cycle

Gameplay should reinforce:

## 1. PLAN

Construct Barkely's instructions.

## 2. PREDICT

Think about what those instructions will cause.

## 3. RUN

Execute the plan.

## 4. OBSERVE

Watch exactly what Barkely does.

## 5. DEBUG

Determine why the result differed from the intended result.

## 6. IMPROVE

Change the plan and try again.

This cycle is more important than scores or rewards.

---

# Educational Progression

## Stage 1 — Sequencing

Teach that commands execute in order.

Use very small maps.

Example objective:

**Help Barkely reach Oliver.**

The player might create:

```text
FORWARD
FORWARD
TURN RIGHT
FORWARD
```

---

## Stage 2 — Turning and Orientation

Teach that Barkely has a direction he is facing.

Turning changes Barkely's direction but does not move him.

The player must mentally track Barkely's orientation.

---

## Stage 3 — Planning

Introduce obstacles such as:

* Trees
* Benches
* Fences
* Water
* Rocks
* Mud
* Playground equipment

The player must plan an entire route.

---

## Stage 4 — Debugging

Give the player a plan that almost works.

For example:

```text
FORWARD
FORWARD
TURN LEFT
FORWARD
FORWARD
```

Barkely reaches the wrong park bench.

The child must determine which instruction needs changing.

Do not immediately reveal the incorrect command.

---

## Stage 5 — Multiple Solutions

Maps should frequently allow multiple correct solutions.

Do NOT normally compare the player's command sequence against one hard-coded answer.

Instead, simulate the commands.

If Barkely legitimately accomplishes the mission objective, the solution succeeds.

This is extremely important.

Reward reasoning rather than guessing the developer's expected answer.

---

## Stage 6 — Efficiency

Once the player succeeds, optionally offer another challenge.

For example:

> Great! Barkely made it using 12 commands.
>
> Can you find a way using 9?

A longer correct solution remains correct.

Efficiency should be a bonus challenge, not a requirement for basic success.

---

## Stage 7 — Repetition

Introduce:

**REPEAT**

Instead of:

```text
FORWARD
FORWARD
FORWARD
FORWARD
```

the player can eventually construct something conceptually equivalent to:

```text
REPEAT 4
    FORWARD
```

Create Highlands paths where repetition becomes naturally useful.

The child should experience the usefulness of repetition before being given a formal explanation of loops.

---

## Stage 8 — Pattern Recognition

Create movement and map patterns.

For example:

```text
FORWARD
FORWARD
TURN RIGHT
```

might occur repeatedly.

Encourage the player to notice repeating structures.

---

# Moving Characters

Later missions should introduce moving characters.

This is an important increase in reasoning difficulty.

The player now needs to think about:

**SPACE + TIME**

rather than only space.

Mudge is particularly useful for this mechanic.

For example, Mudge might patrol:

```text
LEFT
LEFT
WAIT
RIGHT
RIGHT
WAIT
```

Meanwhile Barkely executes his own instructions.

The child must reason:

> "Where will Mudge be when Barkely reaches the trail?"

---

# WAIT

Introduce:

```text
WAIT
```

Some missions should require Barkely to wait for the correct opportunity.

For example:

```text
FORWARD
FORWARD
WAIT
WAIT
FORWARD
TURN RIGHT
FORWARD
```

This teaches that the fastest action is not always the correct action.

---

# Prediction Challenges

Selected missions should ask the player to predict what will happen before RUN.

Examples:

> Where do you think Barkely will finish?

> Will Barkely reach Oliver?

> Will Mudge be on the trail when Barkely gets there?

> Which direction will Barkely be facing?

> Will Barkely reach the ball?

After the prediction, execute the actual program.

Allow the child to compare their mental model with what actually happened.

Do not require prediction on every mission.

---

# Conditions

Eventually introduce simple conditional reasoning.

Examples might conceptually resemble:

```text
IF PATH CLEAR
    FORWARD
```

or:

```text
IF MUDGE NEARBY
    WAIT
```

or:

```text
IF BLOCKED
    TURN RIGHT
```

Keep the presentation intuitive and child-friendly.

The player does not need to be told:

"You are now learning conditional statements."

Let the gameplay demonstrate why conditions are useful.

---

# Routines and Decomposition

Eventually allow the player to create reusable groups of commands.

For example:

```text
CROSS BRIDGE

FORWARD
FORWARD
FORWARD
```

Then Barkely's larger plan could contain:

```text
CROSS BRIDGE
TURN RIGHT
FORWARD
```

This introduces decomposition and abstraction without requiring formal programming terminology.

---

# Mission Ideas

The game should tell a connected adventure story through Highlands.

Do not make missions feel like unrelated worksheets.

Examples include:

## Find Oliver

Barkely needs to navigate through the park to reach Oliver.

---

## Fetch the Ball

Oliver throws Barkely's ball.

Barkely must:

1. Navigate to the ball.
2. Pick it up.
3. Return to Oliver.

---

## Avoid Mudge

Mudge patrols a trail.

Barkely needs to determine a safe route or wait for Mudge to move.

---

## Find Henry

Henry is walking around Highlands.

The player needs to predict where Henry will be and plan Barkely's route accordingly.

---

## Back to Betsey

Barkely has wandered into another area.

The player must determine the correct route back to Betsey.

---

## Creek Crossing

Several paths approach a creek.

Only certain bridges allow Barkely to reach the objective efficiently.

---

## Lost Ball

Barkely's ball is hidden.

The player can use:

```text
SNIFF
```

at strategic locations to obtain clues.

This introduces information gathering before committing to a route.

---

## Gate Puzzle

A switch opens a gate elsewhere in the park.

Barkely must:

1. Reach the switch.
2. Activate it.
3. Navigate through the newly opened route.

---

## Mudge Patrol

Mudge repeatedly patrols an area.

The player must study the pattern and determine when Barkely can safely pass.

---

## Delivery Mission

Barkely needs to pick something up and deliver it to Oliver, Henry, or Betsey.

The shortest route may not necessarily be the correct route.

---

# World Simulation

Separate the simulation from rendering.

The game should have Rust representations for concepts such as:

```text
Barkely
Character
Position
Direction
Tile
World
Command
Program
Mission
Goal
Obstacle
Item
MovementPattern
```

Commands might conceptually resemble:

```rust
enum Command {
    Forward,
    TurnLeft,
    TurnRight,
    Wait,
}
```

The player's program should be represented as structured Rust data rather than unnecessary text parsing.

The simulation engine should be:

* Deterministic
* Testable
* Independent from rendering
* Capable of simulating multiple characters
* Capable of evaluating mission objectives rather than comparing against one predefined solution

This architecture is particularly important once Barkely and Mudge move simultaneously.

---

# Logical Grid vs Visual Movement

The puzzle engine can use a grid.

For example:

```text
(0,0) (1,0) (2,0)
(0,1) (1,1) (2,1)
(0,2) (1,2) (2,2)
```

However, the player should not necessarily feel like Barkely is teleporting between spreadsheet cells.

Animate movement smoothly between logical positions.

For example:

```text
Grid simulation:

(2,3)
  ↓
(2,4)

Visual presentation:

Barkely runs smoothly from one location to the next.
```

This gives us simple, reliable puzzle logic while maintaining attractive presentation.

---

# Graphics

The game should look like a **polished children's 2D adventure**, NOT a programmer demonstration.

Highlands should be colorful and inviting.

Include environmental details such as:

* Trees
* Grass
* Flowers
* Water
* Paths
* Bridges
* Benches
* Fences
* Rocks
* Signs
* Playground equipment

Use animation where practical.

Examples:

* Barkely running
* Barkely turning
* Barkely sniffing
* Barkely's tail wagging
* Mudge walking
* Characters walking
* Water movement
* Leaves moving
* Ball bouncing
* Switch activation
* Gates opening
* Success celebration

Prioritize clarity over visual complexity.

The child should always understand:

* Where Barkely is
* Which direction Barkely faces
* Where Barkely needs to go
* Where Mudge is
* Which objects are interactive
* Which command is currently executing

---

# Audio

Use sound where it improves the experience.

Potential sounds include:

* Barkely barking
* Barkely footsteps
* Mudge barking
* Ball bouncing
* Water
* Birds
* Switches
* Gates
* Success sound
* Gentle failure sound

Do not make sound necessary to solve a puzzle.

Provide volume controls and the ability to disable sound.

---

# Educational Philosophy

The game should NOT feel like homework.

Avoid excessive:

* Scores
* Timers
* Punishment
* Formal programming terminology
* Long explanations
* "WRONG!" messages

Prefer discovery.

If Barkely walks toward a blocked path, show Barkely reaching the obstacle and stopping.

If Mudge prevents Barkely from crossing a trail, show why.

Then optionally provide a gentle hint.

For example:

> Mudge got there first. Can Barkely wait somewhere safe before crossing?

The child should learn from the behavior of the world.

---

# Difficulty

Difficulty should come from **thinking**, not reflex speed.

A 9-year-old should be able to stop and think for as long as necessary.

Early missions should be extremely approachable.

Introduce one important concept at a time.

Only combine concepts after the child has had opportunities to understand them individually.

Avoid sudden difficulty spikes.

---

# Adventure Progression

Highlands should gradually open up.

A possible progression might be:

```text
Park Entrance
      ↓
Open Field
      ↓
Playground
      ↓
Woodland Trail
      ↓
Creek
      ↓
Bridge
      ↓
Sports Fields
      ↓
Hidden Trail
      ↓
Advanced Highlands Missions
```

Completing puzzles should allow Barkely to discover new places and situations.

The child should feel like they are exploring Highlands rather than selecting exercises from a menu.

---

# Saving

Save progress locally.

Save information such as:

* Completed missions
* Unlocked Highlands areas
* Optional efficiency challenges
* Settings

The game must work completely offline.

Do NOT require:

* Accounts
* Internet access
* Telemetry
* Cloud services

---

# Testing

The simulation engine should have substantial Rust unit tests.

Tests should cover:

* Barkely movement
* Direction changes
* Collision
* Goal detection
* Command execution
* WAIT
* Mudge movement
* Character movement patterns
* Items
* Pick up/drop behavior
* Repetition
* Mission success
* Invalid movement
* Multiple valid solutions
* Failure states
* Progression

Run:

```sh
cargo fmt
cargo check
cargo test
cargo clippy
```

Fix meaningful errors and warnings.

---

# Suggested Architecture

Keep simulation/game logic separated from presentation.

A reasonable initial structure might be:

```text
src/
├── main.rs
├── game.rs
├── world.rs
├── characters.rs
├── barkely.rs
├── commands.rs
├── program.rs
├── mission.rs
├── simulation.rs
├── progression.rs
├── save.rs
├── ui.rs
├── animation.rs
└── audio.rs
```

This structure is guidance rather than a mandatory requirement.

If a simpler or better architecture becomes apparent, use it.

Avoid unnecessary abstraction.

---

# Initial Content

Create approximately **12–20 thoughtful, polished missions** demonstrating the progression.

Do NOT generate dozens of repetitive levels merely to increase the level count.

The initial game should demonstrate:

* Sequencing
* Orientation
* Turning
* Obstacles
* Planning
* Debugging
* Multiple solutions
* Efficiency
* Repetition
* Patterns
* Moving characters
* WAIT
* Prediction

Architect the mission system so many additional Highlands missions can later be added without rewriting the engine.

---

# Existing Robot Rescue Island

An older project called **Robot Rescue Island** may be available locally.

It may be inspected for inspiration regarding:

* The original command-sequencing mechanic
* Puzzle philosophy
* Educational approach
* General tone
* What worked well in the previous game

Do NOT copy its technical architecture.

Specifically, do NOT carry forward:

* Tauri
* TypeScript
* Phaser
* npm
* Vite
* WebView-based architecture

This project is a fresh native Rust game.

---

# Scope Management

The most important thing is producing a polished, playable core game.

Do not sacrifice quality by attempting every advanced feature simultaneously.

Prioritize:

1. Barkely movement
2. Command construction
3. RUN
4. STEP
5. Good simulation
6. Fun missions
7. Clear visual feedback
8. Character consistency
9. Attractive Highlands environment
10. Saving/progression

Then add increasingly advanced mechanics such as:

* Mudge patrols
* WAIT
* REPEAT
* Prediction
* Items
* Conditions
* Routines

If an advanced feature would significantly destabilize the first playable version, architect for it and defer it rather than damaging the core game.

---

# Definition of Done

The first major playable version is complete when:

1. A fresh clone can primarily be launched with:

```sh
cargo run
```

2. The game launches into a polished playable interface.

3. Highlands appears as an attractive 2D park environment.

4. Barkely is clearly recognizable as a brown-and-white Jack Russell Terrier.

5. Mudge is recognizable as a larger Rottweiler mix.

6. Oliver is recognizable as a 9-year-old boy with bright orange hair.

7. Henry is recognizable as an 18-year-old boy with blonde hair.

8. Betsey is recognizable as a 47-year-old woman with long brown hair.

9. The player can construct Barkely's movement program.

10. Barkely visibly executes commands one at a time.

11. RUN works.

12. STEP works.

13. Incorrect programs produce understandable physical consequences.

14. Missions support multiple valid solutions where appropriate.

15. The game includes meaningful progression in critical-thinking difficulty.

16. At least some moving-character/prediction gameplay is implemented.

17. Progress saves locally.

18. The game works offline.

19. Core simulation logic has automated tests.

20. No Node/npm/Tauri/JavaScript/TypeScript/WebView runtime is required.

21. The following succeed:

```sh
cargo fmt
cargo check
cargo test
cargo clippy
```

22. The result feels like an actual children's adventure game rather than a technical demonstration.

---

# Codex Working Instructions

Before implementing:

1. Read this entire `PROJECT.md`.
2. Inspect the machine and installed Rust toolchain.
3. Inspect the older Robot Rescue Island project if it is available locally.
4. Use the old project only for gameplay inspiration.
5. Do NOT reproduce its web/Tauri architecture.
6. Develop an implementation plan.
7. Then implement the game.

You are responsible for making routine engineering decisions.

Do not repeatedly stop for approval on ordinary implementation details.

Make reasonable decisions and continue.

Build and run the game during development.

Test the actual gameplay, not merely whether the project compiles.

Run:

```sh
cargo fmt
cargo check
cargo test
cargo clippy
```

Resolve meaningful errors and warnings.

Do not push anything to GitHub without explicit permission.

Do not introduce Node/npm or a web frontend simply because it would be easier.

Maintain the requirement that this is a native Rust game.

When the first playable version is complete, report:

* What was implemented
* Which missions were created
* How the simulation architecture works
* How Barkely's command system works
* What was tested
* Known limitations
* Features deferred to a future version
* Recommended next improvements
* Exact command required to launch the game

---

# Priorities

When making tradeoffs, prioritize in this order:

1. **Fun**
2. **Critical thinking**
3. **Plan → Predict → Run → Observe → Debug → Improve**
4. **Clear cause and effect**
5. **Barkely's personality**
6. **Reliable simulation**
7. **Age-appropriate difficulty**
8. **Attractive presentation**
9. **Simple Rust architecture**
10. **Additional features**

Do not sacrifice the core mechanic merely to add more content.

---

# Central Principle

The most important design principle for the entire project is:

**The child should learn to think precisely by planning Barkely's actions, predicting what will happen, watching Barkely follow those instructions exactly, understanding why the result occurred, and then improving the plan.**

Barkely's Highlands Adventure should make that process feel like play.


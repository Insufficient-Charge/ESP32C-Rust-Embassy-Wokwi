# Unnamed ESP32C3 Rust-Embassy Project

## Authors
- InsufficientCharge (MyMy)
- Other awesome devs that will add their usernames in the future

## Current functionality
Follows Apollolabsdev's implementation of GPIO, but modified to activate a singular LED, blinking it on and off while maintaining a count between tasks.

## Age
Throughout this project, I've found that how old a project is changes what you're capable of implementing with ESP32 Embassy. This repo was developed originally on March 2nd, 2024. Watch the age of the commits; if they're older than 2 years, you may have better luck searching for other Embassy ESP32 WokWi examples.

## Example projects
Many Embassy projects out on Github right now have "examples" folders. If other examples are made using the dynamic used to form this repo, they will be linked in this section. Until then, it is empty. I encourage those working alongside me to develop rust-embedded projects to update this README with hyperlinks to their examples.

## Goals
- Run a no_std project on an ESP32 to be simulated and tested with CI on Wokwi
- Provide a decent example for doing this for other esp32c3+embassy+wokwi projects in the future
- Have some fun in Rust and see what's possible

## Setup
Note the Cargo.toml: as it stands, we need these outdated versions to make everything run in Wokwi. Hopefully this is mended in the future. Outdated crates include:
- esp32c3-hal
- embassy-time
- riscv-rt

Also note: I am using `WSL: Ubuntu` with rust and cargo installed. Though I have Platform IO installed I am not using it.

Install [Rust Embassy](https://embassy.dev/book/dev/getting_started.html)
- To those following along: Within the embassy folder, make a projects file. This is a project. You can pull this repo into your new project directory.

Install [Wokwi for VS Code](https://docs.wokwi.com/vscode/getting-started)
- You will need to create a Wokwi account.
- You will need to enable a community license; this is available for 30 days. I think you can just re-renew it every 30 days; otherwise I'm not sure how anyone gets anything done with Wokwi in a reasonable amount of time.
- The community edition CANNOT edit the diagram using the tool, only simulate. You can edit diagram.json manually.

If you're intending to setup CI, install [Wokwi CI](https://docs.wokwi.com/wokwi-ci/getting-started).
- You can do CI tests using Wokwi's "scenarios".
- These aren't technically mocks, but they might be a good approach for what we're trying to do for the time being.
- Scenarios are NOT YET DOCUMENTED. Behold, Wokwi's [example scenarios](https://docs.wokwi.com/wokwi-ci/getting-started#example-projects).

## How to Run
1. Use `cargo build --release`. This will build a binary in the `target/` directory. 
2. If you're using a physical device, use `cargo run --release`. Otherwise, open diagram.json in VS Code (with the Wokwi VS Code extension properly installed), and click the start simulation button (the "play" button).

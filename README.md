# thol-helper
The Two Hours One Life helper (thol-helper) program's goal is to further parse the data coming from the [twotech project's](https://github.com/twohoursonelife/twotech) game data parsing to filter things in interesting ways.

twotech's project can be a little difficult to build, and the documentation could use some help to guide new developers to getting prerequisites installed. In the meantime, I've made [another project](https://github.com/mtklass/TwoTech-ProcessOutput) that contains the generated minified output from the twotech project (which is what twotech's webpage uses) for the latest data version number.

If you clone that project into the same parent directory as this project, the default location for the `--twotech-data-directory` (`-t`) option will be correct. You can also choose to build the data yourself using the twotech project, and then point the `-t` option to the twotech project directory.

If the data is in place, you can then run thol-helper to filter the data in various ways.

You can either build and run the project in one command, or build the project first, and then directly use the output executable.

So first, run `cargo build`. Then, you can either use `cargo run -- [ARGS]` or `target/debug/thol-helper [ARGS]`

e.g.
```
cargo run -- -o slotted-nonpack-clothing.json --num-slots 2.. --slot-size 1.0.. --clothing Top,Bottom,Shoe,Head
```
This will filter for all clothing items (except shields and packs) that have at least 1 slot (with slot size of 1.0 or greater -- 1.0 means small item).

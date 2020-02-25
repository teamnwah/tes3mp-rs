# tes3mp-plugin

A crate for helping you make tes3mp plugins in Rust

# How to use

Because Rust won't allow you to re-export symbols from other crates, this crate can't be used like normal crates, the advised manner of using this crate is the following:

```bash
cd crate-dir
git status || git init
mkdir extern
git submodule add https://git.cijber.net/teamnwah/tes3mp-plugin extern/tes3mp-plugin
ln -s extern/tes3mp-plugin/tes3mp-plugin/src/plugin src/plugin
```

This will make the `plugin` like it's part of your crate, which allows us to export the C symbols
## goals

* leaderboard compat [x]
* sfx [x]
* doesn't crash [x]
* ctm compat [ ]
* gym compat [ ]

## building

```bash
node src/chr/convert.js src/chr
docker pull mrkits/rust-mos
docker run -it --name rustmos --entrypoint bash -v ${HOME}/tetris/rust:/hostfiles mrkits/rust-mos
docker container exec -it rustmos /bin/bash
cargo build --release
```

## attribution

* original engine remake & implementation help https://github.com/negative-seven/meta_nestris
* linker help https://github.com/jgouly
* original disassembly https://github.com/ejona86/taus
* public domain font https://opengameart.org/content/intrepid-monochrome-8-bit-font
* toolchain https://llvm-mos.org/wiki/Rust

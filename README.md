a proof of concept NES game built with Rust

## building

```bash
node src/chr/convert.js src/chr
docker pull mrkits/rust-mos
docker run -it --name rustmos --entrypoint bash -v ${HOME}/tetris/rust:/hostfiles mrkits/rust-mos
docker container exec -it rustmos /bin/bash
cargo rustc --release
```

## attribution

* linker help https://github.com/jgouly
* public domain font https://opengameart.org/content/intrepid-monochrome-8-bit-font
* toolchain https://llvm-mos.org/wiki/Rust

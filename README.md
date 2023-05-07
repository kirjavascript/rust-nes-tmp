a template for writing NES games in Rust

includes an example game that uses sprites, background tiles, sfx, and controller input

https://user-images.githubusercontent.com/4498266/236694027-57e3ce69-e359-4147-950b-7dbcf5999705.mp4

## building

```bash
node src/chr/convert.js src/chr
docker pull mrkits/rust-mos
docker run -it --name rustmos --entrypoint bash -v ${HOME}/rust-nes-template:/hostfiles mrkits/rust-mos
docker container exec -it rustmos /bin/bash
cargo rustc --release
```

## attribution

* linker help https://github.com/jgouly
* public domain font https://opengameart.org/content/intrepid-monochrome-8-bit-font
* toolchain https://llvm-mos.org/wiki/Rust

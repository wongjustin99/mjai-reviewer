# mjai-reviewer

[![GitHub Workflow Status](https://github.com/Equim-chan/mjai-reviewer/workflows/build/badge.svg)](https://github.com/Equim-chan/mjai-reviewer/actions)
[![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/Equim-chan/mjai-reviewer?include_prereleases)](https://github.com/Equim-chan/mjai-reviewer/releases)
[![dependency status](https://deps.rs/repo/github/Equim-chan/akochan-reviewer/status.svg)](https://deps.rs/repo/github/Equim-chan/akochan-reviewer)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/Equim-chan/mjai-reviewer)
[![License](https://img.shields.io/github/license/Equim-chan/mjai-reviewer)](https://github.com/Equim-chan/mjai-reviewer/blob/master/LICENSE)

Review your Tenhou or Mahjong Soul (Jantama) log with mjai-compatible mahjong AI, supported backends are [Mortal](https://github.com/Equim-chan/Mortal) and [akochan](https://github.com/critter-mj/akochan).

[Demo result page](https://gh.ekyu.moe/mjai-reviewer-demo.html)

mjai-reviewer 1.x.x is incompatible with 0.x.x versions. If you prefer the old version, check out [v0 branch](https://github.com/Equim-chan/mjai-reviewer/tree/v0).

Online version is WIP.

### [How to Review Mahjong Soul Logs (updated 2021-09-26)](https://github.com/Equim-chan/mjai-reviewer/blob/master/mjsoul.adoc)

## Example
```console
$ # Review https://tenhou.net/0/?log=2019050417gm-0029-0000-4f2a8622&tw=2
$ # Note that you may need to quote it in the shell to escape the string
$ mjai-reviewer -b mortal -u "https://tenhou.net/0/?log=2019050417gm-0029-0000-4f2a8622&tw=2"

$ # Use akochan as backend
$ mjai-reviewer -b akochan -u "https://tenhou.net/0/?log=2019050417gm-0029-0000-4f2a8622&tw=2"

$ # Alternatively, you can specify the log ID and player ID manually
$ mjai-reviewer -b mortal -t 2019050417gm-0029-0000-4f2a8622 -a 2

$ # Review a local tenhou.net/6 format log file, note that you must specify a player ID
$ mjai-reviewer -b mortal -i log.json -a 3

$ # Review 東2局1本場 and 東3局 only
$ mjai-reviewer -b mortal -k E2.1,E3 -u "https://tenhou.net/0/?log=2019050417gm-0029-0000-4f2a8622&tw=2"
```

Use the `--help` argument for more details.

## Build
### mjai-reviewer
Follow the instructions [here](https://www.rust-lang.org/learn/get-started) to install Rust toolchains first, if you haven't yet.

```console
$ cd ..
$ git clone https://github.com/Equim-chan/mjai-reviewer.git
$ cargo build --release
```

`mjai-reviewer` binary will be in `target/release` directory.

### Backends
#### Mortal
See [Mortal's documentation](https://mortal.ekyu.moe/user/build.html).

You also need a trained model file to actually use Mortal.

#### Akochan
```console
$ git clone https://github.com/critter-mj/akochan.git
$ cd akochan
```

You have to edit `Makefile` and `ai_src/Makfefile` accordingly. Set up correct path for boost and some other options like `-march=native` of your choice.

<details><summary>On Windows MSYS2 with MinGW-w64 toolchain</summary>
<p>

```console
$ pacman -Syu mingw-w64-x86_64-{toolchain,boost}
```

Edit `Makefile`:

```Makefile
LIBS = -lboost_system-mt -lws2_32 -L./ -lai -s
```

Edit `ai_src/Makefile`:

```Makefile
LIBS = -lboost_system-mt -lws2_32
```

```console
$ cd ai_src
$ make
$ cd ..
$ make
```

</p>
</details>

<details><summary>On MacOS</summary>
<p>

```console
$ brew install llvm libomp boost
$ cd ai_src
$ make -f Makefile_MacOS
$ cd ..
$ make -f Makefile_MacOS
```

</p>
</details>

<details><summary>On Arch Linux</summary>
<p>

```console
$ sudo pacman -Syu base-devel boost
$ make -f Makefile_Linux
$ cd ..
$ make -f Makefile_Linux
```

</p>
</details>

## Docker
Currently the docker image is not maintained and it only supports akochan as a backend.

### Build
```console
$ git clone https://github.com/Equim-chan/mjai-reviewer.git
$ cd mjai-reviewer
$ git clone https://github.com/critter-mj/akochan.git
$ docker build -t mjai-reviewer:latest .
```

### Usage
```console
$ docker run --rm mjai-reviewer:latest -b akochan --no-open -t 2019050417gm-0029-0000-4f2a8622 -a 3 -o - > report.html
$ open report.html  # or just open in browser
```

## Troubleshooting
### Akochan backend
### `Assertion failed` errors on Windows
Set environment variable `OMP_NUM_THREADS=8`.

Under cmd
```console
> set OMP_NUM_THREADS=8
```

Under Powershell
```console
> $env:OMP_NUM_THREADS = 8
```

Under MSYS2 bash
```console
$ export OMP_NUM_THREADS=8
```

### `libai.so` not found on Linux
Try adding the directory of `libai.so` to env `LD_LIBRARY_PATH`.

## License
[Apache-2.0](https://github.com/Equim-chan/mjai-reviewer/blob/master/LICENSE)

akochan itself was licensed separately, see https://github.com/critter-mj/akochan/blob/master/LICENSE.

## Contributors
[![](https://contrib.rocks/image?repo=Equim-chan/mjai-reviewer)](https://github.com/Equim-chan/mjai-reviewer)

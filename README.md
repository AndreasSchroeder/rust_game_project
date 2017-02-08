This is a project for the class "Programmieren in Rust".

This game need OpenAl and libsndfile, please install this packages bevore use.

## Install Packages

### Linux

Fedora:

```
sudo dnf install openal-soft-devel libsndfile-devel
```

Debian or Ubuntu:

```
sudo apt install libopenal-dev libsndfile1-dev
```

### Mac

```
brew install openal-soft libsndfile
```

### Windows

install:

OpenAl: https://www.openal.org/downloads/

libsndfile: http://www.mega-nerd.com/libsndfile/



cargo development:

```
1. msys2-x86_64-20161025.exe installieren
2. ausführen: pacman -S mingw-w64-x86_64-libsndfile mingw-w64-x86_64-openal
3. Windows cmd: rustup default stable-gnu
4. Copy files from C:\msys64\mingw64\lib to C:\Users\YOUR_USER\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\lib
5. Copy files from C:\msys64\mingw64\bin to C:\Users\YOUR_USER\.cargo\bin
```

This is a project for the class "Programmieren in Rust". 

"Chicken Fight 3000 Ultimate Tournament!"

Story:
In the post apocalyptic world, where the government have been overrun by chickens, two brave hero's are there to save the day (by slaugthering all the chicken).

Gameplay:
Top-down Action-Adventure ("Legend of Zelda"-style) for up to 2 players.

Requirments:
This game need OpenAl and libsndfile, please install this packages before use.

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

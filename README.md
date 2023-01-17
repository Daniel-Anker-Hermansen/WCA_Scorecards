## Installation on Linux (using apt)

Running all of the below commands from a freshly installed linux distribution should get the program working.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 
sudo git clone https://github.com/Daniel-Anker-Hermansen/WCA_Scorecards.git
 
sudo apt-get update
 
sudo apt-get -y install cargo
 
sudo apt install pkg-config
 
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig
export PKG_CONFIG_PATH=/usr/X11/lib/pkgconfig
 
sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev openssl libfontconfig1-dev fontconfig
```

## Usage

see ``--help``

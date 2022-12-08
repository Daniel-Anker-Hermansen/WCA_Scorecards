## Installation on Linux

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

From the `WCA_Scorecards` folder you get from git cloning this repository, you can use the program.

For generating scorecards for **subsequent** rounds **during** a competition: `cargo run --release -- --subseq {competitionid}`. This opens localhost:5000. From there, press the round you want to generate scorecards for. As this is for subsequent rounds only, do not click on any round 1.

Always make sure to sync WCA live before calling the program, as it fetches the newewst WCIF. Also make sure there are no un-entered results from the previous round.
____
For **first round scorecards**, you will need to generate two CSVs first: One containing the group- and station-numbers of each competitor, and one for the time limits of each event. To generate those CSVs, you can have a look at this file (documentation to be added once that program is more stable): https://github.com/DanielEgdal/RandomCubing/blob/main/betagroupmaker.py

To generate the scorecards: `cargo run --release -- --r1 {groupAndStationNumbers.csv}  {timeLimits.csv}  {competition name}`. 

Additionally you can add a flag for generating the scorecards into different files, for example if you run multiple stages and want the scorecards colour-cordinated. Add this to the line above: `--stages R-{perStage} G-{perStage} B-{perStage}`. If you only have two stages, just omit one of these stages. 

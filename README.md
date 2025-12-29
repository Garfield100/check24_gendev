# GenDev

Deployed at: <https://gendev.garfield100.net/>

Username: `gendev`

Password: `8gwAZcCJvgMXxcyDDpMhm2dbhKk3NV5B`

Video: <https://youtu.be/pBdHTp7gnC4>

[Concept](CONCEPT.md)

[Dev integration guide](DEVELOPER_GUIDELINE.md)

Build dependencies:
* [The Rust toolchain](https://rust-lang.org/tools/install/)
* [Just (command runner)](https://github.com/casey/just)
* Docker just for Valkey, didn't have time to containerise the rest :(

If you don't want to install Just you can also just open the `Justfile`s and run the commands within.

To run:
1. Run Valkey with `just run_valkey`
2. Either detatch or open a new terminal and run `just run_home` and enjoy the warmth from your CPU as it compiles. Once that's done it will start running.


## Image attributions

### Travel images
<https://unsplash.com/photos/eiffel-tower-during-daytime-Q0-fOL2nqZc>
<https://unsplash.com/photos/brown-and-white-wooden-house-near-green-trees-during-daytime-SlIl9eZjWUc>
<https://unsplash.com/photos/colosseum-arena-photography-VFRTXGw1VjU>

### Horses
<https://unsplash.com/photos/white-horse-on-forest-Y5iPU37b7Zs>
<https://unsplash.com/photos/three-horses-on-green-ground-e4WUwbcur7Y>

### Cars
<https://unsplash.com/photos/red-mercedes-benz-coupe-on-road-qyfco1nfMtg>
<https://unsplash.com/photos/blue-volkswagen-5-door-hatchback-1iVJkBGy6OY>

### SIM cards
<https://unsplash.com/photos/sim-cards-and-their-adaptors-are-on-display-7qlXFeFNv14>
<https://unsplash.com/photos/black-iphone-5-NS-lboO5wak>
<https://unsplash.com/photos/a-blue-sim-card-sits-on-a-dark-surface-fcYNLhYAd5k>
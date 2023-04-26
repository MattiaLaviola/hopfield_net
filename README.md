# Hiopfield networks with GUI

### Why does this exist?
I don't know, I thought that the Hopfield networks were cool, and I wanted to learn Rust.

---

### What's inside

Inside this repository there is an implementation of the original version of the Hopfield network (that I indicated as HebbianSquareDiscrete) and 
a version that uses the Strokey learning rule.

In the future, I'd also like to implement the modern version of the network, and the modern continuous network described in [Hopfield Networks is All You Need](https://arxiv.org/abs/2008.02217).

![Gui screenshot](https://github.com/MattiaLaviola/hopfield_net/blob/master/reade_me_stuff/Screenshot.JPG?raw=true)
---

### How to run 

Just clone the repository and do `cargo run --release`

P.s
If you don't already have it, install the [rust compiler](https://www.rust-lang.org/tools/install)

---

### Notes
The GUI is built using [egui](https://github.com/emilk/egui) or, to be more precise the [eframe](https://github.com/emilk/eframe_template/) template to make the project compilable into a web app. Sadly though I set up the GUI and the networks to 
work on different threads before finding out that rust's threads and web assembly at least for the moment don't work together,
so for now no web app.


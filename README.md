# Hiopfield networks with gui

### Why does this exist?
I don't know, i thougt that the hopfield networks were cool, and I wanted to learn Rust.

---

### What's inside
![Gui screenshot](https://github.com/MattiaLaviola/hopfield_net/blob/master/reade_me_stuff/Screenshot.JPG?raw=true)
Insire this repository there is an implementation of the original version of the Hopfield network (that i indicatd as HebbianSquareDiscrete), and a 
version that uses the strokey leatning rule.

In future I'd also like to implement the modern version of the netwotk, an the modern continuous network desrcibed in [Hopfield Networks is All You Need](https://arxiv.org/abs/2008.02217).


---

### How to run 

Just clone the repository and do `cargo run --release`

P.s
If you don't already have it, istall the [rust compiler](https://www.rust-lang.org/tools/install)

---

### Notes
The gui is built using [egui](https://github.com/emilk/egui) or, to be more precise the [eframe](https://github.com/emilk/eframe_template/) template to make the project compilable into a web app. Sadly though i seta up the GUI and the networks to 
work on different thrads before finding out that rust's threads and web assembly at least for the moment don't really work together,
so for now no web app.


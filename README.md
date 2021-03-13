# Hackathon Fubini Numbers

This project was created in the Relaxdays Code Challenge Vol. 1. See https://sites.google.com/relaxdays.de/hackathon-relaxdays/startseite for more information. My participant ID in the challenge was: CC-VOL1-4

## How to run

```bash
git clone https://github.com/Geo5/hackathon-fubini.git
cd hackathon-fubini
docker build -t fubini
docker run -it fubini [n (default=5)]
```

## Algorithmic Idea

The basic formulas implemented are this [Recurrence relation](https://en.wikipedia.org/wiki/Ordered_Bell_number#Recurrence_and_modular_periodicity) for fubini numbers themself and this [Pascal's triangle recurrence](https://en.wikipedia.org/wiki/Ordered_Bell_number#Recurrence_and_modular_periodicity) for the binomial coefficients in it.

The main program is written in Rust (under [src/main.rs](src/main.rs) ). The python implementation in [fubini.py](fubini.py) was just used as quick correctness check and is considerably slower!

## Runtime

On my machine the program computed the first 4500 fubini numbers in around 100s.
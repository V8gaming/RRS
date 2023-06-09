[![BuildNightly](https://github.com/V8gaming/RBMK-1000/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/V8gaming/RBMK-1000/actions/workflows/rust.yml)
# RRS: Rust Reactor Simulation

RRS (Rust Reactor Sim) is a simulation game that puts you in control of a virtual RBMK-1000 nuclear reactor, inspired by Chernobyl: the legacy continues. Test your engineering skills and manage the reactor to prevent a meltdown.
![](./resources/preview.png)

## Introduction

RRS (Rust Reactor Sim) is an open-source simulation software inspired by the RBMK-1000 reactor of the Chernobyl Nuclear Power Plant. With RRS, you can learn about reactor physics in a safe and educational way.

## Commands

* help (page) - display this page.
* insert rod (rod number) - insert a fuel rod.
* remove rod (rod number) - remove a fuel rod.
* insert rods - insert all fuel rods.
* pull rods - remove all fuel rods.
* scram - remove all fuel rods.
* setpoint (setpoint) - set all fuel rods to a setpoint.
* set rod (rod number) to (setpoint) - set a fuel rod to a setpoint.
* cls - clear the log.
* center core only - insert only the center core.
* setpoint speed (slow|medium|fast) - set the speed of the setpoint change.
* hold rods - hold the rods in place.

## Dev Commands
* dev sp (position)- change the position of the absorber rods to position.
* dev sp speed (number) - set the speed of the absorber rod change.

## Simulation levels
## Level 0
* Everything based on the position of the absorber rods.
* Only the reactor core, and steam turbine is simulated.

## Level 1
* water amounts are simulated.
* electrical power generated from the turbine is simulated.
* Radioactivity of the fuel rods is simulated.

## Level 2
* Flux rate is based on the composition of the fuel rods.
* water composition and deaerator is simulated.
* Fuel burnup is simulated.

## Level 3
* malfunctions are simulated.
* accounting is simulated.

# References
## Steam Table
[Table Generator](https://www.spiraxsarco.com/resources-and-design-tools/steam-tables/superheated-steam-region)
| Pressure (MPa) | Superheat Temperature (°C) |Saturation Temperature | Degrees Superheat | Density of Steam ($\text{kg}/{\text{m}^{3}}$)
|----------|-----------------------|-----------------------|------------------|-------------|
| 0  | 600.000             | 99.9743            | 500.026             |	0.251560
| 1  | 600.000             |184.115            | 415.885              | 2.74700	
| 2  | 600.000             |214.890           | 385.110               |5.26597
| 3  | 600.000             |235.703          | 364.297                | 7.80904
| 4  | 600.000             |251.842        | 348.158                  | 10.3768	
| 5  | 600.000             |265.198     | 334.802                     | 12.9698
| 6  | 600.000             |276.680   | 323.320                       | 15.5886
| 7  | 600.000             |286.803 | 313.197                         | 18.2339


# SVG Checklist
__NEEDS RATIO FIXING & RELATIVE PATHS__
- [x] Line
- [x] Rect
- [ ] Circle
- [ ] Ellipse
- [ ] Image (use jpeg compression (DCT) to render 16 colors)
- [ ] Iframe (will be attemped last)
- [ ] Text
- [ ] SVG
- [ ] Polygon
- [ ] Polyline
- [ ] textPath
- [ ] __Path__
  - [x] M
  - [x] L
  - [x] Q, needs more testing
  - [X] C, needs more testing
  - [X] H
  - [X] V
  - [ ] S, needs complete testing
  - [ ] T, needs complete testing
  - [ ] A, Incomplete
- [ ] __Transformations, needs complete testing__
  - [X] Translate
  - [X] Scale
  - [X] Rotate
  - [X] Skew
  - [ ] Matrix
- [ ] __Style__
  - [ ] Fill(bg) Color
  - [ ] Stroke
    - [x] (fg) Color
    - [ ] (fg) size
    - [ ] linecap
    - [ ] linejoin

## Changes to tui-rs
added FULL to marker enum. and in cavas/mod.rs
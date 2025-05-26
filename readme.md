
# Tube Cutter

  

**Tube Cutter** is a lightweight CAM application written in Rust for generating and running G-code to cut square (and round) tubing using a CNC plasma cutter. It includes direct serial communication with the cutter.

  ## ✨ Features
  * Generate grblHAL compatible G-codes that can cut straight lines.
  * Calculate cuts based on given angles.
  * Basic serial connection to machines.

## 🎯 Goals

- [ ] **Realtime Toolhead Updates** - Monitor the live position of your CNC toolhead.
- [ ] **Simulation Mode** - Preview G-code paths before sending them to the machine
- [ ] **Tube Notching Support** - Design cut paths for square and round tubing 


  

## 📦 Installation

  

### Prerequisites

  

- Rust (latest stable): [Install Rust](https://www.rust-lang.org/tools/install)

- A serial-compatible CNC controller (e.g. GRBLHAL)

  

### Clone the Repository

  

```bash

git  clone  https://github.com/AbnormalMatrix/tube_cutter

cd  tube_cutter

cargo  run  --release

```

Phosphor Icons are licensed under MIT.
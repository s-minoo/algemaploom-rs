<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->

<a name="readme-top"></a>

<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->

<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">CrustMapper: Mapping Algebra for Knowledge Graph Construction</h3>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li><a href="#prerequisites">Prerequisites</a></li>
    <li><a href="#compilation">Compilation</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

Mapping algebra provides operational semantics to the mapping process, opening
the door to study of complexity and expressiveness of existing mapping languages.
This project provides the CLI translator from RML to mapping algebra.

<!-- GETTING STARTED -->

## Prerequisites

To compile the project on your own, you'll need to have
[Rust toolchain](https://www.rust-lang.org/tools/install) installed.

For Linux-based users:

- Rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

For the visualization of the generated mapping plans, you'll need 
python version >= 3.10 and the following packages: 

1. graphviz



## Running

1. Download this repo
2. Run cargo build at the root this repo
   ```sh
   cd {repo dir}
   cargo build --release
   ```
3. Run the CLI translator app
   1. From Cargo 
       ```sh
       cargo run --bin translator -- {args}
       ```
   2. From the compiled translator binary
       ```sh
       cd ./target/release/
       ./translator {args}
       ```
      
4. Visualize the created mapping plan 
   ```sh
   ./visualizer.py {args}
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<div align="center">
  <h1><code>SModelHexer</code></h1>
  <p>
    <strong>A reverse engineering project of the MDL format used in Source games, done in order to provide ease of change of the model and material paths through a GUI, all written in Rust.</strong>
  </p>

  <img alt="logo" src="./assets/icon-256.png" width="256" />
    

  <p style="margin-bottom: 0.5ex;">
    <img
        src="https://img.shields.io/github/downloads/Zabaniya001/SModelHexer/total"
    />
    <img
        src="https://img.shields.io/github/last-commit/Zabaniya001/SModelHexer"
    />
    <img
        src="https://img.shields.io/github/issues/Zabaniya001/SModelHexer"
    />
    <img
        src="https://img.shields.io/github/issues-closed/Zabaniya001/SModelHexer"
    />
    <img
        src="https://img.shields.io/github/repo-size/Zabaniya001/SModelHexer"
    />
  </p>
</div>

## N.B.
This is still in WIP. It's riddled with unwraps and "this will do for now" practices. Open an issue to report bugs. There will be more updates. 

##  How to build the app
1. Clone the repository.
2. Run `rust build` ( if you don't have Rust, download it from the official website ).
4. It's compiled!

## TO-DOs
- Recreate the entire mdl into structs so it's easier to modify things without having to mess with offsets and such.
- Allow users to change every single thing about the mdl.
- Implement vpk unpacking.
- Switch away from `Egui::Window`s and make some sort of side menu where you can select the opened files.
- Fix linux builds.
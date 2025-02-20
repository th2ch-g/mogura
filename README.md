# mogura wasm preview

Try demo [here](https://mogura-rs.github.io/mogura/)

## Current Feature
- Visualize PDB and GRO file
  - dirty PDB file is not supported because of pdbtbx
- Fetch PDB by specifying PDBID
- Animate trajectory for XTC file
  - But wasm does not support it, because groan_rs depends on header file, so libc is needed
- Change drawing method
  - Cartoon method needs secondary structure alignment but dssp is not implemented yet


## TODO
- [ ] implement dssp
- [ ] implement path tracing
- [ ] implement atom selection language
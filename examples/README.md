# Examples for QuickStart

- clone example inputs
```bash
git clone https://github.com/mogura-rs/example-inputs.git
```

- get `mogura` binary with installing
```bash
cargo install --git https://github.com/mogura-rs/mogura mogura --locked
```

- visualize 8gng
```bash
mogura example-inputs/8gng.pdb
```

- visualize MD simulation of chignolin
```bash
mogura example-inputs/chignolin/init.gro example-inputs/chignolin/input.mol.compact.xtc
```

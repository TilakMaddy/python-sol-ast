# Template to generate ASTs for solidity projects in Python

1. Install `uv`
2. Run `forge build` inside `./defi-app` example attached in repo
3. Install `cargo`
4. From the root direcory, run `uv run main.py`

## How to use

1. Write python procedures in `main.py`
2. If you want to use non-default options make changes to `ast-gen/src/main.rs`
3. Run `uv run main.py`

Note: The AST stuff will be written to `output.json`



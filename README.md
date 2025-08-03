# lakeshire
Lakeshire is my experimental addon + bot for World of Warcraft.

## Structure
* `addon`
    * Lua codebase for the addon
* `bot`
    * Contains the Rust codebase for the bot + frontend. Also has a cli.
* `lakeshire-py`
    * Python codebase for the bot. Also has a cli.
* `protos`
    * Shared protobuf definitions. Used by `addon` and `bot`

## Development
Be sure to set up pre-commit hooks:
```bash
pip install pre-commit
pre-commit install
```

# lakeshire
Lakeshire addon/bot for wow

## Structure
* `addon`
    * Lua codebase for the addon
* `bot`
    * Contains the Rust codebase for the bot. Also has a cli.
* `gui`
    * A desktop gui for various bot purposes. Uses `bot` as a library.
* `protos`
    * Shared protobuf definitions. Used by `addon` and `bot`

## Development
Be sure to set up pre-commit hooks:
```bash
pip install pre-commit
pre-commit install
```

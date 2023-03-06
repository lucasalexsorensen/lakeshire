#/bin/bash
eval $(luarocks --lua-dir $HOMEBREW_PREFIX/opt/lua@5.1 path)
lua-5.1 lua-pb/saveast.lua pbast < ../protos/Lakeshire.proto > protos-dist/Lakeshire.lua

cd lakeshire
uv pip install mypy-protobuf
protoc --python_out=. --mypy_out=. --proto_path=../../protos ../../protos/Lakeshire.proto

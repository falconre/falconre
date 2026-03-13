lib_ext := if os() == "macos" { "dylib" } else { "so" }
python := ".venv/bin/python"
sys_python := "python3"

# List available recipes
default:
    @just --list

# Build debug and copy .so into Python package
build: (_cargo "debug")

# Build release and copy .so into Python package
release: (_cargo "release")

# Fast compilation check (no codegen)
check:
    cargo check

# Remove build artifacts and copied .so
clean:
    cargo clean
    rm -f falconre/falconre.so

# Verify the Python import works
test: _ensure-venv build
    {{python}} -c "from falconre.falconre import falcon; print('ok')"

# Create or recreate the Python venv
venv:
    rm -rf .venv
    {{sys_python}} -m venv .venv
    .venv/bin/pip install -r requirements.txt

# Install Python deps into existing venv
install-deps:
    .venv/bin/pip install -r requirements.txt

# Internal: create venv if missing
_ensure-venv:
    #!/usr/bin/env sh
    if [ ! -d .venv ]; then
        {{sys_python}} -m venv .venv
        .venv/bin/pip install -r requirements.txt
    fi

# Internal: cargo build + copy library
_cargo profile:
    cargo build {{ if profile == "release" { "--release" } else { "" } }}
    cp target/{{profile}}/libfalconre.{{lib_ext}} falconre/falconre.so

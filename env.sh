export PS1="\n\[\033[1;32m\][kernel] \[\033[0m\]\u@\h:\w\n\$ "
export PROMPT=$'\n%{\033[1;32m%}[kernel] %{\033[0m%}%n@%m:%~\n$ '

set_target() {
    case "$1" in
        i386)
            export TARGET="i386-unknown-none"
            export TARGET_JSON="$PWD/toolchains/i386-unknown-none.json"
            ;;
        *)
            echo "Unknown target. Supported: i386"
            return 1
            ;;
    esac
    export ARCH="$1"
}

_cargo_build() {
    cargo +nightly build \
        -Zjson-target-spec \
        -Zbuild-std=core,alloc \
        --target "$TARGET_JSON" \
        --target-dir build \
        --release \
        "$@"
}

b() {
    if [ -z "$ARCH" ]; then
        echo "Error: target not set. Use 'set_target <arch>' first."
        return 1
    fi
    _cargo_build
}

bt() {
    if [ -z "$ARCH" ]; then
        echo "Error: target not set. Use 'set_target <arch>' first."
        return 1
    fi
    _cargo_build # TODO
}

r() {
    qemu-system-$ARCH -kernel build/$TARGET/release/kernel -serial stdio $*
}

rd() {
    r -no-reboot -s -S
}

br()  { b && r;  }
btr() { bt && r;  }
bd()  { b && rd; }
cl()  { rm -rf build; echo "Removed all build directories"; }
cbr()  { cl; br; }
cbtr() { cl; btr; }
d()  { lldb build/$TARGET/release/kernel -o "gdb-remote localhost:1234"; }

help() {
    echo "Available commands:"
    echo "  set_target i386"
    echo "  b - build kernel for current target"
    echo "  br - build + run in qemu"
    echo "  bt - build tests + run in qemu"
    echo "  btr - build tests + run in qemu"
    echo "  bd - build + qemu with -s -S"
    echo "  cl - remove build directories"
    echo "  cbr - clean + build + run"
    echo "  cbtr - clean + build tests + run"
    echo "  d - lldb + connect to localhost:1234"
    echo "  r - run qemu"
    echo "  rd - run qemu with -s -S"
    echo "  help"
}
set_target i386
help

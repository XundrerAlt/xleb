export PS1="\n\[\033[1;32m\][kernel] \[\033[0m\]\u@\h:\w\n\$ "
export PROMPT=$'\n%{\033[1;32m%}[kernel] %{\033[0m%}%n@%m:%~\n$ '

set_target() {
    case "$1" in
        i386)
            export TARGET="i386-unknown-none"
            ;;
        *)
            echo "Unknown target. Supported: i386"
            return 1
            ;;
    esac
    export ARCH="$1"
}

b() {
    if [ -z "$ARCH" ]; then
        echo "Error: target not set. Use 'set_target <arch>' first."
        return 1
    fi
    meson setup build -Darch="$ARCH" --cross-file=toolchains/$TARGET.txt --reconfigure
    ninja -C build
}
r() {
    qemu-system-$ARCH -kernel build/kernel -serial stdio -no-reboot
}
rd() {
    qemu-system-$ARCH -kernel build/kernel -serial stdio -no-reboot -s -S
}
br() {
    b
    r
}
bt() {
    if [ -z "$ARCH" ]; then
        echo "Error: target not set. Use 'set_target <arch>' first."
        return 1
    fi
    meson setup build -Darch="$ARCH" --cross-file=toolchains/$TARGET.txt -Denable_tests=true --reconfigure
    ninja -C build
}
btr() {
    bt
    r
}
bd() {
    b
    rd
}
cl() {
    rm -rf build*
    echo "Removed all build directories"
}
cbr() {
    cl
    br
}
cbtr() {
    cl
    btr
}
d() {
    lldb build/kernel -o "gdb-remote localhost:1234"
}
help() {
    echo "Available commands:"
    echo "  set_target i386"
    echo "  b - build kernel for current target"
    echo "  br - build kernel + run with qemu"
    echo "  bt - build tests for current target"
    echo "  btr - build tests + run with qemu"
    echo "  bd - build kernel + run with qemu (with options -s -S)"
    echo "  cl - delete all build directories"
    echo "  cbr - delete all build + build kernel + run with qemu"
    echo "  cbtr - delete all build + build tests + run with qemu"
    echo "  d - start lldb + connect to localhost:1234"
    echo "  r - run with qemu"
    echo "  rd - run with qemu (with options -s -S)"
    echo "  help - print this message"
}
set_target i386
help

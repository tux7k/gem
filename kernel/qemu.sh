set -e
cargo build
objdump -x target/x86_64-unknown-uefi/debug/gem.efi > gem.efi.dump
qemu-system-x86_64 -m 512 -nographic -bios ./bios/OVMF_CODE-pure-efi.fd -device driver=e1000,netdev=n0 -netdev user,id=n0,tftp=target/x86_64-unknown-uefi/debug,bootfile=gem.efi
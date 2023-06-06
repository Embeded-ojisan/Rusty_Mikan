#!/bin/sh
## kvmへのアクセスができない場合は以下
## sudo chmod 777 /dev/kvm
cd ./boot && ./xbuild.sh && cd ../ || exit
cd ./kernel && cargo build && cd ../ || exit

mkdir -p esp/bios/
mkdir -p esp/efi/boot/

cp /usr/share/OVMF/OVMF_CODE.fd esp/bios/
cp /usr/share/OVMF/OVMF_VARS.fd esp/bios/

cp target/x86_64-unknown-uefi/debug/boot.efi esp/efi/boot/bootx64.efi

cp target/x86_64-unknown-none-rusty_mikan/debug/kernel.elf esp/kernel.elf

sudo chmod 777 /dev/kvm

qemu-system-x86_64 -enable-kvm  -monitor stdio                                     \
    -drive if=pflash,format=raw,readonly=on,file=esp/bios/OVMF_CODE.fd  \
    -drive if=pflash,format=raw,readonly=on,file=esp/bios/OVMF_VARS.fd  \
    -drive format=raw,file=fat:rw:esp

#    -drive if=pflash,format=raw,readonly=on,file=bios/OVMF_CODE.fd  \
#    -drive if=pflash,format=raw,readonly=on,file=bios/OVMF_VARS.fd  \
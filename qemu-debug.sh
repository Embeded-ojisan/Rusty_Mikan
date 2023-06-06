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

# 以下を起動後、相手側は以下を実行
# https://blog.masu-mi.me/post/2020/12/10/observe-linux-over-qemu-with-gdb/
# ./espに入り、gdb
## target remote localhost:12345
## file kernel.elf
## b kernel_main
## continue
qemu-system-x86_64 -gdb tcp::12345 -S -enable-kvm                       \
    -drive if=pflash,format=raw,readonly=on,file=esp/bios/OVMF_CODE.fd  \
    -drive if=pflash,format=raw,readonly=on,file=esp/bios/OVMF_VARS.fd  \
    -drive format=raw,file=fat:rw:esp

#    -drive if=pflash,format=raw,readonly=on,file=bios/OVMF_CODE.fd  \
#    -drive if=pflash,format=raw,readonly=on,file=bios/OVMF_VARS.fd  \
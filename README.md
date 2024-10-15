# QTest tools for embedded systems emulation

## Para usarlo, tienes que ejecutar el test, y luego en un terminal ponemos las instrucciones ./qemu.sh  , que son:

 ../qemu_new/build/qemu-system-arm -cpu cortex-m4 -machine netduinoplus2 -semihosting-config 
 enable=on,target=native -monitor stdio -qtest tcp:localhost:3000 -kernel test_v1.elf

(dependiendo si lo hacemos en tcp o en unix o en otro socket este comando cambiar: tcp:localhost:3000)


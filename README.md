# QTest tools for embedded systems emulation

## Para usarlo, tienes que ejecutar el test, y luego en un terminal ponemos las instrucciones ./qemu.sh  , que son:

 ../qemu_new/build/qemu-system-arm -cpu cortex-m4 -machine netduinoplus2 -semihosting-config 
 enable=on,target=native -monitor stdio -qtest tcp:localhost:3000 -kernel test_v1.elf

(dependiendo si lo hacemos en tcp o en unix o en otro socket este comando cambiar: tcp:localhost:3000)





//TO DO: pensar hacer versión mut de todos los registros y hacerlo con macros 
//TO DO?: hacer en registro función de read pin???
//Poner de dependencias en cargo en el otro proyecto este. 

const GPIOAADD: usize = 0x40020000;
const GPIOBADD: usize = 0x40020400;
const GPIOCADD: usize = 0x40020800;
const GPIODADD: usize = 0x40020c00;
const GPIOEADD: usize = 0x40021000;
const GPIOFADD: usize = 0x40021400;
const GPIOGADD: usize = 0x40021800;
const GPIOHADD: usize = 0x40021c00;
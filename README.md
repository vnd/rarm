#### Overview
ARMv7 emulator written in Rust (C-style, not really idiomatic), emulated board -- Versatile Express
* hardware: ARMv7 CPU including CP15, MMU, read-only UART, timer and interrupt controller
* OS: vanilla Linux 3.2 with custom config (turned off SMP, Thumb, hardware FP, etc)
* rudimentary breakpoins, watchpoints, single stepping
* boots to prompt (note: UART is read-only at the moment, so not possible to type)

#### Stats
* about 1 millions instruction per second, which is 60-70 times slower than qemu
* 4000+ lines of code, excluding asserts
* 73 ARM instruction implemented
* about 130 millions instruction executed in total

#### Dependencies
* [Rust](https://www.rust-lang.org/en-US/downloads.html), tested with stable 1.12
* [capstone](http://www.capstone-engine.org) for disassembly, included

#### Prepare
Install Rust, e.g.
```Rust
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```
fetch capstone bindings
```Rust
git submodule init
git submodule update
```

#### Run
run.sh script, which:
* runs release build, which is *much* faster than debug
* sets explicit LIBRARY_PATH and LD_LIBRARY_PATH to keep out incompatible upstream capstone changes
* redirects stdout with debug information to /dev/null

#### Walkthrough
(only the most relevant parts from main.rs and cpu/core.rs, simplified)

Kernel, atags and initramfs are loaded into memory:
```Rust
let mut mem: Memory = Memory::new();
add_memory_regions(&mut mem);
mem.load(ATAGS_ADDR,   "bin/atags");
mem.load(ZIMAGE_ADDR,  "bin/zImage");
mem.load(RAMDISK_ADDR, "bin/initramfs");
```

CPU is created and started, r0-r2 registers are set accordingly to Linux on ARM convention:
```Rust
let mut cpu: CPU = CPU::new(mem, ZIMAGE_ADDR as u32);
// r0 = 0,
cpu.set_reg(0, 0);
// r1 = machine type number (Versatile Express)
cpu.set_reg(1, 2272);
/*  r2 = physical address of tagged list in system RAM, or
         physical address of device tree block (dtb) in system RAM */
cpu.set_reg(2, ATAGS_ADDR as u32);

unsafe { cpu.start(capstone, args.verbose); }
```

CPU starts in Supervisor mode with disabled interrupts:
```Rust
self.set_cpsr_bits(CPSR_M, ProcessorMode::Svc as u32);
self.set_cpsr_bit(CPSR_I, 1);
self.set_cpsr_bit(CPSR_F, 1);
self.set_cpsr_bit(CPSR_A, 1);
```

Main loop: each instruction can return a new address (e.g. jump), if not then execute the next one:
```Rust
let next = match self.execute_insn(insn) {
     Some(addr) => addr,
	 None => self.get_pc() + 4,
};
self.set_pc(next);
```

Omitted above but important:
* timer tick, triggered every 100 instructions
* UART TX interrupt, basically triggered once kernel enables it
* memory fault, which may happen during instruction execution or fetching a new one 

#### Links
[kernel and ramdisk compilation](https://balau82.wordpress.com/2012/03/31/compile-linux-kernel-3-2-for-arm-and-emulate-with-qemu) (except for the custom config)

#### Boot log
```
Uncompressing Linux... done, booting the kernel.
Initializing cgroup subsys cpuset
Linux version 3.2.0 (ivan@T400) (gcc version 4.7.3 (Ubuntu/Linaro 4.7.3-12ubuntu1) ) #5 Fri Apr 15 14:56:26 CEST 2016
bootconsole [earlycon0] enabled
sched_clock: 32 bits at 24MHz, resolution 41ns, wraps every 178956ms
Kernel command line: console=ttyAMA0 earlyprintk=ttyAMA0
PID hash table entries: 64 (order: -4, 256 bytes)
Dentry cache hash table entries: 2048 (order: 1, 8192 bytes)
Inode-cache hash table entries: 1024 (order: 0, 4096 bytes)
Memory: 16MB = 16MB total
Memory: 11276k/11276k available, 5108k reserved, 0K highmem
Virtual kernel memory layout:
    vector  : 0xffff0000 - 0xffff1000   (   4 kB)
    fixmap  : 0xfff00000 - 0xfffe0000   ( 896 kB)
    vmalloc : 0x81800000 - 0xf8000000   (1896 MB)
    lowmem  : 0x80000000 - 0x81000000   (  16 MB)
    modules : 0x7f000000 - 0x80000000   (  16 MB)
      .text : 0x80008000 - 0x803daaf4   (3915 kB)
      .init : 0x803db000 - 0x803fe000   ( 140 kB)
      .data : 0x803fe000 - 0x80420b20   ( 139 kB)
       .bss : 0x80420b44 - 0x8043d76c   ( 116 kB)
SLUB: Genslabs=13, HWalign=32, Order=0-3, MinObjects=0, CPUs=1, Nodes=1
NR_IRQS:128
Console: colour dummy device 80x30
Calibrating delay loop... 99.73 BogoMIPS (lpj=498688)
pid_max: default: 32768 minimum: 301
Mount-cache hash table entries: 512
CPU: Testing write buffer coherency: ok
hw perfevents: enabled with ARMv7 Cortex-A9 PMU driver, 1 counters available
NET: Registered protocol family 16
L2x0 series cache controller enabled
l2x0: 8 ways, CACHE_ID 0x00000000, AUX_CTRL 0x00400000, Cache size: 65536 B
hw-breakpoint: debug architecture 0x4 unsupported.
Serial: AMBA PL011 UART driver
mb:uart0: ttyAMA0 at MMIO 0x10009000 (irq = 37) is a PL011 rev1
console [ttyAMA0] enabled, bootconsole disabled
console [ttyAMA0] enabled, bootconsole disabled
bio: create slab <bio-0> at 0
SCSI subsystem initialized
usbcore: registered new interface driver usbfs
usbcore: registered new interface driver hub
usbcore: registered new device driver usb
Advanced Linux Sound Architecture Driver Version 1.0.24.                                                                                              [2/9238]
Switching to clocksource v2m-timer1
NET: Registered protocol family 2
IP route cache hash table entries: 1024 (order: 0, 4096 bytes)
TCP established hash table entries: 512 (order: 0, 4096 bytes)
TCP bind hash table entries: 512 (order: -1, 2048 bytes)
TCP: Hash tables configured (established 512 bind 512)
TCP reno registered
UDP hash table entries: 256 (order: 0, 4096 bytes)
UDP-Lite hash table entries: 256 (order: 0, 4096 bytes)
NET: Registered protocol family 1
RPC: Registered named UNIX socket transport module.
RPC: Registered udp transport module.
RPC: Registered tcp transport module.
RPC: Registered tcp NFSv4.1 backchannel transport module.
Unpacking initramfs...
Freeing initrd memory: 576K
NetWinder Floating Point Emulator V0.97 (double precision)
JFFS2 version 2.2. (NAND) Â© 2001-2006 Red Hat, Inc.
msgmni has been set to 23
io scheduler noop registered (default)
smsc911x: Driver version 2008-10-21
smsc911x: Device not READY in 100ms aborting
isp1760 isp1760: NXP ISP1760 USB Host Controller
isp1760 isp1760: new USB bus registered, assigned bus number 1
isp1760 isp1760: Scratch test failed.
isp1760 isp1760: can't setup
isp1760 isp1760: USB bus 1 deregistered
isp1760: Failed to register the HCD device
Initializing USB Mass Storage driver...
usbcore: registered new interface driver usb-storage
USB Mass Storage support registered.
mousedev: PS/2 mouse device common for all mice
usbcore: registered new interface driver usbhid
usbhid: USB HID core driver
ALSA device list:
  No soundcards found.
oprofile: using arm/armv7-ca9
TCP cubic registered
NET: Registered protocol family 17
drivers/rtc/hctosys.c: unable to open rtc device (rtc0)
Freeing init memory: 140K
/bin/sh: can't access tty; job control turned off
/ #

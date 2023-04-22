# keebo?

keebo is an unrealized split keyboard design with a trackpoint on one side.

This firmware was developed by following branan's [explorations of Rust on the teensy 3.2](https://branan.github.io/teensy/2017/01/12/bootup.html) and expanding that. At the time there was no functional Rust library for working with the mk20dx256 microcontroller. 

At some point I ran into another developer, [irevoire](https://github.com/irevoire), who wanted to develop this library from scratch. I sent them a message and we started chatting, and I put my firmware development time into his project: 
- https://github.com/irevoire/teensy/pull/4
- https://github.com/irevoire/teensy/pull/5
- https://github.com/irevoire/teensy/pull/1
- [SPI implementation draft](https://github.com/irevoire/teensy/compare/master...dwbrite:teensy-k20dx256:wip/spi)

In addition to these commits, we spent a lot of time discussing the architecture of the project.

This project involved learning how low-level microcontrollers work, fiddling with linker files, and debugging real hardware.

in the end, keebo is die, but i am learn :^))

# DW APB GPIO

`no_std` register layer for the standard Synopsys DW_apb_gpio layout with
little-endian 32-bit MMIO accesses. `DwApbGpio` describes the block; `Port` groups its port registers.

Port widths and optional registers depend on the SoC integration.

Register reference: Synopsys DW_apb_gpio Databook 2.11a (June 2015),
table 6-1 and sections 6.3, 7.3, 9.1 ([mirror](https://gitcode.com/open-source-toolkit/a6736)).

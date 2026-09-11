# DW APB GPIO

`no_std` register layer for the standard Synopsys DW_apb_gpio layout with
32-bit MMIO accesses. `DwApbGpio` describes the block; `Port` groups its port registers.

Port widths and optional registers depend on the SoC integration.

# Realtek RTD2556 / JG2555TB Firmware Memory Map & Architecture

This document outlines the internal architecture of **Realtek RTD2556 / JG2555TB** video scalar firmwares, including Keil C51 bank-switching mechanisms, memory partition offsets, OSD tables, and checksum calculation.

---

## 1. Core Processor Architecture

The RTD2556 controller is built around an enhanced **8051 (MCS-51) 8-bit microcontroller core** operating with extended banking hardware.

* **Address Space**: The native 8051 architecture addresses a maximum of 64 KB (`0x0000`–`0xFFFF`).
* **Bank Switching**: Realtek uses Keil C51 multi-bank memory expansion.
  * **Common Bank (Bank 00)**: Resides in the lower address space and provides interrupt service routines, reset vectors, and common runtime routines.
  * **Switched Banks (Banks 01–12)**: Dynamically mapped into the upper memory window (`0x8000`–`0xFFFF` or `0x0000`–`0xFFFF`) via the Keil trampoline routine (`?B_SWITCH` located at address `0x02C1`).
* **Production Image Size**: Standard production images are exactly **851,968 bytes (832 KB)**, consisting of **13 banks of 64 KB (65,536 bytes)** each.
* **Physical SPI Flash Size**: Physical dumps from Winbond 25Q80 or GD25Q80 chips measure **1,048,576 bytes (1024 KB / 16 banks)**, where banks 13 to 15 are unprogrammed `0xFF` padding.

---

## 2. Bank Allocation Table

| Bank Index | Physical Flash Offset | Primary Function | Contents & Reverse-Engineering Findings |
| :---: | :--- | :--- | :--- |
| **Bank 00** | `0x000000` - `0x00FFFF` | **Boot & Runtime Kernel** | • Reset vector (`LJMP` at `0x0000`)<br>• Interrupt vectors (Timer, External, Serial)<br>• Keil bank switcher (`?B_SWITCH` at `0x02C1`)<br>• Power-on flags (`0x00018`: Direct boot without buttons)<br>• Boot logo bypass flag (`0x00015`: Fast boot bypass) |
| **Bank 01** | `0x010000` - `0x01FFFF` | **Display Modes & Timings** | Standard VESA video mode descriptors (SVGA 800x600, XGA 1024x768, 720p, 1080p, 1200p mode tables around `0x01E8A5`). |
| **Bank 02** | `0x020000` - `0x02FFFF` | **OSD Tables & HDMI EDID** | • Multi-language OSD Text Table (`0x025000`): English, French, German, Spanish, Simplified Chinese, Traditional Chinese, Japanese, Korean, Russian strings.<br>• Dual HDMI VESA EDID 1.3/1.4 descriptors (`0x025BF4`–`0x026078`).<br>• Factory default OSD registers (`0x02D67A`–`0x02D695`: Brightness, Contrast, Color Temp, User RGB gains). |
| **Bank 03** | `0x030000` - `0x03FFFF` | **Scaler Engine & Ratios** | Video scaling horizontal and vertical polyphase filters, aspect ratio mapping (16:9, 4:3, 1:1), and frequency step tables. |
| **Bank 04** | `0x040000` - `0x04FFFF` | **HDMI/TMDS Receiver** | HDMI Hot-Plug Detect (HPD), TMDS physical layer equalizer, HDCP authentication engine, and I2S/SPDIF audio stream extraction. |
| **Bank 05** | `0x050000` - `0x05FFFF` | **eDP Driver & DPLL** | **Core panel timing module.** Display Timing Generator (DTG) registers, pixel clock DPLL synthesis, link training (TPS1/TPS2), and lane count selectors (2-lane vs 4-lane). |
| **Bank 06** | `0x060000` - `0x06FFFF` | **Hardware I/O & Boot Logo** | ADC keypad resistor ladder decoding (4-key vs 5-key schemes), panel power sequencing timers ($T_1 \dots T_5$), PWM backlight modulation, and monochrome boot splash bitmap partition. |
| **Banks 07-10**| `0x070000` - `0x0AFFFF` | **Extended Storage** | Typically filled with unprogrammed `0xFF` bytes; allocated for vendor-specific extensions on custom OEM builds. |
| **Bank 11** | `0x0B0000` - `0x0BFFFF` | **Gamma LUT & Colorimetry** | 16-bit RGB photometric correction curves, sRGB look-up tables, and D-Dithering / I-Dithering matrix blocks (6-bit + FRC vs 8-bit true color). |
| **Bank 12** | `0x0C0000` - `0x0CFFFF` | **Flash Boundary** | End-of-image boundary and padding. |

---

## 3. OSD Language Codes (Bank 00 & Bank 02)

The default OSD language is controlled by opcode immediate values patched at `0x0366B` (Bank 00) and `0x2CFF8` (Bank 02):

| Hex Code | Language Name | Native Display Name |
| :---: | :--- | :--- |
| `0x00` | Traditional Chinese | 繁體中文 |
| `0x01` | Simplified Chinese | 简体中文 |
| `0x02` | English | English (Default) |
| `0x03` | French | Français |
| `0x04` | German | Deutsch |
| `0x05` | Spanish | Español |
| `0x06` | Japanese | 日本語 |
| `0x07` | Korean | 한국어 |
| `0x09` | Russian | Русский |

---

## 4. Checksum Algorithm (Additive 16-bit)

Official Realtek flash tools (`RTDCustomerTool`) validate firmware binaries against a 4-character hexadecimal tag in the filename (e.g. `59BB`, `7ADA`, `6901`, `E043`).

The algorithm is the **16-bit wrapping additive sum of every byte in the flash image**:

```rust
pub fn calculate_checksum(data: &[u8]) -> u16 {
    data.iter().fold(0u16, |acc, &b| acc.wrapping_add(b as u16))
}
```

If a binary is modified without updating this sum, `RTDCustomerTool` will flag an integrity warning during ISP flash operations.

# RTD-Firmwares: Realtek Controller Monorepo & Firmware Studio

Curated firmware repository, reverse-engineering documentation, and native binary engineering tools for **Realtek RTD-based LCD controller boards** (specifically **RTD2556 / JG2555TB**), used to drive laptop and portable monitor LCD panels via HDMI and eDP (Embedded DisplayPort).

---

## 1. Desktop Suite: `RTD-Firmware-Studio`

A native Windows desktop application written in Rust using GPU-accelerated rendering (`egui`/`wgpu`) is available at the root and in `bin/RTD-Firmware-Studio.exe`:

* **Firmware Composer**:
  * **Unified Workflow**: Combines panel timing synthesis, EDID monitor naming, custom boot splash injection, and factory OSD defaults in a single step.
  * **Preset Catalogue Mode**: One-click generation for tested high-refresh panels (BOE NV173FHM-NX4, NE173FHM-NZ6, Innolux N173HCE-G33, AUO B173HAN04.9, AUO B173HAN04.7, etc.).
  * **Manual Datasheet Mode**: Enter custom resolutions (1080p, 1200p, 1440p), refresh rates (60Hz, 120Hz, 144Hz), eDP lane count (4-lane vs 2-lane HBR2), and pixel clocks with automatic VESA CVT-RB calculation.
  * **Integrated Boot Logo Studio**: Upload any JPEG or PNG, configure contrast threshold, Floyd-Steinberg dithering, color inversion, and preview on a simulated 16:9 LCD monitor bezel.
  * **Factory OSD Defaults & Presets**: Collapsible customization suite covering 5 categories:
    * *System & Language*: 9 languages (English, French, German, Spanish, Simplified Chinese, Traditional Chinese, Japanese, Korean, Russian), Direct Power-On (auto boot on power connect without button), default HDMI input source.
    * *Picture & Aspect*: Standard, FPS, RTS, Movie, Text, ECO modes; DCR toggle; 16:9, 4:3, 1:1 scaling.
    * *Color & Calibration*: 6500K Warm, 7500K Neutral, 9300K Cool, sRGB, or **User Custom RGB** with Red, Green, Blue gain sliders and live dynamic color preview swatch; Low Blue Light filters.
    * *Levels & Audio*: Default brightness (0-100%), contrast (0-100%), volume level, and boot mute option.
    * *Gaming & VRR*: FreeSync / Adaptive-Sync toggle and Overdrive (OD) response time acceleration.
  * **VESA EDID Monitor Name**: Injects custom monitor names into descriptor `0xFC`, recognized natively by Windows and GPU drivers.
  * **Automatic Checksum**: Computes 16-bit additive checksums and formats output filenames ready for `RTDCustomerTool`.
* **Binary Inspector & Diagnostic**:
  * Decodes and validates any Realtek `.bin` image.
  * Checks 64 KB monobanks vs 832 KB production vs 1024 KB SPI physical dumps.
  * Validates VESA EDID blocks and checksums.
* **LCD Panel Database & Pinouts**:
  * Searchable technical panel reference with direct Panelook links.

```powershell
# Launch the desktop suite (standalone executable, no installation required):
.\RTD-Firmware-Studio.exe
```

---

## 2. Hardware Flasher: `RTDCustomerTool`

The official Realtek hardware ISP flashing utility is included in the dedicated [`flasher/`](file:///d:/www/RTD-Firmwares/flasher/) directory:

* **Direct Executable (Ready to run, no unzipping required)**:
  ```powershell
  # Launch the flasher utility directly:
  .\flasher\RTDCustomerTool\RTDTool.exe
  ```
* **Original ZIP Archive**: `flasher/RTDCustomerTool_V3.7_20190114.zip`
* **Role**: While `RTD-Firmware-Studio` designs, patches, and validates custom firmwares, `RTDCustomerTool` is used to burn the generated `.bin` images directly into the controller board's SPI flash memory via an HDMI ISP cable.
* **Step-by-step guide**: See [Flashing Guide (`docs/flashing_guide.md`)](file:///d:/www/RTD-Firmwares/docs/flashing_guide.md).

---

## 3. Firmware Catalog (`firmwares/`)

All stock and tested firmware binaries are cleanly organized by resolution, refresh rate, and verified hardware targets:

```
firmwares/
├── 1080p_120hz_144hz/       # High-refresh FHD firmwares (Gaming panels: BOE, Innolux, AUO)
├── 1080p_60hz/              # Standard 60Hz FHD firmwares (LG, Samsung, Panda, Sharp IGZO)
├── 1200p_16_10/             # 1920x1200 16:10 aspect ratio panels (LP156WU1, B101UAN01)
├── 2k_1440p/                # QHD 2560x1440 & Retina 2560x1600 (TV133QHM, N156KME, N133GCA)
├── tested_and_verified/     # Dumps verified on physical hardware (e.g. Essential 15 inch)
└── oem_archives/            # Original manufacturer ZIP archives (historical backups)
```

### Verified Hardware Highlights (`firmwares/tested_and_verified/`)

| Firmware Name | Target Panel(s) | Resolution & Rate | Interface | Checksum |
| :--- | :--- | :---: | :---: | :---: |
| **`Essential_15inch_120Hz_144Hz_MultiPanel.bin`** | NV156FHM-NX1/NX2, LM156LFGL01..05, B156HAN13.0 | 1920x1080 @ 120/144Hz | 40-pin 4-Lane | `7ADA` |
| **`LP156WFG_M133NWF3_B156HAN13_144Hz.bin`** | LG LP156WFG, IVO M133NWF3, AUO B156HAN13.0 | 1920x1080 @ 144Hz | 40-pin 4-Lane | `59BC` |
| **`INNOLUX_N156HHE_GA1_120Hz_E043.bin`** | Innolux N156HHE-GA1 (High-speed 120Hz) | 1920x1080 @ 120Hz | 30-pin 2-Lane HBR2 | `E043` |
| **`LG_LP156WU1_SPB1_1200p_6901.bin`** | LG LP156WU1-SPB1 (16:10 WUXGA) | 1920x1200 @ 60Hz | 30-pin 2-Lane | `6901` |
| **`AUO_B101UAN01_1200p_CBBE.bin`** | AUO B101UAN01 (10.1" 16:10 Compact) | 1920x1200 @ 60Hz | 30-pin 2-Lane | `CBBE` |

---

## 4. Command-Line Utility: `rtd-cli`

A zero-dependency command-line utility is located in `bin/rtd-cli.exe`:

```powershell
# Detailed inspection of a firmware file (EDID, timings, pinout, checksum):
.\bin\rtd-cli.exe --info "firmwares/tested_and_verified/Essential_15inch_120Hz_144Hz_MultiPanel.bin"

# Search compatible firmwares for a panel model:
.\bin\rtd-cli.exe --find NV173FHM-NX4
.\bin\rtd-cli.exe --find B156HAN13

# Scan and map health status of all firmwares in a directory:
.\bin\rtd-cli.exe --scan firmwares/
```

---

## 5. Documentation & Reverse-Engineering

Detailed technical reference guides are provided in the [`docs/`](file:///d:/www/RTD-Firmwares/docs/) directory:

* [**eDP Pinout Reference**](file:///d:/www/RTD-Firmwares/docs/pinouts_edp.md): Detailed 30-pin (2-lane) and 40-pin (4-lane) micro-coaxial pinouts, lane mapping, and backlight wiring.
* [**Memory Map & Bank Architecture**](file:///d:/www/RTD-Firmwares/docs/memory_map.md): Keil C51 8051 bank allocation (Banks 00 to 12), OSD table addresses, EDID offsets, and checksum algorithms.
* [**Flashing Guide**](file:///d:/www/RTD-Firmwares/docs/flashing_guide.md): Step-by-step burning procedure via HDMI ISP programmer cable and `RTDCustomerTool`.

---

## 6. Firmware Memory Map (Summary)

RTD2556 / JG2555TB firmwares use an **8051 (MCS-51) architecture** with Keil C51 bank-switching. Production images are exactly **851,968 bytes (832 KB)**, divided into **13 banks of 64 KB (0x10000 bytes)** each.

| Bank | Physical Range | Primary Function |
| :--- | :--- | :--- |
| **Bank 00** | `0x000000` - `0x00FFFF` | Reset vectors, runtime kernel, Keil bank-switcher (`?B_SWITCH` at `0x02C1`), direct power flag (`0x00018`). |
| **Bank 01** | `0x010000` - `0x01FFFF` | Standard VESA video mode descriptors (SVGA, 720p, 1080p, 1200p tables). |
| **Bank 02** | `0x020000` - `0x02FFFF` | Multi-language OSD text tables, dual HDMI VESA EDID descriptors (`0x025BF4`), and factory default OSD registers (`0x02D67A`–`0x02D695`). |
| **Bank 03** | `0x030000` - `0x03FFFF` | Scaler engine, polyphase filters, aspect ratio look-up tables. |
| **Bank 04** | `0x040000` - `0x04FFFF` | HDMI/TMDS receiver physical layer, HDCP engine, and digital audio extraction. |
| **Bank 05** | `0x050000` - `0x05FFFF` | eDP Driver: pixel clock DPLL synthesis, link training, and Display Timing Generator (DTG). |
| **Bank 06** | `0x060000` - `0x06FFFF` | Keypad resistor ladder decoding, backlight PWM driver, and boot splash bitmap partition. |
| **Bank 11** | `0x0B0000` - `0x0BFFFF` | 16-bit RGB photometric gamma curves and D-Dithering / I-Dithering matrices. |

---

## 7. Building from Source

Both tools are written in standard Rust (Edition 2021):

```powershell
# Build both RTD Firmware Studio and rtd-cli in release mode:
cargo build --release --manifest-path tools/rtd-gui/Cargo.toml
cargo build --release --manifest-path tools/rtd-cli/Cargo.toml
```

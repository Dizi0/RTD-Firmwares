# Flashing Guide: Realtek RTD Scaler Boards via HDMI ISP

This guide details the procedures for dumping and flashing production firmware (`.bin`) images onto **Realtek RTD2556 / JG2555TB / PCB800869** controller boards without desoldering the SPI flash chip, using either:
1. **Direct GPU HDMI Connection (GFX I2C)**: No dedicated programmer dongle required!
2. **Dedicated USB ISP Hardware Programmer**: Using FT232H, CH341A, or official Realtek USB-I2C dongles.

---

## 1. Flashing Utilities: `RTDCustomerTool`

Two official Realtek ISP software packages are provided in the [`flasher/`](file:///d:/www/RTD-Firmwares/flasher/) directory:

* **RTDCustomerTool V3.8 (Recommended, 2022-11-01)**:
  * Location: [`flasher/RTDCustomerTool_V3.8/RTDTool.exe`](file:///d:/www/RTD-Firmwares/flasher/RTDCustomerTool_V3.8/RTDTool.exe)
  * Archive: `flasher/RTDCustomerTool_V3.8_20221101.zip`
  * Features updated communication drivers (`Comm.dll`), improved Windows 10/11 stability, and newer scaler definitions (RTD2537, RTD2556, RTD2795).
* **RTDCustomerTool V3.7 (Historical, 2019-01-14)**:
  * Location: [`flasher/RTDCustomerTool/RTDTool.exe`](file:///d:/www/RTD-Firmwares/flasher/RTDCustomerTool/RTDTool.exe)
  * Archive: `flasher/RTDCustomerTool_V3.7_20190114.zip`

Run `RTDTool.exe` as Administrator on Windows.

---

## 2. Method A: Direct GPU Flashing via HDMI (No Hardware Programmer Needed)

On systems with a dedicated GPU (e.g., **NVIDIA GeForce**, AMD Radeon, or Intel Arc), `RTDCustomerTool` can dump and flash the board's SPI flash memory directly over a **standard HDMI cable** using DDC/CI I2C commands (`Comm_GFXI2C.dll`).

### Hardware Connection:
1. Plug a standard HDMI cable from your PC / laptop GPU HDMI port directly into **HDMI 1** on the RTD controller board.
2. Power the RTD board with its recommended 12V DC power supply (ensure the status LED is lit).

### RTDCustomerTool Configuration:
1. Launch `RTDTool.exe` as Administrator.
2. In the top toolbar, set **Access Method**: `GFXI2C`.
3. Set **Scaler IC**: Select `RTD2537T-CG` or `RTD2556`.
4. Open menu **Communication Option** -> **GFXI2C Adjust Option**:
   * **Graphics Detect Type**: `Manual`
   * **Graphics Type**: `Nvidia` (or your active GPU brand)
   * **Multi Monitor Select**: `Auto Detect Realtek Monitor`
   * **Multi Realtek Monitor Select**: **`User Select`** (CRITICAL)
5. In the `Multi RT Monitor Select` popup window:
   * Select your portable display device: e.g., `Device Name: \\.\DISPLAY2, Output ID: 256` (whichever output ID corresponds to your HDMI port).

> [!IMPORTANT]
> **Troubleshooting `Enter Isp Mode fail! ErrorCode:0xA`**:
> If you leave `Multi Realtek Monitor Select` set to `Auto Detect Realtek Monitor` or `First Realtek Monitor`, the NVIDIA driver probes `DISPLAY1` first (your primary desktop monitor or internal laptop screen). Because that display is not a Realtek scaler, the ISP handshake command times out and fails with `ErrorCode:0xA`.
> **Fix**: Setting `Multi Realtek Monitor Select` -> `User Select` and explicitly selecting `\\.\DISPLAY2` forces the GPU to communicate directly with the RTD controller board.

### Reading (Dumping) Firmware:
1. Go to the **ISP** tab.
2. Click **Read Total** (or specify bank range 0 to 7 / 0 to 12).
3. The tool will read the flash contents and save a `.bin` image on your disk.

### Writing (Flashing) Firmware:
1. In the **ISP** tab, click **Load File** and select your `.bin` firmware.
2. Ensure **Erase Flash**, **Program**, and **Verify** checkboxes are enabled.
3. Click **Auto Program** (or **Burn**).
4. Once `Verify OK` is reported, power-cycle the 12V supply to reboot the board.

---

## 3. Method B: Flashing via USB ISP Programmer Dongle

If your PC does not have a free HDMI port or if you prefer an external programmer:

1. **Hardware**: USB-to-I2C programmer based on FT232H, CH341A, or dedicated Realtek USB-ISP dongle.
2. Connect the programmer's HDMI output to **HDMI 1** on the RTD board.
3. In `RTDTool.exe`, set **Access Method**: `USB-I2C` (or select device under Communication Option).
4. Set I2C Clock speed to **100 kHz** (for cable stability).
5. Load file, click **Auto Program**, and wait for `Verify OK`.

---

## 4. OSD & Calibration Tips

### Unlocking OSD Brightness & Contrast (DCR Lockout)
If the **Brightness** and **Contrast** sliders are grayed out or unselectable in the monitor OSD menu, **Dynamic Contrast Ratio (DCR)** is enabled by default. Go into the Picture / Display menu in the OSD and turn **DCR to OFF**. Manual brightness and contrast controls will immediately unlock.

### Fixing the OSD Brightness Divisor Bug (PWM Divisor 600 vs 100)
On certain OEM RTD2556 / PCB800869 firmwares, the brightness slider jumps abruptly or only adjusts at the very high end of the 0–100 scale:
* **Root Cause**: The Keil C51 routine in Bank 02 (`0x2CFFC`) and Bank 07 (`0x7FE9A`) loads `0x0258` (decimal 600) as the PWM divisor instead of `0x0064` (decimal 100).
* **Fix**: Patching the divisor value from `0x0258` to `0x0064` restores smooth 0–100% linear PWM duty cycle regulation across all 100 steps.

---

## 5. Troubleshooting & Failsafe Recovery

* **Error: "Device Not Found / I2C NACK"**:
  * Check the HDMI cable connection; ensure it is plugged into **HDMI 1**, not HDMI 2.
  * Verify that the board has adequate DC power (do not rely on 5V from HDMI pin 18).
* **Black Screen After Flashing**:
  * If the panel remains completely dark, verify the eDP lane count (40-pin 4-lane vs 30-pin 2-lane).
  * Check if your panel requires HBR (2.7 Gbps) or HBR2 (5.4 Gbps).
  * Flash back one of the verified reference images from `firmwares/tested_and_verified/`.

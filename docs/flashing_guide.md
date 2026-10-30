# Flashing Guide: Realtek RTD2556 / JG2555TB via HDMI ISP

This guide details the procedure for flashing production firmware (`.bin`) images onto **Realtek RTD2556 / JG2555TB** controller boards without unsoldering the SPI flash chip, using an **HDMI ISP programmer cable** and **RTDCustomerTool**.

---

## 1. Hardware Requirements

1. **Target Board**: Realtek RTD2556 (e.g. JG2555TB Dual-mini HDMI board or similar).
2. **Power Supply**: 12V DC power adapter (or 5V USB-C depending on board revision).
3. **HDMI ISP Tool / Cable**:
   * USB-to-I2C programmer based on FT232H, CH341A, or Realtek dedicated USB-ISP dongle.
   * Standard HDMI cable connected between the programmer and the controller board's **HDMI 1** port.
   * *Pin 15 (DDC SCL)*, *Pin 16 (DDC SDA)*, and *Pin 17 (Ground)* on the HDMI connector carry the ISP communication channel.

---

## 2. Software Setup

1. The official flashing utility is provided in `firmwares/oem_archives/RTDCustomerTool_V3.7_20190114.zip` (and extracted in `RTDCustomerTool/`).
2. Run `RTDTool.exe` as Administrator on Windows.

---

## 3. Step-by-Step Burning Procedure

### Step 1: Connect Hardware
1. Connect the HDMI ISP cable to **HDMI 1** on the RTD2556 board.
2. Apply DC power to the controller board (ensure the LED turns RED for Standby or GREEN/BLUE for Active).

### Step 2: Configure RTDCustomerTool
1. Open `RTDCustomerTool.exe`.
2. Under the **Communication / ISP** tab:
   * Select your programmer device (e.g., `Realtek USB-I2C` or `FT232H`).
   * Set I2C Clock speed to **100 kHz** or **400 kHz** (100 kHz is recommended for cable stability).
   * Click **Connect / Detect**. The software should return `Device Detected: RTD2556`.

### Step 3: Load Firmware
1. Go to the **ISP / Flash** tab.
2. Click **Load File** and select your target `.bin` firmware (either from `firmwares/` or composed via `RTD-Firmware-Studio.exe`).
3. Verify that the file size is **851,968 bytes (832 KB)**.
4. Verify that the calculated checksum matches the 4-hex digit tag in the filename.

### Step 4: Program & Verify
1. Enable the following checkboxes:
   * **Erase Flash** (Sector Erase or Full Chip Erase)
   * **Program**
   * **Verify**
2. Click **Auto Program** (or **Burn**).
3. Wait for the progress bar to reach 100% (typically 15 to 45 seconds).
4. Once `Verify OK` is displayed, disconnect the HDMI ISP cable.
5. Power-cycle the board. The new display configuration, splash logo, and OSD defaults will now be active.

---

## 4. Troubleshooting & Failsafe Recovery

* **Error: "Device Not Found / I2C NACK"**:
  * Check the HDMI cable connection; ensure it is plugged into **HDMI 1**, not HDMI 2.
  * Verify that the board has adequate DC power (do not rely on 5V from HDMI pin 18).
* **Black Screen After Flashing**:
  * If the panel remains completely dark, verify the eDP lane count (40-pin 4-lane vs 30-pin 2-lane).
  * Check if your panel requires HBR (2.7 Gbps) or HBR2 (5.4 Gbps).
  * Flash back one of the verified reference images from `firmwares/tested_and_verified/`.

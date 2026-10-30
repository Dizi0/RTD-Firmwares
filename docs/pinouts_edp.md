# eDP Connector Pinout Reference (30-Pin & 40-Pin)

This technical reference details the physical pinout and signal assignments for **Embedded DisplayPort (eDP)** connectors commonly used on Realtek RTD2556 / JG2555TB controller boards and laptop LCD panels.

---

## 1. 30-Pin eDP Connector (Standard 60Hz FHD & HBR2 120Hz)

The 30-pin eDP interface supports **up to 2 Main-Link lanes** (Lane 0 and Lane 1) with an Auxiliary channel (AUX CH) and Hot-Plug Detect (HPD).

* **Pitch**: 0.5 mm or 0.4 mm micro-coaxial (I-PEX 20455-030E / 20453-030T compatible)
* **Standard Resolution**: 1920x1080 @ 60Hz (2.7 Gbps HBR)
* **High-Speed Resolution**: 1920x1080 @ 120Hz (5.4 Gbps HBR2, e.g. Innolux N156HHE-GA1) or 1920x1200 @ 60Hz

| Pin # | Signal Name | Type | Description |
| :---: | :--- | :--- | :--- |
| **1** | NC | No Connect | Reserved / Not Connected |
| **2** | H_GND | Ground | High-speed differential ground |
| **3** | Lane1_N | Input | eDP Main-Link Lane 1 (Negative differential) |
| **4** | Lane1_P | Input | eDP Main-Link Lane 1 (Positive differential) |
| **5** | H_GND | Ground | High-speed differential ground |
| **6** | Lane0_N | Input | eDP Main-Link Lane 0 (Negative differential) |
| **7** | Lane0_P | Input | eDP Main-Link Lane 0 (Positive differential) |
| **8** | H_GND | Ground | High-speed differential ground |
| **9** | AUX_CH_P | Bi-dir | Auxiliary Channel (Positive differential / DDC emulation) |
| **10** | AUX_CH_N | Bi-dir | Auxiliary Channel (Negative differential / DDC emulation) |
| **11** | H_GND | Ground | High-speed differential ground |
| **12** | LCD_VCC | Power | Logic power supply (+3.3V DC to T-CON) |
| **13** | LCD_VCC | Power | Logic power supply (+3.3V DC to T-CON) |
| **14** | LCD_Self_Test | Input | Panel self-test pattern enable (ground for normal operation) |
| **15** | LCD_GND | Ground | Logic Ground |
| **16** | LCD_GND | Ground | Logic Ground |
| **17** | HPD | Output | Hot-Plug Detect signal (active HIGH from panel to scaler) |
| **18** | BL_GND | Ground | LED Backlight power ground |
| **19** | BL_GND | Ground | LED Backlight power ground |
| **20** | BL_GND | Ground | LED Backlight power ground |
| **21** | BL_GND | Ground | LED Backlight power ground |
| **22** | BL_ENABLE | Input | Backlight ON/OFF control (+3.3V = ON, 0V = OFF) |
| **23** | BL_PWM | Input | Backlight brightness dimming (200 Hz - 20 kHz PWM) |
| **24** | NC | No Connect | Reserved |
| **25** | NC | No Connect | Reserved |
| **26** | BL_PWR | Power | LED Backlight supply (+7V to +21V DC, typical +12V) |
| **27** | BL_PWR | Power | LED Backlight supply (+7V to +21V DC, typical +12V) |
| **28** | BL_PWR | Power | LED Backlight supply (+7V to +21V DC, typical +12V) |
| **29** | BL_PWR | Power | LED Backlight supply (+7V to +21V DC, typical +12V) |
| **30** | NC | No Connect | Reserved |

---

## 2. 40-Pin eDP Connector (144Hz / 165Hz / 240Hz Gaming Panels)

The 40-pin eDP interface provides **4 Main-Link lanes** (Lane 0, 1, 2, and 3), allowing up to 10.8 Gbps (4 x HBR) or 21.6 Gbps (4 x HBR2) bandwidth.

* **Target Refresh Rates**: 120Hz, 144Hz, 165Hz, 240Hz @ 1080p FHD, or 60Hz/144Hz @ 1440p QHD / 4K UHD
* **Connector Model**: I-PEX 20455-040E-12 or compatible 0.5mm pitch

| Pin # | Signal Name | Type | Description |
| :---: | :--- | :--- | :--- |
| **1** | NC | No Connect | Reserved |
| **2** | H_GND | Ground | High-speed differential ground |
| **3** | Lane3_N | Input | eDP Main-Link Lane 3 (Negative differential) |
| **4** | Lane3_P | Input | eDP Main-Link Lane 3 (Positive differential) |
| **5** | H_GND | Ground | High-speed differential ground |
| **6** | Lane2_N | Input | eDP Main-Link Lane 2 (Negative differential) |
| **7** | Lane2_P | Input | eDP Main-Link Lane 2 (Positive differential) |
| **8** | H_GND | Ground | High-speed differential ground |
| **9** | Lane1_N | Input | eDP Main-Link Lane 1 (Negative differential) |
| **10** | Lane1_P | Input | eDP Main-Link Lane 1 (Positive differential) |
| **11** | H_GND | Ground | High-speed differential ground |
| **12** | Lane0_N | Input | eDP Main-Link Lane 0 (Negative differential) |
| **13** | Lane0_P | Input | eDP Main-Link Lane 0 (Positive differential) |
| **14** | H_GND | Ground | High-speed differential ground |
| **15** | AUX_CH_P | Bi-dir | Auxiliary Channel Positive |
| **16** | AUX_CH_N | Bi-dir | Auxiliary Channel Negative |
| **17** | H_GND | Ground | High-speed differential ground |
| **18** | LCD_VCC | Power | Logic power supply (+3.3V DC to T-CON) |
| **19** | LCD_VCC | Power | Logic power supply (+3.3V DC to T-CON) |
| **20** | LCD_VCC | Power | Logic power supply (+3.3V DC to T-CON) |
| **21** | LCD_VCC | Power | Logic power supply (+3.3V DC to T-CON) |
| **22** | NC | No Connect | Reserved |
| **23** | LCD_GND | Ground | Logic Ground |
| **24** | LCD_GND | Ground | Logic Ground |
| **25** | LCD_GND | Ground | Logic Ground |
| **26** | LCD_GND | Ground | Logic Ground |
| **27** | HPD | Output | Hot-Plug Detect (active HIGH from panel to scaler) |
| **28** | BL_GND | Ground | LED Backlight power ground |
| **29** | BL_GND | Ground | LED Backlight power ground |
| **30** | BL_GND | Ground | LED Backlight power ground |
| **31** | BL_GND | Ground | LED Backlight power ground |
| **32** | BL_ENABLE | Input | Backlight ON/OFF control (+3.3V = ON, 0V = OFF) |
| **33** | BL_PWM | Input | Backlight brightness dimming (PWM control) |
| **34** | NC | No Connect | Reserved |
| **35** | NC | No Connect | Reserved |
| **36** | BL_PWR | Power | LED Backlight supply (+7V to +21V DC, typical +12V) |
| **37** | BL_PWR | Power | LED Backlight supply (+7V to +21V DC, typical +12V) |
| **38** | BL_PWR | Power | LED Backlight supply (+7V to +21V DC, typical +12V) |
| **39** | BL_PWR | Power | LED Backlight supply (+7V to +21V DC, typical +12V) |
| **40** | NC | No Connect | Reserved |

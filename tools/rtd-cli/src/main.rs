use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone)]
struct EdidInfo {
    offset: usize,
    mfg_id: String,
    monitor_name: String,
    pixel_clock_mhz: f64,
    h_active: u32,
    h_blank: u32,
    h_total: u32,
    v_active: u32,
    v_blank: u32,
    v_total: u32,
    refresh_rate_hz: f64,
    checksum_valid: bool,
}

fn parse_edid_at(data: &[u8], offset: usize) -> Option<EdidInfo> {
    let header = [0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00];
    if offset + 128 > data.len() || &data[offset..offset + 8] != &header {
        return None;
    }

    let edid_slice = &data[offset..offset + 128];
    let checksum: u8 = edid_slice.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    let checksum_valid = checksum == 0;

    let b8 = edid_slice[8];
    let b9 = edid_slice[9];
    let c1 = ((b8 & 0x7c) >> 2) + 64;
    let c2 = (((b8 & 0x03) << 3) | ((b9 & 0xe0) >> 5)) + 64;
    let c3 = (b9 & 0x1f) + 64;
    let mfg_id = format!("{}{}{}", c1 as char, c2 as char, c3 as char);

    let mut monitor_name = "Unspecified".to_string();
    for &desc_off in &[54, 72, 90, 108] {
        if desc_off + 18 <= edid_slice.len() && &edid_slice[desc_off..desc_off + 5] == &[0x00, 0x00, 0x00, 0xfc, 0x00] {
            let name_bytes = &edid_slice[desc_off + 5..desc_off + 18];
            let name_str: String = name_bytes.iter()
                .take_while(|&&b| b != 0x0a && b != 0x00)
                .map(|&b| b as char)
                .collect();
            let trimmed = name_str.trim();
            if !trimmed.is_empty() {
                monitor_name = trimmed.to_string();
            }
            break;
        }
    }

    let pclk1 = (edid_slice[54] as u32) | ((edid_slice[55] as u32) << 8);
    let pclk2 = if edid_slice.len() >= 90 { (edid_slice[72] as u32) | ((edid_slice[73] as u32) << 8) } else { 0 };
    let (target_off, pclk_raw) = if pclk2 > pclk1 { (72, pclk2) } else { (54, pclk1) };

    let pixel_clock_mhz = (pclk_raw as f64) * 10.0 / 1000.0;

    let h_active = (edid_slice[target_off + 2] as u32) | (((edid_slice[target_off + 4] as u32) & 0xf0) << 4);
    let h_blank = (edid_slice[target_off + 3] as u32) | (((edid_slice[target_off + 4] as u32) & 0x0f) << 8);
    let h_total = h_active + h_blank;

    let v_active = (edid_slice[target_off + 5] as u32) | (((edid_slice[target_off + 7] as u32) & 0xf0) << 4);
    let v_blank = (edid_slice[target_off + 6] as u32) | (((edid_slice[target_off + 7] as u32) & 0x0f) << 8);
    let v_total = v_active + v_blank;

    let refresh_rate_hz = if h_total > 0 && v_total > 0 && pclk_raw > 0 {
        (pclk_raw as f64 * 10000.0) / (h_total as f64 * v_total as f64)
    } else {
        0.0
    };

    Some(EdidInfo {
        offset,
        mfg_id,
        monitor_name,
        pixel_clock_mhz,
        h_active,
        h_blank,
        h_total,
        v_active,
        v_blank,
        v_total,
        refresh_rate_hz,
        checksum_valid,
    })
}

fn find_edids(data: &[u8]) -> Vec<EdidInfo> {
    let mut results = Vec::new();
    let header = [0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00];
    let mut i = 0;
    while i + 128 <= data.len() {
        if &data[i..i + 8] == &header {
            if let Some(edid) = parse_edid_at(data, i) {
                results.push(edid);
                i += 128;
                continue;
            }
        }
        i += 1;
    }
    results
}

#[derive(Debug)]
enum FirmwareHealth {
    ProductionReady832K,
    PhysicalDump1024K,
    IncompleteSingleBank64K,
    CustomSize(usize),
}

#[derive(Debug)]
struct FirmwareAnalysis {
    file_path: PathBuf,
    file_size: usize,
    num_banks: usize,
    health: FirmwareHealth,
    has_gamma_table: bool,
    checksum: u16,
    expected_checksum: Option<u16>,
    checksum_match: bool,
    edids: Vec<EdidInfo>,
    pin_type: String,
    lane_config: String,
    keypad_type: String,
    backlight_type: String,
    boot_logo: String,
    factory_notes: Vec<String>,
    compatible_panels: Vec<&'static str>,
}

fn extract_hex_checksum_from_name(name: &str) -> Option<u16> {
    for word in name.split(|c: char| !c.is_alphanumeric()) {
        if word.len() == 4 && word.chars().all(|c| c.is_ascii_hexdigit()) {
            if word.eq_ignore_ascii_case("1080")
                || word.eq_ignore_ascii_case("1200")
                || word.eq_ignore_ascii_case("1440")
                || word.eq_ignore_ascii_case("1600")
                || word.eq_ignore_ascii_case("2021")
                || word.eq_ignore_ascii_case("2022")
                || word.eq_ignore_ascii_case("2023")
                || word.eq_ignore_ascii_case("HDMI")
                || word.eq_ignore_ascii_case("EDP1")
            {
                continue;
            }
            if let Ok(val) = u16::from_str_radix(word, 16) {
                return Some(val);
            }
        }
    }
    None
}

fn analyze_firmware(path: &Path) -> Result<FirmwareAnalysis, String> {
    let data = fs::read(path).map_err(|e| format!("Read error: {}", e))?;
    let file_size = data.len();
    let num_banks = (file_size + 0xFFFF) / 0x10000;

    let health = match file_size {
        851968 => FirmwareHealth::ProductionReady832K,
        1048576 => FirmwareHealth::PhysicalDump1024K,
        65536 => FirmwareHealth::IncompleteSingleBank64K,
        other => FirmwareHealth::CustomSize(other),
    };

    let has_gamma_table = if file_size >= 0x0B0000 + 1024 {
        let sample = &data[0x0B0000..0x0B0100];
        sample.iter().any(|&b| b != 0x00 && b != 0xFF)
    } else {
        false
    };

    let checksum: u16 = data.iter().fold(0u16, |acc, &b| acc.wrapping_add(b as u16));

    let fname = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let expected_checksum = extract_hex_checksum_from_name(&fname);
    let checksum_match = expected_checksum.map_or(false, |exp| exp == checksum);

    let edids = find_edids(&data);

    let (res_w, res_h, hz) = if let Some(edid) = edids.first() {
        (edid.h_active, edid.v_active, edid.refresh_rate_hz.round() as u32)
    } else if fname.contains("1920X1200") {
        (1920, 1200, 60)
    } else if fname.contains("144Hz") {
        (1920, 1080, 144)
    } else if fname.contains("120Hz") || fname.contains("120HZ") {
        (1920, 1080, 120)
    } else {
        (1920, 1080, 60)
    };

    let (pin_type, lane_config) = if fname.contains("30针") || fname.contains("2LANE") {
        ("30-pin (Compact eDP Connector)".to_string(), "eDP 2 Lanes".to_string())
    } else if hz >= 110 || fname.contains("4LANE") || fname.contains("144Hz") || fname.contains("120Hz") || fname.contains("120HZ") {
        ("40-pin (High-Refresh eDP Connector)".to_string(), "eDP 4 Lanes (Standard 120/144Hz)".to_string())
    } else {
        ("30-pin (Standard 60Hz eDP Connector)".to_string(), "eDP 2 Lanes (Standard 60Hz)".to_string())
    };

    let keypad_type = if fname.contains("4键") || fname.contains("四键") {
        "4 Buttons (Menu, +, -, Power)".to_string()
    } else {
        "5 Buttons (Menu, +, -, Exit, Power)".to_string()
    };

    let backlight_type = if fname.contains("固定100") || fname.contains("调节数字亮度") {
        "Digital Dimming (Hardware 100% fixed, digital attenuation)".to_string()
    } else {
        "Hardware PWM (Direct LED driver regulation)".to_string()
    };

    let boot_logo = if fname.contains("无LOGO") {
        "Fast Boot (Neutral initialization without OEM logo)".to_string()
    } else if fname.contains("LOGO") {
        "OEM Boot Logo (Bitmap on startup)".to_string()
    } else {
        "Neutral".to_string()
    };

    let mut factory_notes = Vec::new();
    if fname.contains("优化线损") {
        factory_notes.push("Line Loss Compensation: Boosted signal swing for longer cables".to_string());
    }
    if fname.contains("改善闪线") {
        factory_notes.push("Flicker Correction: Filtered line noise on high pixel clocks".to_string());
    }
    if fname.contains("按键反向") {
        factory_notes.push("Keypad Layout: Inverted +/- button order".to_string());
    }
    if fname.contains("限制到120HZ") {
        factory_notes.push("Refresh Rate Cap: 144Hz profile capped to 120Hz for standard cable tolerance".to_string());
    }
    if fname.contains("限制功率") {
        factory_notes.push("Power Limiter: Reduced backlight drive current".to_string());
    }

    let mut compatible_panels = Vec::new();
    if res_w == 1920 && res_h == 1080 {
        if hz >= 144 || fname.contains("144Hz") {
            compatible_panels.push("BOE NE173FHM-NZ6 (17.3\" 144Hz eDP 4-lane 40-pin)");
            compatible_panels.push("BOE NV173FHM-NX4 (17.3\" 144Hz eDP 4-lane 40-pin)");
            compatible_panels.push("AUO B173HAN04.9 (17.3\" 144Hz eDP 4-lane 40-pin)");
            compatible_panels.push("AUO B173HAN01.4 (17.3\" 144Hz eDP 4-lane 40-pin)");
            compatible_panels.push("Innolux N173HCE-G33 (17.3\" 144Hz eDP 4-lane 40-pin)");
            compatible_panels.push("LG LP156WFG-SPB2 (15.6\" 144Hz eDP 4-lane 40-pin)");
        } else if hz >= 110 || fname.contains("120Hz") || fname.contains("120HZ") {
            if fname.contains("30针") || fname.contains("2LANE") {
                compatible_panels.push("Innolux N156HHE-GA1 (15.6\" 120Hz 30-pin eDP 2-lane)");
                compatible_panels.push("AUO B156HAN13.0 (15.6\" 120Hz 30-pin eDP 2-lane)");
                compatible_panels.push("AUO B173HAN04.7 (17.3\" 120Hz 40-pin 2-lane HBR2)");
            } else {
                compatible_panels.push("AUO B173HAN01.1 HW1A (17.3\" 120Hz 40-pin eDP 4-lane)");
                compatible_panels.push("BOE NE173FHM-NZ6 (17.3\" capped 120Hz 40-pin)");
            }
        } else {
            compatible_panels.push("AUO B173HAN01.1 HW0A (17.3\" 60Hz 30-pin eDP 2-lane)");
            compatible_panels.push("Samsung LTN156HL07 (15.6\" 60Hz 30-pin eDP 2-lane)");
            compatible_panels.push("LG LP156WF6 / SPM1 (15.6\" 60Hz 30-pin eDP 2-lane)");
            compatible_panels.push("Panda LC133LF4L01 (13.3\" 60Hz 30-pin eDP 2-lane)");
            compatible_panels.push("Sharp LQ108M1JW01 (10.8\" 60Hz eDP)");
            compatible_panels.push("BOE NV156FHM-N42 (15.6\" 60Hz 30-pin eDP 2-lane)");
        }
    } else if res_w == 1920 && res_h == 1200 {
        compatible_panels.push("LG LP156WU1-SPB1 (15.6\" 1920x1200 16:10 60Hz)");
        compatible_panels.push("AUO B101UAN01 (10.1\" 1920x1200 16:10 60Hz)");
    }

    Ok(FirmwareAnalysis {
        file_path: path.to_path_buf(),
        file_size,
        num_banks,
        health,
        has_gamma_table,
        checksum,
        expected_checksum,
        checksum_match,
        edids,
        pin_type,
        lane_config,
        keypad_type,
        backlight_type,
        boot_logo,
        factory_notes,
        compatible_panels,
    })
}

fn print_report(res: &FirmwareAnalysis) {
    println!("\n================================================================================");
    println!("REALTEK RTD FIRMWARE ANALYSIS REPORT");
    println!("================================================================================");
    println!("File Path         : {}", res.file_path.display());
    println!("Image Size        : {} bytes ({} KB)", res.file_size, res.file_size / 1024);
    println!("64 KB Bank Count  : {} banks", res.num_banks);

    match &res.health {
        FirmwareHealth::ProductionReady832K => {
            println!("Binary Health     : [OK] Production Ready (832 KB - 13 banks standard)");
        }
        FirmwareHealth::PhysicalDump1024K => {
            println!("Binary Health     : [DUMP] Physical SPI Flash Dump (1024 KB - 16 banks with 0xFF padding)");
        }
        FirmwareHealth::IncompleteSingleBank64K => {
            println!("Binary Health     : [ERROR] INCOMPLETE SINGLE-BANK (64 KB) - FATAL BRICK RISK!");
        }
        FirmwareHealth::CustomSize(sz) => {
            println!("Binary Health     : [WARN] Non-standard image size ({} bytes)", sz);
        }
    }

    if let Some(exp) = res.expected_checksum {
        if res.checksum_match {
            println!("16-bit Checksum   : 0x{:04X} ([MATCH] Matches filename tag 0x{:04X})", res.checksum, exp);
        } else {
            println!("16-bit Checksum   : 0x{:04X} ([MISMATCH] Filename tag indicates 0x{:04X})", res.checksum, exp);
        }
    } else {
        println!("16-bit Checksum   : 0x{:04X} (No hex tag in filename)", res.checksum);
    }

    println!("16-bit Gamma LUT  : {}", if res.has_gamma_table { "[OK] Present in Bank 11 (0x0B0000)" } else { "[FAIL] Missing" });

    println!("\n--- [HARDWARE CONFIGURATION] ---");
    println!("Panel Connector   : {}", res.pin_type);
    println!("eDP Main-Link     : {}", res.lane_config);
    println!("Keypad Scheme     : {}", res.keypad_type);
    println!("Backlight Driver  : {}", res.backlight_type);
    println!("Boot Logo         : {}", res.boot_logo);

    if !res.factory_notes.is_empty() {
        println!("\n--- [OEM FACTORY FLAGS] ---");
        for n in &res.factory_notes {
            println!("  - {}", n);
        }
    }

    println!("\n--- [VESA EDID BLOCKS IN FLASH] ---");
    if res.edids.is_empty() {
        println!("  [WARN] No standard VESA EDID block detected.");
    } else {
        for (i, edid) in res.edids.iter().enumerate() {
            println!("  [EDID #{}] Offset 0x{:06X} (Bank {})", i + 1, edid.offset, edid.offset / 0x10000);
            println!("    • Display Name     : {}", edid.monitor_name);
            println!("    • Manufacturer ID  : {}", edid.mfg_id);
            println!("    • Native Resolution: {} x {} pixels", edid.h_active, edid.v_active);
            println!("    • Pixel Clock      : {:.2} MHz", edid.pixel_clock_mhz);
            println!("    • Refresh Rate     : {:.2} Hz", edid.refresh_rate_hz);
            println!("    • Timings H        : Active {} / Blank {} / Total {}", edid.h_active, edid.h_blank, edid.h_total);
            println!("    • Timings V        : Active {} / Blank {} / Total {}", edid.v_active, edid.v_blank, edid.v_total);
            println!("    • EDID Checksum    : {}", if edid.checksum_valid { "[OK] Valid (0x00)" } else { "[FAIL] Invalid" });
        }
    }

    if !res.compatible_panels.is_empty() {
        println!("\n--- [SUGGESTED / TESTED COMPATIBLE PANELS] ---");
        for p in &res.compatible_panels {
            println!("  - {}", p);
        }
    }
    println!("================================================================================\n");
}

fn collect_bins(dir: &Path, list: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                if !p.file_name().unwrap().to_string_lossy().starts_with('.') && !p.to_string_lossy().contains("RTDCustomerTool") && !p.to_string_lossy().contains("target") {
                    collect_bins(&p, list);
                }
            } else if p.extension().map_or(false, |ext| ext == "bin") {
                list.push(p);
            }
        }
    }
}

fn scan_directory(dir: &Path) {
    println!("\nScanning directory: {}\n", dir.display());
    println!("{:<45} | {:<8} | {:<12} | {:<10} | {:<18} | {:<12}", "File", "Checksum", "Resolution", "Frequency", "Panel Cabling", "Status");
    println!("{:-<120}", "");

    let mut bins = Vec::new();
    collect_bins(dir, &mut bins);

    let mut count = 0;
    for p in bins {
        if let Ok(res) = analyze_firmware(&p) {
            let fname = p.file_name().unwrap().to_string_lossy();
            let short_name = if fname.chars().count() > 43 {
                let s: String = fname.chars().take(40).collect();
                format!("{}...", s)
            } else {
                fname.to_string()
            };

            let (res_str, hz_str) = if let Some(edid) = res.edids.first() {
                (
                    format!("{}x{}", edid.h_active, edid.v_active),
                    format!("{:.0} Hz", edid.refresh_rate_hz),
                )
            } else {
                ("-".to_string(), "-".to_string())
            };

            let status_str = match res.health {
                FirmwareHealth::ProductionReady832K => "[OK] (832K)",
                FirmwareHealth::PhysicalDump1024K => "[DUMP] (1MB)",
                FirmwareHealth::IncompleteSingleBank64K => "[FAIL] (64K)",
                FirmwareHealth::CustomSize(_) => "[SPECIAL]",
            };

            let cable_short = if res.pin_type.contains("40") {
                "40 pins (4L)"
            } else {
                "30 pins (2L)"
            };

            println!("{:<45} | 0x{:04X} | {:<12} | {:<10} | {:<18} | {:<12}", short_name, res.checksum, res_str, hz_str, cable_short, status_str);
            count += 1;
        }
    }
    println!("{:-<120}", "");
    println!("Total firmwares analyzed: {}\n", count);
}

fn find_matching_panels(search: &str, dir: &Path) {
    println!("\nSearching compatible firmwares for: '{}'...\n", search);
    let mut bins = Vec::new();
    collect_bins(dir, &mut bins);

    let search_upper = search.to_uppercase();
    let mut matches = Vec::new();

    for p in bins {
        if let Ok(res) = analyze_firmware(&p) {
            let fname_upper = p.file_name().unwrap().to_string_lossy().to_uppercase();
            let matches_fname = fname_upper.contains(&search_upper);
            let matches_panel_db = res.compatible_panels.iter().any(|p| p.to_uppercase().contains(&search_upper));

            if matches_fname || matches_panel_db {
                matches.push((p, res, matches_fname));
            }
        }
    }

    if matches.is_empty() {
        println!("[!] No firmware found specifically matching '{}'.", search);
        println!("Tip: Try searching by specification keywords (e.g., '144Hz', '120Hz', 'BOE', 'AUO', '40-pin').\n");
    } else {
        println!("[OK] Found {} compatible firmware(s):\n", matches.len());
        for (i, (path, res, direct_match)) in matches.iter().enumerate() {
            println!("{}. {}", i + 1, path.file_name().unwrap().to_string_lossy());
            println!("   • Path         : {}", path.display());
            println!("   • Health       : {}", match res.health {
                FirmwareHealth::ProductionReady832K => "[OK] Production Ready (832 KB)",
                FirmwareHealth::PhysicalDump1024K => "[DUMP] Physical Dump (1024 KB)",
                FirmwareHealth::IncompleteSingleBank64K => "[ERROR] INCOMPLETE (64 KB - Do not flash)",
                FirmwareHealth::CustomSize(_) => "[WARN] Non-standard image size",
            });
            println!("   • Match Type   : {}", if *direct_match { "Direct profile tag match" } else { "Compatible with panel specifications" });
            println!("   • Interface    : {} ({})", res.pin_type, res.lane_config);
            println!("   • Checksum     : 0x{:04X}", res.checksum);
            println!();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("================================================================================");
        println!("RTD Firmware Analyzer (rtd-cli)");
        println!("Command-line inspection & compatibility tool for Realtek RTD eDP controller boards");
        println!("================================================================================");
        println!("Usage:");
        println!("  rtd-cli <file.bin>               : Full detailed firmware analysis");
        println!("  rtd-cli --info <file.bin>        : Full detailed firmware analysis");
        println!("  rtd-cli --scan [directory]       : Summary table with health status");
        println!("  rtd-cli --find <panel_reference> : Search compatible firmwares for a panel");
        println!("\nExamples:");
        println!("  rtd-cli --find NV173FHM-NX4");
        println!("  rtd-cli --find B173HAN01");
        println!("  rtd-cli --scan .");
        return;
    }

    if args[1] == "--scan" {
        let target_dir = if args.len() >= 3 { Path::new(&args[2]) } else { Path::new(".") };
        scan_directory(target_dir);
    } else if args[1] == "--find" {
        if args.len() < 3 {
            eprintln!("Error: please specify a panel reference (e.g.: rtd-cli --find NV173FHM-NX4)");
            return;
        }
        let search = &args[2];
        let target_dir = if args.len() >= 4 { Path::new(&args[3]) } else { Path::new(".") };
        find_matching_panels(search, target_dir);
    } else if args[1] == "--info" {
        if args.len() < 3 {
            eprintln!("Error: please specify a .bin file");
            return;
        }
        let target_path = Path::new(&args[2]);
        match analyze_firmware(target_path) {
            Ok(report) => print_report(&report),
            Err(e) => eprintln!("Analysis error: {}", e),
        }
    } else {
        let target_path = Path::new(&args[1]);
        match analyze_firmware(target_path) {
            Ok(report) => print_report(&report),
            Err(e) => eprintln!("Analysis error: {}", e),
        }
    }
}

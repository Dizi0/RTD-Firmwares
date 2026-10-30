#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::fs;
use std::path::Path;

// Embedded factory golden templates (JG2555TB RTD2556)
const TEMPLATE_144HZ_4L: &[u8] = include_bytes!("../templates/template_144hz_4l.bin");
const TEMPLATE_120HZ_4L: &[u8] = include_bytes!("../templates/template_120hz_4l.bin");
const TEMPLATE_120HZ_2L: &[u8] = include_bytes!("../templates/template_120hz_2l.bin");
const TEMPLATE_60HZ_2L: &[u8] = include_bytes!("../templates/template_60hz_2l.bin");
const TEMPLATE_1200P_2L: &[u8] = include_bytes!("../templates/template_1200p_2l.bin");

fn open_url(url: &str) {
    let _ = std::process::Command::new("rundll32")
        .args(["url.dll,FileProtocolHandler", url])
        .spawn();
}

fn panel_link_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let resp = ui.button(format!("{} ↗", text));
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn open_panelook_search(keyword: &str) {
    let clean = keyword.trim().replace(' ', "+");
    let url = format!("https://www.panelook.com/modelsearch.php?keyword={}", clean);
    open_url(&url);
}

#[derive(Clone, Debug)]
struct PanelSpec {
    model: &'static str,
    brand: &'static str,
    size: &'static str,
    resolution: (u32, u32),
    refresh_rate: u32,
    pins: u32,
    lanes: &'static str,
    pixel_clock_mhz: f64,
    description: &'static str,
    template_tag: &'static str,
}

const KNOWN_PANELS: &[PanelSpec] = &[
    PanelSpec {
        model: "NV173FHM-NX4",
        brand: "BOE",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 144,
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "17.3\" 144Hz FHD IPS/ADS e-Sport Panel - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "N173HCE-G33",
        brand: "Innolux",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 144,
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "17.3\" 144Hz FHD AAS/IPS Display (72% NTSC Color Gamut) - 40-pin 4-Lane",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "B173HAN04.9",
        brand: "AUO",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 144,
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "17.3\" 144Hz FHD AHVA/IPS Gaming Panel - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "NE173FHM-NZ6",
        brand: "BOE",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 144,
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "17.3\" 144Hz FHD IPS Gaming Display - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "B173HAN04.7",
        brand: "AUO",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 120,
        pins: 40,
        lanes: "2 Lanes (HBR2 5.4 Gbps)",
        pixel_clock_mhz: 284.0,
        description: "17.3\" 120Hz/144Hz AHVA/IPS - 40-pin physical connector but 2-Lane HBR2 wiring",
        template_tag: "120Hz_30PIN",
    },
    PanelSpec {
        model: "B173HAN01.1 (HW1A - 120Hz)",
        brand: "AUO",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 120,
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 284.0,
        description: "17.3\" 120Hz FHD AHVA/IPS Panel - 40-pin 4-Lane eDP",
        template_tag: "120Hz_4LANE",
    },
    PanelSpec {
        model: "B173HAN01.1 (HW0A - 60Hz)",
        brand: "AUO",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "17.3\" 60Hz Standard Office IPS Panel - 30-pin 2-Lane eDP",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "LTN156HL07",
        brand: "Samsung",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "15.6\" FHD 60Hz IPS/PLS Display (Samsung) - 30-pin 2-Lane",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "N156HHE-GA1",
        brand: "Innolux",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 120,
        pins: 30,
        lanes: "2 Lanes (HBR2 5.4 Gbps)",
        pixel_clock_mhz: 284.0,
        description: "15.6\" 120Hz High-Speed eDP Display - 30-pin 2-Lane HBR2",
        template_tag: "120Hz_30PIN",
    },
    PanelSpec {
        model: "LP156WF6 / SPM1",
        brand: "LG Display",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "15.6\" FHD 60Hz IPS Display (LG Display) - 30-pin 2-Lane",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "LP156WU1-SPB1",
        brand: "LG Display",
        size: "15.6\"",
        resolution: (1920, 1200),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 154.0,
        description: "15.6\" 16:10 WUXGA (1920x1200) IPS Panel - 30-pin 2-Lane",
        template_tag: "1920x1200_60Hz",
    },
    PanelSpec {
        model: "LC133LF4L01",
        brand: "Panda",
        size: "13.3\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "13.3\" Compact FHD IPS Panel for Portable Monitors - 30-pin",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "TV133QHM-AW0",
        brand: "BOE",
        size: "13.3\"",
        resolution: (2560, 1440),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (HBR2 5.4 Gbps)",
        pixel_clock_mhz: 241.5,
        description: "13.3\" 2K QHD (2560x1440) Ultra-Clear IPS Panel - 30-pin 2-Lane eDP",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "N156KME-GNA",
        brand: "Innolux",
        size: "15.6\"",
        resolution: (2560, 1440),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (HBR2 5.4 Gbps)",
        pixel_clock_mhz: 241.5,
        description: "15.6\" 2K QHD High-DPI Matte IPS Panel - 30-pin 2-Lane eDP",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "N133GCA-GQ1",
        brand: "Innolux",
        size: "13.3\"",
        resolution: (2560, 1600),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (HBR2 5.4 Gbps)",
        pixel_clock_mhz: 268.0,
        description: "13.3\" 16:10 2.5K (2560x1600) Retina IPS Display - 30-pin 2-Lane eDP",
        template_tag: "1920x1200_60Hz",
    },
    PanelSpec {
        model: "B101UAN01",
        brand: "AUO",
        size: "10.1\"",
        resolution: (1920, 1200),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 154.0,
        description: "10.1\" 16:10 WUXGA (1920x1200) Tablet & Compact Display - 30-pin 2-Lane eDP",
        template_tag: "1920x1200_60Hz",
    },
    PanelSpec {
        model: "LQ108M1JW01",
        brand: "Sharp",
        size: "10.8\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "10.8\" FHD IGZO Low-Power Display (Dell Venue / Compact) - 30-pin eDP",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "B156HAN13.0",
        brand: "AUO",
        size: "15.6\"",
        refresh_rate: 144,
        resolution: (1920, 1080),
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "15.6\" 144Hz FHD High-Refresh AHVA Gaming Panel - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "NV156FHM-NX1",
        brand: "BOE",
        size: "15.6\"",
        refresh_rate: 144,
        resolution: (1920, 1080),
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "15.6\" 120Hz/144Hz FHD IPS High-Refresh Gaming Panel (NX1) - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "NV156FHM-NX2",
        brand: "BOE",
        size: "15.6\"",
        refresh_rate: 144,
        resolution: (1920, 1080),
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "15.6\" 144Hz FHD IPS High-Refresh Gaming Panel (NX2) - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "NV156FHM-N4U",
        brand: "BOE",
        size: "15.6\"",
        refresh_rate: 144,
        resolution: (1920, 1080),
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "15.6\" 144Hz FHD IPS 300nit High-Speed Display (N4U) - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "LM156LFGL01",
        brand: "Panda",
        size: "15.6\"",
        refresh_rate: 144,
        resolution: (1920, 1080),
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "15.6\" 120Hz/144Hz FHD FFS Gaming Panel (LM156LFGL01, GL02) - 40-pin 4-Lane",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "LM156LFGL03",
        brand: "Panda",
        size: "15.6\"",
        refresh_rate: 144,
        resolution: (1920, 1080),
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "15.6\" 144Hz FHD FFS Gaming Display (LM156LFGL03, GL05) - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "N156HRA-GAA",
        brand: "Innolux",
        size: "15.6\"",
        refresh_rate: 144,
        resolution: (1920, 1080),
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "15.6\" 144Hz FHD AAS Gaming Display (N156HRA-GAA) - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "LP156WFG-SPB2",
        brand: "LG Display",
        size: "15.6\"",
        refresh_rate: 144,
        resolution: (1920, 1080),
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "15.6\" 144Hz FHD IPS High-Refresh Gaming Display - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "NV173FHM-N44",
        brand: "BOE",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 144,
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "17.3\" 144Hz FHD IPS 300nit 1200:1 Gaming Display - 40-pin 4-Lane",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "NV173FHM-N41",
        brand: "BOE",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "17.3\" 60Hz FHD IPS Matte Standard Display (inc. V8.0, V8.1) - 30-pin 2-Lane",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "NV173FHM-N49",
        brand: "BOE",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "17.3\" 60Hz FHD IPS 250nit Display - 30-pin 2-Lane eDP",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "NV173FHM-N4C",
        brand: "BOE",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "17.3\" 60Hz FHD IPS 100% sRGB Color Accurate Panel - 30-pin 2-Lane",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "NV156FHM-N42",
        brand: "BOE",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "15.6\" 60Hz FHD IPS Standard Laptop Panel (N41, N42, N43) - 30-pin",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "NV156FHM-NY1",
        brand: "BOE",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 144,
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "15.6\" 144Hz FHD IPS High-Refresh Gaming Panel - 40-pin 4-Lane",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "B173HAN03.0",
        brand: "AUO",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 144,
        pins: 40,
        lanes: "4 Lanes (HBR 2.7 Gbps)",
        pixel_clock_mhz: 326.7,
        description: "17.3\" 144Hz FHD AHVA 300nit Gaming Display - 40-pin 4-Lane eDP",
        template_tag: "144Hz_4LANE",
    },
    PanelSpec {
        model: "B173HAN04.5",
        brand: "AUO",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 144,
        pins: 40,
        lanes: "2 Lanes (HBR2 5.4 Gbps)",
        pixel_clock_mhz: 284.0,
        description: "17.3\" 144Hz AHVA - 40-pin physical connector 2-Lane HBR2 wiring",
        template_tag: "120Hz_30PIN",
    },
    PanelSpec {
        model: "B156HAN04.5",
        brand: "AUO",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 120,
        pins: 40,
        lanes: "2 Lanes (HBR2 5.4 Gbps)",
        pixel_clock_mhz: 284.0,
        description: "15.6\" 120Hz FHD AHVA High-Speed Display - 40-pin 2-Lane HBR2",
        template_tag: "120Hz_30PIN",
    },
    PanelSpec {
        model: "B156HAN02.1",
        brand: "AUO",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "15.6\" 60Hz FHD AHVA/IPS Standard Panel (02.0 to 02.4) - 30-pin 2-Lane",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "LP156WF6-SPB1",
        brand: "LG Display",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "15.6\" FHD IPS 300nit 700:1 (SPA1, SPB1, SPB2, SPK1, SPM1) - 30-pin",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "LP156WFC-SPDB",
        brand: "LG Display",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "15.6\" FHD IPS Slim Borderless Display (SPDA, SPDB, SPM1) - 30-pin",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "LP140WF6-SPB1",
        brand: "LG Display",
        size: "14.0\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "14.0\" FHD IPS Ultrabook Display (SPB1, SPB2, SPD1) - 30-pin 2-Lane",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "N140HCE-EN1",
        brand: "Innolux",
        size: "14.0\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "14.0\" FHD AAS 100% sRGB High-Gamut Display (EN1, EN2) - 30-pin",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "N173HCE-E31",
        brand: "Innolux",
        size: "17.3\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "17.3\" 60Hz FHD AAS IPS Display - 30-pin 2-Lane eDP",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "N156HCA-EAB",
        brand: "Innolux",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "15.6\" FHD IPS Slim Narrow-Bezel Display (EAA, EAB, EA3) - 30-pin",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "LTN156HL01",
        brand: "Samsung",
        size: "15.6\"",
        resolution: (1920, 1080),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 148.5,
        description: "15.6\" FHD PLS Display (HL01, HL02, HL06, HL09) - 30-pin 2-Lane",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "LTN140AT30",
        brand: "Samsung",
        size: "14.0\"",
        resolution: (1366, 768),
        refresh_rate: 60,
        pins: 30,
        lanes: "1 Lane (Standard 60Hz)",
        pixel_clock_mhz: 75.4,
        description: "14.0\" HD (1366x768) Standard eDP Laptop Panel - 30-pin",
        template_tag: "60Hz_30PIN",
    },
    PanelSpec {
        model: "LP097QX2",
        brand: "LG Display",
        size: "9.7\"",
        resolution: (1536, 2048),
        refresh_rate: 60,
        pins: 30,
        lanes: "2 Lanes (Standard 60Hz)",
        pixel_clock_mhz: 200.0,
        description: "9.7\" 2048x1536 Retina Ultra-High Density Display (iPad 3/4/5) - 30-pin 2-Lane eDP",
        template_tag: "60Hz_30PIN",
    },
];

const LOGO_RESOLUTIONS: &[(u32, u32, &str)] = &[
    (128, 64, "128x64 (Compact OSD 8x4 Tiles - 1024 B)"),
    (160, 80, "160x80 (Medium OSD 10x5 Tiles - 1600 B)"),
    (192, 64, "192x64 (Wide Banner 12x4 Tiles - 1536 B)"),
    (240, 80, "240x80 (Large Ultra-Wide - 2400 B)"),
];

#[derive(Clone)]
struct ProcessedLogo {
    width: u32,
    height: u32,
    color_image: egui::ColorImage,
    packed_1bit: Vec<u8>,
}

fn process_logo_image(
    raw_bytes: &[u8],
    target_w: u32,
    target_h: u32,
    threshold: u8,
    dither: bool,
    invert: bool,
) -> Result<ProcessedLogo, String> {
    let img = image::load_from_memory(raw_bytes)
        .map_err(|e| format!("Image decode error: {}", e))?;

    let (src_w, src_h) = (img.width(), img.height());
    if src_w == 0 || src_h == 0 {
        return Err("Image has zero dimensions".into());
    }

    let scale = (target_w as f32 / src_w as f32).min(target_h as f32 / src_h as f32);
    let fit_w = ((src_w as f32 * scale).round() as u32).clamp(1, target_w);
    let fit_h = ((src_h as f32 * scale).round() as u32).clamp(1, target_h);

    let resized = img.resize_exact(fit_w, fit_h, image::imageops::FilterType::Lanczos3);
    let luma_img = resized.to_luma8();

    let mut canvas = vec![0.0f32; (target_w * target_h) as usize];
    let offset_x = (target_w - fit_w) / 2;
    let offset_y = (target_h - fit_h) / 2;

    for y in 0..fit_h {
        for x in 0..fit_w {
            let p = luma_img.get_pixel(x, y)[0] as f32;
            let cx = offset_x + x;
            let cy = offset_y + y;
            canvas[(cy * target_w + cx) as usize] = p;
        }
    }

    let mut binary_pixels = vec![false; (target_w * target_h) as usize];
    if dither {
        for y in 0..target_h {
            for x in 0..target_w {
                let idx = (y * target_w + x) as usize;
                let old_val = canvas[idx];
                let is_white = old_val >= threshold as f32;
                let new_val = if is_white { 255.0 } else { 0.0 };
                let err = old_val - new_val;
                binary_pixels[idx] = if invert { !is_white } else { is_white };

                if x + 1 < target_w {
                    canvas[(y * target_w + x + 1) as usize] += err * (7.0 / 16.0);
                }
                if y + 1 < target_h {
                    if x > 0 {
                        canvas[((y + 1) * target_w + x - 1) as usize] += err * (3.0 / 16.0);
                    }
                    canvas[((y + 1) * target_w + x) as usize] += err * (5.0 / 16.0);
                    if x + 1 < target_w {
                        canvas[((y + 1) * target_w + x + 1) as usize] += err * (1.0 / 16.0);
                    }
                }
            }
        }
    } else {
        for i in 0..(target_w * target_h) as usize {
            let is_white = canvas[i] >= threshold as f32;
            binary_pixels[i] = if invert { !is_white } else { is_white };
        }
    }

    let mut rgba = Vec::with_capacity((target_w * target_h * 4) as usize);
    for &is_white in &binary_pixels {
        let (r, g, b, a) = if is_white {
            (240, 240, 245, 255)
        } else {
            (14, 16, 20, 255)
        };
        rgba.push(r);
        rgba.push(g);
        rgba.push(b);
        rgba.push(a);
    }
    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [target_w as usize, target_h as usize],
        &rgba,
    );

    let total_pixels = (target_w * target_h) as usize;
    let byte_count = (total_pixels + 7) / 8;
    let mut packed_1bit = vec![0u8; byte_count];
    for (i, &is_white) in binary_pixels.iter().enumerate() {
        if is_white {
            let byte_idx = i / 8;
            let bit_pos = 7 - (i % 8);
            packed_1bit[byte_idx] |= 1 << bit_pos;
        }
    }

    Ok(ProcessedLogo {
        width: target_w,
        height: target_h,
        color_image,
        packed_1bit,
    })
}

#[derive(Clone, Default)]
struct InspectorData {
    file_path: std::path::PathBuf,
    raw_data: Vec<u8>,
    edid_offsets: Vec<usize>,
    filename: String,
    filesize: usize,
    banks: usize,
    checksum: u16,
    checksum_name: Option<u16>,
    checksum_match: bool,
    monitor_name: String,
    manufacturer: String,
    edid_checksum_valid: bool,
    gamma_table_present: bool,
    resolution: String,
    pixel_clock: String,
    refresh_rate: String,
    cable_type: String,
    keypad_type: String,
    backlight_type: String,
    status: String,
    notes: Vec<String>,
}

struct RtdStudioApp {
    selected_tab: usize,

    // Composer State
    generator_mode: usize, // 0 = Preset Catalogue, 1 = Manual Datasheet Input
    selected_panel_idx: usize,
    custom_screen_name: String,
    export_format: usize, // 0 = 832 KB (Standard HDMI ISP Flash), 1 = 1024 KB (Full SPI Flash Dump)
    composer_status: Option<(String, bool)>,

    // Datasheet Inputs:
    ds_brand: String,
    ds_model: String,
    ds_res_w: String,
    ds_res_h: String,
    ds_refresh_hz: String,
    ds_lane_choice: usize,
    ds_pixel_clock: String,

    // Inspector State
    inspector_data: Option<InspectorData>,
    inspector_edit_name: String,
    inspector_status: Option<(String, bool)>,

    // Search
    panelook_search_query: String,

    // Boot Logo State
    logo_mode: usize, // 0 = Fast Boot (No Logo), 1 = Factory OEM Logo, 2 = Custom Logo
    logo_file_path: Option<std::path::PathBuf>,
    logo_raw_bytes: Option<Vec<u8>>,
    logo_target_res_idx: usize,
    logo_threshold: u8,
    logo_dither: bool,
    logo_invert: bool,
    logo_processed: Option<ProcessedLogo>,
    logo_texture: Option<egui::TextureHandle>,
    logo_status: Option<(String, bool)>,

    // Factory OSD Defaults & Presets
    enable_custom_presets: bool,
    preset_category: usize, // 0=Language & System, 1=Picture & Aspect, 2=Color & Temps, 3=Levels & Audio, 4=Gaming & VRR
    preset_language: usize, // 0=English, 1=French, 2=German, 3=Spanish, 4=Simplified Chinese, 5=Traditional Chinese, 6=Japanese, 7=Korean, 8=Russian
    preset_picture_mode: usize, // 0=Standard, 1=FPS, 2=RTS, 3=Movie, 4=Text/Office, 5=ECO
    preset_aspect_ratio: usize, // 0=16:9 Full, 1=4:3 Pillarbox, 2=Auto 1:1
    preset_color_temp: usize, // 0=Warm 6500K, 1=Neutral 7500K, 2=Cool 9300K, 3=sRGB, 4=User
    preset_user_r: u8, // User Custom RGB: Red Gain (0-100%)
    preset_user_g: u8, // User Custom RGB: Green Gain (0-100%)
    preset_user_b: u8, // User Custom RGB: Blue Gain (0-100%)
    preset_brightness: u8,
    preset_contrast: u8,
    preset_volume: u8,
    preset_mute: bool,
    preset_direct_power: bool,
    preset_auto_input: usize, // 0=Auto Detect, 1=HDMI 1, 2=HDMI 2
    preset_dcr: bool,
    preset_freesync: bool,
    preset_overdrive: usize, // 0=Off, 1=Medium, 2=High
    preset_low_blue: usize,  // 0=Off, 1=25%, 2=50%, 3=75%, 4=100%
}

impl Default for RtdStudioApp {
    fn default() -> Self {
        Self {
            selected_tab: 0,
            generator_mode: 0,
            selected_panel_idx: 0,
            custom_screen_name: KNOWN_PANELS[0].model.to_string(),
            export_format: 0,
            composer_status: None,
            ds_brand: String::new(),
            ds_model: String::new(),
            ds_res_w: String::new(),
            ds_res_h: String::new(),
            ds_refresh_hz: String::new(),
            ds_lane_choice: 0,
            ds_pixel_clock: String::new(),
            inspector_data: None,
            inspector_edit_name: String::new(),
            inspector_status: None,
            panelook_search_query: String::new(),
            logo_mode: 0, // Default to Fast Boot (No Logo) for clean instant turn-on
            logo_file_path: None,
            logo_raw_bytes: None,
            logo_target_res_idx: 0,
            logo_threshold: 128,
            logo_dither: true,
            logo_invert: false,
            logo_processed: None,
            logo_texture: None,
            logo_status: None,
            enable_custom_presets: false,
            preset_category: 0,
            preset_language: 0, // English
            preset_picture_mode: 0, // Standard
            preset_aspect_ratio: 0, // 16:9 Full
            preset_color_temp: 0, // Warm 6500K
            preset_user_r: 50,
            preset_user_g: 50,
            preset_user_b: 50,
            preset_brightness: 80,
            preset_contrast: 50,
            preset_volume: 70,
            preset_mute: false,
            preset_direct_power: true,
            preset_auto_input: 0, // Auto Detect
            preset_dcr: false,
            preset_freesync: true,
            preset_overdrive: 1, // Medium
            preset_low_blue: 0, // Off
        }
    }
}

fn patch_edid_block(
    edid: &mut [u8],
    brand: &str,
    name: &str,
    pclk_10khz: u32,
    h_act: u32,
    v_act: u32,
    h_blank: u32,
    v_blank: u32,
) {
    if edid.len() < 128 {
        return;
    }

    // 1. Manufacturer PNP ID at bytes 8-9 (3 uppercase ASCII chars packed into 15 bits)
    let mfg_bytes = brand.as_bytes();
    let c1 = if !mfg_bytes.is_empty() { (mfg_bytes[0].to_ascii_uppercase() as i16 - 64).clamp(1, 26) as u8 } else { 18 };
    let c2 = if mfg_bytes.len() > 1 { (mfg_bytes[1].to_ascii_uppercase() as i16 - 64).clamp(1, 26) as u8 } else { 20 };
    let c3 = if mfg_bytes.len() > 2 { (mfg_bytes[2].to_ascii_uppercase() as i16 - 64).clamp(1, 26) as u8 } else { 11 };
    edid[8] = (c1 << 2) | ((c2 >> 3) & 0x03);
    edid[9] = ((c2 & 0x07) << 5) | (c3 & 0x1f);

    // 2. Patch Detailed Timings (Find descriptor with highest pclk)
    let pclk1 = (edid[54] as u16) | ((edid[55] as u16) << 8);
    let pclk2 = (edid[72] as u16) | ((edid[73] as u16) << 8);
    let target_offset = if pclk2 > pclk1 { 72 } else { 54 };

    edid[target_offset] = (pclk_10khz & 0xff) as u8;
    edid[target_offset + 1] = ((pclk_10khz >> 8) & 0xff) as u8;
    edid[target_offset + 2] = (h_act & 0xff) as u8;
    edid[target_offset + 3] = (h_blank & 0xff) as u8;
    edid[target_offset + 4] = (((h_act >> 8) & 0x0f) << 4) as u8 | (((h_blank >> 8) & 0x0f) as u8);
    edid[target_offset + 5] = (v_act & 0xff) as u8;
    edid[target_offset + 6] = (v_blank & 0xff) as u8;
    edid[target_offset + 7] = (((v_act >> 8) & 0x0f) << 4) as u8 | (((v_blank >> 8) & 0x0f) as u8);

    // 3. Patch Monitor Name Descriptor (Tag 0x00 0x00 0x00 0xFC 0x00)
    let mut name_offset = None;
    for &desc_off in &[54, 72, 90, 108] {
        if desc_off + 5 <= edid.len() && &edid[desc_off..desc_off + 5] == &[0x00, 0x00, 0x00, 0xfc, 0x00] {
            name_offset = Some(desc_off + 5);
            break;
        }
    }
    let final_name_off = name_offset.unwrap_or_else(|| {
        edid[90..95].copy_from_slice(&[0x00, 0x00, 0x00, 0xfc, 0x00]);
        95
    });

    let clean_name = name.trim();
    let name_bytes = clean_name.as_bytes();
    for i in 0..13 {
        if i < name_bytes.len() {
            edid[final_name_off + i] = name_bytes[i];
        } else if i == name_bytes.len() {
            edid[final_name_off + i] = 0x0a;
        } else {
            edid[final_name_off + i] = 0x20;
        }
    }

    // 4. Recalculate EDID Checksum at byte 127
    let sum: u8 = edid[..127].iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    edid[127] = (0x100u16.wrapping_sub(sum as u16) & 0xff) as u8;
}

impl RtdStudioApp {
    fn inspect_file(&mut self, path: &Path) {
        if let Ok(data) = fs::read(path) {
            let fname = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let filesize = data.len();
            let banks = filesize / 65536;
            let checksum = data.iter().fold(0u16, |acc, &b| acc.wrapping_add(b as u16));

            let mut checksum_name = None;
            for part in fname.split(|c: char| !c.is_alphanumeric()) {
                if part.len() == 4 && part.chars().all(|c| c.is_ascii_hexdigit()) {
                    if let Ok(val) = u16::from_str_radix(part, 16) {
                        if val == checksum {
                            checksum_name = Some(val);
                            break;
                        }
                    }
                }
            }
            let checksum_match = checksum_name.is_some();

            let status = match filesize {
                851968 => "Valid Production Firmware (832 KB, 13 banks)".to_string(),
                1048576 => "Physical SPI Flash Dump (1024 KB, 16 banks)".to_string(),
                65536 => "Incomplete single-bank file (64 KB) - DO NOT FLASH".to_string(),
                s => format!("Non-standard image size ({} bytes)", s),
            };

            let header = [0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00];
            let mut resolution = "-".to_string();
            let mut pixel_clock = "-".to_string();
            let mut refresh_rate = "-".to_string();
            let mut monitor_name = "Generic / Unspecified".to_string();
            let mut manufacturer = "-".to_string();
            let mut edid_checksum_valid = false;

            if let Some(pos) = data.windows(8).position(|w| w == header) {
                if pos + 128 <= data.len() {
                    let ed = &data[pos..pos + 128];

                    // Checksum
                    let sum: u8 = ed.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
                    edid_checksum_valid = sum == 0;

                    // Manufacturer 3-letter PNP code
                    let b8 = ed[8];
                    let b9 = ed[9];
                    let c1 = ((b8 & 0x7c) >> 2) + 64;
                    let c2 = (((b8 & 0x03) << 3) | ((b9 & 0xe0) >> 5)) + 64;
                    let c3 = (b9 & 0x1f) + 64;
                    manufacturer = format!("{}{}{}", c1 as char, c2 as char, c3 as char);

                    // Scan for 0xFC Monitor Name descriptor
                    for &desc_off in &[54, 72, 90, 108] {
                        if desc_off + 18 <= ed.len() && &ed[desc_off..desc_off + 5] == &[0x00, 0x00, 0x00, 0xfc, 0x00] {
                            let name_bytes = &ed[desc_off + 5..desc_off + 18];
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

                    // Native timing: check descriptor 1 (54) vs descriptor 2 (72)
                    let pclk1 = (ed[54] as u32) | ((ed[55] as u32) << 8);
                    let pclk2 = (ed[72] as u32) | ((ed[73] as u32) << 8);
                    let (target_off, pclk_raw) = if pclk2 > pclk1 { (72, pclk2) } else { (54, pclk1) };

                    let pclk_mhz = (pclk_raw as f64) * 10.0 / 1000.0;
                    let h_act = (ed[target_off + 2] as u32) | (((ed[target_off + 4] as u32) & 0xf0) << 4);
                    let h_blk = (ed[target_off + 3] as u32) | (((ed[target_off + 4] as u32) & 0x0f) << 8);
                    let v_act = (ed[target_off + 5] as u32) | (((ed[target_off + 7] as u32) & 0xf0) << 4);
                    let v_blk = (ed[target_off + 6] as u32) | (((ed[target_off + 7] as u32) & 0x0f) << 8);
                    let ht = h_act + h_blk;
                    let vt = v_act + v_blk;
                    if ht > 0 && vt > 0 && pclk_raw > 0 {
                        let hz = (pclk_raw as f64 * 10000.0) / (ht as f64 * vt as f64);
                        resolution = format!("{} x {} pixels", h_act, v_act);
                        pixel_clock = format!("{:.2} MHz", pclk_mhz);
                        refresh_rate = format!("{:.2} Hz", hz);
                    }
                }
            }

            let gamma_table_present = if filesize >= 0x0B0000 + 1024 {
                data[0x0B0000..0x0B0100].iter().any(|&b| b != 0x00 && b != 0xff)
            } else {
                false
            };

            let cable_type = if fname.contains("40 broches") || fname.contains("4LANE") || fname.contains("144Hz") {
                "40-pin (4 Lanes eDP High-Refresh)".to_string()
            } else if fname.contains("30针") && fname.contains("120Hz") {
                "30-pin (2 Lanes eDP HBR2 High-Speed)".to_string()
            } else {
                "30-pin (2 Lanes eDP Standard 60Hz)".to_string()
            };

            let keypad_type = if fname.contains("4键") || fname.contains("四键") {
                "4 Buttons (Menu, +, -, Power)".to_string()
            } else {
                "5 Buttons (Menu, +, -, Exit, Power)".to_string()
            };

            let backlight_type = if fname.contains("固定100") || fname.contains("调节数字亮度") {
                "Digital Dimming (Hardware 100% fixed, digital attenuation)".to_string()
            } else {
                "Hardware PWM (Direct LED driver voltage/frequency regulation)".to_string()
            };

            let mut notes = Vec::new();
            if fname.contains("优化线损") { notes.push("Line Loss Compensation: Boosted signal swing for longer cables".to_string()); }
            if fname.contains("改善闪线") { notes.push("Flicker Correction: Filtered line noise on high pixel clocks".to_string()); }
            if fname.contains("按键反向") { notes.push("Keypad Layout: Inverted +/- button order".to_string()); }
            if fname.contains("无LOGO") { notes.push("Fast Boot: Neutral initialization without OEM logo".to_string()); }

            let mut edid_offsets = Vec::new();
            let mut offset = 0;
            while offset + 128 <= data.len() {
                if &data[offset..offset + 8] == &header {
                    edid_offsets.push(offset);
                    offset += 128;
                } else {
                    offset += 1;
                }
            }

            self.inspector_edit_name = monitor_name.clone();
            self.inspector_status = None;

            self.inspector_data = Some(InspectorData {
                file_path: path.to_path_buf(),
                raw_data: data,
                edid_offsets,
                filename: fname,
                filesize,
                banks,
                checksum,
                checksum_name,
                checksum_match,
                monitor_name,
                manufacturer,
                edid_checksum_valid,
                gamma_table_present,
                resolution,
                pixel_clock,
                refresh_rate,
                cable_type,
                keypad_type,
                backlight_type,
                status,
                notes,
            });
        }
    }

    fn save_inspector_modified_firmware(&mut self) {
        let (mut data, offsets, edit_name, old_path) = match &self.inspector_data {
            Some(d) => (d.raw_data.clone(), d.edid_offsets.clone(), self.inspector_edit_name.clone(), d.file_path.clone()),
            None => return,
        };

        if offsets.is_empty() {
            self.inspector_status = Some(("No VESA EDID header found in this binary to patch.".to_string(), false));
            return;
        }

        let clean_name = edit_name.trim();
        let name_bytes = clean_name.as_bytes();

        for &off in &offsets {
            if off + 128 > data.len() {
                continue;
            }

            // Find or insert 0xFC descriptor
            let mut name_offset = None;
            for &desc_off in &[54, 72, 90, 108] {
                let abs_off = off + desc_off;
                if abs_off + 5 <= off + 128 && &data[abs_off..abs_off + 5] == &[0x00, 0x00, 0x00, 0xfc, 0x00] {
                    name_offset = Some(abs_off + 5);
                    break;
                }
            }
            let final_name_off = name_offset.unwrap_or_else(|| {
                data[off + 90..off + 95].copy_from_slice(&[0x00, 0x00, 0x00, 0xfc, 0x00]);
                off + 95
            });

            for i in 0..13 {
                if i < name_bytes.len() {
                    data[final_name_off + i] = name_bytes[i];
                } else if i == name_bytes.len() {
                    data[final_name_off + i] = 0x0a;
                } else {
                    data[final_name_off + i] = 0x20;
                }
            }

            // Recalculate checksum at byte 127
            let sum: u8 = data[off..off + 127].iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
            data[off + 127] = (0x100u16.wrapping_sub(sum as u16) & 0xff) as u8;
        }

        let new_checksum = data.iter().fold(0u16, |acc, &b| acc.wrapping_add(b as u16));
        let orig_stem = old_path.file_stem().unwrap_or_default().to_string_lossy();
        let suggested_filename = format!("{}_PATCHED_{:04X}.bin", orig_stem, new_checksum);

        if let Some(target_path) = rfd::FileDialog::new()
            .set_file_name(&suggested_filename)
            .add_filter("Realtek Firmware (.bin)", &["bin"])
            .save_file()
        {
            match fs::write(&target_path, &data) {
                Ok(_) => {
                    self.inspect_file(&target_path);
                    self.inspector_status = Some((
                        format!(
                            "Firmware updated successfully!\nNew Checksum: 0x{:04X}\nMonitor Name: {}\nSaved to: {}",
                            new_checksum,
                            clean_name,
                            target_path.file_name().unwrap_or_default().to_string_lossy()
                        ),
                        true,
                    ));
                }
                Err(e) => {
                    self.inspector_status = Some((format!("Failed to save firmware: {}", e), false));
                }
            }
        }
    }

    fn compose_firmware(&mut self) {
        let (brand, model, h_act, v_act, refresh_rate, pclk_mhz, template_tag, pins, lanes) = if self.generator_mode == 0 {
            let panel = &KNOWN_PANELS[self.selected_panel_idx];
            (
                panel.brand.to_string(),
                panel.model.to_string(),
                panel.resolution.0,
                panel.resolution.1,
                panel.refresh_rate,
                panel.pixel_clock_mhz,
                panel.template_tag,
                panel.pins,
                panel.lanes.to_string(),
            )
        } else {
            let brand = if self.ds_brand.trim().is_empty() { "OEM".to_string() } else { self.ds_brand.trim().to_string() };
            let model = if self.ds_model.trim().is_empty() { "CUSTOM_PANEL".to_string() } else { self.ds_model.trim().to_string() };
            let h_act = self.ds_res_w.trim().parse::<u32>().unwrap_or(1920);
            let v_act = self.ds_res_h.trim().parse::<u32>().unwrap_or(1080);
            let refresh_rate = self.ds_refresh_hz.trim().parse::<u32>().unwrap_or(144);
            let auto_pclk = ((h_act + 280) * (v_act + 45) * refresh_rate) as f64 / 1_000_000.0;
            let pclk_mhz = self.ds_pixel_clock.trim().parse::<f64>().unwrap_or(auto_pclk);
            let (template_tag, pins, lanes) = match self.ds_lane_choice {
                0 => (
                    if refresh_rate >= 144 { "144Hz_4LANE" } else { "120Hz_4LANE" },
                    40,
                    "4 Lanes (HBR 2.7 Gbps)".to_string(),
                ),
                1 => (
                    "120Hz_30PIN",
                    40,
                    "2 Lanes (HBR2 5.4 Gbps)".to_string(),
                ),
                2 => (
                    if h_act == 1920 && v_act == 1200 { "1920x1200_60Hz" } else { "60Hz_30PIN" },
                    30,
                    "2 Lanes (Standard 60Hz)".to_string(),
                ),
                _ => (
                    "120Hz_30PIN",
                    30,
                    "2 Lanes (HBR2 120Hz)".to_string(),
                ),
            };
            (brand, model, h_act, v_act, refresh_rate, pclk_mhz, template_tag, pins, lanes)
        };

        let template_bytes: &[u8] = match template_tag {
            "144Hz_4LANE" => TEMPLATE_144HZ_4L,
            "120Hz_4LANE" => TEMPLATE_120HZ_4L,
            "120Hz_30PIN" => TEMPLATE_120HZ_2L,
            "1920x1200_60Hz" => TEMPLATE_1200P_2L,
            _ => TEMPLATE_60HZ_2L,
        };

        let mut data = template_bytes.to_vec();

        let pclk_10khz = (pclk_mhz * 100.0).round() as u32;
        let h_blank = if h_act == 1920 && v_act == 1080 { 280 } else { 160 };
        let v_blank = if v_act == 1080 { 45 } else { 35 };

        if data.len() >= 0x025BF4 + 128 {
            patch_edid_block(&mut data[0x025BF4..0x025BF4 + 128], &brand, &self.custom_screen_name, pclk_10khz, h_act, v_act, h_blank, v_blank);
        }
        if data.len() >= 0x025CF4 + 128 {
            patch_edid_block(&mut data[0x025CF4..0x025CF4 + 128], &brand, &self.custom_screen_name, pclk_10khz, h_act, v_act, h_blank, v_blank);
        }

        // Apply Boot Logo / Splash Behavior
        let mut logo_desc = "Factory OEM Splash";
        if self.logo_mode == 0 {
            if data.len() >= 0x00020 {
                data[0x00015] = 0x00;
            }
            logo_desc = "Fast Boot (< 1.0s direct HDMI, No Logo)";
        } else if self.logo_mode == 2 {
            if let Some(processed) = &self.logo_processed {
                let target_offset = 0x064000;
                if data.len() >= target_offset + processed.packed_1bit.len() {
                    data[target_offset..target_offset + processed.packed_1bit.len()].copy_from_slice(&processed.packed_1bit);
                    logo_desc = "Custom Boot Logo Injected (Bank 06 @ 0x064000)";
                }
            }
        }

        // Apply Factory OSD Defaults & Presets
        let lang_code: u8 = match self.preset_language {
            0 => 0x02, // English
            1 => 0x03, // French
            2 => 0x04, // German
            3 => 0x05, // Spanish
            4 => 0x01, // Simplified Chinese
            5 => 0x00, // Traditional Chinese
            6 => 0x06, // Japanese
            7 => 0x07, // Korean
            _ => 0x09, // Russian
        };
        let lang_name = match self.preset_language {
            0 => "English",
            1 => "Français",
            2 => "Deutsch",
            3 => "Español",
            4 => "简体中文 (Simplified)",
            5 => "繁體中文 (Traditional)",
            6 => "日本語 (Japanese)",
            7 => "한국어 (Korean)",
            _ => "Русский (Russian)",
        };

        let pic_mode_name = match self.preset_picture_mode {
            0 => "Standard",
            1 => "FPS",
            2 => "RTS",
            3 => "Movie",
            4 => "Text",
            _ => "ECO",
        };
        let color_temp_name = match self.preset_color_temp {
            0 => "6500K Warm",
            1 => "7500K Neutral",
            2 => "9300K Cool",
            3 => "sRGB",
            _ => "User Custom RGB",
        };
        let aspect_name = match self.preset_aspect_ratio {
            0 => "16:9 Full",
            1 => "4:3 Pillarbox",
            _ => "Auto 1:1",
        };
        let audio_status = if self.preset_mute {
            "Muted".to_string()
        } else {
            format!("{}%", self.preset_volume)
        };

        if data.len() >= 0x03670 {
            data[0x0366B] = lang_code;
        }
        if data.len() >= 0x2D000 {
            data[0x2CFF7] = 0x74; // MOV A, #imm
            data[0x2CFF8] = lang_code;
            data[0x2CFF9] = 0x00; // NOP
            data[0x2CFFA] = 0x00; // NOP
        }

        let b_val = self.preset_brightness;
        if data.len() >= 0x2D696 {
            data[0x2D67A] = b_val;
            data[0x2D67F] = b_val;
            data[0x2D68B] = b_val;
            data[0x2D683] = self.preset_contrast;

            if self.preset_color_temp == 4 {
                // Patch User RGB gain channels (scaled to 0-255)
                data[0x2D690] = ((self.preset_user_r as u16 * 255) / 100) as u8;
                data[0x2D691] = ((self.preset_user_g as u16 * 255) / 100) as u8;
                data[0x2D692] = ((self.preset_user_b as u16 * 255) / 100) as u8;
            }
        }

        if data.len() >= 0x01000 {
            data[0x00018] = if self.preset_direct_power { 0x01 } else { 0x00 };
        }

        let temp_desc = if self.preset_color_temp == 4 {
            format!("User RGB ({}/{}/{})", self.preset_user_r, self.preset_user_g, self.preset_user_b)
        } else {
            color_temp_name.to_string()
        };

        let presets_summary = format!(
            "Lang: {}, Pic: {}, Temp: {}, Aspect: {}, Backlight: {}%, Audio: {}, VRR: {}",
            lang_name,
            pic_mode_name,
            temp_desc,
            aspect_name,
            self.preset_brightness,
            audio_status,
            if self.preset_freesync { "FreeSync ON" } else { "OFF" }
        );

        if self.export_format == 1 && data.len() == 851968 {
            data.resize(1048576, 0xff);
        } else if self.export_format == 0 && data.len() == 1048576 {
            data.truncate(851968);
        }

        let new_checksum = data.iter().fold(0u16, |acc, &b| acc.wrapping_add(b as u16));
        let clean_model = model.replace('/', "_").replace(' ', "_").replace('"', "").replace('(', "").replace(')', "");
        let mode_tag = match self.logo_mode {
            0 => "_NOLOGO",
            2 => "_LOGO",
            _ => "",
        };
        let suggested_filename = format!(
            "CUSTOM_JG2555TB_{}X{}_{}Hz{}_{:04X}_{}.bin",
            h_act, v_act, refresh_rate, mode_tag, new_checksum, clean_model
        );

        if let Some(target_path) = rfd::FileDialog::new()
            .set_file_name(&suggested_filename)
            .add_filter("Realtek Firmware (.bin)", &["bin"])
            .save_file()
        {
            match fs::write(&target_path, &data) {
                Ok(_) => {
                    self.inspect_file(&target_path);
                    self.composer_status = Some((
                        format!(
                            "Firmware generated successfully:\n• File: {}\n• Windows Display Name: {}\n• Boot Mode: {}\n• Factory Presets: {}\n• 16-bit Checksum: 0x{:04X}\n• Image Size: {} KB ({} bytes)\n• Required Pinout: {}-pin ({})\n• Flash via RTDCustomerTool over HDMI ISP cable\n\nAll parameters have been loaded into Binary Inspector.",
                            target_path.file_name().unwrap_or_default().to_string_lossy(),
                            self.custom_screen_name,
                            logo_desc,
                            presets_summary,
                            new_checksum,
                            data.len() / 1024,
                            data.len(),
                            pins,
                            lanes
                        ),
                        true,
                    ));
                }
                Err(e) => {
                    self.composer_status = Some((format!("Write failed: {}", e), false));
                }
            }
        }
    }

    fn load_logo_file(&mut self, path: &Path, ctx: &egui::Context) {
        if let Ok(bytes) = fs::read(path) {
            self.logo_file_path = Some(path.to_path_buf());
            self.logo_raw_bytes = Some(bytes);
            self.refresh_logo_preview(ctx);
        }
    }

    fn refresh_logo_preview(&mut self, ctx: &egui::Context) {
        if let Some(bytes) = &self.logo_raw_bytes {
            let (tw, th, _) = LOGO_RESOLUTIONS[self.logo_target_res_idx];
            match process_logo_image(bytes, tw, th, self.logo_threshold, self.logo_dither, self.logo_invert) {
                Ok(processed) => {
                    let tex = ctx.load_texture("boot_logo_preview", processed.color_image.clone(), egui::TextureOptions::NEAREST);
                    self.logo_texture = Some(tex);
                    self.logo_processed = Some(processed);
                    self.logo_status = None;
                }
                Err(err) => {
                    self.logo_status = Some((format!("Conversion error: {}", err), false));
                    self.logo_processed = None;
                    self.logo_texture = None;
                }
            }
        }
    }

    #[allow(dead_code)]
    fn inject_logo_into_firmware(&mut self, source_bytes: &[u8], base_name: &str) {
        let Some((target_offset, width, height, packed_bytes)) = self.logo_processed.as_ref().map(|p| (0x064000, p.width, p.height, p.packed_1bit.clone())) else {
            self.logo_status = Some(("No converted logo available. Please load an image first.".into(), false));
            return;
        };

        let mut data = source_bytes.to_vec();
        if data.len() < target_offset + packed_bytes.len() {
            self.logo_status = Some((format!("Firmware image is too small ({} bytes) for Bank 06 logo injection", data.len()), false));
            return;
        }

        data[target_offset..target_offset + packed_bytes.len()].copy_from_slice(&packed_bytes);
        let new_checksum = data.iter().fold(0u16, |acc, &b| acc.wrapping_add(b as u16));
        let suggested_name = format!("{}_CUSTOM_LOGO_{:04X}.bin", base_name.trim_end_matches(".bin"), new_checksum);

        if let Some(target_path) = rfd::FileDialog::new()
            .set_file_name(&suggested_name)
            .add_filter("Realtek Firmware (.bin)", &["bin"])
            .save_file()
        {
            match fs::write(&target_path, &data) {
                Ok(_) => {
                    self.inspect_file(&target_path);
                    self.logo_status = Some((
                        format!(
                            "Logo successfully injected:\n• Target Address: 0x{:06X} (Bank 06 OSD Bitmap ROM)\n• Dimensions: {}x{} ({} bytes)\n• New 16-bit Checksum: 0x{:04X}\n• File: {}\n\nFirmware ready for flashing via RTDCustomerTool.",
                            target_offset,
                            width,
                            height,
                            packed_bytes.len(),
                            new_checksum,
                            target_path.file_name().unwrap_or_default().to_string_lossy()
                        ),
                        true,
                    ));
                }
                Err(e) => {
                    self.logo_status = Some((format!("Save failed: {}", e), false));
                }
            }
        }
    }

    #[allow(dead_code)]
    fn patch_fast_boot_no_logo(&mut self, source_bytes: &[u8], base_name: &str) {
        let mut data = source_bytes.to_vec();
        if data.len() >= 0x00020 {
            data[0x00015] = 0x00;
        }
        let new_checksum = data.iter().fold(0u16, |acc, &b| acc.wrapping_add(b as u16));
        let suggested_name = format!("{}_NOLOGO_FASTBOOT_{:04X}.bin", base_name.trim_end_matches(".bin"), new_checksum);

        if let Some(target_path) = rfd::FileDialog::new()
            .set_file_name(&suggested_name)
            .add_filter("Realtek Firmware (.bin)", &["bin"])
            .save_file()
        {
            match fs::write(&target_path, &data) {
                Ok(_) => {
                    self.inspect_file(&target_path);
                    self.logo_status = Some((
                        format!(
                            "Fast Boot mode enabled:\n• Splash screen delay bypassed (< 1.0s direct HDMI signal)\n• New 16-bit Checksum: 0x{:04X}\n• File: {}",
                            new_checksum,
                            target_path.file_name().unwrap_or_default().to_string_lossy()
                        ),
                        true,
                    ));
                }
                Err(e) => {
                    self.logo_status = Some((format!("Save failed: {}", e), false));
                }
            }
        }
    }
}

impl eframe::App for RtdStudioApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Handle drag and drop files across the window
        ui.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    let path = file.path();
                    if !path.as_os_str().is_empty() {
                        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                        if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "bmp" {
                            self.load_logo_file(path, ui.ctx());
                            self.logo_mode = 2; // Auto switch to Custom Boot Logo
                            self.selected_tab = 0; // Auto switch to Firmware Composer
                            break;
                        } else {
                            self.inspect_file(path);
                            self.selected_tab = 1; // Auto switch to Inspector
                            break;
                        }
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ui, |ui| {
            // Main Navigation Bar (Clean top bar)
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, 0, "Firmware Composer");
                ui.selectable_value(&mut self.selected_tab, 1, "Binary Inspector");
                ui.selectable_value(&mut self.selected_tab, 2, "Panel Database");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("Realtek RTD2556 / JG2555TB").weak());
                });
            });
            ui.separator();
            ui.add_space(6.0);

            // Tab Content
            match self.selected_tab {
                0 => {
                    // Card 1: Target Display & Specifications
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Panel Source:").strong());
                            if ui.selectable_label(self.generator_mode == 0, "Preset Catalogue").clicked() {
                                self.generator_mode = 0;
                                self.custom_screen_name = KNOWN_PANELS[self.selected_panel_idx].model.to_string();
                            }
                            if ui.selectable_label(self.generator_mode == 1, "Manual Datasheet Input").clicked() {
                                self.generator_mode = 1;
                                self.custom_screen_name = if self.ds_model.is_empty() { String::new() } else { self.ds_model.clone() };
                            }
                        });
                        ui.separator();
                        ui.add_space(4.0);

                        if self.generator_mode == 0 {
                            let prev_idx = self.selected_panel_idx;

                            ui.horizontal(|ui| {
                                egui::ComboBox::from_label("Target Model")
                                    .width(420.0)
                                    .selected_text(format!("{} - {} ({} @ {}Hz)", KNOWN_PANELS[self.selected_panel_idx].brand, KNOWN_PANELS[self.selected_panel_idx].model, KNOWN_PANELS[self.selected_panel_idx].size, KNOWN_PANELS[self.selected_panel_idx].refresh_rate))
                                    .show_ui(ui, |ui| {
                                        for (i, panel) in KNOWN_PANELS.iter().enumerate() {
                                             let label = format!("{} {} - {} @ {}Hz ({})", panel.brand, panel.model, panel.size, panel.refresh_rate, panel.lanes);
                                            ui.selectable_value(&mut self.selected_panel_idx, i, label);
                                        }
                                    });

                                if panel_link_button(ui, "View on Panelook").clicked() {
                                    open_panelook_search(KNOWN_PANELS[self.selected_panel_idx].model);
                                }
                            });

                            if self.selected_panel_idx != prev_idx {
                                self.custom_screen_name = KNOWN_PANELS[self.selected_panel_idx].model.to_string();
                            }

                            let panel = &KNOWN_PANELS[self.selected_panel_idx];
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.label("• Connector:");
                                ui.colored_label(egui::Color32::LIGHT_GREEN, format!("{}-pin ({})", panel.pins, panel.lanes));
                                ui.label("• Pixel Clock:");
                                ui.colored_label(egui::Color32::LIGHT_BLUE, format!("{:.1} MHz", panel.pixel_clock_mhz));
                            });
                            ui.label(egui::RichText::new(format!("  Specification: {}", panel.description)).weak());
                        } else {
                            ui.horizontal(|ui| {
                                ui.label("Manufacturer:");
                                ui.add(egui::TextEdit::singleline(&mut self.ds_brand).hint_text("e.g. BOE, AUO, LG").desired_width(120.0));
                                ui.label("Model Reference:");
                                if ui.add(egui::TextEdit::singleline(&mut self.ds_model).hint_text("e.g. LP156WF6-SPB1").desired_width(180.0)).changed() {
                                    self.custom_screen_name = self.ds_model.clone();
                                }
                                if panel_link_button(ui, "Search on Panelook").clicked() {
                                    open_panelook_search(&self.ds_model);
                                }
                            });

                            // Suggestions / Auto-fill based on typing model reference
                            let input_ref = self.ds_model.trim().to_uppercase();
                            if input_ref.len() >= 3 {
                                let suggestions: Vec<_> = KNOWN_PANELS
                                    .iter()
                                    .filter(|p| p.model.to_uppercase().contains(&input_ref))
                                    .take(4)
                                    .collect();

                                if !suggestions.is_empty() {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new("Suggestions:").weak());
                                        for p in suggestions {
                                            if ui.small_button(p.model).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                                self.ds_brand = p.brand.to_string();
                                                self.ds_model = p.model.to_string();
                                                self.custom_screen_name = p.model.to_string();
                                                self.ds_res_w = p.resolution.0.to_string();
                                                self.ds_res_h = p.resolution.1.to_string();
                                                self.ds_refresh_hz = p.refresh_rate.to_string();
                                                self.ds_pixel_clock = format!("{:.1}", p.pixel_clock_mhz);
                                                self.ds_lane_choice = if p.pins == 40 && p.lanes.contains("4") {
                                                    0
                                                } else if p.pins == 40 {
                                                    1
                                                } else if p.refresh_rate >= 120 {
                                                    3
                                                } else {
                                                    2
                                                };
                                            }
                                        }
                                    });
                                }
                            }

                            ui.horizontal(|ui| {
                                ui.label("Resolution:");
                                ui.add(egui::TextEdit::singleline(&mut self.ds_res_w).hint_text("1920").desired_width(55.0));
                                ui.label("x");
                                ui.add(egui::TextEdit::singleline(&mut self.ds_res_h).hint_text("1080").desired_width(55.0));
                                if ui.small_button("1080p FHD").clicked() { self.ds_res_w = "1920".to_string(); self.ds_res_h = "1080".to_string(); }
                                if ui.small_button("1200p 16:10").clicked() { self.ds_res_w = "1920".to_string(); self.ds_res_h = "1200".to_string(); }
                                if ui.small_button("1440p QHD").clicked() { self.ds_res_w = "2560".to_string(); self.ds_res_h = "1440".to_string(); }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Refresh Rate:");
                                ui.add(egui::TextEdit::singleline(&mut self.ds_refresh_hz).hint_text("144").desired_width(45.0));
                                ui.label("Hz");
                                if ui.small_button("60 Hz").clicked() { self.ds_refresh_hz = "60".to_string(); }
                                if ui.small_button("120 Hz").clicked() { self.ds_refresh_hz = "120".to_string(); }
                                if ui.small_button("144 Hz").clicked() { self.ds_refresh_hz = "144".to_string(); }
                            });

                            ui.label("Physical Interface & eDP Main-Link Configuration:");
                            ui.radio_value(&mut self.ds_lane_choice, 0, "40-pin - 4 Lanes eDP (Standard 120/144Hz Gaming Panels - HBR 2.7 Gbps)");
                            ui.radio_value(&mut self.ds_lane_choice, 1, "40-pin - 2 Lanes eDP (High-Speed HBR2 5.4 Gbps, e.g. AUO B173HAN04.7)");
                            ui.radio_value(&mut self.ds_lane_choice, 2, "30-pin - 2 Lanes eDP (Standard 60Hz Office / Media Panels)");
                            ui.radio_value(&mut self.ds_lane_choice, 3, "30-pin - 2 Lanes eDP (High-Speed 120Hz HBR2, e.g. Innolux N156HHE-GA1)");

                            let h: u32 = self.ds_res_w.parse().unwrap_or(1920);
                            let v: u32 = self.ds_res_h.parse().unwrap_or(1080);
                            let hz: u32 = self.ds_refresh_hz.parse().unwrap_or(144);
                            let auto_pclk = ((h + 280) * (v + 45) * hz) as f64 / 1_000_000.0;

                            ui.horizontal(|ui| {
                                ui.label("Pixel Clock:");
                                ui.add(egui::TextEdit::singleline(&mut self.ds_pixel_clock).hint_text("e.g. 326.7").desired_width(65.0));
                                ui.label("MHz");
                                ui.label(egui::RichText::new(format!("(Calculated CVT-RB: {:.1} MHz)", auto_pclk)).weak());
                                if ui.small_button("Apply Calculated").clicked() {
                                    self.ds_pixel_clock = format!("{:.1}", auto_pclk);
                                }
                            });
                        }
                    });

                    ui.add_space(8.0);
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label(egui::RichText::new("Display Identification (VESA EDID)").strong());
                        ui.horizontal(|ui| {
                            ui.label("OS Display Name:");
                            ui.add(egui::TextEdit::singleline(&mut self.custom_screen_name).hint_text("e.g. Gaming Screen").desired_width(260.0));
                            ui.label(egui::RichText::new("(Reported to Windows / macOS / Linux)").weak());
                        });
                        ui.label(egui::RichText::new("Generates standard 832 KB production image (13 banks) ready for HDMI ISP cable.").weak());
                    });

                    // Boot Behavior & Splash Logo Section
                    let mut trigger_pick_logo_img = false;
                    let mut trigger_refresh_logo = false;

                    ui.add_space(8.0);
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label(egui::RichText::new("Boot Behavior & Splash Logo").strong());
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut self.logo_mode, 0, "Fast Boot (No Logo - Instant Signal)");
                            ui.radio_value(&mut self.logo_mode, 1, "Factory OEM Logo");
                            ui.radio_value(&mut self.logo_mode, 2, "Custom Boot Logo");
                        });

                        if self.logo_mode == 2 {
                            ui.add_space(6.0);
                            ui.columns(2, |cols| {
                                cols[0].vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        if ui.button("Load Image (PNG / JPEG / BMP)...").clicked() {
                                            trigger_pick_logo_img = true;
                                        }
                                        if let Some(path) = &self.logo_file_path {
                                            ui.label(egui::RichText::new(path.file_name().unwrap_or_default().to_string_lossy()).strong());
                                        } else {
                                            ui.label(egui::RichText::new("Drag & drop an image anywhere").weak());
                                        }
                                    });

                                    ui.add_space(4.0);
                                    ui.horizontal(|ui| {
                                        ui.label("Resolution:");
                                        for (idx, (_, _, desc)) in LOGO_RESOLUTIONS.iter().enumerate() {
                                            let short_desc = desc.split(' ').next().unwrap_or(desc);
                                            if ui.radio_value(&mut self.logo_target_res_idx, idx, short_desc).clicked() {
                                                trigger_refresh_logo = true;
                                            }
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Threshold:");
                                        if ui.add(egui::Slider::new(&mut self.logo_threshold, 0..=255)).changed() {
                                            trigger_refresh_logo = true;
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        if ui.checkbox(&mut self.logo_dither, "Floyd-Steinberg Dithering").changed() {
                                            trigger_refresh_logo = true;
                                        }
                                        if ui.checkbox(&mut self.logo_invert, "Invert Colors").changed() {
                                            trigger_refresh_logo = true;
                                        }
                                    });

                                    if let Some(processed) = &self.logo_processed {
                                        ui.label(egui::RichText::new(format!("Payload: {}x{} px ({} bytes in Bank 06)", processed.width, processed.height, processed.packed_1bit.len())).weak());
                                    }
                                });

                                cols[1].vertical(|ui| {
                                    egui::Frame::canvas(ui.style())
                                        .fill(egui::Color32::from_rgb(10, 11, 14))
                                        .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(50, 54, 62)))
                                        .inner_margin(8.0)
                                        .show(ui, |ui| {
                                            ui.set_min_size(egui::vec2(280.0, 110.0));
                                            ui.vertical_centered(|ui| {
                                                if let Some(tex) = &self.logo_texture {
                                                    let display_size = egui::vec2(tex.size()[0] as f32 * 1.2, tex.size()[1] as f32 * 1.2);
                                                    ui.add_space(6.0);
                                                    ui.image((tex.id(), display_size));
                                                    ui.add_space(6.0);
                                                } else {
                                                    ui.add_space(24.0);
                                                    ui.label(egui::RichText::new("No Logo Loaded\nClick 'Load Image' or drag a PNG / JPG here").weak());
                                                    ui.add_space(24.0);
                                                }
                                                ui.label(egui::RichText::new("Realtek Boot Splash Preview (1080p @ 144Hz)").small().weak());
                                            });
                                        });
                                });
                            });
                        } else if self.logo_mode == 0 {
                            ui.label(egui::RichText::new("Fast Boot skips splash screen delay (< 1.0s direct HDMI signal, optimal for setups without physical buttons).").weak());
                        } else {
                            ui.label(egui::RichText::new("Standard OEM factory boot splash is preserved.").weak());
                        }
                    });

                    if trigger_pick_logo_img {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Images (*.png, *.jpg, *.jpeg, *.bmp)", &["png", "jpg", "jpeg", "bmp"])
                            .pick_file()
                        {
                            self.load_logo_file(&path, ui.ctx());
                        }
                    }

                    if trigger_refresh_logo {
                        self.refresh_logo_preview(ui.ctx());
                    }

                    // Factory OSD Defaults & Presets Section
                    ui.add_space(8.0);
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Factory OSD Defaults & Presets").strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.checkbox(&mut self.enable_custom_presets, "Customize Default Presets");
                            });
                        });

                        if self.enable_custom_presets {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.selectable_value(&mut self.preset_category, 0, "System & Language");
                                ui.selectable_value(&mut self.preset_category, 1, "Picture & Aspect");
                                ui.selectable_value(&mut self.preset_category, 2, "Color & Calibration");
                                ui.selectable_value(&mut self.preset_category, 3, "Levels & Audio");
                                ui.selectable_value(&mut self.preset_category, 4, "Gaming & VRR");
                            });
                            ui.separator();
                            ui.add_space(4.0);

                            match self.preset_category {
                                0 => {
                                    ui.columns(2, |cols| {
                                        cols[0].vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Default OSD Language:");
                                                egui::ComboBox::from_id_salt("combo_lang")
                                                    .selected_text(match self.preset_language {
                                                        0 => "English",
                                                        1 => "Français (French)",
                                                        2 => "Deutsch (German)",
                                                        3 => "Español (Spanish)",
                                                        4 => "简体中文 (Simplified Chinese)",
                                                        5 => "繁體中文 (Traditional Chinese)",
                                                        6 => "日本語 (Japanese)",
                                                        7 => "한국어 (Korean)",
                                                        _ => "Русский (Russian)",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut self.preset_language, 0, "English");
                                                        ui.selectable_value(&mut self.preset_language, 1, "Français (French)");
                                                        ui.selectable_value(&mut self.preset_language, 2, "Deutsch (German)");
                                                        ui.selectable_value(&mut self.preset_language, 3, "Español (Spanish)");
                                                        ui.selectable_value(&mut self.preset_language, 4, "简体中文 (Simplified Chinese)");
                                                        ui.selectable_value(&mut self.preset_language, 5, "繁體中文 (Traditional Chinese)");
                                                        ui.selectable_value(&mut self.preset_language, 6, "日本語 (Japanese)");
                                                        ui.selectable_value(&mut self.preset_language, 7, "한국어 (Korean)");
                                                        ui.selectable_value(&mut self.preset_language, 8, "Русский (Russian)");
                                                    });
                                            });
                                            ui.add_space(4.0);
                                            ui.checkbox(&mut self.preset_direct_power, "Direct Power-On (Auto-start without button)");
                                            ui.label(egui::RichText::new("Turns on immediately when power is connected.").small().weak());
                                        });

                                        cols[1].vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Input Source at Boot:");
                                                egui::ComboBox::from_id_salt("combo_input")
                                                    .selected_text(match self.preset_auto_input {
                                                        0 => "Auto-Detect Active (Recommended)",
                                                        1 => "Force HDMI 1",
                                                        _ => "Force HDMI 2",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut self.preset_auto_input, 0, "Auto-Detect Active (Recommended)");
                                                        ui.selectable_value(&mut self.preset_auto_input, 1, "Force HDMI 1");
                                                        ui.selectable_value(&mut self.preset_auto_input, 2, "Force HDMI 2");
                                                    });
                                            });
                                        });
                                    });
                                }

                                1 => {
                                    ui.columns(2, |cols| {
                                        cols[0].vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Picture Mode Preset:");
                                                egui::ComboBox::from_id_salt("combo_pic")
                                                    .selected_text(match self.preset_picture_mode {
                                                        0 => "Standard (Balanced)",
                                                        1 => "FPS (First-Person Shooter)",
                                                        2 => "RTS (Real-Time Strategy)",
                                                        3 => "Movie / Cinema",
                                                        4 => "Text / Office",
                                                        _ => "ECO (Energy Saver)",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut self.preset_picture_mode, 0, "Standard (Balanced)");
                                                        ui.selectable_value(&mut self.preset_picture_mode, 1, "FPS (First-Person Shooter)");
                                                        ui.selectable_value(&mut self.preset_picture_mode, 2, "RTS (Real-Time Strategy)");
                                                        ui.selectable_value(&mut self.preset_picture_mode, 3, "Movie / Cinema");
                                                        ui.selectable_value(&mut self.preset_picture_mode, 4, "Text / Office");
                                                        ui.selectable_value(&mut self.preset_picture_mode, 5, "ECO (Energy Saver)");
                                                    });
                                            });
                                            ui.add_space(4.0);
                                            ui.checkbox(&mut self.preset_dcr, "Dynamic Contrast Ratio (DCR)");
                                            ui.label(egui::RichText::new("Leave off for accurate fixed backlight calibration.").small().weak());
                                        });

                                        cols[1].vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Aspect Ratio / Scaling:");
                                                egui::ComboBox::from_id_salt("combo_aspect")
                                                    .selected_text(match self.preset_aspect_ratio {
                                                        0 => "16:9 Full Screen (Default)",
                                                        1 => "4:3 Pillarbox (Retro ratio)",
                                                        _ => "Auto / 1:1 Pixel-to-Pixel",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut self.preset_aspect_ratio, 0, "16:9 Full Screen (Default)");
                                                        ui.selectable_value(&mut self.preset_aspect_ratio, 1, "4:3 Pillarbox (Retro ratio)");
                                                        ui.selectable_value(&mut self.preset_aspect_ratio, 2, "Auto / 1:1 Pixel-to-Pixel");
                                                    });
                                            });
                                        });
                                    });
                                }

                                2 => {
                                    ui.columns(2, |cols| {
                                        cols[0].vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Color Temperature:");
                                                egui::ComboBox::from_id_salt("combo_color")
                                                    .selected_text(match self.preset_color_temp {
                                                        0 => "Warm (6500K D65 Standard)",
                                                        1 => "Neutral (7500K)",
                                                        2 => "Cool (9300K Asian default)",
                                                        3 => "sRGB (Calibrated Gamut)",
                                                        _ => "User Custom RGB",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut self.preset_color_temp, 0, "Warm (6500K D65 Standard)");
                                                        ui.selectable_value(&mut self.preset_color_temp, 1, "Neutral (7500K)");
                                                        ui.selectable_value(&mut self.preset_color_temp, 2, "Cool (9300K Asian default)");
                                                        ui.selectable_value(&mut self.preset_color_temp, 3, "sRGB (Calibrated Gamut)");
                                                        ui.selectable_value(&mut self.preset_color_temp, 4, "User Custom RGB");
                                                    });
                                            });
                                        });

                                        cols[1].vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Low Blue Light Filter:");
                                                egui::ComboBox::from_id_salt("combo_blue")
                                                    .selected_text(match self.preset_low_blue {
                                                        0 => "Off (Normal)",
                                                        1 => "Level 1 (25% Filter)",
                                                        2 => "Level 2 (50% Filter)",
                                                        3 => "Level 3 (75% Filter)",
                                                        _ => "Max (100% Filter)",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut self.preset_low_blue, 0, "Off (Normal)");
                                                        ui.selectable_value(&mut self.preset_low_blue, 1, "Level 1 (25% Filter)");
                                                        ui.selectable_value(&mut self.preset_low_blue, 2, "Level 2 (50% Filter)");
                                                        ui.selectable_value(&mut self.preset_low_blue, 3, "Level 3 (75% Filter)");
                                                        ui.selectable_value(&mut self.preset_low_blue, 4, "Max (100% Filter)");
                                                    });
                                            });
                                        });
                                    });

                                    // User Custom RGB Sliders & Quick Buttons
                                    if self.preset_color_temp == 4 {
                                        ui.add_space(6.0);
                                        ui.separator();
                                        ui.add_space(4.0);
                                        ui.label(egui::RichText::new("User Custom RGB Gain Balance:").strong());

                                        ui.columns(3, |rgb_cols| {
                                            rgb_cols[0].vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.colored_label(egui::Color32::from_rgb(255, 110, 110), "Red:");
                                                    ui.add(egui::Slider::new(&mut self.preset_user_r, 0..=100).suffix("%"));
                                                });
                                            });
                                            rgb_cols[1].vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.colored_label(egui::Color32::from_rgb(110, 255, 110), "Green:");
                                                    ui.add(egui::Slider::new(&mut self.preset_user_g, 0..=100).suffix("%"));
                                                });
                                            });
                                            rgb_cols[2].vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.colored_label(egui::Color32::from_rgb(120, 180, 255), "Blue:");
                                                    ui.add(egui::Slider::new(&mut self.preset_user_b, 0..=100).suffix("%"));
                                                });
                                            });
                                        });

                                        ui.add_space(4.0);
                                        ui.horizontal(|ui| {
                                            if ui.small_button("Balanced (50/50/50)").clicked() {
                                                self.preset_user_r = 50; self.preset_user_g = 50; self.preset_user_b = 50;
                                            }
                                            if ui.small_button("Warm Tint (55/50/45)").clicked() {
                                                self.preset_user_r = 55; self.preset_user_g = 50; self.preset_user_b = 45;
                                            }
                                            if ui.small_button("Cool Tint (45/50/55)").clicked() {
                                                self.preset_user_r = 45; self.preset_user_g = 50; self.preset_user_b = 55;
                                            }
                                            if ui.small_button("Max Dynamic (100%)").clicked() {
                                                self.preset_user_r = 100; self.preset_user_g = 100; self.preset_user_b = 100;
                                            }

                                            ui.add_space(10.0);
                                            ui.label(egui::RichText::new("Color Swatch:").small().weak());
                                            let r_preview = ((self.preset_user_r as f32 / 100.0) * 255.0) as u8;
                                            let g_preview = ((self.preset_user_g as f32 / 100.0) * 255.0) as u8;
                                            let b_preview = ((self.preset_user_b as f32 / 100.0) * 255.0) as u8;
                                            let (rect, _) = ui.allocate_exact_size(egui::vec2(36.0, 18.0), egui::Sense::hover());
                                            ui.painter().rect_filled(rect, 3.0, egui::Color32::from_rgb(r_preview, g_preview, b_preview));
                                            ui.painter().rect_stroke(rect, 3.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(120, 120, 120)), egui::StrokeKind::Outside);
                                        });
                                    }
                                }

                                3 => {
                                    ui.columns(2, |cols| {
                                        cols[0].vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Default Brightness:");
                                                ui.add(egui::Slider::new(&mut self.preset_brightness, 0..=100).suffix("%"));
                                                if ui.small_button("80%").clicked() { self.preset_brightness = 80; }
                                                if ui.small_button("100%").clicked() { self.preset_brightness = 100; }
                                            });

                                            ui.horizontal(|ui| {
                                                ui.label("Default Contrast:");
                                                ui.add(egui::Slider::new(&mut self.preset_contrast, 0..=100).suffix("%"));
                                                if ui.small_button("50%").clicked() { self.preset_contrast = 50; }
                                            });
                                        });

                                        cols[1].vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Audio Volume Level:");
                                                ui.add(egui::Slider::new(&mut self.preset_volume, 0..=100).suffix("%"));
                                                if ui.small_button("70%").clicked() { self.preset_volume = 70; }
                                                if ui.small_button("100%").clicked() { self.preset_volume = 100; }
                                            });

                                            ui.checkbox(&mut self.preset_mute, "Mute Audio by Default (Silent Startup)");
                                        });
                                    });
                                }

                                _ => {
                                    ui.columns(2, |cols| {
                                        cols[0].vertical(|ui| {
                                            ui.checkbox(&mut self.preset_freesync, "FreeSync / Adaptive-Sync (VRR)");
                                            ui.label(egui::RichText::new("Synchronizes refresh rate with NVIDIA G-Sync and AMD FreeSync.").small().weak());
                                        });

                                        cols[1].vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Overdrive (OD) Response Time:");
                                                egui::ComboBox::from_id_salt("combo_od")
                                                    .selected_text(match self.preset_overdrive {
                                                        0 => "Off",
                                                        1 => "Medium (Balanced Anti-Ghosting)",
                                                        _ => "High (Fastest e-Sport)",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut self.preset_overdrive, 0, "Off");
                                                        ui.selectable_value(&mut self.preset_overdrive, 1, "Medium (Balanced Anti-Ghosting)");
                                                        ui.selectable_value(&mut self.preset_overdrive, 2, "High (Fastest e-Sport)");
                                                    });
                                            });
                                        });
                                    });
                                }
                            }
                        } else {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Defaults: English • 80% Brightness • Standard Mode • 6500K • Direct Power-On • FreeSync ON").weak());
                            });
                        }
                    });

                    ui.add_space(12.0);
                    let export_btn = ui.add_sized(
                        [ui.available_width(), 40.0],
                        egui::Button::new(egui::RichText::new("Export Production Firmware (.bin)").strong().size(15.0))
                    );
                    if export_btn.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                        self.compose_firmware();
                    }

                    if let Some((msg, success)) = &self.composer_status {
                        ui.add_space(8.0);
                        let color = if *success { egui::Color32::LIGHT_GREEN } else { egui::Color32::LIGHT_RED };
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.colored_label(color, msg);
                        });
                    }
                }

                1 => {
                    ui.heading("Binary Inspector & Diagnostic");
                    ui.label("Decode and inspect any Realtek RTD .bin firmware image, verify bank integrity and inspect VESA EDID descriptors.");
                    ui.add_space(10.0);

                    // Drag-and-drop zone and file picker
                    let drop_frame = egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(26, 32, 44))
                        .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(60, 100, 160)))
                        .inner_margin(egui::Margin::same(16));

                    drop_frame.show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("Drag and drop your .bin firmware anywhere in this window").strong());
                            ui.label(egui::RichText::new("or click below to choose a file from your disk").weak());
                            ui.add_space(6.0);
                            if ui.add_sized([220.0, 32.0], egui::Button::new("Open Firmware (.bin)..."))
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("Realtek Firmware (.bin)", &["bin"])
                                    .pick_file()
                                {
                                    self.inspect_file(&path);
                                }
                            }
                        });
                    });

                    ui.add_space(10.0);
                    let mut trigger_patch_save = false;
                    if let Some(data) = &self.inspector_data {
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.heading(format!("File: {}", data.filename));
                            ui.label(format!("• Status: {}", data.status));
                            ui.label(format!("• Size: {} bytes ({} banks of 64 KB)", data.filesize, data.banks));
                            ui.horizontal(|ui| {
                                ui.label(format!("• 16-bit Checksum: 0x{:04X}", data.checksum));
                                if let Some(expected) = data.checksum_name {
                                    if data.checksum_match {
                                        ui.colored_label(egui::Color32::LIGHT_GREEN, format!("Matches filename tag (0x{:04X})", expected));
                                    } else {
                                        ui.colored_label(egui::Color32::LIGHT_RED, format!("Differs from filename tag (0x{:04X})", expected));
                                    }
                                }
                            });

                            ui.separator();
                            ui.label(egui::RichText::new("VESA EDID Display Identity (Reported to OS & GPU):").strong());
                            ui.horizontal(|ui| {
                                ui.label("• Monitor Name:");
                                ui.add(egui::TextEdit::singleline(&mut self.inspector_edit_name).desired_width(180.0));
                                let patch_btn = ui.button("Save Patched Firmware...");
                                if patch_btn.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                    trigger_patch_save = true;
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label("• Manufacturer PNP ID:");
                                ui.colored_label(egui::Color32::LIGHT_BLUE, &data.manufacturer);
                                ui.label("• EDID Structure Integrity:");
                                if data.edid_checksum_valid {
                                    ui.colored_label(egui::Color32::LIGHT_GREEN, "Valid VESA Checksum (0x00)");
                                } else {
                                    ui.colored_label(egui::Color32::LIGHT_RED, "Invalid EDID Checksum");
                                }
                            });

                            if let Some((msg, success)) = &self.inspector_status {
                                ui.add_space(4.0);
                                let color = if *success { egui::Color32::LIGHT_GREEN } else { egui::Color32::LIGHT_RED };
                                ui.colored_label(color, msg);
                            }

                            ui.separator();
                            ui.label(egui::RichText::new("Panel Parameters & Driving Signals:").strong());
                            ui.label(format!("• Native Resolution : {}", data.resolution));
                            ui.label(format!("• Refresh Rate      : {}", data.refresh_rate));
                            ui.label(format!("• Pixel Clock       : {}", data.pixel_clock));
                            ui.label(format!("• Required Cabling  : {}", data.cable_type));
                            ui.label(format!("• 16-bit Gamma LUT  : {}", if data.gamma_table_present { "Present in Bank 11 (0x0B0000)" } else { "Missing" }));
                            ui.label(format!("• Keypad Scheme     : {}", data.keypad_type));
                            ui.label(format!("• Backlight Driver  : {}", data.backlight_type));

                            if !data.notes.is_empty() {
                                ui.separator();
                                ui.label(egui::RichText::new("OEM Factory Flags Detected:").strong());
                                for note in &data.notes {
                                    ui.label(format!("  - {}", note));
                                }
                            }
                        });
                    } else {
                        ui.label(egui::RichText::new("No binary loaded. Click the button above to inspect any .bin firmware.").weak());
                    }

                    if trigger_patch_save {
                        self.save_inspector_modified_firmware();
                    }
                }

                2 => {
                    ui.heading("LCD Panel Database & Pinouts");
                    ui.label("Technical reference for high-refresh laptop panels, eDP connector pinouts, and direct links to online specifications.");
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Filter & Search:").strong());
                        ui.add(egui::TextEdit::singleline(&mut self.panelook_search_query).hint_text("Search model, brand, spec...").desired_width(220.0));
                        if panel_link_button(ui, "Search on Panelook").clicked() {
                            open_panelook_search(&self.panelook_search_query);
                        }
                        if !self.panelook_search_query.is_empty() && ui.small_button("Clear").clicked() {
                            self.panelook_search_query.clear();
                        }
                    });
                    ui.add_space(8.0);

                    let query = self.panelook_search_query.trim().to_uppercase();
                    let filtered_panels: Vec<_> = KNOWN_PANELS
                        .iter()
                        .filter(|p| {
                            if query.is_empty() {
                                true
                            } else {
                                p.model.to_uppercase().contains(&query)
                                    || p.brand.to_uppercase().contains(&query)
                                    || p.description.to_uppercase().contains(&query)
                            }
                        })
                        .collect();

                    ui.label(
                        egui::RichText::new(format!("Showing {} of {} known panel profiles", filtered_panels.len(), KNOWN_PANELS.len()))
                            .weak(),
                    );
                    ui.add_space(4.0);

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if filtered_panels.is_empty() {
                            ui.label("No panels match your filter. You can still click 'Search on Panelook' above to look up this reference online.");
                        } else {
                            for panel in filtered_panels {
                                ui.group(|ui| {
                                    ui.set_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        ui.heading(format!("{} {}", panel.brand, panel.model));
                                        ui.colored_label(egui::Color32::LIGHT_GREEN, format!("{}-pin ({})", panel.pins, panel.lanes));
                                        ui.colored_label(egui::Color32::LIGHT_BLUE, format!("{}x{} @ {}Hz", panel.resolution.0, panel.resolution.1, panel.refresh_rate));
                                        
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if panel_link_button(ui, "View on Panelook").clicked() {
                                                open_panelook_search(panel.model);
                                            }
                                        });
                                    });
                                    ui.label(panel.description);
                                });
                                ui.add_space(4.0);
                            }
                        }
                    });
                }

                _ => {}
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([920.0, 640.0])
            .with_min_inner_size([760.0, 500.0])
            .with_title("RTD Firmware Studio"),
        ..Default::default()
    };

    eframe::run_native(
        "RTD Firmware Studio",
        options,
        Box::new(|_cc| Ok(Box::new(RtdStudioApp::default()))),
    )
}

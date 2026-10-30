# RTD Firmware CLI (rtd-cli)

Outil CLI haute performance développé en **Rust** (zéro dépendance externe) pour inspecter, cartographier et vérifier la compatibilité des firmwares **Realtek RTD (RTD2556 / JG2555TB)** avec les dalles d'écrans eDP.

---

## 🚀 Fonctionnalités

- 🔍 **Lecture et Décodage EDID VESA 1.4** : Extrait la résolution native, le Pixel Clock, les timings horizontaux/verticaux et la fréquence réelle.
- 🔑 **Validation d'Intégrité (Checksum 16-bit)** : Calcule `sum(octets) & 0xFFFF` et valide automatiquement le code hexadécimal à 4 caractères du firmware officiel.
- 🔌 **Déduction Matérielle Automatique** :
  - Nombre de broches requis (**30 broches** vs **40 broches**)
  - Configuration des canaux (**eDP 2 Lanes** vs **eDP 4 Lanes**)
  - Type de clavier (**4 touches** vs **5 touches**)
  - Mode de rétroéclairage (**PWM réel** vs **Digital Brightness**)
  - Statut du boot logo (**Sans Logo / 无LOGO** vs avec logo OEM)
- 🎯 **Base de Compatibilité d'Écrans** : Suggère instantanément les dalles compatibles (ex: *BOE NV173FHM-NX4*, *NE173FHM-NZ6*, *AUO B173HAN01.1*, *Samsung LTN156HL07*).
- ⚡ **Recherche Instantanée** : Permet de trouver quel firmware flasher pour une dalle donnée (`--find <reference>`).

---

## 🛠️ Compilation

Le projet utilise Rust standard sans crate externe :

```bash
cargo build --release
```

L'exécutable autonome se trouve dans `tools/rtd-cli/target/release/rtd-cli.exe`.

---

## 📖 Utilisation

### 1. Analyser un firmware spécifique
```bash
rtd-cli "mon_firmware.bin"
```

### 2. Trouver quel firmware flasher pour une dalle
```bash
rtd-cli --find NV173FHM-NX4
rtd-cli --find B173HAN01
```

### 3. Scanner et cartographier tout un dossier de firmwares
```bash
rtd-cli --scan .
```

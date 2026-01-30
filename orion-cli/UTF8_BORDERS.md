# UTF-8 Box Drawing Characters Reference

## Keyboard Shortcuts for Box Drawing

### Windows (Alt + Numpad codes):
```
╔  Alt+201  (U+2554) - Box Double Down and Right
═  Alt+205  (U+2550) - Box Double Horizontal  
╗  Alt+187  (U+2557) - Box Double Down and Left
║  Alt+186  (U+2551) - Box Double Vertical
╚  Alt+200  (U+255A) - Box Double Up and Right
╝  Alt+188  (U+255D) - Box Double Up and Left
╠  Alt+204  (U+2560) - Box Double Vertical and Right
╣  Alt+185  (U+2563) - Box Double Vertical and Left
╦  Alt+203  (U+2566) - Box Double Down and Horizontal
╩  Alt+202  (U+2569) - Box Double Up and Horizontal
╬  Alt+206  (U+256C) - Box Double Vertical and Horizontal
```

### Single Line Boxes:
```
┌  Alt+218  (U+250C) - Box Down and Right
─  Alt+196  (U+2500) - Box Horizontal
┐  Alt+191  (U+2510) - Box Down and Left
│  Alt+179  (U+2502) - Box Vertical
└  Alt+192  (U+2514) - Box Up and Right
┘  Alt+217  (U+2518) - Box Up and Left
├  Alt+195  (U+251C) - Box Vertical and Right
┤  Alt+180  (U+2524) - Box Vertical and Left
┬  Alt+194  (U+252C) - Box Down and Horizontal
┴  Alt+193  (U+2534) - Box Up and Horizontal
┼  Alt+197  (U+253C) - Box Vertical and Horizontal
```

### Dashed Lines:
```
┄  (U+2504) - Box Light Triple Dash Horizontal
┆  (U+2506) - Box Light Triple Dash Vertical  
╌  (U+254C) - Box Light Double Dash Horizontal
╎  (U+254E) - Box Light Double Dash Vertical
```

## Copy-Paste Ready Sets

### ORION CLI Standard Border (67 chars wide):
```
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
```

### Table Borders (comfy-table UTF8_FULL preset):
```
┌───┬───┐
│   │   │
├───┼───┤
│   │   │
└───┴───┘
```

## How to Type in VS Code:

1. **Method 1 - Alt Codes** (Windows):
   - Hold Alt
   - Type the code on numpad
   - Release Alt

2. **Method 2 - Unicode**:
   - Type the code (e.g., 2554)
   - Press Alt+X (converts to ╔)

3. **Method 3 - Copy/Paste**:
   - Copy characters from this file
   - Paste into source code

## Current ORION CLI Usage:

```rust
// Banner (67 chars wide):
println!("╔═══════════════════════════════════════════════════════════════════╗");
println!("║                                                                   ║");  
println!("╚═══════════════════════════════════════════════════════════════════╝");

// Section Headers:
println!("╔═══════════════════════════════════════════════════════════════════╗");
println!("║  Section Title                                                    ║");
println!("╚═══════════════════════════════════════════════════════════════════╝");
```

## Notes:
- ORION uses **double-line** boxes (═ ║) for headers/banners
- comfy-table uses **single-line** boxes (─ │) for data tables  
- All borders should be **67 characters** wide for consistency
- Borders are colored with `.bright_magenta()` or `.bright_cyan()`

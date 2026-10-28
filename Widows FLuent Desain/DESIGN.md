---
name: Fluent Desktop Shell
colors:
  surface: '#f9f9f9'
  surface-dim: '#dadada'
  surface-bright: '#f9f9f9'
  surface-container-lowest: '#ffffff'
  surface-container-low: '#f3f3f3'
  surface-container: '#eeeeee'
  surface-container-high: '#e8e8e8'
  surface-container-highest: '#e2e2e2'
  on-surface: '#1a1c1c'
  on-surface-variant: '#404752'
  inverse-surface: '#2f3131'
  inverse-on-surface: '#f1f1f1'
  outline: '#717783'
  outline-variant: '#c0c7d4'
  surface-tint: '#0060ab'
  primary: '#005faa'
  on-primary: '#ffffff'
  primary-container: '#0078d4'
  on-primary-container: '#ffffff'
  inverse-primary: '#a3c9ff'
  secondary: '#0061a3'
  on-secondary: '#ffffff'
  secondary-container: '#5badff'
  on-secondary-container: '#003f6d'
  tertiary: '#0060a9'
  on-tertiary: '#ffffff'
  tertiary-container: '#2679c9'
  on-tertiary-container: '#ffffff'
  error: '#ba1a1a'
  on-error: '#ffffff'
  error-container: '#ffdad6'
  on-error-container: '#93000a'
  primary-fixed: '#d3e3ff'
  primary-fixed-dim: '#a3c9ff'
  on-primary-fixed: '#001c39'
  on-primary-fixed-variant: '#004883'
  secondary-fixed: '#d1e4ff'
  secondary-fixed-dim: '#9ecaff'
  on-secondary-fixed: '#001d36'
  on-secondary-fixed-variant: '#00497d'
  tertiary-fixed: '#d3e4ff'
  tertiary-fixed-dim: '#a2c9ff'
  on-tertiary-fixed: '#001c38'
  on-tertiary-fixed-variant: '#004881'
  background: '#f9f9f9'
  on-background: '#1a1c1c'
  surface-variant: '#e2e2e2'
typography:
  display:
    fontFamily: Inter
    fontSize: 40px
    fontWeight: '600'
    lineHeight: 52px
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Inter
    fontSize: 28px
    fontWeight: '600'
    lineHeight: 36px
    letterSpacing: -0.015em
  headline-md:
    fontFamily: Inter
    fontSize: 20px
    fontWeight: '600'
    lineHeight: 26px
    letterSpacing: -0.01em
  headline-sm:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '600'
    lineHeight: 22px
    letterSpacing: -0.005em
  body-lg:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  body-md:
    fontFamily: Inter
    fontSize: 13px
    fontWeight: '400'
    lineHeight: 18px
  body-sm:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '400'
    lineHeight: 16px
  label-md:
    fontFamily: Inter
    fontSize: 13px
    fontWeight: '500'
    lineHeight: 18px
  label-sm:
    fontFamily: Inter
    fontSize: 11px
    fontWeight: '500'
    lineHeight: 14px
    letterSpacing: 0.02em
  caption:
    fontFamily: Inter
    fontSize: 10px
    fontWeight: '400'
    lineHeight: 13px
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  gutter: 1rem
  gutter-dense: 0.5rem
  margin: 1.5rem
  margin-window: 0.75rem
  space-xs: 0.25rem
  space-sm: 0.5rem
  space-md: 0.75rem
  space-lg: 1rem
  space-xl: 1.5rem
---

## Brand & Style

This design system establishes an authentic, high-precision desktop interface modeled directly on the canonical desktop shell experience. The visual language balances utility, calm focus, and optical depth through physical material metaphors—specifically translucent background sampling and optical layering.

### Design Principles
- **Materiality & Translucency:** Interfaces leverage backdrop blur and tinted alpha values rather than flat, opaque panels. The foundational layers subtly pick up colors from underneath, producing a cohesive ambient connection across windows and the desktop canvas.
- **Micro-Precision Outlines:** All elevated panels, flyouts, and floating windows use ultra-fine 1px semi-transparent borders to separate structural planes without hard visual friction.
- **Calm & Productive Demeanor:** Motion is purposeful and snappy (150ms–250ms ease-out curves), typography remains clean and functional, and structural framing stays secondary to the primary content workspace.
- **Refined Balance:** Elements use disciplined geometry softened by consistent medium corner rounding, preventing cold sterility while maintaining desktop-grade information density.

## Colors

The palette centers around the signature Fluent blue accent combined with systematic neutral tiers engineered for composite layering over wallpaper or canvas backdrops.

### Accent Roles
- **Primary (`#0078D4`):** Interactive accents, system focus indicators, toggles, checked states, and primary call-to-action buttons.
- **Secondary (`#2B88D8`):** Hover states on primary controls and active selection highlights.
- **Tertiary (`#106EBE`):** Pressed and active click states for primary controls.

### Neutral & Material Surfaces
- **Mica Base (`#F3F3F3` at 85% opacity with `backdrop-filter: blur(30px)`):** Primary window shell, title bars, and background work areas.
- **Acrylic Flyout (`#FFFFFF` at 70% opacity with `backdrop-filter: blur(40px) saturate(180%)`):** Transient surfaces including the Start Menu, taskbar jump lists, context menus, and tooltips.
- **Card / Surface (`#FFFFFF` at 90% opacity):** Elevated content containers within windows, input fields, and pinned item blocks.
- **Subtle Stroke (`#000000` at 6% to 9% opacity):** Border boundaries for windows and dialog surfaces.
- **On-Neutral Primary (`#1B1B1B`):** High-contrast typography for primary labels and headings.
- **On-Neutral Secondary (`#5D5D5D`):** Metadata, breadcrumbs, placeholder text, and inactive iconography.

## Typography

Typography prioritizes legibility, tight tracking, and optical neutrality across high-density desktop surfaces. System typography mimics standard desktop text scaling, using `13px` and `14px` as default operational body baselines.

- **Weight Hierarchy:** Standard interactions and body copy utilize `400 Regular`. Interactive control labels, metadata chips, and section titles utilize `500 Medium` or `600 Semi-Bold`.
- **Anti-Aliasing:** All text rendering must use subpixel anti-aliasing (`-webkit-font-smoothing: antialiased`) to maintain crisp letterforms over translucent background materials.
- **Numbers & Metrics:** Tabular data surfaces (such as file sizes, modified dates, and system metrics) require tabular numerals (`font-feature-settings: 'tnum' on`).

## Layout & Spacing

The layout model is anchored to desktop window management paradigms: a global desktop workspace, anchored navigation shells, floating windows, and transient system overlays.

### Spatial Rhythms
- **4px Core Grid:** All layouts and component dimensions adhere strictly to 4px multiples.
- **Desktop Shell Coordinates:**
  - **Taskbar:** Fixed 48px height, pinned to the bottom screen edge, horizontally centered application tray with 4px element spacing.
  - **Start Menu Flyout:** Fixed width of 640px and maximum height of 720px, anchored centered-bottom with a 12px vertical offset from the taskbar.
  - **Window Frames:** Minimum width 480px, minimum height 320px, with 32px to 40px draggable title bars.
- **Internal Window Grid:** Application windows use an asymmetric 2-column or 3-column pane structure: navigation sidebar (240px fixed width or fluid collapsible), content directory listing, and contextual detail panel.

## Elevation & Depth

Visual hierarchy uses material translucency paired with multi-layer ambient shadow profiles. Shadows avoid harsh pitch-black stamps; instead, they simulate natural diffuse light sources from directly above the workspace.

### Elevation Layers
1. **Desktop Canvas (Level 0):** The base wallpaper surface.
2. **Taskbar & Window Backdrops (Level 1):** Mica material with a subtle perimeter stroke (`1px solid rgba(0, 0, 0, 0.08)`) and ambient shadow:
   `0 2px 8px rgba(0, 0, 0, 0.04), 0 4px 16px rgba(0, 0, 0, 0.06)`.
3. **Floating Windows & App Panels (Level 2):** In-focus windows float over inactive layers using:
   `0 8px 16px rgba(0, 0, 0, 0.08), 0 16px 32px rgba(0, 0, 0, 0.12)`.
   Inactive windows drop down to `0 2px 6px rgba(0, 0, 0, 0.06)` with a muted top title bar.
4. **Transient Flyouts & Start Menu (Level 3):** Highest desktop layer utilizing Acrylic translucency:
   `0 12px 28px rgba(0, 0, 0, 0.16), 0 24px 64px rgba(0, 0, 0, 0.2)`.

### Stroke Architecture
Elevated surfaces feature a dual-edge lighting effect:
- **Top / Left internal highlight:** `inset 0 1px 0 rgba(255, 255, 255, 0.4)`.
- **Outer boundary:** `1px solid rgba(0, 0, 0, 0.09)`.

## Shapes

The geometric identity relies on softened rectangular forms, creating tactile surfaces while preserving screen real estate efficiency.

- **Primary Radius (`roundedness: 2`, 8px):** Standard interactive components, buttons, text inputs, list selections, and cards.
- **Large Radius (`rounded-lg`, 12px):** Top-level system surfaces including application window frames, the Start Menu dialog, action center panels, and notifications.
- **Inner Corner Rule:** When an inner card sits inside an outer window frame with 12px padding, the child element scales down to 6px or 8px radius to maintain concentric geometry.
- **Controls & Chips:** Segmented pills and status chips adopt full circular radiuses (9999px) where visual differentiation from regular buttons is needed.

## Components

### Buttons
- **Standard (Neutral):** Background `rgba(255, 255, 255, 0.7)`, border `1px solid rgba(0, 0, 0, 0.08)`, border-radius `6px`, height `32px`, padding `0 12px`. Hover shifts background to `rgba(255, 255, 255, 0.9)` with subtle bottom shadow `0 1px 2px rgba(0,0,0,0.06)`. Active state triggers `transform: scale(0.98)` and background `rgba(240, 240, 240, 0.9)`.
- **Accent (Primary):** Solid `#0078D4` background, white label, bottom subtle stroke `inset 0 -1px 0 rgba(0, 0, 0, 0.2)`. Hover `#2B88D8`, active `#106EBE`.
- **Subtle / Ghost:** Transparent background; gains `rgba(0, 0, 0, 0.04)` on hover.

### Window Frames & Titlebars
- **Frame:** Corner radius `12px`, border `1px solid rgba(0, 0, 0, 0.1)`, overflow hidden.
- **Titlebar:** 36px height, draggable handle, document title centered or left-aligned at `12px` font size.
- **Window Controls (Minimize, Maximize, Close):** Clustered right. 46px wide by 32px high hit areas. Minimize and Maximize turn `rgba(0, 0, 0, 0.05)` on hover. Close button turns `#E81123` with white iconography.

### Cards & Grid Containers
- **Content Cards:** Background `rgba(255, 255, 255, 0.7)`, border `1px solid rgba(0, 0, 0, 0.05)`, border-radius `8px`, internal padding `12px 16px`. Hover introduces a 1px border shift to `rgba(0, 0, 0, 0.1)` and 2px lift shadow.

### Input Fields & Search Bars
- **Surface:** Background `rgba(255, 255, 255, 0.8)`, border `1px solid rgba(0, 0, 0, 0.12)`, border-bottom `2px solid rgba(0, 0, 0, 0.4)`, radius `6px`, height `32px`.
- **Focus State:** Bottom border switches immediately to `#0078D4` (2px height), background becomes solid `#FFFFFF`.

### Checkboxes & Radios
- **Check Box:** 18px square, border radius `4px`, `1px solid rgba(0, 0, 0, 0.3)`. Checked state fills `#0078D4` with white check glyph.
- **Radio Button:** 18px circle, checked state renders an inner 8px dot colored `#0078D4`.

### Lists & Navigation Trees
- **List Item:** 32px height, border-radius `4px`, padding `0 8px`. Hover background `rgba(0, 0, 0, 0.037)`. Selected item background `rgba(0, 0, 0, 0.06)` with a 3px wide by 16px high `#0078D4` indicator pill anchored to the left margin.

### Taskbar & Start Menu
- **Taskbar:** 48px height, centered 40px icon tiles with 3px blue pill underneath running apps. Active window features an extended 16px pill indicator.
- **Start Menu:** Grid of pinned app shortcuts (6 columns), followed by a 2-column recommended files section, terminated by an account/power status footer bar.
# 🎨 TUI Creation Workflow Guide

## Overview

The AGSi TUI editor now includes a **complete creation workflow** that lets you create materials, models, and components interactively within the terminal UI.

## Quick Start

```bash
# Launch TUI editor
agsipls tui

# Or open an existing file
agsipls tui project.agsi.json
```

## Keyboard Shortcuts

### Navigation
- `d` - Document info view
- `m` - Models view
- `a` - Materials view
- `c` - Components view
- `↑↓` - Navigate lists
- `s` - Save file
- `q` - Quit
- `?` - Help

### Creation
- `n` - Start creation workflow (context-aware)
- `Esc` - Cancel creation
- `Enter` - Confirm/Next step
- `Backspace` - Delete character (text input)

## Creation Workflows

### 1. Creating a Material ⭐ NEW

**Steps:**
1. Navigate to Materials view (`a`)
2. Press `n` to start creation
3. **Enter ID**: Type material ID (e.g., "MAT001")
4. Press `Enter`
5. **Enter Name**: Type material name (e.g., "London Clay")
6. Press `Enter`
7. **Select Type**: Use `↑↓` to choose type, press `Enter`
   - Soil
   - Rock
   - Fill
   - Made Ground
   - Anthropogenic
   - Water
   - Void
   - Unknown
8. **Enter Description** (optional): Type description or just press `Enter`
9. Material created! ✅

**Example:**
```
ID: MAT001
Name: London Clay
Type: Soil
Description: Stiff blue-grey clay
```

### 2. Creating a Model ⭐ NEW

**Steps:**
1. Navigate to Models view (`m`)
2. Press `n` to start creation
3. **Enter ID**: Type model ID (e.g., "MODEL001")
4. Press `Enter`
5. **Enter Name**: Type model name (e.g., "Site Model")
6. Press `Enter`
7. **Select Type**: Use `↑↓` to choose, press `Enter`
   - Stratigraphic
   - Structural
   - Hydrogeological
   - Geotechnical
   - Environmental
   - Composite
8. **Select Dimension**: Use `↑↓` to choose, press `Enter`
   - 1D - One Dimensional
   - 2D - Two Dimensional
   - 3D - Three Dimensional
9. **Enter Description** (optional): Type description or press `Enter`
10. Model created! ✅

**Example:**
```
ID: MODEL001
Name: Silvertown Site Model
Type: Geotechnical
Dimension: 3D
Description: Ground investigation model
```

### 3. Creating a Component ⭐ NEW

**Steps:**
1. Navigate to Components view (`c`)
2. Press `n` to start creation
3. **Select Model**: Use `↑↓` to choose which model, press `Enter`
4. **Enter ID**: Type component ID (e.g., "COMP001")
5. Press `Enter`
6. **Enter Name**: Type component name (e.g., "Clay Layer")
7. Press `Enter`
8. **Select Type**: Use `↑↓` to choose, press `Enter`
   - Layer
   - Lens
   - Volume
   - Fault
   - Intrusion
   - Boundary
9. **Enter Material ID**: Type the material ID (e.g., "MAT001")
10. Press `Enter`
11. Component created! ✅

**Example:**
```
Model: MODEL001
ID: COMP001
Name: Upper Clay Layer
Type: Layer
Material ID: MAT001
```

## Workflow Examples

### Example 1: Complete Ground Model from Scratch

```bash
# 1. Launch TUI
agsipls tui new-project.json

# 2. Create Model first
# Press 'm' -> 'n'
# ID: MODEL001
# Name: My Site
# Type: Geotechnical (↓ then Enter)
# Dimension: 2D (↓ then Enter)
# Description: Site investigation model

# 3. Create Materials
# Press 'a' -> 'n'
# ID: MAT001
# Name: Clay
# Type: Soil (Enter)
# Description: Stiff clay

# Press 'n' again for more materials
# ID: MAT002
# Name: Sand
# Type: Soil
# Description: Dense sand

# 4. Create Components
# Press 'c' -> 'n'
# Select Model: MODEL001 (Enter)
# ID: LAYER001
# Name: Clay Layer
# Type: Layer (Enter)
# Material ID: MAT001

# 5. Save
# Press 's' to save

# 6. Quit
# Press 'q'
```

### Example 2: Adding Materials to Existing Model

```bash
# 1. Open existing file
agsipls tui existing.json

# 2. Go to materials view
# Press 'a'

# 3. Create new material
# Press 'n'
# Follow prompts...

# 4. Save
# Press 's'
```

## Visual Feedback

The TUI provides real-time visual feedback:

- **Yellow text**: Currently being edited
- **Green text**: Completed field
- **Cyan**: Section headers
- **Dialog popups**: Centered creation forms
- **Status bar**: Instructions and feedback
- **Success messages**: "✅ Material created!"

## Tips & Tricks

### 1. Fast Material Creation
- Have your material list ready
- Create all materials first, then models
- Use consistent ID prefixes (MAT001, MAT002, etc.)

### 2. Navigation
- Use arrow keys for all selections
- `Esc` cancels at any step without losing progress
- Status bar shows current step

### 3. Organization
- Create models before components
- Name materials descriptively
- Use clear, unique IDs

### 4. Error Prevention
- Material IDs must be unique
- Components need valid material IDs
- Models are required for components

### 5. Workflow
```
Models → Materials → Components → Save
```

## Troubleshooting

### "❌ Create a model first!"
- You need at least one model before creating components
- Press `m`, then `n` to create a model

### Can't see creation dialog
- Make sure terminal is at least 60x20 characters
- Try maximizing terminal window

### Lost progress in creation
- Press `Esc` to cancel
- Restart creation workflow with `n`

### Material ID not found
- Check material IDs in materials view (`a`)
- Ensure material exists before referencing it

## Advanced Features

### Auto-Model Creation
- If you create a material with no models, a default model is created
- Default: "MODEL001", "Default Model", Geotechnical, 2D

### Component Geometry
- Components are created with placeholder point geometry at (0,0,0)
- Edit geometry data separately via CLI or JSON

### Multi-Step Forms
Each workflow has multiple steps with validation:
1. Text input fields (ID, name, description)
2. Type selection (dropdown lists)
3. Confirmation (automatic on final step)

## Integration with CLI

Created items can be further edited via CLI:

```bash
# Create in TUI, then enhance via CLI
agsipls tui project.json  # Create materials
agsipls edit project.json # Add properties
```

## Keyboard Reference Card

```
┌─────────────────────────────────────┐
│         TUI Creation Keys           │
├─────────────────────────────────────┤
│ Navigation                          │
│   d,m,a,c  - Switch views           │
│   ↑↓       - Navigate lists         │
│                                     │
│ Creation                            │
│   n        - New item               │
│   Enter    - Confirm/Next           │
│   Esc      - Cancel                 │
│   Backspace- Delete char            │
│                                     │
│ General                             │
│   s        - Save                   │
│   q        - Quit                   │
│   ?        - Help                   │
└─────────────────────────────────────┘
```

## Summary

The TUI creation workflow provides:

✅ **Interactive forms** for all major entities
✅ **Type-safe selections** with dropdown lists
✅ **Real-time feedback** with color coding
✅ **Multi-step wizards** guiding you through creation
✅ **Validation** preventing invalid data
✅ **Undo capability** via Esc key
✅ **Visual dialogs** for better UX

**Result**: Create complete ground models without leaving the terminal! 🎉

---

**Next**: Try `agsipls tui` and press `n` to start creating!

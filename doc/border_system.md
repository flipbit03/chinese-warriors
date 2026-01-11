# Terrain Border System

This document describes the full mechanism of how terrain borders work in Chinese Warriors, for reimplementation in other systems.

## Overview

The terrain border system automatically calculates and renders visual borders between different terrain types. It uses a **strength-based precedence system** where terrains are ranked by a numerical strength value. Only weaker terrains display borders when adjacent to stronger terrains.

**Key principle**: A tile only draws a border toward a neighbor if that neighbor's terrain is *stronger* than its own.

---

## Key Files

| File | Purpose |
|------|---------|
| `src/world/tile/border/mod.rs` | Core border calculation algorithm |
| `src/world/tile/border/structs.rs` | Data structures (TileBorder, BorderSpec, TileBorderType) |
| `src/world/tile/border/neighbor.rs` | Corner/edge detection helper |
| `src/world/tile/terrain/mod.rs` | Terrain definitions and BaseTerrain enum |
| `src/world/tile/terrain/generator/util.rs` | Strength calculation from biome ordering |
| `src/world/spawner/drawer/draw_functions.rs` | Border rendering with Z-ordering |

---

## Data Structures

### Terrain

```rust
pub struct Terrain {
    pub name: String,
    pub strength: usize,           // Precedence value (0 = weakest)
    pub base: BaseTerrain,         // Visual type (Grass, Stone, Water, etc.)
    pub color: Option<ColorTriplet>,
    pub walkable: bool,
    pub move_speed_multiplier: f32,
}
```

The `strength` field determines border precedence. Higher strength = terrain is "on top" and does not show borders.

### Border Structures

```rust
pub struct TileBorder {
    pub terrain: Terrain,      // The stronger terrain this border represents
    pub spec: BorderSpec,      // Which corners/edges have borders
}

pub struct BorderSpec {
    pub upper_left: Option<TileBorderType>,
    pub upper_right: Option<TileBorderType>,
    pub bottom_left: Option<TileBorderType>,
    pub bottom_right: Option<TileBorderType>,
}

pub enum TileBorderType {
    VerticalWall = 1,
    HorizontalWall = 2,
    InnerCorner = 4,
    OuterCorner = 8,
    Diagonal = 16,
}
```

Each tile is divided into 4 quadrants. Each quadrant can have one border type (or none).

### WorldTile

```rust
pub struct WorldTile {
    pub position: TilePosition,
    pub terrain: Terrain,
    pub borders: Vec<TileBorder>,  // Multiple borders for different stronger terrains
}
```

A tile can have **multiple borders** when adjacent to multiple stronger terrain types.

---

## Strength Calculation and Precedence

### The Formula

Strength is calculated hierarchically based on biome order and terrain position within the biome:

```
strength = biome_order * 10 + terrain_order_in_biome
```

This is computed in `src/world/tile/terrain/generator/util.rs`.

### Example

Given 3 biomes in order:

| Biome | Biome Order | Terrain | Terrain Order | Strength |
|-------|-------------|---------|---------------|----------|
| RockDesert | 0 | Sand | 0 | 0 |
| RockDesert | 0 | SandStone | 1 | 1 |
| RockDesert | 0 | RedStone | 2 | 2 |
| Desert | 1 | Sand | 0 | 10 |
| Desert | 1 | SandStone | 1 | 11 |
| Plains | 2 | ShallowWater | 0 | 20 |
| Plains | 2 | Clay | 1 | 21 |
| Plains | 2 | Grass | 2 | 22 |

### Why This Works

- Different biomes can use the same base terrain (e.g., "Sand" in RockDesert vs Desert)
- But they have different strengths due to biome ordering
- When regions transition between biomes, borders appear where stronger biome terrains meet weaker ones
- The multiplier (10) leaves room for up to 10 terrains per biome

### Precedence Rule

A tile with terrain strength `S` only displays borders toward neighbors with strength `> S`.

The **strongest terrain** (highest index in global array) never displays any borders.

---

## Border Calculation Algorithm

Located in `TileBorder::from()` in `src/world/tile/border/mod.rs`.

### Input

A 3x3 matrix of neighbors centered on the current tile:

```
[top_left]    [top]    [top_right]
[left]       [center]  [right]
[bottom_left][bottom]  [bottom_right]
```

### Phase 1: Find Stronger Terrains

1. Look up the center tile's terrain index in the global terrains array
2. If it's the strongest terrain, return empty (no borders)
3. Otherwise, iterate through all terrains with higher strength

```rust
// Pseudocode
if our_terrain_index + 1 == terrain_count {
    return Vec::new();  // Strongest terrain gets no borders
}

for each stronger_terrain in terrains[our_index + 1..] {
    // Calculate borders for this stronger terrain
}
```

### Phase 2: Check Neighbor Presence

For each stronger terrain, check which of the 8 neighbors have that terrain:

```rust
let top = neighbor_top.terrain.base == stronger_terrain.base;
let right = neighbor_right.terrain.base == stronger_terrain.base;
let bottom = neighbor_bottom.terrain.base == stronger_terrain.base;
let left = neighbor_left.terrain.base == stronger_terrain.base;
// ... and 4 diagonals
```

### Phase 3A: Four-Directional Pattern Matching

When 2+ cardinal neighbors have the stronger terrain, use pattern matching:

| Pattern (T=top, R=right, B=bottom, L=left) | Result |
|--------------------------------------------|--------|
| T + R + B + L (all 4) | All inner corners |
| T + R + B (missing L) | Open-C shape (left open) |
| T + B (opposite pair) | Vertical tunnel |
| L + R (opposite pair) | Horizontal tunnel |
| T + L (diagonal pair) | Diagonal + possible outer corner |

This is implemented in `get_tileborder_from_stronger_terrain()`.

### Phase 3B: Corner Detection

For edges not covered by pattern matching, each corner is checked independently using the function `get_border_from_neighbor_effects()`:

```rust
pub fn get_border_from_neighbor_effects(
    horizontal_neighbor: bool,  // Is left/right neighbor stronger?
    vertical_neighbor: bool,    // Is top/bottom neighbor stronger?
    diagonal_neighbor: bool,    // Is diagonal neighbor stronger?
) -> Option<TileBorderType> {
    match (horizontal_neighbor, vertical_neighbor, diagonal_neighbor) {
        (true, true, _) => Some(InnerCorner),
        (true, false, _) => Some(VerticalWall),
        (false, true, _) => Some(HorizontalWall),
        (false, false, true) => Some(OuterCorner),
        (false, false, false) => None,
    }
}
```

**Corner mapping:**

| Corner | Checks These Neighbors |
|--------|------------------------|
| Upper-Left | left, top, top_left |
| Upper-Right | right, top, top_right |
| Bottom-Left | left, bottom, bottom_left |
| Bottom-Right | right, bottom, bottom_right |

### Logic Explanation

- **InnerCorner**: When both adjacent cardinal neighbors are stronger (forms an L-shape)
- **VerticalWall**: When only the horizontal neighbor is stronger
- **HorizontalWall**: When only the vertical neighbor is stronger
- **OuterCorner**: When only the diagonal neighbor is stronger (isolated corner poke)
- **None**: When no relevant neighbors are stronger

---

## Complete Example

**Scenario**: A Grass tile (strength 22) with ShallowWater (strength 20) to the right.

Wait, that's backwards. Let's correct: Grass (strength 22) is *stronger* than Water (strength 20). So the Water tile would show borders toward Grass, not the other way around.

**Corrected Scenario**: A ShallowWater tile (strength 20) with Grass (strength 22) to the right.

1. **Find stronger terrains**: Grass (22) is stronger than ShallowWater (20)
2. **Check neighbors**: Grass is to the right
3. **Pattern matching**: Single direction doesn't trigger patterns
4. **Corner detection**:
   - Upper-right: (right=true, top=false) → VerticalWall
   - Bottom-right: (right=true, bottom=false) → VerticalWall
   - Upper-left: None
   - Bottom-left: None
5. **Result**:
   ```rust
   TileBorder {
       terrain: Grass,
       spec: BorderSpec {
           upper_left: None,
           upper_right: Some(VerticalWall),
           bottom_right: Some(VerticalWall),
           bottom_left: None,
       }
   }
   ```

---

## Multiple Borders Per Tile

A tile can have multiple `TileBorder` entries when adjacent to multiple different stronger terrains.

**Example**: A Sand tile (strength 1) next to both Water (strength 20) and Grass (strength 22):

```rust
borders = [
    TileBorder { terrain: Water, spec: BorderSpec { ... } },
    TileBorder { terrain: Grass, spec: BorderSpec { ... } },
]
```

Each border is rendered as a separate layer with proper Z-ordering.

---

## Rendering

### Border Textures

There are **20 border sprites per base terrain**, stored in:
```
art/terrain/{base_terrain}/border/border{0-19}.png
```

### Texture Index Mapping

The `get_texture_indexes()` method maps border types to sprite indices:

| Index Range | Border Type |
|-------------|-------------|
| 0-5 | Horizontal walls (6 variants for different corners) |
| 6-7 | Vertical walls |
| 8-11 | Inner corners (4 orientations) |
| 12-15 | Outer corners (4 orientations) |
| 16-19 | Diagonals (4 orientations) |

Specific corner mappings:

```rust
// Upper-left corner
HorizontalWall => 0
VerticalWall => 7
InnerCorner => 8
OuterCorner => 12
Diagonal => 18

// Upper-right corner
HorizontalWall => 1
VerticalWall => 2
InnerCorner => 9
OuterCorner => 13
Diagonal => 16

// Bottom-right corner
HorizontalWall => 4
VerticalWall => 3
InnerCorner => 10
OuterCorner => 14
Diagonal => 17

// Bottom-left corner
HorizontalWall => 5
VerticalWall => 6
InnerCorner => 11
OuterCorner => 15
Diagonal => 19
```

### Z-Ordering

Rendering is layered to prevent z-fighting:

1. **Base terrain**: `Z = terrain.strength / 10000.0`
2. **Decorations**: `Z = base + 0.00001`
3. **Borders**: `Z = base + (terrain_order / 1000.0) + 0.0001`

This ensures:
- Stronger terrains render above weaker ones
- Borders render above their parent terrain
- Multiple border layers stack correctly

---

## Reimplementation Guide

To reimplement this system:

### 1. Define Terrain Strengths

Assign each terrain type a unique strength value. Higher = stronger = renders on top.

### 2. Build Neighbor Matrix

For each tile, gather the 8 surrounding neighbors with their terrain types.

### 3. Calculate Borders

For each terrain stronger than the current tile:
1. Check which neighbors have that terrain
2. Apply pattern matching for multi-directional cases
3. Apply corner detection for remaining edges
4. Store the resulting BorderSpec

### 4. Render Borders

For each TileBorder in the tile's border list:
1. Get the texture indices for each corner
2. Render with appropriate Z-offset based on terrain strength

### 5. Key Considerations

- **Same base terrain, different strength**: This allows biome transitions
- **Empty borders for strongest terrain**: Optimization and visual correctness
- **Multiple borders per tile**: Essential for complex terrain intersections
- **Z-ordering by strength**: Prevents visual artifacts

---

## Performance Notes

- Border calculation: O(terrain_count × 8) per tile - done once at generation
- Multiple sprites per tile, but batched by the renderer
- Chunk system (3×3 tiles) reduces draw call overhead
- Terrains are pre-cached in TerrainGenerator for fast lookup

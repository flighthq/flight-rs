# Automatic Rust Generation

Upstream commit: `5d24729f7360475e28a105ae0caeeaa2e1328260`

| Metric | Count |
| --- | ---: |
| Inventoried packages | 131 |
| Default-generated packages | 125 |
| Emittable packages | 2 |
| Blocked packages | 123 |
| Cultivated packages | 1 |
| Host-bound packages | 4 |
| Excluded packages | 1 |
| Source/package blockers | 896 |

| Package | Disposition | Status | Sources emitted/attempted | API generated/expected | Missing | Dependents direct/transitive | Opaque sources | Blockers | Promoted |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | :---: |
| `@flighthq/accessibility` | generated | blocked | 1/2 | 0/8 | 8 | 1/1 | 0 | 2 | no |
| `@flighthq/adjustments` | generated | blocked | 4/19 | 9/49 | 40 | 6/25 | 1 | 16 | no |
| `@flighthq/animation` | generated | blocked | 2/4 | 5/18 | 13 | 3/7 | 1 | 3 | no |
| `@flighthq/app` | generated | blocked | 1/2 | 0/42 | 42 | 4/4 | 0 | 2 | no |
| `@flighthq/application` | generated | blocked | 1/3 | 0/83 | 83 | 3/3 | 0 | 3 | no |
| `@flighthq/assets` | generated | blocked | 1/2 | 0/10 | 10 | 1/1 | 0 | 2 | no |
| `@flighthq/audio` | generated | blocked | 2/4 | 10/20 | 10 | 2/2 | 1 | 3 | no |
| `@flighthq/binpack` | generated | blocked | 1/2 | 0/1 | 1 | 1/1 | 0 | 2 | no |
| `@flighthq/bitmapfont` | generated | blocked | 1/3 | 0/7 | 7 | 2/2 | 0 | 3 | no |
| `@flighthq/bitmapfont-formats` | generated | blocked | 2/5 | 1/4 | 3 | 1/1 | 0 | 4 | no |
| `@flighthq/bitmaptext` | generated | blocked | 1/3 | 0/15 | 15 | 1/1 | 0 | 3 | no |
| `@flighthq/camera` | generated | blocked | 6/10 | 13/31 | 18 | 4/4 | 0 | 5 | no |
| `@flighthq/camera2d` | generated | blocked | 6/8 | 6/8 | 2 | 1/1 | 0 | 3 | no |
| `@flighthq/capture` | generated | blocked | 2/3 | 5/10 | 5 | 1/1 | 1 | 2 | no |
| `@flighthq/clip` | generated | blocked | 1/2 | 0/23 | 23 | 1/1 | 0 | 2 | no |
| `@flighthq/clipboard` | generated | blocked | 1/2 | 0/32 | 32 | 4/4 | 0 | 2 | no |
| `@flighthq/clock` | generated | blocked | 11/12 | 12/14 | 2 | 1/1 | 0 | 2 | no |
| `@flighthq/collision` | generated | blocked | 3/6 | 3/19 | 16 | 1/1 | 0 | 4 | no |
| `@flighthq/color` | generated | blocked | 6/10 | 20/32 | 12 | 10/29 | 0 | 5 | no |
| `@flighthq/connectivity` | generated | blocked | 1/2 | 0/14 | 14 | 2/2 | 0 | 2 | no |
| `@flighthq/debug` | generated | blocked | 1/3 | 0/9 | 9 | 1/1 | 0 | 3 | no |
| `@flighthq/device` | generated | blocked | 1/2 | 0/14 | 14 | 2/2 | 0 | 2 | no |
| `@flighthq/dialog` | generated | blocked | 1/2 | 0/15 | 15 | 5/5 | 0 | 2 | no |
| `@flighthq/displayobject` | generated | blocked | 2/8 | 3/46 | 43 | 14/24 | 0 | 7 | no |
| `@flighthq/displayobject-canvas` | generated | blocked | 7/31 | 10/94 | 84 | 5/5 | 5 | 25 | no |
| `@flighthq/displayobject-dom` | host-bound | host-bound | 0/0 | 0/58 | 58 | 1/1 | 0 | 0 | no |
| `@flighthq/displayobject-gl` | generated | blocked | 5/28 | 5/89 | 85 | 1/1 | 0 | 24 | no |
| `@flighthq/displayobject-wgpu` | generated | blocked | 6/29 | 8/95 | 88 | 1/1 | 1 | 24 | no |
| `@flighthq/easing` | generated | emittable | 20/20 | 48/48 | 0 | 2/3 | 0 | 0 | yes |
| `@flighthq/effects` | generated | blocked | 7/72 | 18/112 | 94 | 4/4 | 0 | 66 | no |
| `@flighthq/effects-canvas` | generated | blocked | 7/48 | 19/102 | 88 | 1/1 | 5 | 42 | no |
| `@flighthq/effects-gl` | generated | blocked | 5/58 | 15/135 | 120 | 1/1 | 3 | 54 | no |
| `@flighthq/effects-wgpu` | generated | blocked | 3/56 | 9/128 | 119 | 1/1 | 1 | 54 | no |
| `@flighthq/entity` | generated | blocked | 2/6 | 3/12 | 9 | 20/61 | 0 | 5 | no |
| `@flighthq/filesystem` | generated | blocked | 1/2 | 0/43 | 43 | 2/2 | 0 | 2 | no |
| `@flighthq/flow` | generated | blocked | 5/10 | 4/9 | 5 | 1/1 | 0 | 6 | no |
| `@flighthq/font` | generated | blocked | 2/8 | 1/15 | 14 | 1/1 | 0 | 7 | no |
| `@flighthq/geolocation` | generated | blocked | 1/2 | 0/12 | 12 | 2/2 | 0 | 2 | no |
| `@flighthq/geometry` | generated | blocked | 9/27 | 32/377 | 345 | 40/53 | 0 | 19 | no |
| `@flighthq/glyphatlas` | generated | blocked | 4/7 | 5/14 | 9 | 1/1 | 1 | 4 | no |
| `@flighthq/haptics` | generated | blocked | 1/2 | 0/13 | 13 | 2/2 | 0 | 2 | no |
| `@flighthq/host-capacitor` | host-bound | host-bound | 0/0 | 0/63 | 63 | 0/0 | 0 | 0 | no |
| `@flighthq/host-electron` | host-bound | host-bound | 0/0 | 0/57 | 57 | 0/0 | 0 | 0 | no |
| `@flighthq/host-tauri` | host-bound | host-bound | 0/0 | 0/51 | 51 | 0/0 | 0 | 0 | no |
| `@flighthq/image` | generated | blocked | 1/3 | 0/20 | 20 | 11/24 | 0 | 3 | yes |
| `@flighthq/image-codec` | generated | blocked | 2/8 | 1/16 | 15 | 3/26 | 1 | 7 | no |
| `@flighthq/input` | generated | blocked | 1/2 | 0/40 | 40 | 1/1 | 0 | 2 | no |
| `@flighthq/interaction` | generated | blocked | 7/16 | 20/83 | 63 | 1/1 | 1 | 10 | no |
| `@flighthq/intl` | generated | blocked | 1/8 | 0/14 | 14 | 1/1 | 0 | 8 | no |
| `@flighthq/ipc` | generated | blocked | 1/2 | 0/17 | 17 | 2/2 | 0 | 2 | no |
| `@flighthq/keyboard` | generated | blocked | 1/2 | 0/20 | 20 | 2/2 | 0 | 2 | no |
| `@flighthq/lifecycle` | generated | blocked | 1/2 | 0/13 | 13 | 1/1 | 0 | 2 | no |
| `@flighthq/lighting` | generated | blocked | 2/11 | 4/37 | 33 | 1/1 | 1 | 10 | no |
| `@flighthq/loader` | generated | blocked | 1/2 | 0/13 | 13 | 3/3 | 0 | 2 | no |
| `@flighthq/log` | generated | blocked | 1/2 | 0/65 | 65 | 7/16 | 0 | 2 | no |
| `@flighthq/materials` | generated | blocked | 3/12 | 10/68 | 58 | 7/28 | 0 | 10 | no |
| `@flighthq/math` | generated | blocked | 12/16 | 43/73 | 30 | 4/4 | 1 | 5 | no |
| `@flighthq/media` | generated | blocked | 1/4 | 0/42 | 42 | 1/1 | 0 | 4 | no |
| `@flighthq/mediasession` | generated | blocked | 1/2 | 0/10 | 10 | 1/1 | 0 | 2 | no |
| `@flighthq/menu` | generated | blocked | 1/3 | 0/17 | 17 | 3/3 | 0 | 3 | no |
| `@flighthq/mesh` | generated | blocked | 7/12 | 28/67 | 39 | 6/20 | 0 | 6 | no |
| `@flighthq/motionpath` | generated | blocked | 7/8 | 6/7 | 1 | 1/1 | 1 | 2 | no |
| `@flighthq/movieclip` | generated | blocked | 1/3 | 0/23 | 23 | 1/1 | 0 | 3 | no |
| `@flighthq/net` | generated | blocked | 1/2 | 0/4 | 4 | 1/1 | 0 | 2 | no |
| `@flighthq/node` | generated | blocked | 5/16 | 44/105 | 61 | 23/32 | 1 | 12 | no |
| `@flighthq/notification` | generated | blocked | 1/2 | 0/26 | 26 | 4/4 | 0 | 2 | no |
| `@flighthq/particleemitter` | generated | blocked | 5/11 | 4/51 | 47 | 1/1 | 3 | 7 | no |
| `@flighthq/particles` | generated | blocked | 5/11 | 9/50 | 41 | 3/3 | 1 | 7 | no |
| `@flighthq/particles-formats` | generated | blocked | 5/21 | 11/79 | 68 | 1/1 | 1 | 17 | no |
| `@flighthq/path` | generated | blocked | 18/23 | 27/50 | 23 | 8/8 | 13 | 6 | no |
| `@flighthq/path-boolean` | generated | blocked | 3/8 | 4/12 | 9 | 1/1 | 0 | 6 | no |
| `@flighthq/path-formats` | generated | blocked | 1/2 | 0/3 | 3 | 1/1 | 0 | 2 | no |
| `@flighthq/permissions` | generated | blocked | 1/2 | 0/5 | 5 | 1/1 | 0 | 2 | no |
| `@flighthq/picking` | generated | blocked | 1/2 | 0/6 | 6 | 1/1 | 0 | 2 | no |
| `@flighthq/platform` | generated | blocked | 1/2 | 0/16 | 16 | 3/3 | 0 | 2 | no |
| `@flighthq/power` | generated | blocked | 1/2 | 0/19 | 19 | 2/2 | 0 | 2 | no |
| `@flighthq/protocol` | generated | blocked | 1/2 | 0/20 | 20 | 2/2 | 0 | 2 | no |
| `@flighthq/render` | generated | blocked | 9/17 | 18/63 | 45 | 9/13 | 4 | 9 | no |
| `@flighthq/render-gl` | generated | blocked | 12/24 | 18/75 | 58 | 4/4 | 6 | 13 | no |
| `@flighthq/render-wgpu` | generated | blocked | 5/18 | 7/68 | 61 | 5/5 | 3 | 14 | no |
| `@flighthq/scene` | generated | blocked | 10/14 | 20/43 | 23 | 6/6 | 3 | 5 | no |
| `@flighthq/scene-formats` | generated | blocked | 6/16 | 78/15 | 14 | 2/2 | 1 | 11 | no |
| `@flighthq/scene-gl` | generated | blocked | 8/53 | 21/184 | 163 | 1/1 | 2 | 46 | no |
| `@flighthq/scene-resources` | generated | blocked | 2/16 | 5/37 | 32 | 1/1 | 1 | 15 | no |
| `@flighthq/scene-wgpu` | generated | blocked | 4/42 | 9/140 | 131 | 1/1 | 0 | 39 | no |
| `@flighthq/screen` | generated | blocked | 1/2 | 0/31 | 31 | 2/2 | 0 | 2 | no |
| `@flighthq/sdk` | generated | blocked | 14/14 | 0/5923 | 5923 | 0/0 | 0 | 1 | no |
| `@flighthq/sensors` | generated | blocked | 1/2 | 0/32 | 32 | 1/1 | 0 | 2 | no |
| `@flighthq/shading` | generated | blocked | 10/17 | 15/37 | 22 | 2/2 | 0 | 8 | no |
| `@flighthq/shape` | generated | blocked | 1/7 | 0/42 | 42 | 7/8 | 0 | 7 | no |
| `@flighthq/shape-formats` | generated | blocked | 1/2 | 0/5 | 5 | 1/1 | 0 | 2 | no |
| `@flighthq/share` | generated | blocked | 1/2 | 0/14 | 14 | 2/2 | 0 | 2 | no |
| `@flighthq/shell` | generated | blocked | 1/2 | 0/14 | 14 | 3/3 | 0 | 2 | no |
| `@flighthq/shortcut` | generated | blocked | 1/2 | 0/26 | 26 | 3/3 | 0 | 2 | no |
| `@flighthq/signals` | generated | blocked | 2/6 | 1/14 | 13 | 42/72 | 1 | 5 | no |
| `@flighthq/skeleton3d` | generated | emittable | 6/6 | 16/16 | 0 | 3/16 | 1 | 0 | no |
| `@flighthq/snapshot` | generated | blocked | 1/5 | 0/4 | 4 | 1/1 | 0 | 5 | no |
| `@flighthq/socket` | generated | blocked | 1/2 | 0/11 | 11 | 1/1 | 0 | 2 | no |
| `@flighthq/spatial` | generated | blocked | 2/3 | 9/10 | 1 | 2/2 | 0 | 2 | no |
| `@flighthq/spring` | generated | blocked | 7/8 | 11/12 | 1 | 1/1 | 0 | 2 | no |
| `@flighthq/sprite` | generated | blocked | 1/4 | 0/64 | 64 | 3/3 | 0 | 4 | no |
| `@flighthq/spritesheet` | generated | blocked | 1/8 | 0/32 | 32 | 2/2 | 0 | 8 | no |
| `@flighthq/spritesheet-formats` | generated | blocked | 6/16 | 29/55 | 26 | 1/1 | 3 | 11 | no |
| `@flighthq/statusbar` | generated | blocked | 1/2 | 0/16 | 16 | 2/2 | 0 | 2 | no |
| `@flighthq/storage` | generated | blocked | 1/2 | 0/39 | 39 | 2/2 | 0 | 2 | no |
| `@flighthq/surface` | cultivated | cultivated | 0/0 | 0/136 | 136 | 6/9 | 0 | 0 | yes |
| `@flighthq/text` | generated | blocked | 3/6 | 4/86 | 82 | 8/9 | 0 | 4 | no |
| `@flighthq/text-markup` | generated | blocked | 2/5 | 1/8 | 7 | 1/1 | 1 | 4 | no |
| `@flighthq/textbidi` | generated | blocked | 4/5 | 5/6 | 1 | 1/1 | 0 | 2 | no |
| `@flighthq/textinput` | generated | blocked | 1/5 | 0/55 | 55 | 5/6 | 0 | 5 | no |
| `@flighthq/textlayout` | generated | blocked | 9/13 | 23/47 | 25 | 9/11 | 1 | 5 | no |
| `@flighthq/textsegment` | generated | blocked | 3/4 | 8/11 | 3 | 1/1 | 0 | 2 | no |
| `@flighthq/textshaper` | generated | blocked | 4/9 | 17/31 | 16 | 3/12 | 0 | 6 | no |
| `@flighthq/textshaper-canvas` | generated | blocked | 1/2 | 0/3 | 3 | 1/1 | 0 | 2 | no |
| `@flighthq/texture` | generated | blocked | 1/5 | 0/42 | 42 | 5/6 | 0 | 5 | no |
| `@flighthq/texture-formats` | generated | blocked | 3/9 | 2/6 | 4 | 1/1 | 0 | 7 | no |
| `@flighthq/textureatlas` | generated | blocked | 1/4 | 0/20 | 20 | 8/13 | 0 | 4 | no |
| `@flighthq/textureatlas-formats` | generated | blocked | 3/8 | 20/29 | 9 | 2/2 | 2 | 6 | no |
| `@flighthq/tilemap-formats` | generated | blocked | 3/9 | 6/16 | 10 | 1/1 | 0 | 7 | no |
| `@flighthq/tileset` | generated | blocked | 1/3 | 0/9 | 9 | 3/8 | 0 | 3 | no |
| `@flighthq/timeline` | generated | blocked | 1/2 | 0/16 | 16 | 2/2 | 0 | 2 | no |
| `@flighthq/tool-capture` | excluded | excluded | 0/0 | 0/57 | 57 | 0/0 | 0 | 0 | no |
| `@flighthq/tray` | generated | blocked | 1/2 | 0/23 | 23 | 3/3 | 0 | 2 | no |
| `@flighthq/tween` | generated | blocked | 4/9 | 8/35 | 28 | 2/2 | 3 | 6 | no |
| `@flighthq/types` | generated | blocked | 581/590 | 1213/1261 | 48 | 129/129 | 170 | 10 | yes |
| `@flighthq/updater` | generated | blocked | 1/2 | 0/23 | 23 | 2/2 | 0 | 2 | no |
| `@flighthq/useragent` | generated | blocked | 1/3 | 0/12 | 12 | 3/6 | 0 | 3 | no |
| `@flighthq/velocity` | generated | blocked | 3/4 | 2/20 | 18 | 3/3 | 1 | 2 | no |
| `@flighthq/video` | generated | blocked | 2/4 | 8/16 | 8 | 2/2 | 1 | 3 | no |
| `@flighthq/webcam` | generated | blocked | 1/3 | 0/10 | 10 | 1/1 | 0 | 3 | no |
| `@flighthq/xml` | generated | blocked | 2/3 | 4/7 | 3 | 5/5 | 1 | 2 | no |

## Blockers

### `@flighthq/accessibility`

- **package** `upstream/packages/accessibility/src`: Generated crate is missing 8 of 8 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/accessibility/src/accessibility.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")

### `@flighthq/adjustments`

- **package** `upstream/packages/adjustments/src`: Generated crate is missing 40 of 49 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/adjustments/src/brightnessContrastAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/adjustments/src/channelMixerAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/adjustments/src/colorBlindSimulationAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/adjustments/src/colorGradeAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/adjustments/src/colorLut.ts`: new-expression Rust lowering is not implemented: array
- **emission** `upstream/packages/adjustments/src/colorLutAdjustment.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/adjustments/src/colorMatrixMath.ts`: new-expression Rust lowering is not implemented: array
- **emission** `upstream/packages/adjustments/src/colorTransformAdjustment.ts`: object field colorMatrix is not present in structural type
- **emission** `upstream/packages/adjustments/src/exposureAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/adjustments/src/grayscaleAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/adjustments/src/hueSaturationAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/adjustments/src/invertAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/adjustments/src/liftGammaGainAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/adjustments/src/lookupTableGradeAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/adjustments/src/sepiaAdjustment.ts`: object literal requires an inferred structural type

### `@flighthq/animation`

- **package** `upstream/packages/animation/src`: Generated crate is missing 13 of 18 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/animation/src/animationPlayer.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/animation/src/animationTrack.ts`: new-expression Rust lowering is not implemented: array

### `@flighthq/app`

- **package** `upstream/packages/app/src`: Generated crate is missing 42 of 42 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/app/src/app.ts`: optional property Rust lowering is not implemented

### `@flighthq/application`

- **package** `upstream/packages/application/src`: Generated crate is missing 83 of 83 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/application/src/application.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/application/src/window.ts`: optional call Rust lowering is not implemented

### `@flighthq/assets`

- **package** `upstream/packages/assets/src`: Generated crate is missing 10 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/assets/src/assetLibrary.ts`: new-expression Rust lowering is not implemented: error

### `@flighthq/audio`

- **package** `upstream/packages/audio/src`: Generated crate is missing 10 of 20 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/audio/src/audioFormat.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Audio")
- **emission** `upstream/packages/audio/src/audioResourceFrom.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("AudioBuffer")

### `@flighthq/binpack`

- **package** `upstream/packages/binpack/src`: Generated crate is missing 1 of 1 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/binpack/src/packRectangles.ts`: optional property Rust lowering is not implemented

### `@flighthq/bitmapfont`

- **package** `upstream/packages/bitmapfont/src`: Generated crate is missing 7 of 7 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/bitmapfont/src/bitmapFont.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/bitmapfont/src/glyphSource.ts`: optional property Rust lowering is not implemented

### `@flighthq/bitmapfont-formats`

- **package** `upstream/packages/bitmapfont-formats/src`: Generated crate is missing 3 of 4 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/bitmapfont-formats/src/bitmapFontFnt.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/bitmapfont-formats/src/bitmapFontJson.ts`: try Rust lowering is not implemented
- **emission** `upstream/packages/bitmapfont-formats/src/bitmapFontRecord.ts`: optional property Rust lowering is not implemented

### `@flighthq/bitmaptext`

- **package** `upstream/packages/bitmaptext/src`: Generated crate is missing 15 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/bitmaptext/src/bitmapText.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/bitmaptext/src/updateBitmapText.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")

### `@flighthq/camera`

- **package** `upstream/packages/camera/src`: Generated crate is missing 18 of 31 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/camera/src/camera.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/camera/src/intersection.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/camera/src/projection.ts`: Math.tan Rust lowering is not implemented
- **emission** `upstream/packages/camera/src/shadowCamera.ts`: Math.hypot Rust lowering is not implemented

### `@flighthq/camera2d`

- **package** `upstream/packages/camera2d/src`: Generated crate is missing 2 of 8 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/camera2d/src/camera2d.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/camera2d/src/follow.ts`: optional property Rust lowering is not implemented

### `@flighthq/capture`

- **package** `upstream/packages/capture/src`: Generated crate is missing 5 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/capture/src/captureBaseline.ts`: object literal requires an inferred structural type

### `@flighthq/clip`

- **package** `upstream/packages/clip/src`: Generated crate is missing 23 of 23 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/clip/src/clipRegion.ts`: new-expression Rust lowering is not implemented: array

### `@flighthq/clipboard`

- **package** `upstream/packages/clipboard/src`: Generated crate is missing 32 of 32 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/clipboard/src/clipboard.ts`: typeof Rust lowering is not implemented

### `@flighthq/clock`

- **package** `upstream/packages/clock/src`: Generated crate is missing 2 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/clock/src/createClock.ts`: optional property Rust lowering is not implemented

### `@flighthq/collision`

- **package** `upstream/packages/collision/src`: Generated crate is missing 16 of 19 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/collision/src/segmentCollision.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/collision/src/shapeCollision.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/collision/src/testCollision.ts`: fall-through switch cases require explicit Rust lowering

### `@flighthq/color`

- **package** `upstream/packages/color/src`: Generated crate is missing 12 of 32 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/color/src/colorFromKelvin.ts`: Math.log Rust lowering is not implemented
- **emission** `upstream/packages/color/src/hslColor.ts`: cannot infer uninitialized local h
- **emission** `upstream/packages/color/src/hsvColor.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/color/src/oklab.ts`: Math.cbrt Rust lowering is not implemented

### `@flighthq/connectivity`

- **package** `upstream/packages/connectivity/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/connectivity/src/connectivity.ts`: optional property Rust lowering is not implemented

### `@flighthq/debug`

- **package** `upstream/packages/debug/src`: Generated crate is missing 9 of 9 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/debug/src/debug.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/debug/src/debugTiming.ts`: object literal requires an inferred structural type

### `@flighthq/device`

- **package** `upstream/packages/device/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/device/src/device.ts`: optional property Rust lowering is not implemented

### `@flighthq/dialog`

- **package** `upstream/packages/dialog/src`: Generated crate is missing 15 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/dialog/src/dialog.ts`: typeof Rust lowering is not implemented

### `@flighthq/displayobject`

- **package** `upstream/packages/displayobject/src`: Generated crate is missing 43 of 46 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject/src/bitmap.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject/src/displayObject.ts`: spread Rust lowering is not implemented
- **emission** `upstream/packages/displayobject/src/htmlView.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject/src/renderView.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject/src/stage.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject/src/video.ts`: optional property Rust lowering is not implemented

### `@flighthq/displayobject-canvas`

- **package** `upstream/packages/displayobject-canvas/src`: Generated crate is missing 84 of 94 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-canvas/src/canvasBitmap.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasCache.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-canvas/src/canvasClip.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-canvas/src/canvasCSSFilterBinding.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/displayobject-canvas/src/canvasDisplayObject.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-canvas/src/canvasImageSource.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/displayobject-canvas/src/canvasMaterialRegistry.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasMaterials.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-canvas/src/canvasParticleEmitter.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasQuadBatch.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasRenderState.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-canvas/src/canvasRenderTarget.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/displayobject-canvas/src/canvasRichText.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasScale9Mapper.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/displayobject-canvas/src/canvasScale9Shape.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasShape.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasShapeCommands.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-canvas/src/canvasShapeRegistry.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/displayobject-canvas/src/canvasSprite.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasTextInput.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/displayobject-canvas/src/canvasTextLabel.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasTilemap.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasTransform.ts`: Math.fround Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-canvas/src/canvasVideo.ts`: optional property Rust lowering is not implemented

### `@flighthq/displayobject-gl`

- **package** `upstream/packages/displayobject-gl/src`: Generated crate is missing 85 of 89 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-gl/src/enableGlColorAdjustmentGuards.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glBitmap.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glCache.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glClip.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glClipContours.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/displayobject-gl/src/glColorAdjustment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glDefaultMaterial.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glDisplayObject.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-gl/src/glParticleEmitter.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-gl/src/glQuadBatch.ts`: optional element access Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-gl/src/glRichText.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glScale9Shape.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glShape.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glShapeMesh.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-gl/src/glSprite.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-gl/src/glSpriteBatch.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-gl/src/glSpriteRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glTestHelper.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glTextInput.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glTextLabel.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glTilemap.ts`: optional element access Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-gl/src/glVelocity.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-gl/src/glVideo.ts`: object literal requires an inferred structural type

### `@flighthq/displayobject-wgpu`

- **package** `upstream/packages/displayobject-wgpu/src`: Generated crate is missing 88 of 95 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-wgpu/src/enableWgpuColorAdjustmentGuards.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuBitmap.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuCache.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuClip.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuClipContours.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuColorAdjustment.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuDefaultMaterial.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuDisplayObject.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuParticleEmitter.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuQuadBatch.ts`: optional element access Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuRenderStats.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuRichText.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuScale9Shape.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuShape.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuShapeMesh.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuSprite.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuSpriteBatch.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuSpriteRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuTextInput.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuTextLabel.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuTilemap.ts`: optional element access Rust lowering is not implemented
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuVelocity.ts`: upstream/packages/displayobject-wgpu/src/wgpuVelocity.ts: cannot infer return type for defaultWgpuDisplayObjectVelocityWriter
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuVideo.ts`: object literal requires an inferred structural type

### `@flighthq/effects`

- **package** `upstream/packages/effects/src`: Generated crate is missing 94 of 112 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects/src/autoExposureEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/barrelDistortionEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/bevelEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/blendEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/blendModeMath.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/effects/src/bloomEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/blurDownsample.ts`: Math.log2 Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/blurEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/bokehDepthOfFieldEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/cameraMotionBlurEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/chromaticAberrationEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/colorTemperatureMath.ts`: Math.log Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/compositeEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/compositeOperatorMath.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/effects/src/contactShadowsEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/convolutionEffect.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/crtEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/customShaderEffect.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/depthMath.ts`: Math.acos Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/directionalBlurEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/displacementEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/ditherEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/dropShadowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/filmEmulationEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/filmGrainEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/fxaaEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/gaussianKernel.ts`: Math.exp Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/gaussianMath.ts`: Math.exp Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/glitchEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/godRaysEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/gradientBevelEffect.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/gradientGlowEffect.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/halftoneEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/innerGlowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/innerShadowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/kuwaharaEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/kuwaharaMath.ts`: Math.exp Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/lensDirtEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/lensDistortionEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/lensFlareEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/medianEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/motionBlurEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/outerGlowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/outlineEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/panniniProjectionEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/pixelateEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/posterizeEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/radialBlurEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/renderEffectDefaults.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/renderEffectInputs.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/renderEffectInterpolation.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Set")
- **emission** `upstream/packages/effects/src/scanlinesEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/screenSpaceFogEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/sharpenEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/sketchEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/smaaEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/ssaoEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/ssrEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/taaEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/tiltShiftEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/toneMapEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/toneMapMath.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/effects/src/vignetteEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/volumetricLightEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects/src/whiteBalanceEffect.ts`: object literal requires an inferred structural type

### `@flighthq/effects-canvas`

- **package** `upstream/packages/effects-canvas/src`: Generated crate is missing 88 of 102 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-canvas/src/canvasBloomEffect.ts`: upstream/packages/effects-canvas/src/canvasBloomEffect.ts: cannot infer return type for defaultCanvasBloomEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasBlurEffect.ts`: upstream/packages/effects-canvas/src/canvasBlurEffect.ts: cannot infer return type for defaultCanvasBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasBokehDepthOfFieldEffect.ts`: upstream/packages/effects-canvas/src/canvasBokehDepthOfFieldEffect.ts: cannot infer return type for defaultCanvasBokehDepthOfFieldEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasCameraMotionBlurEffect.ts`: upstream/packages/effects-canvas/src/canvasCameraMotionBlurEffect.ts: cannot infer return type for defaultCanvasCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasChromaticAberrationEffect.ts`: upstream/packages/effects-canvas/src/canvasChromaticAberrationEffect.ts: cannot infer return type for defaultCanvasChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasConvolutionEffect.ts`: upstream/packages/effects-canvas/src/canvasConvolutionEffect.ts: cannot infer return type for defaultCanvasConvolutionEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasCrtEffect.ts`: upstream/packages/effects-canvas/src/canvasCrtEffect.ts: cannot infer return type for defaultCanvasCrtEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasDirectionalBlurEffect.ts`: upstream/packages/effects-canvas/src/canvasDirectionalBlurEffect.ts: cannot infer return type for defaultCanvasDirectionalBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasDisplacementEffect.ts`: upstream/packages/effects-canvas/src/canvasDisplacementEffect.ts: cannot infer return type for defaultCanvasDisplacementEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasDitherEffect.ts`: upstream/packages/effects-canvas/src/canvasDitherEffect.ts: cannot infer return type for defaultCanvasDitherEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasDropShadowEffect.ts`: upstream/packages/effects-canvas/src/canvasDropShadowEffect.ts: cannot infer return type for defaultCanvasDropShadowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasFilmGrainEffect.ts`: upstream/packages/effects-canvas/src/canvasFilmGrainEffect.ts: cannot infer return type for defaultCanvasFilmGrainEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasFxaaEffect.ts`: upstream/packages/effects-canvas/src/canvasFxaaEffect.ts: cannot infer return type for defaultCanvasFxaaEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasGlitchEffect.ts`: upstream/packages/effects-canvas/src/canvasGlitchEffect.ts: cannot infer return type for defaultCanvasGlitchEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasGodRaysEffect.ts`: upstream/packages/effects-canvas/src/canvasGodRaysEffect.ts: cannot infer return type for defaultCanvasGodRaysEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasHalftoneEffect.ts`: upstream/packages/effects-canvas/src/canvasHalftoneEffect.ts: cannot infer return type for defaultCanvasHalftoneEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasKuwaharaEffect.ts`: upstream/packages/effects-canvas/src/canvasKuwaharaEffect.ts: cannot infer return type for defaultCanvasKuwaharaEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasLensDirtEffect.ts`: upstream/packages/effects-canvas/src/canvasLensDirtEffect.ts: cannot infer return type for defaultCanvasLensDirtEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasLensDistortionEffect.ts`: upstream/packages/effects-canvas/src/canvasLensDistortionEffect.ts: cannot infer return type for defaultCanvasLensDistortionEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasLensFlareEffect.ts`: upstream/packages/effects-canvas/src/canvasLensFlareEffect.ts: cannot infer return type for defaultCanvasLensFlareEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasMedianEffect.ts`: upstream/packages/effects-canvas/src/canvasMedianEffect.ts: cannot infer return type for defaultCanvasMedianEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasMotionBlurEffect.ts`: upstream/packages/effects-canvas/src/canvasMotionBlurEffect.ts: cannot infer return type for defaultCanvasMotionBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasOuterGlowEffect.ts`: upstream/packages/effects-canvas/src/canvasOuterGlowEffect.ts: cannot infer return type for defaultCanvasOuterGlowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasOutlineEffect.ts`: upstream/packages/effects-canvas/src/canvasOutlineEffect.ts: cannot infer return type for defaultCanvasOutlineEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasPixelateEffect.ts`: upstream/packages/effects-canvas/src/canvasPixelateEffect.ts: cannot infer return type for defaultCanvasPixelateEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasPosterizeEffect.ts`: upstream/packages/effects-canvas/src/canvasPosterizeEffect.ts: cannot infer return type for defaultCanvasPosterizeEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasRadialBlurEffect.ts`: upstream/packages/effects-canvas/src/canvasRadialBlurEffect.ts: cannot infer return type for defaultCanvasRadialBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasRenderEffectPipeline.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-canvas/src/canvasRenderEffectRegistry.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/effects-canvas/src/canvasScanlinesEffect.ts`: upstream/packages/effects-canvas/src/canvasScanlinesEffect.ts: cannot infer return type for defaultCanvasScanlinesEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasScreenSpaceFogEffect.ts`: upstream/packages/effects-canvas/src/canvasScreenSpaceFogEffect.ts: cannot infer return type for defaultCanvasScreenSpaceFogEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasSharpenEffect.ts`: upstream/packages/effects-canvas/src/canvasSharpenEffect.ts: cannot infer return type for defaultCanvasSharpenEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasSketchEffect.ts`: upstream/packages/effects-canvas/src/canvasSketchEffect.ts: cannot infer return type for defaultCanvasSketchEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasSmaaEffect.ts`: upstream/packages/effects-canvas/src/canvasSmaaEffect.ts: cannot infer return type for defaultCanvasSmaaEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasSsaoEffect.ts`: upstream/packages/effects-canvas/src/canvasSsaoEffect.ts: cannot infer return type for defaultCanvasSsaoEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasSsrEffect.ts`: upstream/packages/effects-canvas/src/canvasSsrEffect.ts: cannot infer return type for defaultCanvasSsrEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasTaaEffect.ts`: upstream/packages/effects-canvas/src/canvasTaaEffect.ts: cannot infer return type for defaultCanvasTaaEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasTiltShiftEffect.ts`: upstream/packages/effects-canvas/src/canvasTiltShiftEffect.ts: cannot infer return type for defaultCanvasTiltShiftEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasToneMapEffect.ts`: upstream/packages/effects-canvas/src/canvasToneMapEffect.ts: cannot infer return type for defaultCanvasToneMapEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasVignetteEffect.ts`: upstream/packages/effects-canvas/src/canvasVignetteEffect.ts: cannot infer return type for defaultCanvasVignetteEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasWhiteBalanceEffect.ts`: upstream/packages/effects-canvas/src/canvasWhiteBalanceEffect.ts: cannot infer return type for defaultCanvasWhiteBalanceEffectRunner

### `@flighthq/effects-gl`

- **package** `upstream/packages/effects-gl/src`: Generated crate is missing 120 of 135 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-gl/src/glBevelEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-gl/src/glBlendEffect.ts`: upstream/packages/effects-gl/src/glBlendEffect.ts: cannot infer return type for defaultGlBlendEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBloomEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-gl/src/glBlurEffect.ts`: anonymous structural type has no synthesized Rust identity
- **emission** `upstream/packages/effects-gl/src/glBokehDepthOfFieldEffect.ts`: upstream/packages/effects-gl/src/glBokehDepthOfFieldEffect.ts: cannot infer return type for defaultGlBokehDepthOfFieldEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCameraMotionBlurEffect.ts`: upstream/packages/effects-gl/src/glCameraMotionBlurEffect.ts: cannot infer return type for defaultGlCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glChromaticAberrationEffect.ts`: upstream/packages/effects-gl/src/glChromaticAberrationEffect.ts: cannot infer return type for defaultGlChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCompositeEffect.ts`: upstream/packages/effects-gl/src/glCompositeEffect.ts: cannot infer return type for defaultGlCompositeEffectRunner
- **emission** `upstream/packages/effects-gl/src/glConvolutionEffect.ts`: upstream/packages/effects-gl/src/glConvolutionEffect.ts: cannot infer return type for defaultGlConvolutionEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCrtEffect.ts`: upstream/packages/effects-gl/src/glCrtEffect.ts: cannot infer return type for defaultGlCrtEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCustomShaderEffect.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/effects-gl/src/glDirectionalBlurEffect.ts`: upstream/packages/effects-gl/src/glDirectionalBlurEffect.ts: cannot infer return type for defaultGlDirectionalBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDisplacementEffect.ts`: upstream/packages/effects-gl/src/glDisplacementEffect.ts: cannot infer return type for defaultGlDisplacementEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDitherEffect.ts`: upstream/packages/effects-gl/src/glDitherEffect.ts: cannot infer return type for defaultGlDitherEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDropShadowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-gl/src/glEffectBlitShader.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/effects-gl/src/glEffectBoxBlur.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/effects-gl/src/glEffectProgramCache.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/effects-gl/src/glEffectTintShader.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/effects-gl/src/glFilmGrainEffect.ts`: upstream/packages/effects-gl/src/glFilmGrainEffect.ts: cannot infer return type for defaultGlFilmGrainEffectRunner
- **emission** `upstream/packages/effects-gl/src/glFxaaEffect.ts`: upstream/packages/effects-gl/src/glFxaaEffect.ts: cannot infer return type for defaultGlFxaaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGlitchEffect.ts`: upstream/packages/effects-gl/src/glGlitchEffect.ts: cannot infer return type for defaultGlGlitchEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGodRaysEffect.ts`: upstream/packages/effects-gl/src/glGodRaysEffect.ts: cannot infer return type for defaultGlGodRaysEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGradientBevelEffect.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/effects-gl/src/glGradientGlowEffect.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/effects-gl/src/glHalftoneEffect.ts`: upstream/packages/effects-gl/src/glHalftoneEffect.ts: cannot infer return type for defaultGlHalftoneEffectRunner
- **emission** `upstream/packages/effects-gl/src/glInnerGlowEffect.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/effects-gl/src/glInnerShadowEffect.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/effects-gl/src/glKuwaharaEffect.ts`: upstream/packages/effects-gl/src/glKuwaharaEffect.ts: cannot infer return type for defaultGlKuwaharaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensDirtEffect.ts`: upstream/packages/effects-gl/src/glLensDirtEffect.ts: cannot infer return type for defaultGlLensDirtEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensDistortionEffect.ts`: upstream/packages/effects-gl/src/glLensDistortionEffect.ts: cannot infer return type for defaultGlLensDistortionEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensFlareEffect.ts`: upstream/packages/effects-gl/src/glLensFlareEffect.ts: cannot infer return type for defaultGlLensFlareEffectRunner
- **emission** `upstream/packages/effects-gl/src/glMedianEffect.ts`: upstream/packages/effects-gl/src/glMedianEffect.ts: cannot infer return type for defaultGlMedianEffectRunner
- **emission** `upstream/packages/effects-gl/src/glMotionBlurEffect.ts`: upstream/packages/effects-gl/src/glMotionBlurEffect.ts: cannot infer return type for defaultGlMotionBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glOuterGlowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-gl/src/glOutlineEffect.ts`: upstream/packages/effects-gl/src/glOutlineEffect.ts: cannot infer return type for defaultGlOutlineEffectRunner
- **emission** `upstream/packages/effects-gl/src/glPixelateEffect.ts`: upstream/packages/effects-gl/src/glPixelateEffect.ts: cannot infer return type for defaultGlPixelateEffectRunner
- **emission** `upstream/packages/effects-gl/src/glPosterizeEffect.ts`: upstream/packages/effects-gl/src/glPosterizeEffect.ts: cannot infer return type for defaultGlPosterizeEffectRunner
- **emission** `upstream/packages/effects-gl/src/glRadialBlurEffect.ts`: upstream/packages/effects-gl/src/glRadialBlurEffect.ts: cannot infer return type for defaultGlRadialBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glRenderEffectPipeline.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-gl/src/glRenderEffectRegistry.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/effects-gl/src/glScanlinesEffect.ts`: upstream/packages/effects-gl/src/glScanlinesEffect.ts: cannot infer return type for defaultGlScanlinesEffectRunner
- **emission** `upstream/packages/effects-gl/src/glScreenSpaceFogEffect.ts`: upstream/packages/effects-gl/src/glScreenSpaceFogEffect.ts: cannot infer return type for defaultGlScreenSpaceFogEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSharpenEffect.ts`: upstream/packages/effects-gl/src/glSharpenEffect.ts: cannot infer return type for defaultGlSharpenEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSketchEffect.ts`: upstream/packages/effects-gl/src/glSketchEffect.ts: cannot infer return type for defaultGlSketchEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSmaaEffect.ts`: upstream/packages/effects-gl/src/glSmaaEffect.ts: cannot infer return type for defaultGlSmaaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSsaoEffect.ts`: upstream/packages/effects-gl/src/glSsaoEffect.ts: cannot infer return type for defaultGlSsaoEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSsrEffect.ts`: upstream/packages/effects-gl/src/glSsrEffect.ts: cannot infer return type for defaultGlSsrEffectRunner
- **emission** `upstream/packages/effects-gl/src/glTaaEffect.ts`: upstream/packages/effects-gl/src/glTaaEffect.ts: cannot infer return type for defaultGlTaaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glTiltShiftEffect.ts`: upstream/packages/effects-gl/src/glTiltShiftEffect.ts: cannot infer return type for defaultGlTiltShiftEffectRunner
- **emission** `upstream/packages/effects-gl/src/glToneMapEffect.ts`: upstream/packages/effects-gl/src/glToneMapEffect.ts: cannot infer return type for defaultGlToneMapEffectRunner
- **emission** `upstream/packages/effects-gl/src/glVignetteEffect.ts`: upstream/packages/effects-gl/src/glVignetteEffect.ts: cannot infer return type for defaultGlVignetteEffectRunner
- **emission** `upstream/packages/effects-gl/src/glWhiteBalanceEffect.ts`: upstream/packages/effects-gl/src/glWhiteBalanceEffect.ts: cannot infer return type for defaultGlWhiteBalanceEffectRunner

### `@flighthq/effects-wgpu`

- **package** `upstream/packages/effects-wgpu/src`: Generated crate is missing 119 of 128 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-wgpu/src/wgpuBevelEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuBloomEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuBlurEffect.ts`: anonymous structural type has no synthesized Rust identity
- **emission** `upstream/packages/effects-wgpu/src/wgpuBokehDepthOfFieldEffect.ts`: upstream/packages/effects-wgpu/src/wgpuBokehDepthOfFieldEffect.ts: cannot infer return type for defaultWgpuBokehDepthOfFieldEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCameraMotionBlurEffect.ts`: upstream/packages/effects-wgpu/src/wgpuCameraMotionBlurEffect.ts: cannot infer return type for defaultWgpuCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuChromaticAberrationEffect.ts`: upstream/packages/effects-wgpu/src/wgpuChromaticAberrationEffect.ts: cannot infer return type for defaultWgpuChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuColorLutPass.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuConvolutionEffect.ts`: upstream/packages/effects-wgpu/src/wgpuConvolutionEffect.ts: cannot infer return type for defaultWgpuConvolutionEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCrtEffect.ts`: upstream/packages/effects-wgpu/src/wgpuCrtEffect.ts: cannot infer return type for defaultWgpuCrtEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDirectionalBlurEffect.ts`: upstream/packages/effects-wgpu/src/wgpuDirectionalBlurEffect.ts: cannot infer return type for defaultWgpuDirectionalBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDisplacementEffect.ts`: upstream/packages/effects-wgpu/src/wgpuDisplacementEffect.ts: cannot infer return type for defaultWgpuDisplacementEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDitherEffect.ts`: upstream/packages/effects-wgpu/src/wgpuDitherEffect.ts: cannot infer return type for defaultWgpuDitherEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDropShadowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuEffectBlitShader.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/effects-wgpu/src/wgpuEffectBoxBlur.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/effects-wgpu/src/wgpuEffectGradientRamp.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/effects-wgpu/src/wgpuEffectPass.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuEffectProgramCache.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/effects-wgpu/src/wgpuEffectTintShader.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/effects-wgpu/src/wgpuFilmGrainEffect.ts`: upstream/packages/effects-wgpu/src/wgpuFilmGrainEffect.ts: cannot infer return type for defaultWgpuFilmGrainEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuFxaaEffect.ts`: upstream/packages/effects-wgpu/src/wgpuFxaaEffect.ts: cannot infer return type for defaultWgpuFxaaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGlitchEffect.ts`: upstream/packages/effects-wgpu/src/wgpuGlitchEffect.ts: cannot infer return type for defaultWgpuGlitchEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGodRaysEffect.ts`: upstream/packages/effects-wgpu/src/wgpuGodRaysEffect.ts: cannot infer return type for defaultWgpuGodRaysEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGradientBevelEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuGradientGlowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuHalftoneEffect.ts`: upstream/packages/effects-wgpu/src/wgpuHalftoneEffect.ts: cannot infer return type for defaultWgpuHalftoneEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuInnerGlowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuInnerShadowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuKuwaharaEffect.ts`: upstream/packages/effects-wgpu/src/wgpuKuwaharaEffect.ts: cannot infer return type for defaultWgpuKuwaharaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensDirtEffect.ts`: upstream/packages/effects-wgpu/src/wgpuLensDirtEffect.ts: cannot infer return type for defaultWgpuLensDirtEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensDistortionEffect.ts`: upstream/packages/effects-wgpu/src/wgpuLensDistortionEffect.ts: cannot infer return type for defaultWgpuLensDistortionEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensFlareEffect.ts`: upstream/packages/effects-wgpu/src/wgpuLensFlareEffect.ts: cannot infer return type for defaultWgpuLensFlareEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuMedianEffect.ts`: upstream/packages/effects-wgpu/src/wgpuMedianEffect.ts: cannot infer return type for defaultWgpuMedianEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuMotionBlurEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuOuterGlowEffect.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/effects-wgpu/src/wgpuOutlineEffect.ts`: upstream/packages/effects-wgpu/src/wgpuOutlineEffect.ts: cannot infer return type for defaultWgpuOutlineEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuPixelateEffect.ts`: upstream/packages/effects-wgpu/src/wgpuPixelateEffect.ts: cannot infer return type for defaultWgpuPixelateEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuPosterizeEffect.ts`: upstream/packages/effects-wgpu/src/wgpuPosterizeEffect.ts: cannot infer return type for defaultWgpuPosterizeEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuRadialBlurEffect.ts`: upstream/packages/effects-wgpu/src/wgpuRadialBlurEffect.ts: cannot infer return type for defaultWgpuRadialBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuRenderEffectPipeline.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/effects-wgpu/src/wgpuRenderEffectRegistry.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/effects-wgpu/src/wgpuScanlinesEffect.ts`: upstream/packages/effects-wgpu/src/wgpuScanlinesEffect.ts: cannot infer return type for defaultWgpuScanlinesEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuScreenSpaceFogEffect.ts`: upstream/packages/effects-wgpu/src/wgpuScreenSpaceFogEffect.ts: cannot infer return type for defaultWgpuScreenSpaceFogEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSharpenEffect.ts`: upstream/packages/effects-wgpu/src/wgpuSharpenEffect.ts: cannot infer return type for defaultWgpuSharpenEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSketchEffect.ts`: upstream/packages/effects-wgpu/src/wgpuSketchEffect.ts: cannot infer return type for defaultWgpuSketchEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSmaaEffect.ts`: upstream/packages/effects-wgpu/src/wgpuSmaaEffect.ts: cannot infer return type for defaultWgpuSmaaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSsaoEffect.ts`: upstream/packages/effects-wgpu/src/wgpuSsaoEffect.ts: cannot infer return type for defaultWgpuSsaoEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSsrEffect.ts`: upstream/packages/effects-wgpu/src/wgpuSsrEffect.ts: cannot infer return type for defaultWgpuSsrEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuTaaEffect.ts`: upstream/packages/effects-wgpu/src/wgpuTaaEffect.ts: cannot infer return type for defaultWgpuTaaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuTiltShiftEffect.ts`: upstream/packages/effects-wgpu/src/wgpuTiltShiftEffect.ts: cannot infer return type for defaultWgpuTiltShiftEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuToneMapEffect.ts`: upstream/packages/effects-wgpu/src/wgpuToneMapEffect.ts: cannot infer return type for defaultWgpuToneMapEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuVignetteEffect.ts`: upstream/packages/effects-wgpu/src/wgpuVignetteEffect.ts: cannot infer return type for defaultWgpuVignetteEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuWhiteBalanceEffect.ts`: upstream/packages/effects-wgpu/src/wgpuWhiteBalanceEffect.ts: cannot infer return type for defaultWgpuWhiteBalanceEffectRunner

### `@flighthq/entity`

- **package** `upstream/packages/entity/src`: Generated crate is missing 9 of 12 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/entity/src/binding.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/entity/src/clone.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/entity/src/entity.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/entity/src/guards.ts`: new-expression Rust lowering is not implemented: proxy

### `@flighthq/filesystem`

- **package** `upstream/packages/filesystem/src`: Generated crate is missing 43 of 43 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/filesystem/src/filesystem.ts`: object literal requires an inferred structural type

### `@flighthq/flow`

- **package** `upstream/packages/flow/src`: Generated crate is missing 5 of 9 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/flow/src/clearFlowStack.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/flow/src/popFlowState.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/flow/src/pushFlowState.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/flow/src/replaceFlowState.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/flow/src/updateFlowStack.ts`: optional call Rust lowering is not implemented

### `@flighthq/font`

- **package** `upstream/packages/font/src`: Generated crate is missing 14 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/font/src/font.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/font/src/fontFormat.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/font/src/fontFrom.ts`: upstream/packages/font/src/fontFrom.ts: async Rust lowering is not implemented for loadFontFromBytes
- **emission** `upstream/packages/font/src/fontResourceFrom.ts`: upstream/packages/font/src/fontResourceFrom.ts: async Rust lowering is not implemented for loadFontResourceFromBytes
- **emission** `upstream/packages/font/src/fontShorthand.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/font/src/fontStatus.ts`: upstream/packages/font/src/fontStatus.ts: async Rust lowering is not implemented for whenFontsReady

### `@flighthq/geolocation`

- **package** `upstream/packages/geolocation/src`: Generated crate is missing 12 of 12 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/geolocation/src/geolocation.ts`: typeof Rust lowering is not implemented

### `@flighthq/geometry`

- **package** `upstream/packages/geometry/src`: Generated crate is missing 345 of 377 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/geometry/src/aabb.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/boundingSphere.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/capsule.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/frustum.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/matrix.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/matrix3.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/matrix4.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/geometry/src/obb.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/plane.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/quaternion.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/ray3d.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/rectangle.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/transform2d.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/transform3d.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/typedarray.ts`: new-expression Rust lowering is not implemented: int16_array
- **emission** `upstream/packages/geometry/src/vector2.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/vector3.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/geometry/src/vector4.ts`: object literal requires an inferred structural type

### `@flighthq/glyphatlas`

- **package** `upstream/packages/glyphatlas/src`: Generated crate is missing 9 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/glyphatlas/src/glyphAtlas.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/glyphatlas/src/glyphAtlasEntry.ts`: spread Rust lowering is not implemented
- **emission** `upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts`: object literal requires an inferred structural type

### `@flighthq/haptics`

- **package** `upstream/packages/haptics/src`: Generated crate is missing 13 of 13 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/haptics/src/haptics.ts`: typeof Rust lowering is not implemented

### `@flighthq/image`

- **package** `upstream/packages/image/src`: Generated crate is missing 20 of 20 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/image/src/imageResource.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/image/src/imageResourceFrom.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("globalThis").image_data

### `@flighthq/image-codec`

- **package** `upstream/packages/image-codec/src`: Generated crate is missing 15 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/image-codec/src/decodeImage.ts`: upstream/packages/image-codec/src/decodeImage.ts: async Rust lowering is not implemented for decodeImage
- **emission** `upstream/packages/image-codec/src/encodeImage.ts`: upstream/packages/image-codec/src/encodeImage.ts: async Rust lowering is not implemented for encodeImage
- **emission** `upstream/packages/image-codec/src/imageDecoderRegistry.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/image-codec/src/imageEncoderRegistry.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/image-codec/src/registerWebImageDecoders.ts`: await Rust lowering is not implemented
- **emission** `upstream/packages/image-codec/src/registerWebImageEncoders.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("OffscreenCanvas")

### `@flighthq/input`

- **package** `upstream/packages/input/src`: Generated crate is missing 40 of 40 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/input/src/inputManager.ts`: optional property Rust lowering is not implemented

### `@flighthq/interaction`

- **package** `upstream/packages/interaction/src`: Generated crate is missing 63 of 83 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/interaction/src/enableInteractionGuards.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/interaction/src/focusManager.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/interaction/src/hitTests.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/interaction/src/interactionManager.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/interaction/src/interactionSpatialIndex.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/interaction/src/nodeInteractionState.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/interaction/src/registerBitmapHitTest.ts`: try Rust lowering is not implemented
- **emission** `upstream/packages/interaction/src/registerShapeHitTest.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/interaction/src/registerTextHitTest.ts`: object literal requires an inferred structural type

### `@flighthq/intl`

- **package** `upstream/packages/intl/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/intl/src/cache.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/intl/src/collator.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Intl").collator
- **emission** `upstream/packages/intl/src/datetime.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/intl/src/list.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Intl").list_format
- **emission** `upstream/packages/intl/src/number.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/intl/src/plural.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/intl/src/relativeTime.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Intl").relative_time_format

### `@flighthq/ipc`

- **package** `upstream/packages/ipc/src`: Generated crate is missing 17 of 17 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/ipc/src/ipc.ts`: typeof Rust lowering is not implemented

### `@flighthq/keyboard`

- **package** `upstream/packages/keyboard/src`: Generated crate is missing 20 of 20 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/keyboard/src/keyboard.ts`: optional call Rust lowering is not implemented

### `@flighthq/lifecycle`

- **package** `upstream/packages/lifecycle/src`: Generated crate is missing 13 of 13 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/lifecycle/src/lifecycle.ts`: object literal requires an inferred structural type

### `@flighthq/lighting`

- **package** `upstream/packages/lighting/src`: Generated crate is missing 33 of 37 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/lighting/src/ambientLight.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/lighting/src/areaLight.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/lighting/src/directionalLight.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/lighting/src/environment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/lighting/src/hemisphereLight.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/lighting/src/lightIntensity.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/lighting/src/pointLight.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/lighting/src/sceneLights.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/lighting/src/spotLight.ts`: object literal requires an inferred structural type

### `@flighthq/loader`

- **package** `upstream/packages/loader/src`: Generated crate is missing 13 of 13 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/loader/src/resourceLoader.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("AbortController")

### `@flighthq/log`

- **package** `upstream/packages/log/src`: Generated crate is missing 65 of 65 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/log/src/log.ts`: object literal requires an inferred structural type

### `@flighthq/materials`

- **package** `upstream/packages/materials/src`: Generated crate is missing 58 of 68 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/materials/src/classicMaterials.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/materials/src/colorTransform.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/materials/src/customShaderMaterial.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/materials/src/material.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/materials/src/materialPresets.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/materials/src/pbrExtensionMaterials.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/materials/src/pbrMaterials.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/materials/src/phongToPbr.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/materials/src/unlitMaterials.ts`: optional property Rust lowering is not implemented

### `@flighthq/math`

- **package** `upstream/packages/math/src`: Generated crate is missing 30 of 73 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/math/src/interpolationAdvanced.ts`: Math.exp Rust lowering is not implemented
- **emission** `upstream/packages/math/src/numberTheory.ts`: Math.trunc Rust lowering is not implemented
- **emission** `upstream/packages/math/src/randomDistributions.ts`: Math.log Rust lowering is not implemented
- **emission** `upstream/packages/math/src/rounding.ts`: Math.trunc Rust lowering is not implemented

### `@flighthq/media`

- **package** `upstream/packages/media/src`: Generated crate is missing 42 of 42 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/media/src/audioChannel.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/media/src/audioMixer.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/media/src/videoChannel.ts`: optional property Rust lowering is not implemented

### `@flighthq/mediasession`

- **package** `upstream/packages/mediasession/src`: Generated crate is missing 10 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/mediasession/src/mediasession.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("MediaMetadata")

### `@flighthq/menu`

- **package** `upstream/packages/menu/src`: Generated crate is missing 17 of 17 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/menu/src/menu-templates.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/menu/src/menu.ts`: object spread Rust lowering is not implemented

### `@flighthq/mesh`

- **package** `upstream/packages/mesh/src`: Generated crate is missing 39 of 67 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/mesh/src/meshGeometry.ts`: new-expression Rust lowering is not implemented: uint16_array
- **emission** `upstream/packages/mesh/src/meshGeometryBuilders.ts`: spread Rust lowering is not implemented
- **emission** `upstream/packages/mesh/src/meshGeometryIndex.ts`: new-expression Rust lowering is not implemented: uint16_array
- **emission** `upstream/packages/mesh/src/meshGeometryLayout.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/mesh/src/meshGeometryOperations.ts`: new-expression Rust lowering is not implemented: uint16_array

### `@flighthq/motionpath`

- **package** `upstream/packages/motionpath/src`: Generated crate is missing 1 of 7 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/motionpath/src/getMotionPathHeading.ts`: Math.atan2 Rust lowering is not implemented

### `@flighthq/movieclip`

- **package** `upstream/packages/movieclip/src`: Generated crate is missing 23 of 23 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/movieclip/src/movieClip.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/movieclip/src/spritesheetTimelineSource.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")

### `@flighthq/net`

- **package** `upstream/packages/net/src`: Generated crate is missing 4 of 4 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/net/src/net.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("AbortController")

### `@flighthq/node`

- **package** `upstream/packages/node/src`: Generated crate is missing 61 of 105 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/node/src/boundsRectangle.ts`: cannot infer uninitialized local bounds
- **emission** `upstream/packages/node/src/hasAppearance.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/node/src/hasBlendMode.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/node/src/hasBoundsRectangle.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/node/src/hasClip.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/node/src/hasMaterial.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/node/src/hasTransform2d.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/node/src/hasTransform3d.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/node/src/hierarchy.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Set")
- **emission** `upstream/packages/node/src/node.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/node/src/viewport.ts`: optional property Rust lowering is not implemented

### `@flighthq/notification`

- **package** `upstream/packages/notification/src`: Generated crate is missing 26 of 26 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/notification/src/notification.ts`: optional call Rust lowering is not implemented

### `@flighthq/particleemitter`

- **package** `upstream/packages/particleemitter/src`: Generated crate is missing 47 of 51 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/particleemitter/src/emitParticleBurst.ts`: Math.atan2 Rust lowering is not implemented
- **emission** `upstream/packages/particleemitter/src/emitParticleBurst3D.ts`: Math.cbrt Rust lowering is not implemented
- **emission** `upstream/packages/particleemitter/src/particleEmitter.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/particleemitter/src/particleEmitter3D.ts`: object field blendMode is not present in structural type
- **emission** `upstream/packages/particleemitter/src/updateParticleEmitter.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/particleemitter/src/updateParticleEmitter3D.ts`: optional property Rust lowering is not implemented

### `@flighthq/particles`

- **package** `upstream/packages/particles/src`: Generated crate is missing 41 of 50 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/particles/src/applyParticleCollisions.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/particles/src/applyParticleForces.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/particles/src/curve.ts`: new-expression Rust lowering is not implemented: array
- **emission** `upstream/packages/particles/src/particleEmitterConfig.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/particles/src/updateParticleObjects.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/particles/src/validateParticleEmitterConfig.ts`: object spread Rust lowering is not implemented

### `@flighthq/particles-formats`

- **package** `upstream/packages/particles-formats/src`: Generated crate is missing 68 of 79 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/particles-formats/src/detect.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/formatRegistry.ts`: try Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/libgdxParse.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/libgdxSchema.ts`: anonymous structural type has no synthesized Rust identity
- **emission** `upstream/packages/particles-formats/src/libgdxSerialize.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/parseParticleConfig.ts`: try Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/particleDesignerParse.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/particles-formats/src/particleDesignerSerialize.ts`: Math.atan2 Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/pixiParse.ts`: try Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/spineParse.ts`: try Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/spineSerialize.ts`: Math.atan2 Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/starlingPexParse.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/starlingPexSerialize.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/unityParse.ts`: try Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/unitySchema.ts`: anonymous structural type has no synthesized Rust identity
- **emission** `upstream/packages/particles-formats/src/unitySerialize.ts`: object literal requires an inferred structural type

### `@flighthq/path`

- **package** `upstream/packages/path/src`: Generated crate is missing 23 of 50 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/path/src/forEachPathSegment.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/path/src/path.ts`: Math.tan Rust lowering is not implemented
- **emission** `upstream/packages/path/src/pathMeshPool.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/path/src/reversePath.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/path/src/strokePath.ts`: object literal requires an inferred structural type

### `@flighthq/path-boolean`

- **package** `upstream/packages/path-boolean/src`: Generated crate is missing 9 of 12 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/path-boolean/src/booleanPaths.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/path-boolean/src/martinezKernel.ts`: new-expression Rust lowering is not implemented: event_heap
- **emission** `upstream/packages/path-boolean/src/offsetPath.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/path-boolean/src/simplifyPath.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/path-boolean/src/unionAllPaths.ts`: optional property Rust lowering is not implemented

### `@flighthq/path-formats`

- **package** `upstream/packages/path-formats/src`: Generated crate is missing 3 of 3 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/path-formats/src/svgPathData.ts`: optional property Rust lowering is not implemented

### `@flighthq/permissions`

- **package** `upstream/packages/permissions/src`: Generated crate is missing 5 of 5 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/permissions/src/permission.ts`: upstream/packages/permissions/src/permission.ts: async Rust lowering is not implemented for readWebPermissionState

### `@flighthq/picking`

- **package** `upstream/packages/picking/src`: Generated crate is missing 6 of 6 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/picking/src/pickScene.ts`: optional property Rust lowering is not implemented

### `@flighthq/platform`

- **package** `upstream/packages/platform/src`: Generated crate is missing 16 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/platform/src/platform.ts`: optional property Rust lowering is not implemented

### `@flighthq/power`

- **package** `upstream/packages/power/src`: Generated crate is missing 19 of 19 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/power/src/power.ts`: try Rust lowering is not implemented

### `@flighthq/protocol`

- **package** `upstream/packages/protocol/src`: Generated crate is missing 20 of 20 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/protocol/src/protocol.ts`: typeof Rust lowering is not implemented

### `@flighthq/render`

- **package** `upstream/packages/render/src`: Generated crate is missing 45 of 63 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render/src/enableColorAdjustmentGuards.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render/src/renderCache.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render/src/renderColorTransform.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/render/src/renderProxy.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render/src/renderQueue.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render/src/renderState.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render/src/renderViewport.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/render/src/sceneRender.ts`: object literal requires an inferred structural type

### `@flighthq/render-gl`

- **package** `upstream/packages/render-gl/src`: Generated crate is missing 58 of 75 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-gl/src/glCompressedTexture.ts`: optional element access Rust lowering is not implemented
- **emission** `upstream/packages/render-gl/src/glDraw.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/render-gl/src/glFullscreenPass.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render-gl/src/glLinearToSrgbPass.ts`: upstream/packages/render-gl/src/glLinearToSrgbPass.ts: cannot infer return type for NOOP
- **emission** `upstream/packages/render-gl/src/glMaterialRegistry.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/render-gl/src/glPresentRenderTarget.ts`: upstream/packages/render-gl/src/glPresentRenderTarget.ts: cannot infer return type for NOOP
- **emission** `upstream/packages/render-gl/src/glRenderPass.ts`: anonymous structural type has no synthesized Rust identity
- **emission** `upstream/packages/render-gl/src/glRenderState.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render-gl/src/glRenderTarget.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/render-gl/src/glShader.ts`: object field program is not present in structural type
- **emission** `upstream/packages/render-gl/src/glShaderBinding.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/render-gl/src/glTestHelper.ts`: object literal requires an inferred structural type

### `@flighthq/render-wgpu`

- **package** `upstream/packages/render-wgpu/src`: Generated crate is missing 61 of 68 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-wgpu/src/wgpuBackground.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/render-wgpu/src/wgpuDraw.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/render-wgpu/src/wgpuFullscreenPass.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render-wgpu/src/wgpuMaterialRegistry.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/render-wgpu/src/wgpuMipmap.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render-wgpu/src/wgpuRenderState.ts`: upstream/packages/render-wgpu/src/wgpuRenderState.ts: async Rust lowering is not implemented for createWgpuRenderState
- **emission** `upstream/packages/render-wgpu/src/wgpuRenderTarget.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render-wgpu/src/wgpuScissor.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render-wgpu/src/wgpuShader.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render-wgpu/src/wgpuShaderBinding.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/render-wgpu/src/wgpuSurface.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/render-wgpu/src/wgpuTestHelper.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/render-wgpu/src/wgpuTextureUpload.ts`: object literal requires an inferred structural type

### `@flighthq/scene`

- **package** `upstream/packages/scene/src`: Generated crate is missing 23 of 43 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene/src/billboardCamera.ts`: Math.hypot Rust lowering is not implemented
- **emission** `upstream/packages/scene/src/mesh.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene/src/scene.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene/src/sceneAnimation.ts`: typeof Rust lowering is not implemented

### `@flighthq/scene-formats`

- **package** `upstream/packages/scene-formats/src`: Generated crate is missing 14 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-formats/src/awdParse.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/scene-formats/src/gltfParse.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/scene-formats/src/gltfSchema.ts`: anonymous structural type has no synthesized Rust identity
- **emission** `upstream/packages/scene-formats/src/md2Parse.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/scene-formats/src/md5AnimParse.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/scene-formats/src/md5Parse.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/scene-formats/src/mtlParse.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/scene-formats/src/objParse.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/scene-formats/src/shared.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-formats/src/threeDsParse.ts`: optional property Rust lowering is not implemented

### `@flighthq/scene-gl`

- **package** `upstream/packages/scene-gl/src`: Generated crate is missing 163 of 184 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-gl/src/anisotropyPbrGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/blinnPhongGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/clearcoatPbrGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/depthGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/drawGlScene.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/scene-gl/src/emissiveGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/enableGlSceneColorSpaceGuards.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/enableGlSceneCustomShaderGuards.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/scene-gl/src/glClassicPrelude.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/scene-gl/src/glDebugPrelude.ts`: object field locModel is not present in structural type
- **emission** `upstream/packages/scene-gl/src/glEnvironmentIblBake.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/scene-gl/src/glEnvironmentSkybox.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/glLitProgram.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/scene-gl/src/glMatcapPrelude.ts`: object field locModel is not present in structural type
- **emission** `upstream/packages/scene-gl/src/glMeshProgram.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/glMeshUpload.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/glParticleEmitter3D.ts`: new-expression Rust lowering is not implemented: uint16_array
- **emission** `upstream/packages/scene-gl/src/glPbrProgramCache.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/scene-gl/src/glPbrStandardBlock.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/scene-gl/src/glSceneRuntime.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/glSceneTestHelper.ts`: anonymous structural type has no synthesized Rust identity
- **emission** `upstream/packages/scene-gl/src/glShadedBuiltInModifiers.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/glShadedPrelude.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/scene-gl/src/glShadowMap.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/glToonPrelude.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/scene-gl/src/glUnlitPrelude.ts`: object field locJointTexture is not present in structural type
- **emission** `upstream/packages/scene-gl/src/glWireframePrelude.ts`: object field locModel is not present in structural type
- **emission** `upstream/packages/scene-gl/src/glWireframeUpload.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/scene-gl/src/iridescencePbrGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/lambertGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/matcapGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/normalGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/phongGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/sheenPbrGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/specularPbrGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/standardPbrGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/subsurfacePbrGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/toonGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/transmissionVolumePbrGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/unlitGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/vertexColorGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-gl/src/wireframeGlMeshMaterialRenderer.ts`: object literal requires an inferred structural type

### `@flighthq/scene-resources`

- **package** `upstream/packages/scene-resources/src`: Generated crate is missing 32 of 37 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-resources/src/getSceneResourceTextures.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Set")
- **emission** `upstream/packages/scene-resources/src/load3ds.ts`: upstream/packages/scene-resources/src/load3ds.ts: async Rust lowering is not implemented for loadSceneFrom3ds
- **emission** `upstream/packages/scene-resources/src/loadGltf.ts`: upstream/packages/scene-resources/src/loadGltf.ts: async Rust lowering is not implemented for loadSceneFromGlb
- **emission** `upstream/packages/scene-resources/src/loadMd2.ts`: upstream/packages/scene-resources/src/loadMd2.ts: async Rust lowering is not implemented for loadSceneFromMd2
- **emission** `upstream/packages/scene-resources/src/loadMd5.ts`: upstream/packages/scene-resources/src/loadMd5.ts: async Rust lowering is not implemented for loadSceneFromMd5Mesh
- **emission** `upstream/packages/scene-resources/src/loadObj.ts`: upstream/packages/scene-resources/src/loadObj.ts: async Rust lowering is not implemented for loadSceneFromObj
- **emission** `upstream/packages/scene-resources/src/loadSceneFromAwd.ts`: upstream/packages/scene-resources/src/loadSceneFromAwd.ts: async Rust lowering is not implemented for loadSceneFromAwd
- **emission** `upstream/packages/scene-resources/src/loadSceneOptions.ts`: upstream/packages/scene-resources/src/loadSceneOptions.ts: async Rust lowering is not implemented for resolveScenesWithOptions
- **emission** `upstream/packages/scene-resources/src/resolveSceneResources.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Set")
- **emission** `upstream/packages/scene-resources/src/resolveSceneResourcesAndWait.ts`: upstream/packages/scene-resources/src/resolveSceneResourcesAndWait.ts: async Rust lowering is not implemented for resolveSceneResourcesAndWait
- **emission** `upstream/packages/scene-resources/src/revealSceneResourcesOnResolve.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/scene-resources/src/sceneMaterialTextureRegistry.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/scene-resources/src/sceneResourceFetch.ts`: try Rust lowering is not implemented
- **emission** `upstream/packages/scene-resources/src/sceneResourceResolver.ts`: object literal requires an inferred structural type

### `@flighthq/scene-wgpu`

- **package** `upstream/packages/scene-wgpu/src`: Generated crate is missing 131 of 140 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-wgpu/src/anisotropyPbrWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/blinnPhongWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/clearcoatPbrWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/depthWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/drawWgpuScene.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/emissiveWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/iridescencePbrWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/lambertWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/matcapWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/normalWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/phongWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/sheenPbrWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/specularPbrWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/subsurfacePbrWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/toonWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/transmissionVolumePbrWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/unlitWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/vertexColorWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuEnvironmentCube.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuMeshUpload.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/scene-wgpu/src/wgpuPbrPipelineCache.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuSceneRuntime.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuSceneTestHelper.ts`: anonymous structural type has no synthesized Rust identity
- **emission** `upstream/packages/scene-wgpu/src/wgpuShadowMap.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/scene-wgpu/src/wgpuWireframeUpload.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")
- **emission** `upstream/packages/scene-wgpu/src/wireframeWgpuMeshMaterialRenderer.ts`: object literal requires an inferred structural type

### `@flighthq/screen`

- **package** `upstream/packages/screen/src`: Generated crate is missing 31 of 31 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/screen/src/screen.ts`: typeof Rust lowering is not implemented

### `@flighthq/sdk`

- **package** `upstream/packages/sdk/src`: Generated crate is missing 5923 of 5923 upstream exports; re-export or declaration synthesis is required.

### `@flighthq/sensors`

- **package** `upstream/packages/sensors/src`: Generated crate is missing 32 of 32 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/sensors/src/sensors.ts`: Math.sign Rust lowering is not implemented

### `@flighthq/shading`

- **package** `upstream/packages/shading/src`: Generated crate is missing 22 of 37 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shading/src/createEnvReflectModifier.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/shading/src/createShadedMaterial.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/shading/src/createVertexDisplaceModifier.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/shading/src/isBuiltInModifierSlot.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Set")
- **emission** `upstream/packages/shading/src/modifierRegistry.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/shading/src/orderModifierStack.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/shading/src/registerBuiltInModifiers.ts`: object literal requires an inferred structural type

### `@flighthq/shape`

- **package** `upstream/packages/shape/src`: Generated crate is missing 42 of 42 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shape/src/scale9Shape.ts`: object field commands is not present in structural type
- **emission** `upstream/packages/shape/src/shape.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/shape/src/shapeCommands.ts`: Math.tan Rust lowering is not implemented
- **emission** `upstream/packages/shape/src/shapeFill.ts`: fall-through switch cases require explicit Rust lowering
- **emission** `upstream/packages/shape/src/shapeHitTestBuiltins.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/shape/src/shapeHitTestRegistry.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")

### `@flighthq/shape-formats`

- **package** `upstream/packages/shape-formats/src`: Generated crate is missing 5 of 5 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shape-formats/src/shapeJson.ts`: object literal requires an inferred structural type

### `@flighthq/share`

- **package** `upstream/packages/share/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/share/src/share.ts`: try Rust lowering is not implemented

### `@flighthq/shell`

- **package** `upstream/packages/shell/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shell/src/shell.ts`: typeof Rust lowering is not implemented

### `@flighthq/shortcut`

- **package** `upstream/packages/shortcut/src`: Generated crate is missing 26 of 26 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shortcut/src/shortcut.ts`: object literal requires an inferred structural type

### `@flighthq/signals`

- **package** `upstream/packages/signals/src`: Generated crate is missing 13 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/signals/src/emitter.ts`: spread Rust lowering is not implemented
- **emission** `upstream/packages/signals/src/internal.ts`: upstream/packages/signals/src/internal.ts: cannot infer return type for nullSignalEmit
- **emission** `upstream/packages/signals/src/slot.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/signals/src/throttle.ts`: optional property Rust lowering is not implemented

### `@flighthq/snapshot`

- **package** `upstream/packages/snapshot/src`: Generated crate is missing 4 of 4 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/snapshot/src/captureSnapshot.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/snapshot/src/equalsSnapshot.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/snapshot/src/interpolateSnapshots.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/snapshot/src/restoreSnapshot.ts`: typeof Rust lowering is not implemented

### `@flighthq/socket`

- **package** `upstream/packages/socket/src`: Generated crate is missing 11 of 11 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/socket/src/socket.ts`: optional property Rust lowering is not implemented

### `@flighthq/spatial`

- **package** `upstream/packages/spatial/src`: Generated crate is missing 1 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/spatial/src/uniformGrid.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")

### `@flighthq/spring`

- **package** `upstream/packages/spring/src`: Generated crate is missing 1 of 12 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/spring/src/updateSpring.ts`: Math.exp Rust lowering is not implemented

### `@flighthq/sprite`

- **package** `upstream/packages/sprite/src`: Generated crate is missing 64 of 64 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/sprite/src/quadBatch.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/sprite/src/sprite.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/sprite/src/tilemap.ts`: object literal requires an inferred structural type

### `@flighthq/spritesheet`

- **package** `upstream/packages/spritesheet/src`: Generated crate is missing 32 of 32 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/spritesheet/src/spritesheet.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/spritesheet/src/spritesheetAnimation.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/spritesheet/src/spritesheetData.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/spritesheet/src/spritesheetFrame.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/spritesheet/src/spritesheetFrom.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/spritesheet/src/spritesheetPlayer.ts`: spread Rust lowering is not implemented
- **emission** `upstream/packages/spritesheet/src/spritesheetValidation.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Set")

### `@flighthq/spritesheet-formats`

- **package** `upstream/packages/spritesheet-formats/src`: Generated crate is missing 26 of 55 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/spritesheet-formats/src/asepriteParse.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/spritesheet-formats/src/asepriteSerialize.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/spritesheet-formats/src/cocosPlistParse.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/spritesheet-formats/src/cocosPlistSerialize.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/spritesheet-formats/src/libgdxAtlasParse.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/spritesheet-formats/src/spritesheetDetect.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/spritesheet-formats/src/starlingParse.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/spritesheet-formats/src/starlingSerialize.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/spritesheet-formats/src/texturePackerParse.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/spritesheet-formats/src/texturePackerSerialize.ts`: object literal requires an inferred structural type

### `@flighthq/statusbar`

- **package** `upstream/packages/statusbar/src`: Generated crate is missing 16 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/statusbar/src/statusbar.ts`: object literal requires an inferred structural type

### `@flighthq/storage`

- **package** `upstream/packages/storage/src`: Generated crate is missing 39 of 39 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/storage/src/storage.ts`: try Rust lowering is not implemented

### `@flighthq/text`

- **package** `upstream/packages/text/src`: Generated crate is missing 82 of 86 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/text/src/nativeText.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/text/src/richText.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/text/src/textLabel.ts`: optional property Rust lowering is not implemented

### `@flighthq/text-markup`

- **package** `upstream/packages/text-markup/src`: Generated crate is missing 7 of 8 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/text-markup/src/markupNamedColors.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/text-markup/src/markupTagRegistry.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/text-markup/src/textMarkup.ts`: object literal requires an inferred structural type

### `@flighthq/textbidi`

- **package** `upstream/packages/textbidi/src`: Generated crate is missing 1 of 6 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textbidi/src/resolveBidiLevels.ts`: new-expression Rust lowering is not implemented: array

### `@flighthq/textinput`

- **package** `upstream/packages/textinput/src`: Generated crate is missing 55 of 55 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textinput/src/selectableRichTextManager.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/textinput/src/textInput.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/textinput/src/textInputEditing.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/textinput/src/textInputManager.ts`: object literal requires an inferred structural type

### `@flighthq/textlayout`

- **package** `upstream/packages/textlayout/src`: Generated crate is missing 25 of 47 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textlayout/src/richTextContent.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/textlayout/src/richTextQuery.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/textlayout/src/textFormat.ts`: object spread Rust lowering is not implemented
- **emission** `upstream/packages/textlayout/src/textLayout.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Set")

### `@flighthq/textsegment`

- **package** `upstream/packages/textsegment/src`: Generated crate is missing 3 of 11 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textsegment/src/textSegmenterBackend.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")

### `@flighthq/textshaper`

- **package** `upstream/packages/textshaper/src`: Generated crate is missing 16 of 31 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textshaper/src/textShaper.ts`: optional call Rust lowering is not implemented
- **emission** `upstream/packages/textshaper/src/textShaperCache.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/textshaper/src/textShaperCluster.ts`: new-expression Rust lowering is not implemented: array
- **emission** `upstream/packages/textshaper/src/textShaperItemize.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/textshaper/src/textShaperSignals.ts`: object literal requires an inferred structural type

### `@flighthq/textshaper-canvas`

- **package** `upstream/packages/textshaper-canvas/src`: Generated crate is missing 3 of 3 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textshaper-canvas/src/canvasTextShaper.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")

### `@flighthq/texture`

- **package** `upstream/packages/texture/src`: Generated crate is missing 42 of 42 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/texture/src/cubeTexture.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/texture/src/sampler.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/texture/src/texture.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/texture/src/videoTexture.ts`: object literal requires an inferred structural type

### `@flighthq/texture-formats`

- **package** `upstream/packages/texture-formats/src`: Generated crate is missing 4 of 6 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/texture-formats/src/byteReader.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("DataView")
- **emission** `upstream/packages/texture-formats/src/parseAtf.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Set")
- **emission** `upstream/packages/texture-formats/src/parseBasis.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/texture-formats/src/parseDds.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/texture-formats/src/parseKtx2.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/texture-formats/src/textureLevelLayout.ts`: object literal requires an inferred structural type

### `@flighthq/textureatlas`

- **package** `upstream/packages/textureatlas/src`: Generated crate is missing 20 of 20 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textureatlas/src/textureAtlas.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/textureatlas/src/textureAtlasFrom.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/textureatlas/src/textureAtlasRegion.ts`: object literal requires an inferred structural type

### `@flighthq/textureatlas-formats`

- **package** `upstream/packages/textureatlas-formats/src`: Generated crate is missing 9 of 29 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textureatlas-formats/src/textureAtlasAsepriteParse.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/textureatlas-formats/src/textureAtlasDetect.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/textureatlas-formats/src/textureAtlasLibgdxParse.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/textureatlas-formats/src/textureAtlasPackerParse.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/textureatlas-formats/src/textureAtlasStarlingParse.ts`: object literal requires an inferred structural type

### `@flighthq/tilemap-formats`

- **package** `upstream/packages/tilemap-formats/src`: Generated crate is missing 10 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tilemap-formats/src/tiledColor.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/tilemap-formats/src/tiledJsonParse.ts`: typeof Rust lowering is not implemented
- **emission** `upstream/packages/tilemap-formats/src/tiledLayerData.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/tilemap-formats/src/tiledProject.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")
- **emission** `upstream/packages/tilemap-formats/src/tiledTmxFormat.ts`: regular expression Rust lowering is not implemented
- **emission** `upstream/packages/tilemap-formats/src/tiledXmlParse.ts`: object literal requires an inferred structural type

### `@flighthq/tileset`

- **package** `upstream/packages/tileset/src`: Generated crate is missing 9 of 9 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tileset/src/tileset.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/tileset/src/tilesetFrom.ts`: object literal requires an inferred structural type

### `@flighthq/timeline`

- **package** `upstream/packages/timeline/src`: Generated crate is missing 16 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/timeline/src/timeline.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Map")

### `@flighthq/tray`

- **package** `upstream/packages/tray/src`: Generated crate is missing 23 of 23 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tray/src/tray.ts`: object literal requires an inferred structural type

### `@flighthq/tween`

- **package** `upstream/packages/tween/src`: Generated crate is missing 28 of 35 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tween/src/colorTween.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/tween/src/timer.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/tween/src/tween.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/tween/src/tweenManager.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/tween/src/tweenStagger.ts`: optional property Rust lowering is not implemented

### `@flighthq/types`

- **package** `upstream/packages/types/src`: Generated crate is missing 48 of 1261 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/types/src/AppearanceFlags.ts`: upstream/packages/types/src/AppearanceFlags.ts:1: unsupported Rust declaration enum AppearanceFlags
- **emission** `upstream/packages/types/src/BatchBarrier.ts`: upstream/packages/types/src/BatchBarrier.ts:4: unsupported Rust declaration enum BatchBarrier
- **emission** `upstream/packages/types/src/BatchFormat.ts`: upstream/packages/types/src/BatchFormat.ts:13: unsupported Rust declaration enum BatchFormat
- **emission** `upstream/packages/types/src/GlRenderState.ts`: anonymous structural type has no synthesized Rust identity
- **emission** `upstream/packages/types/src/Ipc.ts`: upstream/packages/types/src/Ipc.ts:50: unsupported Rust declaration class IpcTimeoutError
- **emission** `upstream/packages/types/src/Log.ts`: upstream/packages/types/src/Log.ts:6: unsupported Rust declaration enum LogLevel
- **emission** `upstream/packages/types/src/ParticleObjectsUpdateOptions.ts`: anonymous structural type has no synthesized Rust identity
- **emission** `upstream/packages/types/src/WellKnownMenuItemRole.ts`: object literal requires an inferred structural type
- **emission** `upstream/packages/types/src/WgpuRenderState.ts`: anonymous structural type has no synthesized Rust identity

### `@flighthq/updater`

- **package** `upstream/packages/updater/src`: Generated crate is missing 23 of 23 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/updater/src/updater.ts`: object literal requires an inferred structural type

### `@flighthq/useragent`

- **package** `upstream/packages/useragent/src`: Generated crate is missing 12 of 12 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/useragent/src/userAgent.ts`: try Rust lowering is not implemented
- **emission** `upstream/packages/useragent/src/userAgentParse.ts`: regular expression Rust lowering is not implemented

### `@flighthq/velocity`

- **package** `upstream/packages/velocity/src`: Generated crate is missing 18 of 20 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/velocity/src/velocityField.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("WeakMap")

### `@flighthq/video`

- **package** `upstream/packages/video/src`: Generated crate is missing 8 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/video/src/videoFormat.ts`: optional property Rust lowering is not implemented
- **emission** `upstream/packages/video/src/videoResourceFrom.ts`: upstream/packages/video/src/videoResourceFrom.ts: async Rust lowering is not implemented for loadVideoResourceFromBlob

### `@flighthq/webcam`

- **package** `upstream/packages/webcam/src`: Generated crate is missing 10 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/webcam/src/webcam.ts`: new-expression Rust lowering is not implemented: (_runtime.global_value)("Promise")
- **emission** `upstream/packages/webcam/src/webcamStream.ts`: object literal requires an inferred structural type

### `@flighthq/xml`

- **package** `upstream/packages/xml/src`: Generated crate is missing 3 of 7 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/xml/src/xmlParse.ts`: object literal requires an inferred structural type

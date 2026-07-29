# Automatic Rust Generation

Upstream commit: `5d24729f7360475e28a105ae0caeeaa2e1328260`

| Metric | Count |
| --- | ---: |
| Inventoried packages | 131 |
| Default-generated packages | 125 |
| Emittable packages | 58 |
| Blocked packages | 67 |
| Compiled candidates | 0 |
| Compile-blocked candidates | 1 |
| Dependency-blocked candidates | 56 |
| Cultivated packages | 1 |
| Host-bound packages | 4 |
| Excluded packages | 1 |
| Source/package blockers | 475 |

| Package | Disposition | Status | Candidate | Sources emitted/attempted | API generated/expected | Missing | Dependents direct/transitive | Opaque sources | Blockers | Target |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `@flighthq/accessibility` | generated | emittable | dependency-blocked | 2/2 | 8/8 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/adjustments` | generated | emittable | dependency-blocked | 19/19 | 49/49 | 0 | 6/25 | 0 | 0 | no |
| `@flighthq/animation` | generated | emittable | dependency-blocked | 4/4 | 18/18 | 0 | 3/7 | 1 | 0 | no |
| `@flighthq/app` | generated | emittable | dependency-blocked | 2/2 | 42/42 | 0 | 4/4 | 1 | 0 | no |
| `@flighthq/application` | generated | emittable | dependency-blocked | 3/3 | 83/83 | 0 | 3/3 | 2 | 0 | no |
| `@flighthq/assets` | generated | emittable | dependency-blocked | 2/2 | 10/10 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/audio` | generated | emittable | dependency-blocked | 4/4 | 20/20 | 0 | 2/2 | 2 | 0 | no |
| `@flighthq/binpack` | generated | blocked | source-blocked | 1/2 | 0/1 | 1 | 1/1 | 0 | 2 | no |
| `@flighthq/bitmapfont` | generated | emittable | dependency-blocked | 3/3 | 7/7 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/bitmapfont-formats` | generated | emittable | dependency-blocked | 5/5 | 9/4 | 0 | 1/1 | 3 | 0 | no |
| `@flighthq/bitmaptext` | generated | blocked | source-blocked | 1/3 | 0/15 | 15 | 1/1 | 0 | 3 | no |
| `@flighthq/camera` | generated | emittable | dependency-blocked | 10/10 | 31/31 | 0 | 4/4 | 0 | 0 | no |
| `@flighthq/camera2d` | generated | emittable | dependency-blocked | 8/8 | 8/8 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/capture` | generated | blocked | source-blocked | 2/3 | 5/10 | 5 | 1/1 | 1 | 2 | no |
| `@flighthq/clip` | generated | emittable | dependency-blocked | 2/2 | 23/23 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/clipboard` | generated | blocked | source-blocked | 1/2 | 0/32 | 32 | 4/4 | 0 | 2 | no |
| `@flighthq/clock` | generated | emittable | dependency-blocked | 12/12 | 14/14 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/collision` | generated | blocked | source-blocked | 5/6 | 9/19 | 10 | 1/1 | 0 | 2 | no |
| `@flighthq/color` | generated | emittable | dependency-blocked | 10/10 | 32/32 | 0 | 10/29 | 0 | 0 | no |
| `@flighthq/connectivity` | generated | emittable | dependency-blocked | 2/2 | 14/14 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/debug` | generated | blocked | source-blocked | 2/3 | 4/9 | 5 | 1/1 | 1 | 2 | no |
| `@flighthq/device` | generated | emittable | dependency-blocked | 2/2 | 14/14 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/dialog` | generated | blocked | source-blocked | 1/2 | 0/15 | 15 | 5/5 | 0 | 2 | no |
| `@flighthq/displayobject` | generated | blocked | source-blocked | 6/8 | 28/46 | 18 | 14/24 | 5 | 3 | no |
| `@flighthq/displayobject-canvas` | generated | blocked | source-blocked | 16/31 | 51/94 | 43 | 5/5 | 14 | 16 | no |
| `@flighthq/displayobject-dom` | host-bound | host-bound | not-applicable | 0/0 | 0/58 | 58 | 1/1 | 0 | 0 | no |
| `@flighthq/displayobject-gl` | generated | blocked | source-blocked | 7/28 | 23/89 | 68 | 1/1 | 3 | 22 | no |
| `@flighthq/displayobject-wgpu` | generated | blocked | source-blocked | 7/29 | 25/95 | 71 | 1/1 | 3 | 23 | no |
| `@flighthq/easing` | generated | emittable | promoted | 20/20 | 48/48 | 0 | 2/3 | 0 | 0 | full |
| `@flighthq/effects` | generated | emittable | dependency-blocked | 72/72 | 112/112 | 0 | 4/4 | 2 | 0 | no |
| `@flighthq/effects-canvas` | generated | blocked | source-blocked | 9/48 | 29/102 | 78 | 1/1 | 7 | 40 | no |
| `@flighthq/effects-gl` | generated | blocked | source-blocked | 11/58 | 31/135 | 104 | 1/1 | 8 | 48 | no |
| `@flighthq/effects-wgpu` | generated | blocked | source-blocked | 9/56 | 21/128 | 107 | 1/1 | 1 | 48 | no |
| `@flighthq/entity` | generated | blocked | source-blocked | 3/6 | 5/12 | 7 | 20/61 | 0 | 4 | no |
| `@flighthq/filesystem` | generated | blocked | source-blocked | 1/2 | 0/43 | 43 | 2/2 | 0 | 2 | no |
| `@flighthq/flow` | generated | emittable | dependency-blocked | 10/10 | 9/9 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/font` | generated | emittable | dependency-blocked | 8/8 | 15/15 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/geolocation` | generated | blocked | source-blocked | 1/2 | 0/12 | 12 | 2/2 | 0 | 2 | no |
| `@flighthq/geometry` | generated | emittable | dependency-blocked | 27/27 | 377/377 | 0 | 40/53 | 0 | 0 | no |
| `@flighthq/glyphatlas` | generated | emittable | dependency-blocked | 7/7 | 14/14 | 0 | 1/1 | 2 | 0 | no |
| `@flighthq/haptics` | generated | emittable | dependency-blocked | 2/2 | 13/13 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/host-capacitor` | host-bound | host-bound | not-applicable | 0/0 | 0/63 | 63 | 0/0 | 0 | 0 | no |
| `@flighthq/host-electron` | host-bound | host-bound | not-applicable | 0/0 | 0/57 | 57 | 0/0 | 0 | 0 | no |
| `@flighthq/host-tauri` | host-bound | host-bound | not-applicable | 0/0 | 0/51 | 51 | 0/0 | 0 | 0 | no |
| `@flighthq/image` | generated | emittable | dependency-blocked | 3/3 | 20/20 | 0 | 11/24 | 2 | 0 | partial |
| `@flighthq/image-codec` | generated | blocked | source-blocked | 6/8 | 14/16 | 2 | 3/26 | 0 | 3 | no |
| `@flighthq/input` | generated | emittable | dependency-blocked | 2/2 | 40/40 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/interaction` | generated | blocked | source-blocked | 9/16 | 21/83 | 62 | 1/1 | 8 | 8 | no |
| `@flighthq/intl` | generated | blocked | source-blocked | 1/8 | 0/14 | 14 | 1/1 | 0 | 8 | no |
| `@flighthq/ipc` | generated | blocked | source-blocked | 1/2 | 0/17 | 17 | 2/2 | 0 | 2 | no |
| `@flighthq/keyboard` | generated | emittable | dependency-blocked | 2/2 | 20/20 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/lifecycle` | generated | emittable | dependency-blocked | 2/2 | 13/13 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/lighting` | generated | emittable | dependency-blocked | 11/11 | 37/37 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/loader` | generated | blocked | source-blocked | 1/2 | 0/13 | 13 | 3/3 | 0 | 2 | no |
| `@flighthq/log` | generated | blocked | source-blocked | 1/2 | 0/65 | 65 | 7/16 | 0 | 2 | no |
| `@flighthq/materials` | generated | emittable | dependency-blocked | 12/12 | 68/68 | 0 | 7/28 | 1 | 0 | no |
| `@flighthq/math` | generated | emittable | dependency-blocked | 16/16 | 73/73 | 0 | 4/4 | 0 | 0 | no |
| `@flighthq/media` | generated | emittable | dependency-blocked | 4/4 | 42/42 | 0 | 1/1 | 3 | 0 | no |
| `@flighthq/mediasession` | generated | blocked | source-blocked | 1/2 | 0/10 | 10 | 1/1 | 0 | 2 | no |
| `@flighthq/menu` | generated | blocked | source-blocked | 2/3 | 6/17 | 11 | 3/3 | 0 | 2 | no |
| `@flighthq/mesh` | generated | blocked | source-blocked | 11/12 | 55/67 | 12 | 6/20 | 1 | 2 | no |
| `@flighthq/motionpath` | generated | emittable | dependency-blocked | 8/8 | 7/7 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/movieclip` | generated | blocked | source-blocked | 2/3 | 1/23 | 22 | 1/1 | 1 | 2 | no |
| `@flighthq/net` | generated | blocked | source-blocked | 1/2 | 0/4 | 4 | 1/1 | 0 | 2 | no |
| `@flighthq/node` | generated | blocked | source-blocked | 8/16 | 30/105 | 75 | 23/32 | 3 | 9 | no |
| `@flighthq/notification` | generated | blocked | source-blocked | 1/2 | 0/26 | 26 | 4/4 | 0 | 2 | no |
| `@flighthq/particleemitter` | generated | blocked | source-blocked | 10/11 | 30/51 | 21 | 1/1 | 9 | 2 | no |
| `@flighthq/particles` | generated | emittable | dependency-blocked | 11/11 | 50/50 | 0 | 3/3 | 1 | 0 | no |
| `@flighthq/particles-formats` | generated | blocked | source-blocked | 14/21 | 54/79 | 25 | 1/1 | 5 | 8 | no |
| `@flighthq/path` | generated | emittable | dependency-blocked | 23/23 | 50/50 | 0 | 8/8 | 0 | 0 | no |
| `@flighthq/path-boolean` | generated | blocked | source-blocked | 7/8 | 12/12 | 1 | 1/1 | 0 | 2 | no |
| `@flighthq/path-formats` | generated | emittable | dependency-blocked | 2/2 | 3/3 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/permissions` | generated | blocked | source-blocked | 1/2 | 0/5 | 5 | 1/1 | 0 | 2 | no |
| `@flighthq/picking` | generated | blocked | source-blocked | 1/2 | 0/6 | 6 | 1/1 | 0 | 2 | no |
| `@flighthq/platform` | generated | emittable | dependency-blocked | 2/2 | 16/16 | 0 | 3/3 | 1 | 0 | no |
| `@flighthq/power` | generated | emittable | dependency-blocked | 2/2 | 19/19 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/protocol` | generated | emittable | dependency-blocked | 2/2 | 20/20 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/render` | generated | blocked | source-blocked | 6/17 | 21/63 | 42 | 9/13 | 3 | 12 | no |
| `@flighthq/render-gl` | generated | blocked | source-blocked | 12/24 | 29/75 | 54 | 4/4 | 8 | 13 | no |
| `@flighthq/render-wgpu` | generated | blocked | source-blocked | 5/18 | 13/68 | 55 | 5/5 | 4 | 14 | no |
| `@flighthq/scene` | generated | blocked | source-blocked | 9/14 | 26/43 | 17 | 6/6 | 7 | 6 | no |
| `@flighthq/scene-formats` | generated | blocked | source-blocked | 8/16 | 82/15 | 10 | 2/2 | 1 | 9 | no |
| `@flighthq/scene-gl` | generated | blocked | source-blocked | 50/53 | 174/184 | 10 | 1/1 | 23 | 4 | no |
| `@flighthq/scene-resources` | generated | blocked | source-blocked | 14/16 | 32/37 | 5 | 1/1 | 13 | 3 | no |
| `@flighthq/scene-wgpu` | generated | blocked | source-blocked | 7/42 | 14/140 | 126 | 1/1 | 3 | 36 | no |
| `@flighthq/screen` | generated | emittable | dependency-blocked | 2/2 | 31/31 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/sdk` | generated | blocked | source-blocked | 14/14 | 0/5923 | 5923 | 0/0 | 0 | 1 | no |
| `@flighthq/sensors` | generated | blocked | source-blocked | 1/2 | 0/32 | 32 | 1/1 | 0 | 2 | no |
| `@flighthq/shading` | generated | blocked | source-blocked | 16/17 | 36/37 | 1 | 2/2 | 2 | 2 | no |
| `@flighthq/shape` | generated | blocked | source-blocked | 5/7 | 31/42 | 11 | 7/8 | 2 | 3 | no |
| `@flighthq/shape-formats` | generated | blocked | source-blocked | 1/2 | 0/5 | 5 | 1/1 | 0 | 2 | no |
| `@flighthq/share` | generated | blocked | source-blocked | 1/2 | 0/14 | 14 | 2/2 | 0 | 2 | no |
| `@flighthq/shell` | generated | emittable | dependency-blocked | 2/2 | 14/14 | 0 | 3/3 | 1 | 0 | no |
| `@flighthq/shortcut` | generated | blocked | source-blocked | 1/2 | 0/26 | 26 | 3/3 | 0 | 2 | no |
| `@flighthq/signals` | generated | emittable | dependency-blocked | 6/6 | 15/14 | 0 | 42/72 | 0 | 0 | no |
| `@flighthq/skeleton3d` | generated | emittable | dependency-blocked | 6/6 | 16/16 | 0 | 3/16 | 2 | 0 | no |
| `@flighthq/snapshot` | generated | emittable | dependency-blocked | 5/5 | 4/4 | 0 | 1/1 | 4 | 0 | no |
| `@flighthq/socket` | generated | emittable | dependency-blocked | 2/2 | 11/11 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/spatial` | generated | emittable | dependency-blocked | 3/3 | 10/10 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/spring` | generated | emittable | dependency-blocked | 8/8 | 12/12 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/sprite` | generated | blocked | source-blocked | 3/4 | 34/64 | 30 | 3/3 | 2 | 2 | no |
| `@flighthq/spritesheet` | generated | emittable | dependency-blocked | 8/8 | 32/32 | 0 | 2/2 | 4 | 0 | no |
| `@flighthq/spritesheet-formats` | generated | blocked | source-blocked | 14/16 | 51/55 | 4 | 1/1 | 7 | 3 | no |
| `@flighthq/statusbar` | generated | blocked | source-blocked | 1/2 | 0/16 | 16 | 2/2 | 0 | 2 | no |
| `@flighthq/storage` | generated | blocked | source-blocked | 1/2 | 0/39 | 39 | 2/2 | 0 | 2 | no |
| `@flighthq/surface` | cultivated | cultivated | not-applicable | 0/0 | 0/136 | 136 | 6/9 | 0 | 0 | partial |
| `@flighthq/text` | generated | blocked | source-blocked | 2/6 | 1/86 | 85 | 8/9 | 0 | 5 | no |
| `@flighthq/text-markup` | generated | blocked | source-blocked | 4/5 | 6/8 | 2 | 1/1 | 1 | 2 | no |
| `@flighthq/textbidi` | generated | emittable | dependency-blocked | 5/5 | 6/6 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/textinput` | generated | blocked | source-blocked | 2/5 | 55/55 | 0 | 5/6 | 1 | 3 | no |
| `@flighthq/textlayout` | generated | blocked | source-blocked | 11/13 | 51/47 | 0 | 9/11 | 1 | 2 | no |
| `@flighthq/textsegment` | generated | emittable | dependency-blocked | 4/4 | 11/11 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/textshaper` | generated | blocked | source-blocked | 7/9 | 28/31 | 5 | 3/12 | 1 | 3 | no |
| `@flighthq/textshaper-canvas` | generated | emittable | dependency-blocked | 2/2 | 3/3 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/texture` | generated | emittable | dependency-blocked | 5/5 | 42/42 | 0 | 5/6 | 0 | 0 | no |
| `@flighthq/texture-formats` | generated | blocked | source-blocked | 8/9 | 8/6 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/textureatlas` | generated | emittable | dependency-blocked | 4/4 | 20/20 | 0 | 8/13 | 1 | 0 | no |
| `@flighthq/textureatlas-formats` | generated | emittable | dependency-blocked | 8/8 | 29/29 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/tilemap-formats` | generated | blocked | source-blocked | 7/9 | 16/16 | 0 | 1/1 | 2 | 2 | no |
| `@flighthq/tileset` | generated | emittable | dependency-blocked | 3/3 | 9/9 | 0 | 3/8 | 1 | 0 | no |
| `@flighthq/timeline` | generated | emittable | dependency-blocked | 2/2 | 16/16 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/tool-capture` | excluded | excluded | not-applicable | 0/0 | 0/57 | 57 | 0/0 | 0 | 0 | no |
| `@flighthq/tray` | generated | blocked | source-blocked | 1/2 | 0/23 | 23 | 3/3 | 0 | 2 | no |
| `@flighthq/tween` | generated | blocked | source-blocked | 6/9 | 13/35 | 23 | 2/2 | 1 | 4 | no |
| `@flighthq/types` | generated | blocked | source-blocked | 552/590 | 1082/1261 | 179 | 129/129 | 68 | 39 | partial |
| `@flighthq/updater` | generated | blocked | source-blocked | 1/2 | 0/23 | 23 | 2/2 | 0 | 2 | no |
| `@flighthq/useragent` | generated | emittable | dependency-blocked | 3/3 | 12/12 | 0 | 3/6 | 1 | 0 | no |
| `@flighthq/velocity` | generated | emittable | dependency-blocked | 4/4 | 20/20 | 0 | 3/3 | 2 | 0 | no |
| `@flighthq/video` | generated | blocked | source-blocked | 3/4 | 12/16 | 4 | 2/2 | 1 | 2 | no |
| `@flighthq/webcam` | generated | blocked | source-blocked | 1/3 | 0/10 | 10 | 1/1 | 0 | 3 | no |
| `@flighthq/xml` | generated | emittable | compile-blocked | 3/3 | 7/7 | 0 | 5/5 | 2 | 0 | no |

## Blockers

### `@flighthq/binpack`

- **package** `upstream/packages/binpack/src`: Generated crate is missing 1 of 1 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/binpack/src/packRectangles.ts`: compareRectangleId: typeof operand has no inferred Rust type: {"kind":"identifier","name":"a"}

### `@flighthq/bitmaptext`

- **package** `upstream/packages/bitmaptext/src`: Generated crate is missing 15 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/bitmaptext/src/bitmapText.ts`: computeBitmapTextLocalBoundsRectangle: entity runtime field localBoundsRectangle is ambiguous or absent on static receiver BitmapTextRuntime
- **emission** `upstream/packages/bitmaptext/src/updateBitmapText.ts`: updateBitmapText: entity runtime field quadBatches is ambiguous or absent on static receiver BitmapTextRuntime

### `@flighthq/capture`

- **package** `upstream/packages/capture/src`: Generated crate is missing 5 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/capture/src/captureBaseline.ts`: formatCaptureBaseline: JSON.stringify requires a portable scalar or structural array

### `@flighthq/clipboard`

- **package** `upstream/packages/clipboard/src`: Generated crate is missing 32 of 32 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/clipboard/src/clipboard.ts`: createWebClipboardBackend: typeof operand has no inferred Rust type: {"kind":"property","name":"read","object":{"kind":"identifier","name":"cb"},"optional":false}

### `@flighthq/collision`

- **package** `upstream/packages/collision/src`: Generated crate is missing 10 of 19 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/collision/src/shapeCollision.ts`: minOverlapAxis: cannot infer mutable top-level value minOverlapAxis

### `@flighthq/debug`

- **package** `upstream/packages/debug/src`: Generated crate is missing 5 of 9 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/debug/src/debug.ts`: _collectDebugChannels: spread Rust lowering is not implemented

### `@flighthq/dialog`

- **package** `upstream/packages/dialog/src`: Generated crate is missing 15 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/dialog/src/dialog.ts`: createWebDialogBackend: object literal requires an inferred structural type (target={"kind":"named","name":"Promise","arguments":[{"kind":"dynamic"}]}, properties=buttonIndex,cancelled,checkboxChecked)

### `@flighthq/displayobject`

- **package** `upstream/packages/displayobject/src`: Generated crate is missing 18 of 46 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject/src/displayObject.ts`: addDisplayObjectColorAdjustment: entity runtime field colorAdjustments is ambiguous or absent on static receiver DisplayObjectRuntime
- **emission** `upstream/packages/displayobject/src/stage.ts`: createStage: entity runtime field stage is ambiguous or absent on static receiver DisplayObjectRuntime

### `@flighthq/displayobject-canvas`

- **package** `upstream/packages/displayobject-canvas/src`: Generated crate is missing 43 of 94 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-canvas/src/canvasBitmap.ts`: drawCanvasBitmap: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasDisplayObject.ts`: renderCanvasDisplayObject: entity runtime field children is ambiguous or absent on static receiver DisplayObjectRuntime
- **emission** `upstream/packages/displayobject-canvas/src/canvasImageSource.ts`: resolveCanvasImageSource: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=element,version)
- **emission** `upstream/packages/displayobject-canvas/src/canvasParticleEmitter.ts`: drawCanvasParticleEmitter: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasQuadBatch.ts`: drawCanvasQuadBatch: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasRenderState.ts`: createCanvasRenderState: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state
- **emission** `upstream/packages/displayobject-canvas/src/canvasRenderTarget.ts`: beginCanvasRenderPass: optional element access requires an inferred nullable collection
- **emission** `upstream/packages/displayobject-canvas/src/canvasRichText.ts`: drawCanvasRichText: entity runtime field input is ambiguous or absent on static receiver RichTextRuntime
- **emission** `upstream/packages/displayobject-canvas/src/canvasScale9Shape.ts`: drawCanvasScale9Shape: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasShape.ts`: drawCanvasShape: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasSprite.ts`: drawCanvasSprite: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasTextInput.ts`: drawCanvasTextInputOverlay: entity runtime field textLayout is ambiguous or absent on static receiver RichTextRuntime
- **emission** `upstream/packages/displayobject-canvas/src/canvasTextLabel.ts`: drawCanvasTextLabel: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasTilemap.ts`: drawCanvasTilemap: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasVideo.ts`: drawCanvasVideo: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}

### `@flighthq/displayobject-gl`

- **package** `upstream/packages/displayobject-gl/src`: Generated crate is missing 68 of 89 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-gl/src/enableGlColorAdjustmentGuards.ts`: areGlColorAdjustmentGuardsEnabled: entity runtime field glColorAdjustmentGuard is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glBitmap.ts`: destroyGlBitmapData: entity runtime field imageResourceTextureCache is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glCache.ts`: createGlCacheState: entity runtime field defaultBitmapShader is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glClip.ts`: popOneGlClip: entity runtime field clipForms is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glClipContours.ts`: popGlClipContours: entity runtime field currentMaskDepth is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glClipRectangle.ts`: popGlClipRectangle: entity runtime field currentScissorRect is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glColorAdjustment.ts`: enableGlColorAdjustment: entity runtime field glColorAdjustmentFold is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glDisplayObject.ts`: renderGlDisplayObject: entity runtime field tempStack is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glParticleEmitter.ts`: ensureParticleShader: entity runtime field particleShader is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glQuadBatch.ts`: submitGlQuadBatch: entity runtime field spriteBatchCount is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glRichText.ts`: drawGlRichText: entity runtime field input is ambiguous or absent on static receiver RichTextRuntime
- **emission** `upstream/packages/displayobject-gl/src/glScale9Shape.ts`: drawGlScale9Shape: entity runtime field currentTexture is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glShape.ts`: destroyGlShapeData: entity runtime field imageResourceTextureCache is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glShapeMesh.ts`: drawGlShapeMeshBatch: entity runtime field currentProgram is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glSprite.ts`: renderGlSprite: entity runtime field tempStack is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glSpriteBatch.ts`: ensureGlQuadBatchShader: entity runtime field quadBatchShader is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glSpriteRenderer.ts`: submitGlSpriteNode: entity runtime field spriteBatchCount is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glTextLabel.ts`: destroyGlTextLabelData: entity runtime field imageResourceTextureCache is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glTilemap.ts`: submitGlTilemap: entity runtime field spriteBatchCount is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/displayobject-gl/src/glVelocity.ts`: defaultGlDisplayObjectVelocityWriter: upstream/packages/displayobject-gl/src/glVelocity.ts: cannot infer return type for defaultGlDisplayObjectVelocityWriter
- **emission** `upstream/packages/displayobject-gl/src/glVideo.ts`: destroyGlVideoData: entity runtime field videoTextureCache is ambiguous or absent on static receiver GlRenderStateRuntime

### `@flighthq/displayobject-wgpu`

- **package** `upstream/packages/displayobject-wgpu/src`: Generated crate is missing 71 of 95 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-wgpu/src/enableWgpuColorAdjustmentGuards.ts`: areWgpuColorAdjustmentGuardsEnabled: entity runtime field wgpuColorAdjustmentGuard is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuBitmap.ts`: drawWgpuBitmap: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuCache.ts`: createWgpuCacheState: entity runtime field uniformBindGroupLayout is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuClip.ts`: popOneWgpuClip: entity runtime field clipForms is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuClipContours.ts`: popWgpuClipContours: entity runtime field clipContourStack is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuClipRectangle.ts`: popWgpuClipRectangle: entity runtime field scissorStack is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuColorAdjustment.ts`: enableWgpuColorAdjustment: entity runtime field wgpuColorAdjustmentFold is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuDisplayObject.ts`: renderWgpuDisplayObject: entity runtime field tempStack is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuParticleEmitter.ts`: ensureParticleResources: entity runtime field uniformBindGroupLayout is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuQuadBatch.ts`: submitWgpuQuadBatch: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuRenderStats.ts`: ensureWgpuRenderStatsMutable: object literal requires an inferred structural type (target={"arguments":[{"arguments":[],"kind":"named","name":"WgpuRenderStats"}],"kind":"named","name":"Mutable"}, properties=batchFlushCount,drawCallCount,instanceCount,textureUploadCount)
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuRichText.ts`: drawWgpuRichText: entity runtime field input is ambiguous or absent on static receiver RichTextRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuScale9Shape.ts`: drawWgpuScale9Shape: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuShape.ts`: destroyWgpuShapeData: entity runtime field imageResourceTextureCache is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuShapeMesh.ts`: drawWgpuShapeMeshes: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuSprite.ts`: renderWgpuSprite: entity runtime field tempStack is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuSpriteBatch.ts`: ensureWgpuQuadBatchResources: entity runtime field uniformBindGroupLayout is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuSpriteRenderer.ts`: submitWgpuSpriteNode: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuTextLabel.ts`: destroyWgpuTextLabelData: entity runtime field imageResourceTextureCache is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuTilemap.ts`: submitWgpuTilemap: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuVelocity.ts`: defaultWgpuDisplayObjectVelocityWriter: upstream/packages/displayobject-wgpu/src/wgpuVelocity.ts: cannot infer return type for defaultWgpuDisplayObjectVelocityWriter
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuVideo.ts`: drawWgpuVideo: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime

### `@flighthq/effects-canvas`

- **package** `upstream/packages/effects-canvas/src`: Generated crate is missing 78 of 102 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-canvas/src/canvasBloomEffect.ts`: defaultCanvasBloomEffectRunner: upstream/packages/effects-canvas/src/canvasBloomEffect.ts: cannot infer return type for defaultCanvasBloomEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasBlurEffect.ts`: defaultCanvasBlurEffectRunner: upstream/packages/effects-canvas/src/canvasBlurEffect.ts: cannot infer return type for defaultCanvasBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasBokehDepthOfFieldEffect.ts`: defaultCanvasBokehDepthOfFieldEffectRunner: upstream/packages/effects-canvas/src/canvasBokehDepthOfFieldEffect.ts: cannot infer return type for defaultCanvasBokehDepthOfFieldEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasCameraMotionBlurEffect.ts`: defaultCanvasCameraMotionBlurEffectRunner: upstream/packages/effects-canvas/src/canvasCameraMotionBlurEffect.ts: cannot infer return type for defaultCanvasCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasChromaticAberrationEffect.ts`: defaultCanvasChromaticAberrationEffectRunner: upstream/packages/effects-canvas/src/canvasChromaticAberrationEffect.ts: cannot infer return type for defaultCanvasChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasConvolutionEffect.ts`: defaultCanvasConvolutionEffectRunner: upstream/packages/effects-canvas/src/canvasConvolutionEffect.ts: cannot infer return type for defaultCanvasConvolutionEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasCrtEffect.ts`: defaultCanvasCrtEffectRunner: upstream/packages/effects-canvas/src/canvasCrtEffect.ts: cannot infer return type for defaultCanvasCrtEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasDirectionalBlurEffect.ts`: defaultCanvasDirectionalBlurEffectRunner: upstream/packages/effects-canvas/src/canvasDirectionalBlurEffect.ts: cannot infer return type for defaultCanvasDirectionalBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasDisplacementEffect.ts`: defaultCanvasDisplacementEffectRunner: upstream/packages/effects-canvas/src/canvasDisplacementEffect.ts: cannot infer return type for defaultCanvasDisplacementEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasDitherEffect.ts`: defaultCanvasDitherEffectRunner: upstream/packages/effects-canvas/src/canvasDitherEffect.ts: cannot infer return type for defaultCanvasDitherEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasDropShadowEffect.ts`: defaultCanvasDropShadowEffectRunner: upstream/packages/effects-canvas/src/canvasDropShadowEffect.ts: cannot infer return type for defaultCanvasDropShadowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasFilmGrainEffect.ts`: defaultCanvasFilmGrainEffectRunner: upstream/packages/effects-canvas/src/canvasFilmGrainEffect.ts: cannot infer return type for defaultCanvasFilmGrainEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasFxaaEffect.ts`: defaultCanvasFxaaEffectRunner: upstream/packages/effects-canvas/src/canvasFxaaEffect.ts: cannot infer return type for defaultCanvasFxaaEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasGlitchEffect.ts`: defaultCanvasGlitchEffectRunner: upstream/packages/effects-canvas/src/canvasGlitchEffect.ts: cannot infer return type for defaultCanvasGlitchEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasGodRaysEffect.ts`: defaultCanvasGodRaysEffectRunner: upstream/packages/effects-canvas/src/canvasGodRaysEffect.ts: cannot infer return type for defaultCanvasGodRaysEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasHalftoneEffect.ts`: defaultCanvasHalftoneEffectRunner: upstream/packages/effects-canvas/src/canvasHalftoneEffect.ts: cannot infer return type for defaultCanvasHalftoneEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasKuwaharaEffect.ts`: defaultCanvasKuwaharaEffectRunner: upstream/packages/effects-canvas/src/canvasKuwaharaEffect.ts: cannot infer return type for defaultCanvasKuwaharaEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasLensDirtEffect.ts`: defaultCanvasLensDirtEffectRunner: upstream/packages/effects-canvas/src/canvasLensDirtEffect.ts: cannot infer return type for defaultCanvasLensDirtEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasLensDistortionEffect.ts`: defaultCanvasLensDistortionEffectRunner: upstream/packages/effects-canvas/src/canvasLensDistortionEffect.ts: cannot infer return type for defaultCanvasLensDistortionEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasLensFlareEffect.ts`: defaultCanvasLensFlareEffectRunner: upstream/packages/effects-canvas/src/canvasLensFlareEffect.ts: cannot infer return type for defaultCanvasLensFlareEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasMedianEffect.ts`: defaultCanvasMedianEffectRunner: upstream/packages/effects-canvas/src/canvasMedianEffect.ts: cannot infer return type for defaultCanvasMedianEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasMotionBlurEffect.ts`: defaultCanvasMotionBlurEffectRunner: upstream/packages/effects-canvas/src/canvasMotionBlurEffect.ts: cannot infer return type for defaultCanvasMotionBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasOuterGlowEffect.ts`: defaultCanvasOuterGlowEffectRunner: upstream/packages/effects-canvas/src/canvasOuterGlowEffect.ts: cannot infer return type for defaultCanvasOuterGlowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasOutlineEffect.ts`: defaultCanvasOutlineEffectRunner: upstream/packages/effects-canvas/src/canvasOutlineEffect.ts: cannot infer return type for defaultCanvasOutlineEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasPixelateEffect.ts`: defaultCanvasPixelateEffectRunner: upstream/packages/effects-canvas/src/canvasPixelateEffect.ts: cannot infer return type for defaultCanvasPixelateEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasPosterizeEffect.ts`: defaultCanvasPosterizeEffectRunner: upstream/packages/effects-canvas/src/canvasPosterizeEffect.ts: cannot infer return type for defaultCanvasPosterizeEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasRadialBlurEffect.ts`: defaultCanvasRadialBlurEffectRunner: upstream/packages/effects-canvas/src/canvasRadialBlurEffect.ts: cannot infer return type for defaultCanvasRadialBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasScanlinesEffect.ts`: defaultCanvasScanlinesEffectRunner: upstream/packages/effects-canvas/src/canvasScanlinesEffect.ts: cannot infer return type for defaultCanvasScanlinesEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasScreenSpaceFogEffect.ts`: defaultCanvasScreenSpaceFogEffectRunner: upstream/packages/effects-canvas/src/canvasScreenSpaceFogEffect.ts: cannot infer return type for defaultCanvasScreenSpaceFogEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasSharpenEffect.ts`: defaultCanvasSharpenEffectRunner: upstream/packages/effects-canvas/src/canvasSharpenEffect.ts: cannot infer return type for defaultCanvasSharpenEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasSketchEffect.ts`: defaultCanvasSketchEffectRunner: upstream/packages/effects-canvas/src/canvasSketchEffect.ts: cannot infer return type for defaultCanvasSketchEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasSmaaEffect.ts`: defaultCanvasSmaaEffectRunner: upstream/packages/effects-canvas/src/canvasSmaaEffect.ts: cannot infer return type for defaultCanvasSmaaEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasSsaoEffect.ts`: defaultCanvasSsaoEffectRunner: upstream/packages/effects-canvas/src/canvasSsaoEffect.ts: cannot infer return type for defaultCanvasSsaoEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasSsrEffect.ts`: defaultCanvasSsrEffectRunner: upstream/packages/effects-canvas/src/canvasSsrEffect.ts: cannot infer return type for defaultCanvasSsrEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasTaaEffect.ts`: defaultCanvasTaaEffectRunner: upstream/packages/effects-canvas/src/canvasTaaEffect.ts: cannot infer return type for defaultCanvasTaaEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasTiltShiftEffect.ts`: defaultCanvasTiltShiftEffectRunner: upstream/packages/effects-canvas/src/canvasTiltShiftEffect.ts: cannot infer return type for defaultCanvasTiltShiftEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasToneMapEffect.ts`: defaultCanvasToneMapEffectRunner: upstream/packages/effects-canvas/src/canvasToneMapEffect.ts: cannot infer return type for defaultCanvasToneMapEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasVignetteEffect.ts`: defaultCanvasVignetteEffectRunner: upstream/packages/effects-canvas/src/canvasVignetteEffect.ts: cannot infer return type for defaultCanvasVignetteEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasWhiteBalanceEffect.ts`: defaultCanvasWhiteBalanceEffectRunner: upstream/packages/effects-canvas/src/canvasWhiteBalanceEffect.ts: cannot infer return type for defaultCanvasWhiteBalanceEffectRunner

### `@flighthq/effects-gl`

- **package** `upstream/packages/effects-gl/src`: Generated crate is missing 104 of 135 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-gl/src/glBevelEffect.ts`: applyBevelEffectToGl: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,passes)
- **emission** `upstream/packages/effects-gl/src/glBlendEffect.ts`: defaultGlBlendEffectRunner: upstream/packages/effects-gl/src/glBlendEffect.ts: cannot infer return type for defaultGlBlendEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBloomEffect.ts`: applyBloomEffectToGl: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY)
- **emission** `upstream/packages/effects-gl/src/glBlurEffect.ts`: applyBlurEffectToGl: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY)
- **emission** `upstream/packages/effects-gl/src/glBokehDepthOfFieldEffect.ts`: defaultGlBokehDepthOfFieldEffectRunner: upstream/packages/effects-gl/src/glBokehDepthOfFieldEffect.ts: cannot infer return type for defaultGlBokehDepthOfFieldEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCameraMotionBlurEffect.ts`: defaultGlCameraMotionBlurEffectRunner: upstream/packages/effects-gl/src/glCameraMotionBlurEffect.ts: cannot infer return type for defaultGlCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glChromaticAberrationEffect.ts`: defaultGlChromaticAberrationEffectRunner: upstream/packages/effects-gl/src/glChromaticAberrationEffect.ts: cannot infer return type for defaultGlChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCompositeEffect.ts`: defaultGlCompositeEffectRunner: upstream/packages/effects-gl/src/glCompositeEffect.ts: cannot infer return type for defaultGlCompositeEffectRunner
- **emission** `upstream/packages/effects-gl/src/glConvolutionEffect.ts`: defaultGlConvolutionEffectRunner: upstream/packages/effects-gl/src/glConvolutionEffect.ts: cannot infer return type for defaultGlConvolutionEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCrtEffect.ts`: defaultGlCrtEffectRunner: upstream/packages/effects-gl/src/glCrtEffect.ts: cannot infer return type for defaultGlCrtEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCustomShaderEffect.ts`: defaultGlCustomShaderEffectRunner: upstream/packages/effects-gl/src/glCustomShaderEffect.ts: cannot infer return type for defaultGlCustomShaderEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDirectionalBlurEffect.ts`: defaultGlDirectionalBlurEffectRunner: upstream/packages/effects-gl/src/glDirectionalBlurEffect.ts: cannot infer return type for defaultGlDirectionalBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDisplacementEffect.ts`: defaultGlDisplacementEffectRunner: upstream/packages/effects-gl/src/glDisplacementEffect.ts: cannot infer return type for defaultGlDisplacementEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDitherEffect.ts`: defaultGlDitherEffectRunner: upstream/packages/effects-gl/src/glDitherEffect.ts: cannot infer return type for defaultGlDitherEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDropShadowEffect.ts`: applyDropShadowEffectToGl: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,passes)
- **emission** `upstream/packages/effects-gl/src/glFilmGrainEffect.ts`: defaultGlFilmGrainEffectRunner: upstream/packages/effects-gl/src/glFilmGrainEffect.ts: cannot infer return type for defaultGlFilmGrainEffectRunner
- **emission** `upstream/packages/effects-gl/src/glFxaaEffect.ts`: defaultGlFxaaEffectRunner: upstream/packages/effects-gl/src/glFxaaEffect.ts: cannot infer return type for defaultGlFxaaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGlitchEffect.ts`: defaultGlGlitchEffectRunner: upstream/packages/effects-gl/src/glGlitchEffect.ts: cannot infer return type for defaultGlGlitchEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGodRaysEffect.ts`: defaultGlGodRaysEffectRunner: upstream/packages/effects-gl/src/glGodRaysEffect.ts: cannot infer return type for defaultGlGodRaysEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGradientBevelEffect.ts`: applyGradientBevelEffectToGl: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,passes)
- **emission** `upstream/packages/effects-gl/src/glGradientGlowEffect.ts`: applyGradientGlowEffectToGl: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,passes)
- **emission** `upstream/packages/effects-gl/src/glHalftoneEffect.ts`: defaultGlHalftoneEffectRunner: upstream/packages/effects-gl/src/glHalftoneEffect.ts: cannot infer return type for defaultGlHalftoneEffectRunner
- **emission** `upstream/packages/effects-gl/src/glInnerGlowEffect.ts`: applyInnerGlowEffectToGl: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,edgeColor,passes)
- **emission** `upstream/packages/effects-gl/src/glInnerShadowEffect.ts`: applyInnerShadowEffectToGl: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,edgeColor,passes)
- **emission** `upstream/packages/effects-gl/src/glKuwaharaEffect.ts`: defaultGlKuwaharaEffectRunner: upstream/packages/effects-gl/src/glKuwaharaEffect.ts: cannot infer return type for defaultGlKuwaharaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensDirtEffect.ts`: defaultGlLensDirtEffectRunner: upstream/packages/effects-gl/src/glLensDirtEffect.ts: cannot infer return type for defaultGlLensDirtEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensDistortionEffect.ts`: defaultGlLensDistortionEffectRunner: upstream/packages/effects-gl/src/glLensDistortionEffect.ts: cannot infer return type for defaultGlLensDistortionEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensFlareEffect.ts`: defaultGlLensFlareEffectRunner: upstream/packages/effects-gl/src/glLensFlareEffect.ts: cannot infer return type for defaultGlLensFlareEffectRunner
- **emission** `upstream/packages/effects-gl/src/glMedianEffect.ts`: defaultGlMedianEffectRunner: upstream/packages/effects-gl/src/glMedianEffect.ts: cannot infer return type for defaultGlMedianEffectRunner
- **emission** `upstream/packages/effects-gl/src/glMotionBlurEffect.ts`: defaultGlMotionBlurEffectRunner: upstream/packages/effects-gl/src/glMotionBlurEffect.ts: cannot infer return type for defaultGlMotionBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glOuterGlowEffect.ts`: applyOuterGlowEffectToGl: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,passes)
- **emission** `upstream/packages/effects-gl/src/glOutlineEffect.ts`: defaultGlOutlineEffectRunner: upstream/packages/effects-gl/src/glOutlineEffect.ts: cannot infer return type for defaultGlOutlineEffectRunner
- **emission** `upstream/packages/effects-gl/src/glPixelateEffect.ts`: defaultGlPixelateEffectRunner: upstream/packages/effects-gl/src/glPixelateEffect.ts: cannot infer return type for defaultGlPixelateEffectRunner
- **emission** `upstream/packages/effects-gl/src/glPosterizeEffect.ts`: defaultGlPosterizeEffectRunner: upstream/packages/effects-gl/src/glPosterizeEffect.ts: cannot infer return type for defaultGlPosterizeEffectRunner
- **emission** `upstream/packages/effects-gl/src/glRadialBlurEffect.ts`: defaultGlRadialBlurEffectRunner: upstream/packages/effects-gl/src/glRadialBlurEffect.ts: cannot infer return type for defaultGlRadialBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glScanlinesEffect.ts`: defaultGlScanlinesEffectRunner: upstream/packages/effects-gl/src/glScanlinesEffect.ts: cannot infer return type for defaultGlScanlinesEffectRunner
- **emission** `upstream/packages/effects-gl/src/glScreenSpaceFogEffect.ts`: defaultGlScreenSpaceFogEffectRunner: upstream/packages/effects-gl/src/glScreenSpaceFogEffect.ts: cannot infer return type for defaultGlScreenSpaceFogEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSharpenEffect.ts`: defaultGlSharpenEffectRunner: upstream/packages/effects-gl/src/glSharpenEffect.ts: cannot infer return type for defaultGlSharpenEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSketchEffect.ts`: defaultGlSketchEffectRunner: upstream/packages/effects-gl/src/glSketchEffect.ts: cannot infer return type for defaultGlSketchEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSmaaEffect.ts`: defaultGlSmaaEffectRunner: upstream/packages/effects-gl/src/glSmaaEffect.ts: cannot infer return type for defaultGlSmaaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSsaoEffect.ts`: defaultGlSsaoEffectRunner: upstream/packages/effects-gl/src/glSsaoEffect.ts: cannot infer return type for defaultGlSsaoEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSsrEffect.ts`: defaultGlSsrEffectRunner: upstream/packages/effects-gl/src/glSsrEffect.ts: cannot infer return type for defaultGlSsrEffectRunner
- **emission** `upstream/packages/effects-gl/src/glTaaEffect.ts`: defaultGlTaaEffectRunner: upstream/packages/effects-gl/src/glTaaEffect.ts: cannot infer return type for defaultGlTaaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glTiltShiftEffect.ts`: defaultGlTiltShiftEffectRunner: upstream/packages/effects-gl/src/glTiltShiftEffect.ts: cannot infer return type for defaultGlTiltShiftEffectRunner
- **emission** `upstream/packages/effects-gl/src/glToneMapEffect.ts`: defaultGlToneMapEffectRunner: upstream/packages/effects-gl/src/glToneMapEffect.ts: cannot infer return type for defaultGlToneMapEffectRunner
- **emission** `upstream/packages/effects-gl/src/glVignetteEffect.ts`: defaultGlVignetteEffectRunner: upstream/packages/effects-gl/src/glVignetteEffect.ts: cannot infer return type for defaultGlVignetteEffectRunner
- **emission** `upstream/packages/effects-gl/src/glWhiteBalanceEffect.ts`: defaultGlWhiteBalanceEffectRunner: upstream/packages/effects-gl/src/glWhiteBalanceEffect.ts: cannot infer return type for defaultGlWhiteBalanceEffectRunner

### `@flighthq/effects-wgpu`

- **package** `upstream/packages/effects-wgpu/src`: Generated crate is missing 107 of 128 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-wgpu/src/wgpuBevelEffect.ts`: applyBevelEffectToWgpu: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,passes)
- **emission** `upstream/packages/effects-wgpu/src/wgpuBloomEffect.ts`: applyBloomEffectToWgpu: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY)
- **emission** `upstream/packages/effects-wgpu/src/wgpuBlurEffect.ts`: applyBlurEffectToWgpu: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY)
- **emission** `upstream/packages/effects-wgpu/src/wgpuBokehDepthOfFieldEffect.ts`: defaultWgpuBokehDepthOfFieldEffectRunner: upstream/packages/effects-wgpu/src/wgpuBokehDepthOfFieldEffect.ts: cannot infer return type for defaultWgpuBokehDepthOfFieldEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCameraMotionBlurEffect.ts`: defaultWgpuCameraMotionBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuCameraMotionBlurEffect.ts: cannot infer return type for defaultWgpuCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuChromaticAberrationEffect.ts`: defaultWgpuChromaticAberrationEffectRunner: upstream/packages/effects-wgpu/src/wgpuChromaticAberrationEffect.ts: cannot infer return type for defaultWgpuChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuColorLutPass.ts`: REPLACE_BLEND: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=color,alpha)
- **emission** `upstream/packages/effects-wgpu/src/wgpuConvolutionEffect.ts`: defaultWgpuConvolutionEffectRunner: upstream/packages/effects-wgpu/src/wgpuConvolutionEffect.ts: cannot infer return type for defaultWgpuConvolutionEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCrtEffect.ts`: defaultWgpuCrtEffectRunner: upstream/packages/effects-wgpu/src/wgpuCrtEffect.ts: cannot infer return type for defaultWgpuCrtEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDirectionalBlurEffect.ts`: defaultWgpuDirectionalBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuDirectionalBlurEffect.ts: cannot infer return type for defaultWgpuDirectionalBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDisplacementEffect.ts`: defaultWgpuDisplacementEffectRunner: upstream/packages/effects-wgpu/src/wgpuDisplacementEffect.ts: cannot infer return type for defaultWgpuDisplacementEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDitherEffect.ts`: defaultWgpuDitherEffectRunner: upstream/packages/effects-wgpu/src/wgpuDitherEffect.ts: cannot infer return type for defaultWgpuDitherEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDropShadowEffect.ts`: applyDropShadowEffectToWgpu: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,passes)
- **emission** `upstream/packages/effects-wgpu/src/wgpuEffectPass.ts`: PREMUL_BLEND: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=color,alpha)
- **emission** `upstream/packages/effects-wgpu/src/wgpuFilmGrainEffect.ts`: defaultWgpuFilmGrainEffectRunner: upstream/packages/effects-wgpu/src/wgpuFilmGrainEffect.ts: cannot infer return type for defaultWgpuFilmGrainEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuFxaaEffect.ts`: defaultWgpuFxaaEffectRunner: upstream/packages/effects-wgpu/src/wgpuFxaaEffect.ts: cannot infer return type for defaultWgpuFxaaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGlitchEffect.ts`: defaultWgpuGlitchEffectRunner: upstream/packages/effects-wgpu/src/wgpuGlitchEffect.ts: cannot infer return type for defaultWgpuGlitchEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGodRaysEffect.ts`: defaultWgpuGodRaysEffectRunner: upstream/packages/effects-wgpu/src/wgpuGodRaysEffect.ts: cannot infer return type for defaultWgpuGodRaysEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGradientBevelEffect.ts`: applyGradientBevelEffectToWgpu: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,passes)
- **emission** `upstream/packages/effects-wgpu/src/wgpuGradientGlowEffect.ts`: applyGradientGlowEffectToWgpu: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,passes)
- **emission** `upstream/packages/effects-wgpu/src/wgpuHalftoneEffect.ts`: defaultWgpuHalftoneEffectRunner: upstream/packages/effects-wgpu/src/wgpuHalftoneEffect.ts: cannot infer return type for defaultWgpuHalftoneEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuInnerGlowEffect.ts`: applyInnerGlowEffectToWgpu: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,edgeColor,passes)
- **emission** `upstream/packages/effects-wgpu/src/wgpuInnerShadowEffect.ts`: applyInnerShadowEffectToWgpu: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,edgeColor,passes)
- **emission** `upstream/packages/effects-wgpu/src/wgpuKuwaharaEffect.ts`: defaultWgpuKuwaharaEffectRunner: upstream/packages/effects-wgpu/src/wgpuKuwaharaEffect.ts: cannot infer return type for defaultWgpuKuwaharaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensDirtEffect.ts`: defaultWgpuLensDirtEffectRunner: upstream/packages/effects-wgpu/src/wgpuLensDirtEffect.ts: cannot infer return type for defaultWgpuLensDirtEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensDistortionEffect.ts`: defaultWgpuLensDistortionEffectRunner: upstream/packages/effects-wgpu/src/wgpuLensDistortionEffect.ts: cannot infer return type for defaultWgpuLensDistortionEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensFlareEffect.ts`: defaultWgpuLensFlareEffectRunner: upstream/packages/effects-wgpu/src/wgpuLensFlareEffect.ts: cannot infer return type for defaultWgpuLensFlareEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuMedianEffect.ts`: defaultWgpuMedianEffectRunner: upstream/packages/effects-wgpu/src/wgpuMedianEffect.ts: cannot infer return type for defaultWgpuMedianEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuMotionBlurEffect.ts`: defaultWgpuMotionBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuMotionBlurEffect.ts: cannot infer return type for defaultWgpuMotionBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuOuterGlowEffect.ts`: applyOuterGlowEffectToWgpu: object literal requires an inferred structural type (target=unknown, properties=blurX,blurY,passes)
- **emission** `upstream/packages/effects-wgpu/src/wgpuOutlineEffect.ts`: defaultWgpuOutlineEffectRunner: upstream/packages/effects-wgpu/src/wgpuOutlineEffect.ts: cannot infer return type for defaultWgpuOutlineEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuPixelateEffect.ts`: defaultWgpuPixelateEffectRunner: upstream/packages/effects-wgpu/src/wgpuPixelateEffect.ts: cannot infer return type for defaultWgpuPixelateEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuPosterizeEffect.ts`: defaultWgpuPosterizeEffectRunner: upstream/packages/effects-wgpu/src/wgpuPosterizeEffect.ts: cannot infer return type for defaultWgpuPosterizeEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuRadialBlurEffect.ts`: defaultWgpuRadialBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuRadialBlurEffect.ts: cannot infer return type for defaultWgpuRadialBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuRenderEffectPipeline.ts`: presentWgpuRenderEffectResult: entity runtime field commandEncoder is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/effects-wgpu/src/wgpuScanlinesEffect.ts`: defaultWgpuScanlinesEffectRunner: upstream/packages/effects-wgpu/src/wgpuScanlinesEffect.ts: cannot infer return type for defaultWgpuScanlinesEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuScreenSpaceFogEffect.ts`: defaultWgpuScreenSpaceFogEffectRunner: upstream/packages/effects-wgpu/src/wgpuScreenSpaceFogEffect.ts: cannot infer return type for defaultWgpuScreenSpaceFogEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSharpenEffect.ts`: defaultWgpuSharpenEffectRunner: upstream/packages/effects-wgpu/src/wgpuSharpenEffect.ts: cannot infer return type for defaultWgpuSharpenEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSketchEffect.ts`: defaultWgpuSketchEffectRunner: upstream/packages/effects-wgpu/src/wgpuSketchEffect.ts: cannot infer return type for defaultWgpuSketchEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSmaaEffect.ts`: defaultWgpuSmaaEffectRunner: upstream/packages/effects-wgpu/src/wgpuSmaaEffect.ts: cannot infer return type for defaultWgpuSmaaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSsaoEffect.ts`: defaultWgpuSsaoEffectRunner: upstream/packages/effects-wgpu/src/wgpuSsaoEffect.ts: cannot infer return type for defaultWgpuSsaoEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSsrEffect.ts`: defaultWgpuSsrEffectRunner: upstream/packages/effects-wgpu/src/wgpuSsrEffect.ts: cannot infer return type for defaultWgpuSsrEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuTaaEffect.ts`: defaultWgpuTaaEffectRunner: upstream/packages/effects-wgpu/src/wgpuTaaEffect.ts: cannot infer return type for defaultWgpuTaaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuTiltShiftEffect.ts`: defaultWgpuTiltShiftEffectRunner: upstream/packages/effects-wgpu/src/wgpuTiltShiftEffect.ts: cannot infer return type for defaultWgpuTiltShiftEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuToneMapEffect.ts`: defaultWgpuToneMapEffectRunner: upstream/packages/effects-wgpu/src/wgpuToneMapEffect.ts: cannot infer return type for defaultWgpuToneMapEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuVignetteEffect.ts`: defaultWgpuVignetteEffectRunner: upstream/packages/effects-wgpu/src/wgpuVignetteEffect.ts: cannot infer return type for defaultWgpuVignetteEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuWhiteBalanceEffect.ts`: defaultWgpuWhiteBalanceEffectRunner: upstream/packages/effects-wgpu/src/wgpuWhiteBalanceEffect.ts: cannot infer return type for defaultWgpuWhiteBalanceEffectRunner

### `@flighthq/entity`

- **package** `upstream/packages/entity/src`: Generated crate is missing 7 of 12 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/entity/src/binding.ts`: attachEntityBinding: entity runtime field binding is ambiguous or absent on static receiver EntityRuntime
- **emission** `upstream/packages/entity/src/clone.ts`: cloneEntity: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state
- **emission** `upstream/packages/entity/src/runtime.ts`: createEntityRuntime: entity runtime field binding is ambiguous or absent on static receiver EntityRuntime

### `@flighthq/filesystem`

- **package** `upstream/packages/filesystem/src`: Generated crate is missing 43 of 43 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/filesystem/src/filesystem.ts`: createWebFileSystemBackend: await Rust lowering is not implemented

### `@flighthq/geolocation`

- **package** `upstream/packages/geolocation/src`: Generated crate is missing 12 of 12 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/geolocation/src/geolocation.ts`: createWebGeolocationBackend: typeof operand has no inferred Rust type: {"kind":"property","name":"clearWatch","object":{"kind":"identifier","name":"geo"},"optional":false}

### `@flighthq/image-codec`

- **package** `upstream/packages/image-codec/src`: Generated crate is missing 2 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/image-codec/src/registerWebImageDecoders.ts`: decodeImageWithCanvas: await Rust lowering is not implemented
- **emission** `upstream/packages/image-codec/src/registerWebImageEncoders.ts`: createCanvasImageEncoder: await Rust lowering is not implemented

### `@flighthq/interaction`

- **package** `upstream/packages/interaction/src`: Generated crate is missing 62 of 83 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/interaction/src/enableInteractionGuards.ts`: hasEligibleNodeInSubtree: entity runtime field children is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/interaction/src/focusManager.ts`: collectFocusStops: entity runtime field children is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/interaction/src/hitTests.ts`: findFirstHit: entity runtime field children is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/interaction/src/interactionManager.ts`: enableInteractionSignals: entity runtime field interactionSignals is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/interaction/src/interactionSpatialIndex.ts`: collectSpatialCandidates: entity runtime field children is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/interaction/src/nodeInteractionState.ts`: enableNodeInteractionState: entity runtime field interactionState is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/interaction/src/spatialQuery.ts`: hitTestAreaQuery: entity runtime field children is ambiguous or absent on static receiver NodeRuntime

### `@flighthq/intl`

- **package** `upstream/packages/intl/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/intl/src/cache.ts`: getCacheKey: typeof operand has no inferred Rust type: {"kind":"identifier","name":"locale"}
- **emission** `upstream/packages/intl/src/collator.ts`: getCollator: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.Collator")
- **emission** `upstream/packages/intl/src/datetime.ts`: formatDateValue: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.DateTimeFormat")
- **emission** `upstream/packages/intl/src/list.ts`: formatList: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.ListFormat")
- **emission** `upstream/packages/intl/src/number.ts`: formatCompactNumber: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=notation,spread)
- **emission** `upstream/packages/intl/src/plural.ts`: selectOrdinalCategory: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=type,spread)
- **emission** `upstream/packages/intl/src/relativeTime.ts`: formatRelativeTime: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.RelativeTimeFormat")

### `@flighthq/ipc`

- **package** `upstream/packages/ipc/src`: Generated crate is missing 17 of 17 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/ipc/src/ipc.ts`: invokeIpcWithTimeout: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/loader`

- **package** `upstream/packages/loader/src`: Generated crate is missing 13 of 13 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/loader/src/resourceLoader.ts`: cancelResourceLoad: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/log`

- **package** `upstream/packages/log/src`: Generated crate is missing 65 of 65 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/log/src/log.ts`: createChildLogContext: object literal requires an inferred structural type (target=unknown, properties=spread,spread)

### `@flighthq/mediasession`

- **package** `upstream/packages/mediasession/src`: Generated crate is missing 10 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/mediasession/src/mediasession.ts`: createWebMediaSessionBackend: typeof operand has no inferred Rust type: {"kind":"property","name":"setPositionState","object":{"kind":"identifier","name":"session"},"optional":false}

### `@flighthq/menu`

- **package** `upstream/packages/menu/src`: Generated crate is missing 11 of 17 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/menu/src/menu.ts`: showWebContextMenu: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/mesh`

- **package** `upstream/packages/mesh/src`: Generated crate is missing 12 of 67 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/mesh/src/meshGeometry.ts`: destroyMeshGeometryGlData: entity runtime field webglData is ambiguous or absent on static receiver MeshGeometryRuntime

### `@flighthq/movieclip`

- **package** `upstream/packages/movieclip/src`: Generated crate is missing 22 of 23 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/movieclip/src/movieClip.ts`: createMovieClipRuntime: entity runtime field movieClipSignals is ambiguous or absent on static receiver MovieClipRuntime

### `@flighthq/net`

- **package** `upstream/packages/net/src`: Generated crate is missing 4 of 4 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/net/src/net.ts`: createWebNetBackend: await Rust lowering is not implemented

### `@flighthq/node`

- **package** `upstream/packages/node/src`: Generated crate is missing 75 of 105 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/node/src/boundsRectangle.ts`: getNodeLocalBoundsRectangle: entity runtime field localBoundsRectangle is ambiguous or absent on static receiver HasBoundsRectangleRuntime
- **emission** `upstream/packages/node/src/hasBoundsRectangle.ts`: initBoundsRectangleRuntimeTrait: entity runtime field boundsRectangle is ambiguous or absent on static receiver HasBoundsRectangleRuntime
- **emission** `upstream/packages/node/src/hasTransform2d.ts`: initTransform2DRuntimeTrait: entity runtime field localMatrix is ambiguous or absent on static receiver HasTransform2DRuntime
- **emission** `upstream/packages/node/src/hasTransform3d.ts`: initTransform3DRuntimeTrait: entity runtime field localMatrix4 is ambiguous or absent on static receiver HasTransform3DRuntime
- **emission** `upstream/packages/node/src/hierarchy.ts`: addNodeChildAt: entity runtime field children is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/node/src/node.ts`: createNodeRuntime: entity runtime field appearanceId is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/node/src/revision.ts`: computeNodeWorldTransformRevision: entity runtime field localTransformId is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/node/src/traversal.ts`: findNode: entity runtime field children is ambiguous or absent on static receiver NodeRuntime

### `@flighthq/notification`

- **package** `upstream/packages/notification/src`: Generated crate is missing 26 of 26 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/notification/src/notification.ts`: createServiceWorkerNotificationBackend: await Rust lowering is not implemented

### `@flighthq/particleemitter`

- **package** `upstream/packages/particleemitter/src`: Generated crate is missing 21 of 51 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/particleemitter/src/particleEmitter.ts`: copyLocalBoundsRectangle: entity runtime field localBoundsRectangle is ambiguous or absent on static receiver ParticleEmitterRuntime

### `@flighthq/particles-formats`

- **package** `upstream/packages/particles-formats/src`: Generated crate is missing 25 of 79 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/particles-formats/src/libgdxParse.ts`: sectionsToDocument: object literal requires an inferred structural type (target=unknown, properties=shape,edges,side)
- **emission** `upstream/packages/particles-formats/src/libgdxSerialize.ts`: documentToText: spread Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/particleDesignerParse.ts`: num: typeof operand has no inferred Rust type: {"kind":"identifier","name":"v"}
- **emission** `upstream/packages/particles-formats/src/spineParse.ts`: rawToDocument: object literal requires an inferred structural type (target={"kind":"primitive","name":"Float"}, properties=low,high)
- **emission** `upstream/packages/particles-formats/src/spineSerialize.ts`: serializeSpineParticle: JSON.stringify requires a portable scalar or structural array
- **emission** `upstream/packages/particles-formats/src/starlingPexParse.ts`: extractAttr: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/particles-formats/src/unitySerialize.ts`: configToDocument: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"x","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"y","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"z","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}

### `@flighthq/path-boolean`

- **package** `upstream/packages/path-boolean/src`: Generated crate is missing 1 of 12 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/path-boolean/src/martinezKernel.ts`: buildArrangement: new-expression Rust lowering is not implemented: event_heap

### `@flighthq/permissions`

- **package** `upstream/packages/permissions/src`: Generated crate is missing 5 of 5 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/permissions/src/permission.ts`: requestWebGeolocationPermission: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/picking`

- **package** `upstream/packages/picking/src`: Generated crate is missing 6 of 6 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/picking/src/pickScene.ts`: pickNode: entity runtime field children is ambiguous or absent on static receiver NodeRuntime

### `@flighthq/render`

- **package** `upstream/packages/render/src`: Generated crate is missing 42 of 63 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render/src/enableColorAdjustmentGuards.ts`: areColorAdjustmentGuardsEnabled: entity runtime field colorAdjustmentChannelMixingGuard is ambiguous or absent on static receiver RenderStateRuntime
- **emission** `upstream/packages/render/src/explainDisplayObjectRender.ts`: explainDisplayObjectRender: entity runtime field rendererMap is ambiguous or absent on static receiver RenderStateRuntime
- **emission** `upstream/packages/render/src/renderAppearance.ts`: updateRenderProxyAppearance: entity runtime field currentFrameId is ambiguous or absent on static receiver RenderStateRuntime
- **emission** `upstream/packages/render/src/renderColorTransform.ts`: updateRenderProxyColorTransform: entity runtime field resolvedColorTransform is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/render/src/renderer.ts`: copyRenderersFromRenderState: entity runtime field rendererMap is ambiguous or absent on static receiver RenderStateRuntime
- **emission** `upstream/packages/render/src/renderProxy.ts`: createRenderProxy: entity runtime field rendererMap is ambiguous or absent on static receiver RenderStateRuntime
- **emission** `upstream/packages/render/src/renderProxyAdapter.ts`: applyRenderProxyAdapter: entity runtime field renderProxyAdapterMap is ambiguous or absent on static receiver RenderStateRuntime
- **emission** `upstream/packages/render/src/renderQueue.ts`: buildRenderQueue: entity runtime field renderProxyMap is ambiguous or absent on static receiver RenderStateRuntime
- **emission** `upstream/packages/render/src/renderState.ts`: createRenderStateRuntime: entity runtime field colorAdjustmentChannelMixingGuard is ambiguous or absent on static receiver RenderStateRuntime
- **emission** `upstream/packages/render/src/renderTransform2d.ts`: updateRenderProxy2DTransform: entity runtime field currentFrameId is ambiguous or absent on static receiver RenderStateRuntime
- **emission** `upstream/packages/render/src/sceneRender.ts`: collectVisibleMeshes: entity runtime field children is ambiguous or absent on static receiver NodeRuntime

### `@flighthq/render-gl`

- **package** `upstream/packages/render-gl/src`: Generated crate is missing 54 of 75 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-gl/src/glBackground.ts`: renderGlBackground: entity runtime field renderTargetViewport is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/render-gl/src/glCompressedTexture.ts`: registerGlCompressedTextureDecoder: entity runtime field compressedTextureDecoder is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/render-gl/src/glDraw.ts`: applyGlBlendMode: entity runtime field currentBlendMode is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/render-gl/src/glFullscreenPass.ts`: clearGlRenderTarget: entity runtime field currentFramebuffer is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/render-gl/src/glMaterialRegistry.ts`: getGlMaterialRenderer: entity runtime field materialRendererMap is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/render-gl/src/glReadback.ts`: readGlRenderTargetPixels: entity runtime field currentFramebuffer is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/render-gl/src/glRenderPass.ts`: beginGlRenderPass: entity runtime field currentFramebuffer is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/render-gl/src/glRenderState.ts`: createGlRenderState: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=alpha,antialias,powerPreference,stencil,spread)
- **emission** `upstream/packages/render-gl/src/glRenderTarget.ts`: createGlRenderTarget: entity runtime field currentTexture is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/render-gl/src/glShader.ts`: createDefaultGlBitmapShader: entity runtime field renderTargetViewport is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/render-gl/src/glShaderBinding.ts`: getGlMaterialShader: entity runtime field materialBitmapShaderMap is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/render-gl/src/glShaderRegistry.ts`: registerGlBitmapShader: entity runtime field defaultBitmapShader is ambiguous or absent on static receiver GlRenderStateRuntime

### `@flighthq/render-wgpu`

- **package** `upstream/packages/render-wgpu/src`: Generated crate is missing 55 of 68 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-wgpu/src/wgpuBackground.ts`: ensureWgpuDepthStencil: entity runtime field depthStencilTexture is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/render-wgpu/src/wgpuDraw.ts`: applyWgpuBlendMode: entity runtime field currentBlendMode is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/render-wgpu/src/wgpuFullscreenPass.ts`: drawWgpuFullscreenPass: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/render-wgpu/src/wgpuMaterialRegistry.ts`: getWgpuMaterialRenderer: entity runtime field materialRendererMap is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/render-wgpu/src/wgpuMipmap.ts`: generateWgpuMipmaps: entity runtime field mipmapBindGroupLayout is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/render-wgpu/src/wgpuRenderState.ts`: destroyWgpuRenderState: entity runtime field uniformBuffer is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/render-wgpu/src/wgpuRenderTarget.ts`: beginWgpuRenderPassEncoder: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=r,g,b,a)
- **emission** `upstream/packages/render-wgpu/src/wgpuScissor.ts`: applyWgpuScissorRect: entity runtime field currentScissorRect is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/render-wgpu/src/wgpuShader.ts`: buildStencilFaceState: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=compare,passOp,failOp,depthFailOp)
- **emission** `upstream/packages/render-wgpu/src/wgpuShaderBinding.ts`: resolveWgpuShader: entity runtime field webgpuShaderBindingResolver is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/render-wgpu/src/wgpuShaderRegistry.ts`: registerWgpuBitmapShader: entity runtime field defaultBitmapShader is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/render-wgpu/src/wgpuSurface.ts`: acquireWgpuFrameCaptureTexture: entity runtime field frameCaptureEnabled is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/render-wgpu/src/wgpuTestHelper.ts`: installWgpuConstants: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=MAP_READ,MAP_WRITE,COPY_SRC,COPY_DST,INDEX,VERTEX,UNIFORM,STORAGE,INDIRECT,QUERY_RESOLVE)

### `@flighthq/scene`

- **package** `upstream/packages/scene/src`: Generated crate is missing 17 of 43 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene/src/billboardCamera.ts`: orientBillboardSubtree: entity runtime field children is ambiguous or absent on static receiver SceneNodeRuntime
- **emission** `upstream/packages/scene/src/sceneNode.ts`: createSceneNodeRuntime: entity runtime field traits is ambiguous or absent on static receiver SceneNodeRuntime
- **emission** `upstream/packages/scene/src/sceneNodeAppearance.ts`: ensureSceneNodeWorldAlpha: entity runtime field parent is ambiguous or absent on static receiver SceneNodeRuntime
- **emission** `upstream/packages/scene/src/sceneNodeBounds.ts`: _accumulateWorldBounds: entity runtime field children is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/scene/src/sceneNodeCulling.ts`: _cullNode: entity runtime field children is ambiguous or absent on static receiver NodeRuntime

### `@flighthq/scene-formats`

- **package** `upstream/packages/scene-formats/src`: Generated crate is missing 10 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-formats/src/awdParse.ts`: createSceneFromAwd: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene-formats/src/gltfParse.ts`: buildGltfAnimationClip: object literal requires an inferred structural type (target=unknown, properties=components,interpolation,quaternion,times,values)
- **emission** `upstream/packages/scene-formats/src/gltfSchema.ts`: GltfNormalTextureInfo: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"KHR_texture_transform","optional":true,"type":{"arguments":[],"kind":"named","name":"GltfTextureTransform"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene-formats/src/md2Parse.ts`: createSceneFromMd2: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene-formats/src/md5AnimParse.ts`: buildAnimationClip: object literal requires an inferred structural type (target=unknown, properties=components,times,values)
- **emission** `upstream/packages/scene-formats/src/md5Parse.ts`: createSceneFromMd5Mesh: object literal requires an inferred structural type (target=unknown, properties=x,y,z)
- **emission** `upstream/packages/scene-formats/src/shared.ts`: findSceneSkeletonJoints: spread Rust lowering is not implemented
- **emission** `upstream/packages/scene-formats/src/threeDsParse.ts`: createSceneFrom3ds: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/scene-gl`

- **package** `upstream/packages/scene-gl/src`: Generated crate is missing 10 of 184 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-gl/src/glParticleEmitter3D.ts`: collectParticleEmitter3DNodes: entity runtime field children is ambiguous or absent on static receiver NodeRuntime
- **emission** `upstream/packages/scene-gl/src/glSceneRuntime.ts`: getGlSceneRuntime: entity runtime field sceneMeshMaterialRegistry is ambiguous or absent on static receiver GlRenderStateRuntime
- **emission** `upstream/packages/scene-gl/src/glSceneTestHelper.ts`: makeFakeGl2: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=calls,ARRAY_BUFFER,ELEMENT_ARRAY_BUFFER,STATIC_DRAW,FLOAT,UNSIGNED_BYTE,UNSIGNED_SHORT,UNSIGNED_INT,TRIANGLES,TEXTURE0,TEXTURE1,TEXTURE_2D,VERTEX_SHADER,FRAGMENT_SHADER,COMPILE_STATUS,LINK_STATUS,ACTIVE_UNIFORMS,FLOAT_VEC2,FLOAT_VEC3,FLOAT_VEC4,FLOAT_MAT2,FLOAT_MAT3,FLOAT_MAT4,BLEND,CULL_FACE,BACK,DEPTH_TEST,LESS,ONE,ONE_MINUS_SRC_ALPHA,SRC_ALPHA,FUNC_ADD,FRAMEBUFFER,COLOR_BUFFER_BIT,DEPTH_BUFFER_BIT,COLOR,DEPTH_STENCIL,MAX_VERTEX_UNIFORM_VECTORS,RGBA32F,NEAREST,CLAMP_TO_EDGE,TEXTURE_MIN_FILTER,TEXTURE_MAG_FILTER,TEXTURE_WRAP_S,TEXTURE_WRAP_T,getParameter,createShader,shaderSource,compileShader,getShaderParameter,getShaderInfoLog,deleteShader,createProgram,attachShader,linkProgram,getProgramParameter,getActiveUniform,getProgramInfoLog,useProgram,getUniformLocation,createBuffer,bindBuffer,bufferData,createVertexArray,bindVertexArray,deleteBuffer,deleteFramebuffer,deleteProgram,deleteRenderbuffer,deleteTexture,deleteVertexArray,enableVertexAttribArray,getAttribLocation,vertexAttribPointer,vertexAttribIPointer,vertexAttrib4f,vertexAttribDivisor,bufferSubData,bindFramebuffer,blendEquation,blendFunc,clear,clearColor,clearDepth,clearBufferfv,clearBufferfi,cullFace,depthFunc,depthMask,flush,viewport,disable,enable,drawElements,drawElementsInstanced,drawArrays,activeTexture,bindTexture,createTexture,texParameteri,texImage2D,texSubImage2D,pixelStorei,uniform1i,uniform1f,uniform1fv,uniform2f,uniform2fv,uniform3f,uniform3fv,uniform4f,uniform4fv,uniformMatrix3fv,uniformMatrix4fv)

### `@flighthq/scene-resources`

- **package** `upstream/packages/scene-resources/src`: Generated crate is missing 5 of 37 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-resources/src/revealSceneResourcesOnResolve.ts`: revealSceneResourcesOnResolve: object literal requires an inferred structural type (target={"kind":"union","variants":[{"arguments":[{"arguments":[],"kind":"named","name":"T"}],"kind":"named","name":"NumericProps"},{"arguments":[],"kind":"named","name":"TweenOptions"}]}, properties=alpha)
- **emission** `upstream/packages/scene-resources/src/sceneResourceFetch.ts`: createWebSceneResourceFetch: await Rust lowering is not implemented

### `@flighthq/scene-wgpu`

- **package** `upstream/packages/scene-wgpu/src`: Generated crate is missing 126 of 140 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-wgpu/src/anisotropyPbrWgpuMeshMaterialRenderer.ts`: anisotropyPbrWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/blinnPhongWgpuMeshMaterialRenderer.ts`: blinnPhongWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/clearcoatPbrWgpuMeshMaterialRenderer.ts`: clearcoatPbrWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/depthWgpuMeshMaterialRenderer.ts`: depthWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/emissiveWgpuMeshMaterialRenderer.ts`: emissiveWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/iridescencePbrWgpuMeshMaterialRenderer.ts`: iridescencePbrWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/lambertWgpuMeshMaterialRenderer.ts`: lambertWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/matcapWgpuMeshMaterialRenderer.ts`: matcapWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/normalWgpuMeshMaterialRenderer.ts`: normalWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/phongWgpuMeshMaterialRenderer.ts`: phongWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/sheenPbrWgpuMeshMaterialRenderer.ts`: sheenPbrWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts`: specularGlossinessPbrWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/specularPbrWgpuMeshMaterialRenderer.ts`: specularPbrWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts`: standardPbrWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/subsurfacePbrWgpuMeshMaterialRenderer.ts`: subsurfacePbrWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/toonWgpuMeshMaterialRenderer.ts`: toonWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/transmissionVolumePbrWgpuMeshMaterialRenderer.ts`: transmissionVolumePbrWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/unlitWgpuMeshMaterialRenderer.ts`: unlitWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/vertexColorWgpuMeshMaterialRenderer.ts`: vertexColorWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts`: compileWgpuClassicPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module,shadowBindGroupLayout)
- **emission** `upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts`: compileWgpuDebugPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module)
- **emission** `upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts`: BAKE_CLEAR: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=r,g,b,a)
- **emission** `upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts`: drawWgpuEnvironmentSkybox: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts`: compileWgpuMatcapPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module)
- **emission** `upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts`: beginWgpuMeshDraw: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/wgpuMeshUpload.ts`: ensureWgpuMeshUpload: entity runtime field webgpuData is ambiguous or absent on static receiver MeshGeometryRuntime
- **emission** `upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts`: wgpuParticleBlendState: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=operation,srcFactor,dstFactor)
- **emission** `upstream/packages/scene-wgpu/src/wgpuPbrPipelineCache.ts`: compileWgpuPbrPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module,pbrSampleBindGroupLayout)
- **emission** `upstream/packages/scene-wgpu/src/wgpuSceneRuntime.ts`: getWgpuSceneRuntime: entity runtime field sceneMeshMaterialRegistry is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/wgpuSceneTestHelper.ts`: installWgpuConstants: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=MAP_READ,MAP_WRITE,COPY_SRC,COPY_DST,INDEX,VERTEX,UNIFORM,STORAGE,INDIRECT,QUERY_RESOLVE)
- **emission** `upstream/packages/scene-wgpu/src/wgpuShadowMap.ts`: drawWgpuSceneShadowMap: entity runtime field commandEncoder is ambiguous or absent on static receiver WgpuRenderStateRuntime
- **emission** `upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts`: compileWgpuToonPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module,shadowBindGroupLayout)
- **emission** `upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts`: compileWgpuUnlitPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module)
- **emission** `upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts`: compileWgpuWireframePipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module,topology)
- **emission** `upstream/packages/scene-wgpu/src/wireframeWgpuMeshMaterialRenderer.ts`: wireframeWgpuMeshMaterialRenderer: entity runtime field renderPass is ambiguous or absent on static receiver WgpuRenderStateRuntime

### `@flighthq/sdk`

- **package** `upstream/packages/sdk/src`: Generated crate is missing 5923 of 5923 upstream exports; re-export or declaration synthesis is required.

### `@flighthq/sensors`

- **package** `upstream/packages/sensors/src`: Generated crate is missing 32 of 32 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/sensors/src/sensors.ts`: createWebSensorsBackend: await Rust lowering is not implemented

### `@flighthq/shading`

- **package** `upstream/packages/shading/src`: Generated crate is missing 1 of 37 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shading/src/orderModifierStack.ts`: orderModifierStack: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=index,modifier)

### `@flighthq/shape`

- **package** `upstream/packages/shape/src`: Generated crate is missing 11 of 42 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shape/src/shape.ts`: copyShapeCommands: spread Rust lowering is not implemented
- **emission** `upstream/packages/shape/src/shapeHitTestBuiltins.ts`: enableShapeHitTesting: object literal requires an inferred structural type (target={"arguments":[{"arguments":[],"kind":"named","name":"K"}],"kind":"named","name":"ShapeHitTestCommand"}, properties=key,hitTest)

### `@flighthq/shape-formats`

- **package** `upstream/packages/shape-formats/src`: Generated crate is missing 5 of 5 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shape-formats/src/shapeJson.ts`: formatShapeJson: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=bitmap)

### `@flighthq/share`

- **package** `upstream/packages/share/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/share/src/share.ts`: createWebShareBackend: object literal requires an inferred structural type (target={"kind":"named","name":"Promise","arguments":[{"kind":"dynamic"}]}, properties=completed,activityType,dismissed)

### `@flighthq/shortcut`

- **package** `upstream/packages/shortcut/src`: Generated crate is missing 26 of 26 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shortcut/src/shortcut.ts`: parseAcceleratorDetailed: in-operator requires a static property name or an opaque host receiver

### `@flighthq/sprite`

- **package** `upstream/packages/sprite/src`: Generated crate is missing 30 of 64 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/sprite/src/quadBatch.ts`: copyLocalBoundsRectangle: entity runtime field localBoundsRectangle is ambiguous or absent on static receiver QuadBatchRuntime

### `@flighthq/spritesheet-formats`

- **package** `upstream/packages/spritesheet-formats/src`: Generated crate is missing 4 of 55 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/spritesheet-formats/src/asepriteSerialize.ts`: dataToMeta: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=direction,from,name,to,spread)
- **emission** `upstream/packages/spritesheet-formats/src/texturePackerSerialize.ts`: dataToMeta: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=direction,from,name,to)

### `@flighthq/statusbar`

- **package** `upstream/packages/statusbar/src`: Generated crate is missing 16 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/statusbar/src/statusbar.ts`: pushStatusBarStyleEntry: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"handle","optional":false,"type":{"arguments":[],"kind":"named","name":"StatusBarStyleEntryHandle"}},{"name":"entry","optional":false,"type":{"arguments":[],"kind":"named","name":"StatusBarStyleEntry"}}],"kind":"anonymous"}

### `@flighthq/storage`

- **package** `upstream/packages/storage/src`: Generated crate is missing 39 of 39 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/storage/src/storage.ts`: setStorageJSON: JSON.stringify requires a portable scalar or structural array

### `@flighthq/text`

- **package** `upstream/packages/text/src`: Generated crate is missing 85 of 86 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/text/src/nativeText.ts`: computeNativeTextLocalBoundsRectangle: entity runtime field measuredWidth is ambiguous or absent on static receiver NativeTextRuntime
- **emission** `upstream/packages/text/src/richText.ts`: createRichTextRuntime: entity runtime field buildTextLayoutParams is ambiguous or absent on static receiver RichTextRuntime
- **emission** `upstream/packages/text/src/textLabel.ts`: createTextLabelRuntime: entity runtime field buildTextLayoutParams is ambiguous or absent on static receiver TextLabelRuntime
- **emission** `upstream/packages/text/src/textLabelLayout.ts`: ensureTextLayout: entity runtime field textLayout is ambiguous or absent on static receiver TextLabelRuntime

### `@flighthq/text-markup`

- **package** `upstream/packages/text-markup/src`: Generated crate is missing 2 of 8 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/text-markup/src/textMarkup.ts`: handleMarkupToken: multiple object spreads require ordered Rust lowering

### `@flighthq/textinput`

- **emission** `upstream/packages/textinput/src/selectableRichTextManager.ts`: blurSelectableRichText: entity runtime field selectionBeginIndex is ambiguous or absent on static receiver RichTextRuntime
- **emission** `upstream/packages/textinput/src/textInput.ts`: disableTextInput: entity runtime field input is ambiguous or absent on static receiver RichTextRuntime
- **emission** `upstream/packages/textinput/src/textInputManager.ts`: dispatchTextInputPointerDown: entity runtime field textLayout is ambiguous or absent on static receiver RichTextRuntime

### `@flighthq/textlayout`

- **emission** `upstream/packages/textlayout/src/richTextContent.ts`: clearRichTextContent: entity runtime field richTextContent is ambiguous or absent on static receiver RichTextRuntime
- **emission** `upstream/packages/textlayout/src/textLayoutRuntime.ts`: clearTextLayoutResult: entity runtime field textLayout is ambiguous or absent on static receiver TextLabelRuntime

### `@flighthq/textshaper`

- **package** `upstream/packages/textshaper/src`: Generated crate is missing 5 of 31 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textshaper/src/textShaper.ts`: setTextShaperBackend: optional call requires an inferred nullable function: {"kind":"identifier","name":"_textShaperBackendHook"}
- **emission** `upstream/packages/textshaper/src/textShaperItemize.ts`: shapeTextRuns: object literal requires an inferred structural type (target=unknown, properties=spread,direction,script)

### `@flighthq/texture-formats`

- **emission** `upstream/packages/texture-formats/src/byteReader.ts`: createByteReader: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/tilemap-formats`

- **emission** `upstream/packages/tilemap-formats/src/tiledJsonParse.ts`: boolField: typeof operand has no inferred Rust type: {"kind":"identifier","name":"value"}
- **emission** `upstream/packages/tilemap-formats/src/tiledXmlParse.ts`: buildTiledLayerFromXml: object literal requires an inferred structural type (target={"arguments":[],"kind":"named","name":"TiledLayer"}, properties=spread,data,height,type,width)

### `@flighthq/tray`

- **package** `upstream/packages/tray/src`: Generated crate is missing 23 of 23 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tray/src/tray.ts`: getTrayIcons: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=id)

### `@flighthq/tween`

- **package** `upstream/packages/tween/src`: Generated crate is missing 23 of 35 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tween/src/colorTween.ts`: createColorTween: object literal requires an inferred structural type (target={"kind":"union","variants":[{"arguments":[{"arguments":[],"kind":"named","name":"T"}],"kind":"named","name":"NumericProps"},{"arguments":[],"kind":"named","name":"TweenOptions"}]}, properties=b,g,r)
- **emission** `upstream/packages/tween/src/timer.ts`: createTweenTimer: object literal requires an inferred structural type (target={"kind":"union","variants":[{"arguments":[],"kind":"named","name":"T"},{"kind":"primitive","name":"Float"}]}, properties=)
- **emission** `upstream/packages/tween/src/tween.ts`: makeTween: object literal requires an inferred structural type (target={"kind":"primitive","name":"Float"}, properties=change,key,start)

### `@flighthq/types`

- **package** `upstream/packages/types/src`: Generated crate is missing 179 of 1261 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/types/src/Billboard.ts`: BillboardRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Bitmap.ts`: BitmapRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/BitmapText.ts`: BitmapTextRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/CanvasRenderState.ts`: CanvasRenderStateRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/DisplayContainer.ts`: DisplayContainerRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/DisplayObject.ts`: DisplayObjectRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/DomRenderState.ts`: DomRenderStateRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Entity.ts`: EntityRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/GlRenderState.ts`: GlRenderStateRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Group.ts`: GroupRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/HasAppearance.ts`: HasAppearanceRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/HasBoundsRectangle.ts`: HasBoundsRectangleRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/HasTransform2D.ts`: HasTransform2DRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/HasTransform3D.ts`: HasTransform3DRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/HtmlView.ts`: HtmlViewRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/InstancedMesh.ts`: InstancedMeshRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/LodMesh.ts`: LodMeshRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Mesh.ts`: MeshRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/MeshGeometry.ts`: MeshGeometryRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/MovieClip.ts`: MovieClipRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/NativeText.ts`: NativeTextRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Node.ts`: NodeRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/ParticleEmitter.ts`: ParticleEmitterRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/ParticleEmitter3D.ts`: ParticleEmitter3DRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/QuadBatch.ts`: QuadBatchRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/RenderState.ts`: RenderStateRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/RenderView.ts`: RenderViewRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/RichText.ts`: RichTextRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Scale9Shape.ts`: Scale9ShapeRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Scene.ts`: SceneRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/SceneNode.ts`: SceneNodeRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Shape.ts`: ShapeRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Sprite.ts`: SpriteRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Stage.ts`: StageRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/TextLabel.ts`: TextLabelRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Tilemap.ts`: TilemapRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/Video.ts`: VideoRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits
- **emission** `upstream/packages/types/src/WgpuRenderState.ts`: WgpuRenderStateRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits

### `@flighthq/updater`

- **package** `upstream/packages/updater/src`: Generated crate is missing 23 of 23 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/updater/src/updater.ts`: attachAppUpdater: object literal requires an inferred structural type (target={"kind":"primitive","name":"Float"}, properties=spread,phase)

### `@flighthq/video`

- **package** `upstream/packages/video/src`: Generated crate is missing 4 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/video/src/videoResourceFrom.ts`: loadVideoResourceFromUrl: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/webcam`

- **package** `upstream/packages/webcam/src`: Generated crate is missing 10 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/webcam/src/webcam.ts`: createWebWebcamBackend: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/webcam/src/webcamStream.ts`: WebcamStreamRuntime: aggregate native entity runtime closure is unavailable: entity runtime extension NodeRuntime retains generic field canAddChild: Traits

## Candidate compile blockers

### `@flighthq/xml`

- **E0425** `generated/candidates/flighthq-xml/src/xml_parse.rs`: cannot find value `string` in this scope
- **E0425** `generated/candidates/flighthq-xml/src/xml_parse.rs`: cannot find value `string` in this scope
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no method named `is_some` found for enum `OpaqueHostValue` in the current scope
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: can't compare `String` with `OpaqueHostValue`
- **E0070** `generated/candidates/flighthq-xml/src/xml_parse.rs`: invalid left-hand side of assignment
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0609** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no field `index_of` on type `String`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0609** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no field `slice` on type `String`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0609** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no field `slice` on type `String`
- **E0368** `generated/candidates/flighthq-xml/src/xml_parse.rs`: binary assignment operation `+=` cannot be applied to type `&str`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0609** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no field `slice` on type `String`
- **E0599** `generated/candidates/flighthq-xml/src/xml_query.rs`: no method named `is_some` found for struct `String` in the current scope
- **E0599** `generated/candidates/flighthq-xml/src/xml_query.rs`: no method named `is_none` found for struct `String` in the current scope
- **E0425** `generated/candidates/flighthq-xml/src/xml_query.rs`: cannot find function `number` in this scope
- **E0609** `generated/candidates/flighthq-xml/src/xml_query.rs`: no field `filter` on type `Vec<xml_parse::XmlElement>`
- **E0308** `generated/candidates/flighthq-xml/src/xml_query.rs`: mismatched types

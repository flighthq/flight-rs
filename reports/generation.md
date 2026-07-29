# Automatic Rust Generation

Upstream commit: `5d24729f7360475e28a105ae0caeeaa2e1328260`

| Metric | Count |
| --- | ---: |
| Inventoried packages | 131 |
| Default-generated packages | 125 |
| Emittable packages | 67 |
| Blocked packages | 58 |
| Compiled candidates | 28 |
| Compile-blocked candidates | 23 |
| Dependency-blocked candidates | 15 |
| Cultivated packages | 1 |
| Host-bound packages | 4 |
| Excluded packages | 1 |
| Source/package blockers | 322 |

| Package | Disposition | Status | Candidate | Sources emitted/attempted | API generated/expected | Missing | Dependents direct/transitive | Opaque sources | Blockers | Target |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `@flighthq/accessibility` | generated | emittable | compile-blocked | 2/2 | 8/8 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/adjustments` | generated | emittable | compiled | 19/19 | 49/49 | 0 | 6/25 | 0 | 0 | no |
| `@flighthq/animation` | generated | emittable | compile-blocked | 4/4 | 18/18 | 0 | 3/7 | 1 | 0 | no |
| `@flighthq/app` | generated | emittable | compile-blocked | 2/2 | 42/42 | 0 | 4/4 | 1 | 0 | no |
| `@flighthq/application` | generated | emittable | compiled | 3/3 | 83/83 | 0 | 3/3 | 2 | 0 | no |
| `@flighthq/assets` | generated | emittable | compile-blocked | 2/2 | 10/10 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/audio` | generated | emittable | compile-blocked | 4/4 | 20/20 | 0 | 2/2 | 2 | 0 | no |
| `@flighthq/binpack` | generated | blocked | source-blocked | 1/2 | 0/1 | 1 | 1/1 | 0 | 2 | no |
| `@flighthq/bitmapfont` | generated | emittable | compiled | 3/3 | 7/7 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/bitmapfont-formats` | generated | emittable | dependency-blocked | 5/5 | 9/4 | 0 | 1/1 | 3 | 0 | no |
| `@flighthq/bitmaptext` | generated | emittable | dependency-blocked | 3/3 | 15/15 | 0 | 1/1 | 2 | 0 | no |
| `@flighthq/camera` | generated | emittable | compiled | 10/10 | 31/31 | 0 | 4/4 | 0 | 0 | no |
| `@flighthq/camera2d` | generated | emittable | compiled | 8/8 | 8/8 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/capture` | generated | blocked | source-blocked | 2/3 | 5/10 | 5 | 1/1 | 1 | 2 | no |
| `@flighthq/clip` | generated | emittable | compile-blocked | 2/2 | 23/23 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/clipboard` | generated | blocked | source-blocked | 1/2 | 0/32 | 32 | 4/4 | 0 | 2 | no |
| `@flighthq/clock` | generated | emittable | compile-blocked | 12/12 | 14/14 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/collision` | generated | blocked | source-blocked | 5/6 | 9/19 | 10 | 1/1 | 0 | 2 | no |
| `@flighthq/color` | generated | emittable | compiled | 10/10 | 32/32 | 0 | 10/29 | 0 | 0 | no |
| `@flighthq/connectivity` | generated | emittable | compile-blocked | 2/2 | 14/14 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/debug` | generated | blocked | source-blocked | 2/3 | 4/9 | 5 | 1/1 | 1 | 2 | no |
| `@flighthq/device` | generated | emittable | compiled | 2/2 | 14/14 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/dialog` | generated | blocked | source-blocked | 1/2 | 0/15 | 15 | 5/5 | 0 | 2 | no |
| `@flighthq/displayobject` | generated | blocked | source-blocked | 7/8 | 36/46 | 10 | 14/24 | 6 | 2 | no |
| `@flighthq/displayobject-canvas` | generated | blocked | source-blocked | 17/31 | 53/94 | 41 | 5/5 | 15 | 15 | no |
| `@flighthq/displayobject-dom` | host-bound | host-bound | not-applicable | 0/0 | 0/58 | 58 | 1/1 | 0 | 0 | no |
| `@flighthq/displayobject-gl` | generated | blocked | source-blocked | 25/28 | 79/89 | 12 | 1/1 | 15 | 4 | no |
| `@flighthq/displayobject-wgpu` | generated | blocked | source-blocked | 22/29 | 69/95 | 27 | 1/1 | 13 | 8 | no |
| `@flighthq/easing` | generated | emittable | promoted | 20/20 | 48/48 | 0 | 2/3 | 0 | 0 | full |
| `@flighthq/effects` | generated | emittable | compile-blocked | 72/72 | 112/112 | 0 | 4/4 | 2 | 0 | no |
| `@flighthq/effects-canvas` | generated | blocked | source-blocked | 9/48 | 29/102 | 78 | 1/1 | 7 | 40 | no |
| `@flighthq/effects-gl` | generated | blocked | source-blocked | 11/58 | 31/135 | 104 | 1/1 | 8 | 48 | no |
| `@flighthq/effects-wgpu` | generated | blocked | source-blocked | 10/56 | 26/128 | 102 | 1/1 | 2 | 47 | no |
| `@flighthq/entity` | generated | emittable | compiled | 6/6 | 12/12 | 0 | 20/61 | 1 | 0 | no |
| `@flighthq/filesystem` | generated | blocked | source-blocked | 1/2 | 0/43 | 43 | 2/2 | 0 | 2 | no |
| `@flighthq/flow` | generated | emittable | compiled | 10/10 | 9/9 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/font` | generated | emittable | compile-blocked | 8/8 | 15/15 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/geolocation` | generated | blocked | source-blocked | 1/2 | 0/12 | 12 | 2/2 | 0 | 2 | no |
| `@flighthq/geometry` | generated | emittable | compiled | 27/27 | 377/377 | 0 | 40/53 | 0 | 0 | no |
| `@flighthq/glyphatlas` | generated | emittable | dependency-blocked | 7/7 | 14/14 | 0 | 1/1 | 2 | 0 | no |
| `@flighthq/haptics` | generated | emittable | compiled | 2/2 | 13/13 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/host-capacitor` | host-bound | host-bound | not-applicable | 0/0 | 0/63 | 63 | 0/0 | 0 | 0 | no |
| `@flighthq/host-electron` | host-bound | host-bound | not-applicable | 0/0 | 0/57 | 57 | 0/0 | 0 | 0 | no |
| `@flighthq/host-tauri` | host-bound | host-bound | not-applicable | 0/0 | 0/51 | 51 | 0/0 | 0 | 0 | no |
| `@flighthq/image` | generated | emittable | compile-blocked | 3/3 | 20/20 | 0 | 11/24 | 2 | 0 | partial |
| `@flighthq/image-codec` | generated | blocked | source-blocked | 6/8 | 14/16 | 2 | 3/26 | 0 | 3 | no |
| `@flighthq/input` | generated | emittable | compiled | 2/2 | 40/40 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/interaction` | generated | blocked | source-blocked | 10/16 | 34/83 | 49 | 1/1 | 9 | 7 | no |
| `@flighthq/intl` | generated | blocked | source-blocked | 1/8 | 0/14 | 14 | 1/1 | 0 | 8 | no |
| `@flighthq/ipc` | generated | blocked | source-blocked | 1/2 | 0/17 | 17 | 2/2 | 0 | 2 | no |
| `@flighthq/keyboard` | generated | emittable | compiled | 2/2 | 20/20 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/lifecycle` | generated | emittable | compiled | 2/2 | 13/13 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/lighting` | generated | emittable | compiled | 11/11 | 37/37 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/loader` | generated | blocked | source-blocked | 1/2 | 0/13 | 13 | 3/3 | 0 | 2 | no |
| `@flighthq/log` | generated | blocked | source-blocked | 1/2 | 0/65 | 65 | 7/16 | 0 | 2 | no |
| `@flighthq/materials` | generated | emittable | compile-blocked | 12/12 | 68/68 | 0 | 7/28 | 1 | 0 | no |
| `@flighthq/math` | generated | emittable | compiled | 16/16 | 73/73 | 0 | 4/4 | 0 | 0 | no |
| `@flighthq/media` | generated | emittable | compile-blocked | 4/4 | 42/42 | 0 | 1/1 | 3 | 0 | no |
| `@flighthq/mediasession` | generated | blocked | source-blocked | 1/2 | 0/10 | 10 | 1/1 | 0 | 2 | no |
| `@flighthq/menu` | generated | blocked | source-blocked | 2/3 | 6/17 | 11 | 3/3 | 0 | 2 | no |
| `@flighthq/mesh` | generated | emittable | compile-blocked | 12/12 | 67/67 | 0 | 6/20 | 2 | 0 | no |
| `@flighthq/motionpath` | generated | emittable | compiled | 8/8 | 7/7 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/movieclip` | generated | emittable | dependency-blocked | 3/3 | 23/23 | 0 | 1/1 | 2 | 0 | no |
| `@flighthq/net` | generated | blocked | source-blocked | 1/2 | 0/4 | 4 | 1/1 | 0 | 2 | no |
| `@flighthq/node` | generated | blocked | source-blocked | 12/16 | 54/105 | 51 | 23/32 | 6 | 5 | no |
| `@flighthq/notification` | generated | blocked | source-blocked | 1/2 | 0/26 | 26 | 4/4 | 0 | 2 | no |
| `@flighthq/particleemitter` | generated | emittable | dependency-blocked | 11/11 | 51/51 | 0 | 1/1 | 10 | 0 | no |
| `@flighthq/particles` | generated | emittable | compile-blocked | 11/11 | 50/50 | 0 | 3/3 | 1 | 0 | no |
| `@flighthq/particles-formats` | generated | blocked | source-blocked | 14/21 | 54/79 | 25 | 1/1 | 5 | 8 | no |
| `@flighthq/path` | generated | emittable | compiled | 23/23 | 50/50 | 0 | 8/8 | 0 | 0 | no |
| `@flighthq/path-boolean` | generated | blocked | source-blocked | 7/8 | 12/12 | 1 | 1/1 | 0 | 2 | no |
| `@flighthq/path-formats` | generated | emittable | compile-blocked | 2/2 | 3/3 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/permissions` | generated | blocked | source-blocked | 1/2 | 0/5 | 5 | 1/1 | 0 | 2 | no |
| `@flighthq/picking` | generated | blocked | source-blocked | 1/2 | 0/6 | 6 | 1/1 | 0 | 2 | no |
| `@flighthq/platform` | generated | emittable | compiled | 2/2 | 16/16 | 0 | 3/3 | 1 | 0 | no |
| `@flighthq/power` | generated | emittable | compiled | 2/2 | 19/19 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/protocol` | generated | emittable | compile-blocked | 2/2 | 20/20 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/render` | generated | blocked | source-blocked | 14/17 | 39/63 | 24 | 9/13 | 10 | 4 | no |
| `@flighthq/render-gl` | generated | blocked | source-blocked | 22/24 | 66/75 | 18 | 4/4 | 15 | 3 | no |
| `@flighthq/render-wgpu` | generated | blocked | source-blocked | 12/18 | 32/68 | 36 | 5/5 | 9 | 7 | no |
| `@flighthq/scene` | generated | blocked | source-blocked | 9/14 | 26/43 | 17 | 6/6 | 7 | 6 | no |
| `@flighthq/scene-formats` | generated | blocked | source-blocked | 9/16 | 83/15 | 9 | 2/2 | 2 | 8 | no |
| `@flighthq/scene-gl` | generated | blocked | source-blocked | 51/53 | 182/184 | 2 | 1/1 | 24 | 3 | no |
| `@flighthq/scene-resources` | generated | blocked | source-blocked | 14/16 | 32/37 | 5 | 1/1 | 13 | 3 | no |
| `@flighthq/scene-wgpu` | generated | blocked | source-blocked | 30/42 | 67/140 | 73 | 1/1 | 18 | 13 | no |
| `@flighthq/screen` | generated | emittable | compiled | 2/2 | 31/31 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/sdk` | generated | blocked | source-blocked | 14/14 | 0/5923 | 5923 | 0/0 | 0 | 1 | no |
| `@flighthq/sensors` | generated | blocked | source-blocked | 1/2 | 0/32 | 32 | 1/1 | 0 | 2 | no |
| `@flighthq/shading` | generated | blocked | source-blocked | 16/17 | 36/37 | 1 | 2/2 | 2 | 2 | no |
| `@flighthq/shape` | generated | blocked | source-blocked | 5/7 | 31/42 | 11 | 7/8 | 2 | 3 | no |
| `@flighthq/shape-formats` | generated | blocked | source-blocked | 1/2 | 0/5 | 5 | 1/1 | 0 | 2 | no |
| `@flighthq/share` | generated | blocked | source-blocked | 1/2 | 0/14 | 14 | 2/2 | 0 | 2 | no |
| `@flighthq/shell` | generated | emittable | compile-blocked | 2/2 | 14/14 | 0 | 3/3 | 1 | 0 | no |
| `@flighthq/shortcut` | generated | blocked | source-blocked | 1/2 | 0/26 | 26 | 3/3 | 0 | 2 | no |
| `@flighthq/signals` | generated | emittable | compiled | 6/6 | 15/14 | 0 | 42/72 | 0 | 0 | no |
| `@flighthq/skeleton3d` | generated | emittable | dependency-blocked | 6/6 | 16/16 | 0 | 3/16 | 2 | 0 | no |
| `@flighthq/snapshot` | generated | emittable | compile-blocked | 5/5 | 4/4 | 0 | 1/1 | 4 | 0 | no |
| `@flighthq/socket` | generated | emittable | compile-blocked | 2/2 | 11/11 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/spatial` | generated | emittable | compiled | 3/3 | 10/10 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/spring` | generated | emittable | compiled | 8/8 | 12/12 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/sprite` | generated | emittable | dependency-blocked | 4/4 | 64/64 | 0 | 3/3 | 3 | 0 | no |
| `@flighthq/spritesheet` | generated | emittable | dependency-blocked | 8/8 | 32/32 | 0 | 2/2 | 4 | 0 | no |
| `@flighthq/spritesheet-formats` | generated | blocked | source-blocked | 14/16 | 51/55 | 4 | 1/1 | 7 | 3 | no |
| `@flighthq/statusbar` | generated | blocked | source-blocked | 1/2 | 0/16 | 16 | 2/2 | 0 | 2 | no |
| `@flighthq/storage` | generated | blocked | source-blocked | 1/2 | 0/39 | 39 | 2/2 | 0 | 2 | no |
| `@flighthq/surface` | cultivated | cultivated | not-applicable | 0/0 | 0/136 | 136 | 6/9 | 0 | 0 | partial |
| `@flighthq/text` | generated | blocked | source-blocked | 5/6 | 70/86 | 16 | 8/9 | 3 | 2 | no |
| `@flighthq/text-markup` | generated | blocked | source-blocked | 4/5 | 6/8 | 2 | 1/1 | 1 | 2 | no |
| `@flighthq/textbidi` | generated | emittable | compile-blocked | 5/5 | 6/6 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/textinput` | generated | emittable | dependency-blocked | 5/5 | 55/55 | 0 | 5/6 | 4 | 0 | no |
| `@flighthq/textlayout` | generated | emittable | dependency-blocked | 13/13 | 51/47 | 0 | 9/11 | 1 | 0 | no |
| `@flighthq/textsegment` | generated | emittable | compiled | 4/4 | 11/11 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/textshaper` | generated | blocked | source-blocked | 7/9 | 28/31 | 5 | 3/12 | 1 | 3 | no |
| `@flighthq/textshaper-canvas` | generated | emittable | dependency-blocked | 2/2 | 3/3 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/texture` | generated | emittable | compiled | 5/5 | 42/42 | 0 | 5/6 | 0 | 0 | no |
| `@flighthq/texture-formats` | generated | blocked | source-blocked | 8/9 | 8/6 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/textureatlas` | generated | emittable | dependency-blocked | 4/4 | 20/20 | 0 | 8/13 | 1 | 0 | no |
| `@flighthq/textureatlas-formats` | generated | emittable | dependency-blocked | 8/8 | 29/29 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/tilemap-formats` | generated | blocked | source-blocked | 7/9 | 16/16 | 0 | 1/1 | 2 | 2 | no |
| `@flighthq/tileset` | generated | emittable | dependency-blocked | 3/3 | 9/9 | 0 | 3/8 | 1 | 0 | no |
| `@flighthq/timeline` | generated | emittable | compile-blocked | 2/2 | 16/16 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/tool-capture` | excluded | excluded | not-applicable | 0/0 | 0/57 | 57 | 0/0 | 0 | 0 | no |
| `@flighthq/tray` | generated | blocked | source-blocked | 1/2 | 0/23 | 23 | 3/3 | 0 | 2 | no |
| `@flighthq/tween` | generated | blocked | source-blocked | 6/9 | 13/35 | 23 | 2/2 | 1 | 4 | no |
| `@flighthq/types` | generated | emittable | compiled | 590/590 | 1261/1261 | 0 | 129/129 | 76 | 0 | partial |
| `@flighthq/updater` | generated | blocked | source-blocked | 1/2 | 0/23 | 23 | 2/2 | 0 | 2 | no |
| `@flighthq/useragent` | generated | emittable | compiled | 3/3 | 12/12 | 0 | 3/6 | 1 | 0 | no |
| `@flighthq/velocity` | generated | emittable | dependency-blocked | 4/4 | 20/20 | 0 | 3/3 | 2 | 0 | no |
| `@flighthq/video` | generated | blocked | source-blocked | 3/4 | 12/16 | 4 | 2/2 | 1 | 2 | no |
| `@flighthq/webcam` | generated | blocked | source-blocked | 1/3 | 0/10 | 10 | 1/1 | 0 | 3 | no |
| `@flighthq/xml` | generated | emittable | compile-blocked | 3/3 | 7/7 | 0 | 5/5 | 2 | 0 | no |

## Blockers

### `@flighthq/binpack`

- **package** `upstream/packages/binpack/src`: Generated crate is missing 1 of 1 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/binpack/src/packRectangles.ts`: compareRectangleId: typeof operand has no inferred Rust type: {"kind":"identifier","name":"a"}

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

- **package** `upstream/packages/displayobject/src`: Generated crate is missing 10 of 46 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject/src/displayObject.ts`: createDisplayObjectRuntime: entity runtime field traits is unavailable on static receiver DisplayObjectRuntime: entity runtime extension NodeRuntime field traits retains generic parameter Traits

### `@flighthq/displayobject-canvas`

- **package** `upstream/packages/displayobject-canvas/src`: Generated crate is missing 41 of 94 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-canvas/src/canvasBitmap.ts`: drawCanvasBitmap: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasDisplayObject.ts`: renderCanvasDisplayObject: entity runtime field children is unavailable on static receiver DisplayObjectRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/displayobject-canvas/src/canvasImageSource.ts`: resolveCanvasImageSource: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=element,version)
- **emission** `upstream/packages/displayobject-canvas/src/canvasParticleEmitter.ts`: drawCanvasParticleEmitter: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasQuadBatch.ts`: drawCanvasQuadBatch: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasRenderState.ts`: createCanvasRenderState: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state
- **emission** `upstream/packages/displayobject-canvas/src/canvasRenderTarget.ts`: beginCanvasRenderPass: optional element access requires an inferred nullable collection
- **emission** `upstream/packages/displayobject-canvas/src/canvasRichText.ts`: drawCanvasRichTextField: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasScale9Shape.ts`: drawCanvasScale9Shape: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasShape.ts`: drawCanvasShape: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasSprite.ts`: drawCanvasSprite: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasTextLabel.ts`: drawCanvasTextLabel: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasTilemap.ts`: drawCanvasTilemap: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
- **emission** `upstream/packages/displayobject-canvas/src/canvasVideo.ts`: drawCanvasVideo: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}

### `@flighthq/displayobject-gl`

- **package** `upstream/packages/displayobject-gl/src`: Generated crate is missing 12 of 89 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-gl/src/glDisplayObject.ts`: renderGlDisplayObject: entity runtime field children is unavailable on static receiver DisplayObjectRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/displayobject-gl/src/glSprite.ts`: renderGlSprite: entity runtime field children is unavailable on static receiver DisplayObjectRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/displayobject-gl/src/glVelocity.ts`: defaultGlDisplayObjectVelocityWriter: upstream/packages/displayobject-gl/src/glVelocity.ts: cannot infer return type for defaultGlDisplayObjectVelocityWriter

### `@flighthq/displayobject-wgpu`

- **package** `upstream/packages/displayobject-wgpu/src`: Generated crate is missing 27 of 95 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuClipContours.ts`: ensureClipContourPipelines: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=arrayStride,attributes)
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuDisplayObject.ts`: renderWgpuDisplayObject: entity runtime field children is unavailable on static receiver DisplayObjectRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuRenderStats.ts`: ensureWgpuRenderStatsMutable: object literal requires an inferred structural type (target={"arguments":[{"arguments":[],"kind":"named","name":"WgpuRenderStats"}],"kind":"named","name":"Mutable"}, properties=batchFlushCount,drawCallCount,instanceCount,textureUploadCount)
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuShapeMesh.ts`: ensureShapeMeshPipeline: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=arrayStride,attributes)
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuSprite.ts`: renderWgpuSprite: entity runtime field children is unavailable on static receiver DisplayObjectRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuSpriteBatch.ts`: NORMAL_BLEND: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=color,alpha)
- **emission** `upstream/packages/displayobject-wgpu/src/wgpuVelocity.ts`: defaultWgpuDisplayObjectVelocityWriter: upstream/packages/displayobject-wgpu/src/wgpuVelocity.ts: cannot infer return type for defaultWgpuDisplayObjectVelocityWriter

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

- **package** `upstream/packages/effects-wgpu/src`: Generated crate is missing 102 of 128 upstream exports; re-export or declaration synthesis is required.
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

- **package** `upstream/packages/interaction/src`: Generated crate is missing 49 of 83 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/interaction/src/enableInteractionGuards.ts`: hasEligibleNodeInSubtree: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/interaction/src/focusManager.ts`: collectFocusStops: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/interaction/src/hitTests.ts`: findFirstHit: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/interaction/src/interactionManager.ts`: hasInteractionSignalSubscriberInGraph: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/interaction/src/interactionSpatialIndex.ts`: collectSpatialCandidates: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/interaction/src/spatialQuery.ts`: hitTestAreaQuery: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits

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

### `@flighthq/net`

- **package** `upstream/packages/net/src`: Generated crate is missing 4 of 4 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/net/src/net.ts`: createWebNetBackend: await Rust lowering is not implemented

### `@flighthq/node`

- **package** `upstream/packages/node/src`: Generated crate is missing 51 of 105 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/node/src/boundsRectangle.ts`: recomputeWorldBoundsRectangle: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/node/src/hierarchy.ts`: addNodeChildAt: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/node/src/node.ts`: createNodeRuntime: entity runtime field canAddChild is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field canAddChild retains generic parameter Traits
- **emission** `upstream/packages/node/src/traversal.ts`: findNode: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits

### `@flighthq/notification`

- **package** `upstream/packages/notification/src`: Generated crate is missing 26 of 26 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/notification/src/notification.ts`: createServiceWorkerNotificationBackend: await Rust lowering is not implemented

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
- **emission** `upstream/packages/picking/src/pickScene.ts`: pickNode: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits

### `@flighthq/render`

- **package** `upstream/packages/render/src`: Generated crate is missing 24 of 63 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render/src/renderProxy.ts`: walkNode: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/render/src/renderQueue.ts`: buildRenderQueue: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/render/src/sceneRender.ts`: collectVisibleMeshes: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits

### `@flighthq/render-gl`

- **package** `upstream/packages/render-gl/src`: Generated crate is missing 18 of 75 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-gl/src/glDraw.ts`: bindGlVideoTexture: object literal requires an inferred structural type (target=unknown, properties=texture,uploadedFrameId)
- **emission** `upstream/packages/render-gl/src/glRenderState.ts`: createGlRenderState: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=alpha,antialias,powerPreference,stencil,spread)

### `@flighthq/render-wgpu`

- **package** `upstream/packages/render-wgpu/src`: Generated crate is missing 36 of 68 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-wgpu/src/wgpuBackground.ts`: renderWgpuBackground: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=r,g,b,a)
- **emission** `upstream/packages/render-wgpu/src/wgpuDraw.ts`: drawWgpuQuadWithTransform: object literal requires an inferred structural type (target=unknown, properties=alpha,transform2D)
- **emission** `upstream/packages/render-wgpu/src/wgpuRenderState.ts`: getWgpuSampler: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=minFilter,magFilter,addressModeU,addressModeV)
- **emission** `upstream/packages/render-wgpu/src/wgpuRenderTarget.ts`: beginWgpuRenderPassEncoder: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=r,g,b,a)
- **emission** `upstream/packages/render-wgpu/src/wgpuShader.ts`: buildStencilFaceState: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=compare,passOp,failOp,depthFailOp)
- **emission** `upstream/packages/render-wgpu/src/wgpuTestHelper.ts`: installWgpuConstants: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=MAP_READ,MAP_WRITE,COPY_SRC,COPY_DST,INDEX,VERTEX,UNIFORM,STORAGE,INDIRECT,QUERY_RESOLVE)

### `@flighthq/scene`

- **package** `upstream/packages/scene/src`: Generated crate is missing 17 of 43 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene/src/billboardCamera.ts`: orientBillboardSubtree: entity runtime field children is unavailable on static receiver SceneNodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/scene/src/sceneNode.ts`: createSceneNodeRuntime: entity runtime field traits is unavailable on static receiver SceneNodeRuntime: entity runtime extension NodeRuntime field traits retains generic parameter Traits
- **emission** `upstream/packages/scene/src/sceneNodeAppearance.ts`: ensureSceneNodeWorldAlpha: entity runtime field parent is unavailable on static receiver SceneNodeRuntime: entity runtime extension NodeRuntime field parent retains generic parameter Traits
- **emission** `upstream/packages/scene/src/sceneNodeBounds.ts`: _accumulateWorldBounds: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/scene/src/sceneNodeCulling.ts`: _cullNode: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits

### `@flighthq/scene-formats`

- **package** `upstream/packages/scene-formats/src`: Generated crate is missing 9 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-formats/src/awdParse.ts`: createSceneFromAwd: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene-formats/src/gltfParse.ts`: buildGltfAnimationClip: object literal requires an inferred structural type (target=unknown, properties=components,interpolation,quaternion,times,values)
- **emission** `upstream/packages/scene-formats/src/gltfSchema.ts`: GltfNormalTextureInfo: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"KHR_texture_transform","optional":true,"type":{"arguments":[],"kind":"named","name":"GltfTextureTransform"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene-formats/src/md2Parse.ts`: createSceneFromMd2: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene-formats/src/md5AnimParse.ts`: buildAnimationClip: object literal requires an inferred structural type (target=unknown, properties=components,times,values)
- **emission** `upstream/packages/scene-formats/src/shared.ts`: findSceneSkeletonJoints: spread Rust lowering is not implemented
- **emission** `upstream/packages/scene-formats/src/threeDsParse.ts`: createSceneFrom3ds: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/scene-gl`

- **package** `upstream/packages/scene-gl/src`: Generated crate is missing 2 of 184 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-gl/src/glParticleEmitter3D.ts`: collectParticleEmitter3DNodes: entity runtime field children is unavailable on static receiver NodeRuntime: entity runtime extension NodeRuntime field children retains generic parameter Traits
- **emission** `upstream/packages/scene-gl/src/glSceneTestHelper.ts`: makeFakeGl2: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=calls,ARRAY_BUFFER,ELEMENT_ARRAY_BUFFER,STATIC_DRAW,FLOAT,UNSIGNED_BYTE,UNSIGNED_SHORT,UNSIGNED_INT,TRIANGLES,TEXTURE0,TEXTURE1,TEXTURE_2D,VERTEX_SHADER,FRAGMENT_SHADER,COMPILE_STATUS,LINK_STATUS,ACTIVE_UNIFORMS,FLOAT_VEC2,FLOAT_VEC3,FLOAT_VEC4,FLOAT_MAT2,FLOAT_MAT3,FLOAT_MAT4,BLEND,CULL_FACE,BACK,DEPTH_TEST,LESS,ONE,ONE_MINUS_SRC_ALPHA,SRC_ALPHA,FUNC_ADD,FRAMEBUFFER,COLOR_BUFFER_BIT,DEPTH_BUFFER_BIT,COLOR,DEPTH_STENCIL,MAX_VERTEX_UNIFORM_VECTORS,RGBA32F,NEAREST,CLAMP_TO_EDGE,TEXTURE_MIN_FILTER,TEXTURE_MAG_FILTER,TEXTURE_WRAP_S,TEXTURE_WRAP_T,getParameter,createShader,shaderSource,compileShader,getShaderParameter,getShaderInfoLog,deleteShader,createProgram,attachShader,linkProgram,getProgramParameter,getActiveUniform,getProgramInfoLog,useProgram,getUniformLocation,createBuffer,bindBuffer,bufferData,createVertexArray,bindVertexArray,deleteBuffer,deleteFramebuffer,deleteProgram,deleteRenderbuffer,deleteTexture,deleteVertexArray,enableVertexAttribArray,getAttribLocation,vertexAttribPointer,vertexAttribIPointer,vertexAttrib4f,vertexAttribDivisor,bufferSubData,bindFramebuffer,blendEquation,blendFunc,clear,clearColor,clearDepth,clearBufferfv,clearBufferfi,cullFace,depthFunc,depthMask,flush,viewport,disable,enable,drawElements,drawElementsInstanced,drawArrays,activeTexture,bindTexture,createTexture,texParameteri,texImage2D,texSubImage2D,pixelStorei,uniform1i,uniform1f,uniform1fv,uniform2f,uniform2fv,uniform3f,uniform3fv,uniform4f,uniform4fv,uniformMatrix3fv,uniformMatrix4fv)

### `@flighthq/scene-resources`

- **package** `upstream/packages/scene-resources/src`: Generated crate is missing 5 of 37 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-resources/src/revealSceneResourcesOnResolve.ts`: revealSceneResourcesOnResolve: object literal requires an inferred structural type (target={"kind":"union","variants":[{"arguments":[{"arguments":[],"kind":"named","name":"T"}],"kind":"named","name":"NumericProps"},{"arguments":[],"kind":"named","name":"TweenOptions"}]}, properties=alpha)
- **emission** `upstream/packages/scene-resources/src/sceneResourceFetch.ts`: createWebSceneResourceFetch: await Rust lowering is not implemented

### `@flighthq/scene-wgpu`

- **package** `upstream/packages/scene-wgpu/src`: Generated crate is missing 73 of 140 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-wgpu/src/wgpuClassicPrelude.ts`: compileWgpuClassicPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module,shadowBindGroupLayout)
- **emission** `upstream/packages/scene-wgpu/src/wgpuDebugPrelude.ts`: compileWgpuDebugPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module)
- **emission** `upstream/packages/scene-wgpu/src/wgpuEnvironmentIblBake.ts`: BAKE_CLEAR: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=r,g,b,a)
- **emission** `upstream/packages/scene-wgpu/src/wgpuMatcapPrelude.ts`: compileWgpuMatcapPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module)
- **emission** `upstream/packages/scene-wgpu/src/wgpuMeshPipeline.ts`: VERTEX_BUFFER_LAYOUTS: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=arrayStride,attributes)
- **emission** `upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts`: wgpuParticleBlendState: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=operation,srcFactor,dstFactor)
- **emission** `upstream/packages/scene-wgpu/src/wgpuPbrPipelineCache.ts`: compileWgpuPbrPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module,pbrSampleBindGroupLayout)
- **emission** `upstream/packages/scene-wgpu/src/wgpuSceneTestHelper.ts`: installWgpuConstants: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=MAP_READ,MAP_WRITE,COPY_SRC,COPY_DST,INDEX,VERTEX,UNIFORM,STORAGE,INDIRECT,QUERY_RESOLVE)
- **emission** `upstream/packages/scene-wgpu/src/wgpuShadowMap.ts`: SHADOW_VERTEX_BUFFER_LAYOUTS: object literal requires an inferred structural type (target={"kind":"dynamic"}, properties=arrayStride,attributes)
- **emission** `upstream/packages/scene-wgpu/src/wgpuToonPrelude.ts`: compileWgpuToonPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module,shadowBindGroupLayout)
- **emission** `upstream/packages/scene-wgpu/src/wgpuUnlitPrelude.ts`: compileWgpuUnlitPipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module)
- **emission** `upstream/packages/scene-wgpu/src/wgpuWireframePrelude.ts`: compileWgpuWireframePipeline: object literal requires an inferred structural type (target=unknown, properties=doubleSided,format,materialBindGroupLayout,module,topology)

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

### `@flighthq/spritesheet-formats`

- **package** `upstream/packages/spritesheet-formats/src`: Generated crate is missing 4 of 55 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/spritesheet-formats/src/asepriteSerialize.ts`: dataToMeta: object literal requires an inferred structural type (target={"kind":"primitive","name":"Void"}, properties=direction,from,name,to,spread)
- **emission** `upstream/packages/spritesheet-formats/src/texturePackerSerialize.ts`: dataToMeta: object literal requires an inferred structural type (target={"kind":"primitive","name":"Void"}, properties=direction,from,name,to)

### `@flighthq/statusbar`

- **package** `upstream/packages/statusbar/src`: Generated crate is missing 16 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/statusbar/src/statusbar.ts`: pushStatusBarStyleEntry: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"handle","optional":false,"type":{"arguments":[],"kind":"named","name":"StatusBarStyleEntryHandle"}},{"name":"entry","optional":false,"type":{"arguments":[],"kind":"named","name":"StatusBarStyleEntry"}}],"kind":"anonymous"}

### `@flighthq/storage`

- **package** `upstream/packages/storage/src`: Generated crate is missing 39 of 39 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/storage/src/storage.ts`: setStorageJSON: JSON.stringify requires a portable scalar or structural array

### `@flighthq/text`

- **package** `upstream/packages/text/src`: Generated crate is missing 16 of 86 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/text/src/nativeText.ts`: patchNativeTextStyle: multiple object spreads require ordered Rust lowering

### `@flighthq/text-markup`

- **package** `upstream/packages/text-markup/src`: Generated crate is missing 2 of 8 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/text-markup/src/textMarkup.ts`: handleMarkupToken: multiple object spreads require ordered Rust lowering

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

### `@flighthq/updater`

- **package** `upstream/packages/updater/src`: Generated crate is missing 23 of 23 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/updater/src/updater.ts`: attachAppUpdater: object literal requires an inferred structural type (target={"kind":"primitive","name":"Float"}, properties=spread,phase)

### `@flighthq/video`

- **package** `upstream/packages/video/src`: Generated crate is missing 4 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/video/src/videoResourceFrom.ts`: loadVideoResourceFromUrl: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/webcam`

- **package** `upstream/packages/webcam/src`: Generated crate is missing 10 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/webcam/src/webcam.ts`: createWebWebcamBackend: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/webcam/src/webcamStream.ts`: WebcamStreamRuntime: aggregate native entity runtime closure is unavailable: imported EntityRuntime aggregate cannot acquire package-local storage fields: WebcamStreamRuntime.binding, WebcamStreamRuntime.mediaStream, WebcamStreamRuntime.videoElement

## Candidate compile blockers

### `@flighthq/accessibility`

- **E0608** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: cannot index into a value of type `(std::string::String, OpaqueHostValue)`
- **E0608** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: cannot index into a value of type `(std::string::String, OpaqueHostValue)`
- **E0308** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: no method named `is_some` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-accessibility/src/accessibility.rs`: no method named `is_some` found for enum `OpaqueHostValue` in the current scope

### `@flighthq/animation`

- **E0308** `generated/candidates/flighthq-animation/src/animation_clip.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-animation/src/animation_player.rs`: mismatched types
- **E0369** `generated/candidates/flighthq-animation/src/animation_player.rs`: cannot subtract `f64` from `Option<f64>`
- **E0308** `generated/candidates/flighthq-animation/src/animation_track.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-animation/src/animation_track.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0308** `generated/candidates/flighthq-animation/src/animation_track.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-animation/src/animation_track.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-animation/src/animation_track.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-animation/src/animation_track.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-animation/src/animation_track.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0308** `generated/candidates/flighthq-animation/src/animation_track.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-animation/src/animation_track.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`

### `@flighthq/app`

- **E0609** `generated/candidates/flighthq-app/src/app.rs`: no field `then` on type `()`
- **E0308** `generated/candidates/flighthq-app/src/app.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-app/src/app.rs`: the trait bound `std::string::String: Pattern` is not satisfied
- **E0609** `generated/candidates/flighthq-app/src/app.rs`: no field `slice` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-app/src/app.rs`: no field `length` on type `std::string::String`

### `@flighthq/assets`

- **E0425** `generated/candidates/flighthq-assets/src/asset_library.rs`: cannot find value `runtime` in this scope
- **E0308** `generated/candidates/flighthq-assets/src/asset_library.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-assets/src/asset_library.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-assets/src/asset_library.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-assets/src/asset_library.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-assets/src/asset_library.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-assets/src/asset_library.rs`: cannot index into a value of type `(std::string::String, AssetEntry)`
- **E0608** `generated/candidates/flighthq-assets/src/asset_library.rs`: cannot index into a value of type `(std::string::String, AssetEntry)`
- **E0308** `generated/candidates/flighthq-assets/src/asset_library.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-assets/src/asset_library.rs`: can't compare `std::string::String` with `Option<std::string::String>`
- **E0277** `generated/candidates/flighthq-assets/src/asset_library.rs`: can't compare `std::string::String` with `Option<std::string::String>`
- **E0308** `generated/candidates/flighthq-assets/src/asset_library.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-assets/src/asset_library.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-assets/src/asset_library.rs`: mismatched types

### `@flighthq/audio`

- **E0308** `generated/candidates/flighthq-audio/src/audio_format.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-audio/src/audio_format.rs`: no field `byte_length` on type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0609** `generated/candidates/flighthq-audio/src/audio_format.rs`: no field `byte_length` on type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0609** `generated/candidates/flighthq-audio/src/audio_format.rs`: no field `byte_length` on type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-audio/src/audio_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0606** `generated/candidates/flighthq-audio/src/audio_format.rs`: casting `&FlightUnion2<Vec<u8>, Vec<u8>>` as `usize` is invalid
- **E0369** `generated/candidates/flighthq-audio/src/audio_resource.rs`: cannot multiply `OpaqueHostValue` by `f64`
- **E0308** `generated/candidates/flighthq-audio/src/audio_resource_from.rs`: mismatched types

### `@flighthq/clip`

- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: no method named `is_some` found for struct `ClipRegion` in the current scope
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: no method named `as_mut` found for struct `ClipRegion` in the current scope
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: no method named `as_mut` found for struct `ClipRegion` in the current scope
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: no method named `as_mut` found for struct `ClipRegion` in the current scope
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: no method named `as_mut` found for struct `ClipRegion` in the current scope
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: no method named `as_mut` found for struct `ClipRegion` in the current scope
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: no method named `as_mut` found for struct `ClipRegion` in the current scope
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: no method named `as_mut` found for struct `ClipRegion` in the current scope
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: no method named `as_mut` found for struct `ClipRegion` in the current scope
- **E0308** `generated/candidates/flighthq-clip/src/clip_region.rs`: mismatched types
- **E0631** `generated/candidates/flighthq-clip/src/clip_region.rs`: type mismatch in closure arguments
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: the method `collect` exists for struct `Map<Cloned<Iter<'_, Vec<Vec<f64>>>>, {closure@clip_region.rs:146:22}>`, but its trait bounds were not satisfied
- **E0308** `generated/candidates/flighthq-clip/src/clip_region.rs`: mismatched types
- **E0631** `generated/candidates/flighthq-clip/src/clip_region.rs`: type mismatch in closure arguments
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: the method `collect` exists for struct `Map<Cloned<Iter<'_, Vec<Vec<f64>>>>, {closure@clip_region.rs:184:22}>`, but its trait bounds were not satisfied
- **E0308** `generated/candidates/flighthq-clip/src/clip_region.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-clip/src/clip_region.rs`: a value of type `Vec<Vec<f64>>` cannot be built from an iterator over elements of type `OpaqueHostValue`
- **E0308** `generated/candidates/flighthq-clip/src/clip_region.rs`: mismatched types
- **E0631** `generated/candidates/flighthq-clip/src/clip_region.rs`: type mismatch in closure arguments
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: the method `collect` exists for struct `Map<Cloned<Iter<'_, Vec<Vec<f64>>>>, {closure@clip_region.rs:395:26}>`, but its trait bounds were not satisfied
- **E0308** `generated/candidates/flighthq-clip/src/clip_region.rs`: mismatched types
- **E0631** `generated/candidates/flighthq-clip/src/clip_region.rs`: type mismatch in closure arguments
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: the method `collect` exists for struct `Map<Cloned<Iter<'_, Vec<Vec<f64>>>>, {closure@clip_region.rs:405:30}>`, but its trait bounds were not satisfied
- **E0308** `generated/candidates/flighthq-clip/src/clip_region.rs`: mismatched types
- **E0631** `generated/candidates/flighthq-clip/src/clip_region.rs`: type mismatch in closure arguments
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: the method `collect` exists for struct `Map<Cloned<Iter<'_, Vec<Vec<f64>>>>, {closure@clip_region.rs:420:26}>`, but its trait bounds were not satisfied
- **E0308** `generated/candidates/flighthq-clip/src/clip_region.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-clip/src/clip_region.rs`: a value of type `Vec<Vec<f64>>` cannot be built from an iterator over elements of type `OpaqueHostValue`
- **E0308** `generated/candidates/flighthq-clip/src/clip_region.rs`: mismatched types
- **E0631** `generated/candidates/flighthq-clip/src/clip_region.rs`: type mismatch in closure arguments
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: the method `collect` exists for struct `Map<Cloned<Iter<'_, Vec<Vec<f64>>>>, {closure@clip_region.rs:751:26}>`, but its trait bounds were not satisfied
- **E0308** `generated/candidates/flighthq-clip/src/clip_region.rs`: mismatched types
- **E0631** `generated/candidates/flighthq-clip/src/clip_region.rs`: type mismatch in closure arguments
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: the method `collect` exists for struct `Map<Cloned<Iter<'_, Vec<Vec<f64>>>>, {closure@clip_region.rs:761:30}>`, but its trait bounds were not satisfied
- **E0308** `generated/candidates/flighthq-clip/src/clip_region.rs`: mismatched types
- **E0631** `generated/candidates/flighthq-clip/src/clip_region.rs`: type mismatch in closure arguments
- **E0599** `generated/candidates/flighthq-clip/src/clip_region.rs`: the method `collect` exists for struct `Map<Cloned<Iter<'_, Vec<Vec<f64>>>>, {closure@clip_region.rs:776:26}>`, but its trait bounds were not satisfied

### `@flighthq/clock`

- **E0308** `generated/candidates/flighthq-clock/src/add_clock_child.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/add_clock_child.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/dispose_clock.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/get_clock_effective_scale.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/get_clock_effective_scale.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/get_clock_parent.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/is_clock_effectively_paused.rs`: mismatched types

### `@flighthq/connectivity`

- **E0308** `generated/candidates/flighthq-connectivity/src/connectivity.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-connectivity/src/connectivity.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-connectivity/src/connectivity.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-connectivity/src/connectivity.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-connectivity/src/connectivity.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-connectivity/src/connectivity.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-connectivity/src/connectivity.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-connectivity/src/connectivity.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-connectivity/src/connectivity.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-connectivity/src/connectivity.rs`: mismatched types

### `@flighthq/effects`

- **E0425** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: cannot find type `AdvancedBlendMode` in this scope
- **E0425** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: cannot find type `AdvancedBlendMode` in this scope
- **E0425** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: cannot find type `AdvancedBlendMode` in this scope
- **E0425** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: cannot find type `AdvancedBlendMode` in this scope
- **E0425** `generated/candidates/flighthq-effects/src/composite_operator_math.rs`: cannot find type `CompositeOperator` in this scope
- **E0070** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: invalid left-hand side of assignment
- **E0070** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: invalid left-hand side of assignment
- **E0070** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: invalid left-hand side of assignment
- **E0070** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: invalid left-hand side of assignment
- **E0070** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: invalid left-hand side of assignment
- **E0070** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: invalid left-hand side of assignment
- **E0608** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0308** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-effects/src/blend_mode_math.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-effects/src/composite_operator_math.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0608** `generated/candidates/flighthq-effects/src/composite_operator_math.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f64>, Vec<f32>>`
- **E0308** `generated/candidates/flighthq-effects/src/linear_sampled_gaussian.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-effects/src/linear_sampled_gaussian.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-effects/src/linear_sampled_gaussian.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-effects/src/linear_sampled_gaussian.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: no method named `iter` found for unit type `()` in the current scope
- **E0277** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0277** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0308** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: mismatched types
- **E0070** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: invalid left-hand side of assignment
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: no method named `iter` found for unit type `()` in the current scope
- **E0277** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0070** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: invalid left-hand side of assignment
- **E0308** `generated/candidates/flighthq-effects/src/render_effect_defaults.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for unit type `()` in the current scope
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for unit type `()` in the current scope
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0277** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0070** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: invalid left-hand side of assignment
- **E0277** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0070** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: invalid left-hand side of assignment
- **E0277** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0070** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: invalid left-hand side of assignment
- **E0277** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0599** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: no method named `iter` found for reference `&RenderEffect` in the current scope
- **E0070** `generated/candidates/flighthq-effects/src/render_effect_interpolation.rs`: invalid left-hand side of assignment
- **E0308** `generated/candidates/flighthq-effects/src/custom_shader_effect.rs`: mismatched types

### `@flighthq/font`

- **E0609** `generated/candidates/flighthq-font/src/font_format.rs`: no field `byte_length` on type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-font/src/font_format.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0606** `generated/candidates/flighthq-font/src/font_format.rs`: casting `&FlightUnion2<Vec<u8>, Vec<u8>>` as `usize` is invalid
- **E0277** `generated/candidates/flighthq-font/src/font_shorthand.rs`: `Option<std::string::String>` doesn't implement `std::fmt::Display`

### `@flighthq/image`

- **E0609** `generated/candidates/flighthq-image/src/image_resource.rs`: no field `byte_length` on type `&Vec<u8>`
- **E0609** `generated/candidates/flighthq-image/src/image_resource_from.rs`: no field `width` on type `()`
- **E0609** `generated/candidates/flighthq-image/src/image_resource_from.rs`: no field `height` on type `()`
- **E0308** `generated/candidates/flighthq-image/src/image_resource_from.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-image/src/image_resource_from.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-image/src/image_resource_from.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-image/src/image_resource_from.rs`: mismatched types

### `@flighthq/materials`

- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `diffuse` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `diffuse_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `normal_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `normal_scale` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `shininess` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `specular` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `specular_map` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/classic_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `diffuse` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `diffuse_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `emissive` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `emissive_map` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/classic_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `diffuse` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `diffuse_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `normal_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `normal_scale` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `shininess` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `specular` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/classic_materials.rs`: no field `specular_map` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/classic_materials.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/color_transform.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/custom_shader_material.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-materials/src/material.rs`: no method named `iter` found for unit type `()` in the current scope
- **E0277** `generated/candidates/flighthq-materials/src/material.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0277** `generated/candidates/flighthq-materials/src/material.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0599** `generated/candidates/flighthq-materials/src/material.rs`: no method named `iter` found for unit type `()` in the current scope
- **E0277** `generated/candidates/flighthq-materials/src/material.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0599** `generated/candidates/flighthq-materials/src/material.rs`: no method named `is_some` found for enum `OpaqueHostValue` in the current scope
- **E0277** `generated/candidates/flighthq-materials/src/material.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0070** `generated/candidates/flighthq-materials/src/material.rs`: invalid left-hand side of assignment
- **E0277** `generated/candidates/flighthq-materials/src/material.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0070** `generated/candidates/flighthq-materials/src/material.rs`: invalid left-hand side of assignment
- **E0070** `generated/candidates/flighthq-materials/src/material.rs`: invalid left-hand side of assignment
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/material_presets.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `anisotropy_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `anisotropy_rotation` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `anisotropy_strength` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `standard` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `clearcoat` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `clearcoat_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `clearcoat_normal_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `clearcoat_roughness` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `clearcoat_roughness_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `standard` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `iridescence` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `iridescence_ior` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `iridescence_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `iridescence_thickness_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `iridescence_thickness_max` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `iridescence_thickness_min` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `standard` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `sheen_color` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `sheen_color_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `sheen_roughness` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `sheen_roughness_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `standard` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `specular` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `specular_color` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `specular_color_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `specular_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `standard` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `standard` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `subsurface` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `subsurface_color` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `subsurface_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `thickness` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `thickness_map` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `attenuation_color` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `attenuation_distance` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `ior` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `standard` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `thickness` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `thickness_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `transmission` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: no field `transmission_map` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/pbr_extension_materials.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0308** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `diffuse` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `diffuse_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `emissive` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `emissive_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `emissive_strength` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `glossiness` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `normal_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `normal_scale` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `occlusion_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `occlusion_strength` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `specular` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: no field `specular_glossiness_map` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: mismatched types
- **E0063** `generated/candidates/flighthq-materials/src/pbr_materials.rs`: missing fields `base_color`, `base_color_map`, `emissive` and 9 other fields in initializer of `StandardPbrMaterialProperties`
- **E0308** `generated/candidates/flighthq-materials/src/phong_to_pbr.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/phong_to_pbr.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/surface_material.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-materials/src/surface_material.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `far` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `near` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `emissive` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `emissive_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `emissive_strength` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `matcap` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `tint` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `normal_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `normal_scale` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `base_color` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `base_color_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `ramp` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `steps` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `base_color` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `base_color_map` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `base_color_video_map` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `tint` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `color` on type `SurfaceMaterial`
- **E0609** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: no field `thickness` on type `SurfaceMaterial`
- **E0308** `generated/candidates/flighthq-materials/src/unlit_materials.rs`: mismatched types

### `@flighthq/media`

- **E0369** `generated/candidates/flighthq-media/src/audio_channel.rs`: cannot subtract `f64` from `OpaqueHostValue`
- **E0369** `generated/candidates/flighthq-media/src/audio_channel.rs`: cannot multiply `OpaqueHostValue` by `f64`
- **E0609** `generated/candidates/flighthq-media/src/audio_channel.rs`: no field `gain` on type `&mut OpaqueHostValue`
- **E0609** `generated/candidates/flighthq-media/src/audio_channel.rs`: no field `playback_rate` on type `&mut OpaqueHostValue`
- **E0609** `generated/candidates/flighthq-media/src/audio_channel.rs`: no field `buffer` on type `()`
- **E0609** `generated/candidates/flighthq-media/src/audio_channel.rs`: no field `playback_rate` on type `()`
- **E0609** `generated/candidates/flighthq-media/src/audio_channel.rs`: no field `gain` on type `()`
- **E0609** `generated/candidates/flighthq-media/src/audio_channel.rs`: no field `connect` on type `()`
- **E0609** `generated/candidates/flighthq-media/src/audio_channel.rs`: no field `connect` on type `()`
- **E0609** `generated/candidates/flighthq-media/src/audio_channel.rs`: no field `onended` on type `()`
- **E0308** `generated/candidates/flighthq-media/src/audio_channel.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-media/src/audio_channel.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-media/src/audio_channel.rs`: mismatched types
- **E0369** `generated/candidates/flighthq-media/src/audio_channel.rs`: cannot subtract `f64` from `OpaqueHostValue`
- **E0609** `generated/candidates/flighthq-media/src/audio_channel.rs`: no field `start` on type `()`
- **E0609** `generated/candidates/flighthq-media/src/audio_channel.rs`: no field `catch` on type `()`
- **E0609** `generated/candidates/flighthq-media/src/audio_mixer.rs`: no field `gain` on type `()`
- **E0609** `generated/candidates/flighthq-media/src/audio_mixer.rs`: no field `connect` on type `()`
- **E0308** `generated/candidates/flighthq-media/src/audio_mixer.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-media/src/audio_mixer.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-media/src/audio_mixer.rs`: mismatched types
- **E0282** `generated/candidates/flighthq-media/src/audio_mixer.rs`: type annotations needed
- **E0609** `generated/candidates/flighthq-media/src/audio_mixer.rs`: no field `gain` on type `OpaqueHostValue`
- **E0609** `generated/candidates/flighthq-media/src/audio_mixer.rs`: no field `gain` on type `OpaqueHostValue`
- **E0308** `generated/candidates/flighthq-media/src/audio_mixer.rs`: mismatched types
- **E0615** `generated/candidates/flighthq-media/src/audio_mixer.rs`: attempted to take value of method `get` on type `Vec<(AudioBus, OpaqueHostValue)>`
- **E0615** `generated/candidates/flighthq-media/src/audio_mixer.rs`: attempted to take value of method `get` on type `Vec<(AudioBus, OpaqueHostValue)>`
- **E0369** `generated/candidates/flighthq-media/src/video_channel.rs`: cannot multiply `OpaqueHostValue` by `f64`
- **E0369** `generated/candidates/flighthq-media/src/video_channel.rs`: cannot multiply `OpaqueHostValue` by `f64`
- **E0596** `generated/candidates/flighthq-media/src/audio_mixer.rs`: cannot borrow `channel` as mutable, as it is not declared as mutable
- **E0596** `generated/candidates/flighthq-media/src/audio_mixer.rs`: cannot borrow `channel` as mutable, as it is not declared as mutable
- **E0596** `generated/candidates/flighthq-media/src/audio_mixer.rs`: cannot borrow `channel` as mutable, as it is not declared as mutable

### `@flighthq/mesh`

- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: mismatched types
- **E0605** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: non-primitive cast: `Vec<u32>` as `u32`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: mismatched types
- **E0615** `generated/candidates/flighthq-mesh/src/mesh_geometry_attributes.rs`: attempted to take value of method `starts_with` on type `std::string::String`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: arguments to this function are incorrect
- **E0282** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: type annotations needed
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: arguments to this function are incorrect
- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: a value of type `Vec<Vec<f64>>` cannot be built from an iterator over elements of type `OpaqueHostValue`
- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: a value of type `Vec<Vec<f64>>` cannot be built from an iterator over elements of type `OpaqueHostValue`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_compute.rs`: cannot multiply `u32` by `f64`
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_index.rs`: the trait bound `Vec<f64>: Extend<u32>` is not satisfied
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_index.rs`: the trait bound `Vec<f64>: Extend<u32>` is not satisfied
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_index.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_index.rs`: cannot multiply `u32` by `f64`
- **E0615** `generated/candidates/flighthq-mesh/src/mesh_geometry_layout.rs`: attempted to take value of method `starts_with` on type `std::string::String`
- **E0615** `generated/candidates/flighthq-mesh/src/mesh_geometry_layout.rs`: attempted to take value of method `starts_with` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: no field `length` on type `&FlightUnion2<Vec<f64>, FlightUnion2<Vec<u16>, Vec<u32>>>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: no field `length` on type `&FlightUnion2<Vec<f64>, FlightUnion2<Vec<u16>, Vec<u32>>>`
- **E0608** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: cannot index into a value of type `Option<FlightUnion2<Vec<f64>, FlightUnion2<Vec<u16>, Vec<u32>>>>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: no field `length` on type `&FlightUnion2<Vec<f64>, FlightUnion2<Vec<u16>, Vec<u32>>>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: no field `length` on type `&FlightUnion2<Vec<f64>, FlightUnion2<Vec<u16>, Vec<u32>>>`
- **E0608** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: cannot index into a value of type `Option<FlightUnion2<Vec<f64>, FlightUnion2<Vec<u16>, Vec<u32>>>>`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: cannot add `f64` to `u32`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: mismatched types
- **E0615** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: attempted to take value of method `starts_with` on type `std::string::String`
- **E0425** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: cannot find function `is_finite` in this scope
- **E0425** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: cannot find function `is_finite` in this scope
- **E0425** `generated/candidates/flighthq-mesh/src/mesh_geometry_operations.rs`: cannot find function `is_finite` in this scope
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: cannot multiply `f32` by `f64`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_transforms.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-mesh/src/morph_mesh_geometry.rs`: cannot index into a value of type `Option<Vec<f32>>`
- **E0608** `generated/candidates/flighthq-mesh/src/morph_mesh_geometry.rs`: cannot index into a value of type `Option<Vec<f32>>`

### `@flighthq/particles`

- **E0425** `generated/candidates/flighthq-particles/src/particle_emitter_state.rs`: cannot find value `math` in this scope
- **E0425** `generated/candidates/flighthq-particles/src/particle_objects_state.rs`: cannot find value `math` in this scope
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: no field `kind` on type `FlightUnion2<CircleCollider, FlightUnion2<PlaneCollider, ...>>`
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_collisions.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: arguments to this function are incorrect
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0608** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: cannot index into a value of type `LazyLock<std::sync::Mutex<Vec<f64>>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `kind` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `x` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `y` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `z` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `strength` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `strength` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `strength` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `z` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `x` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `y` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `strength` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `falloff` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `radius` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `z` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `x` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `y` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `strength` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `falloff` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `radius` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `axis_x` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `axis_y` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `axis_z` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `scale` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `strength` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `strength` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0609** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: no field `strength` on type `FlightUnion2<AttractorForce, FlightUnion2<DragForce, ...>>`
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-particles/src/apply_particle_forces.rs`: cannot divide `f64` by `Option<f64>`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f32>, Vec<f64>>`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f32>, Vec<f64>>`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut FlightUnion2<Vec<f32>, Vec<f64>>`
- **E0308** `generated/candidates/flighthq-particles/src/curve.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/curve.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-particles/src/curve.rs`: no field `length` on type `&Vec<f64>`
- **E0609** `generated/candidates/flighthq-particles/src/curve.rs`: no field `length` on type `&Vec<f64>`
- **E0308** `generated/candidates/flighthq-particles/src/curve.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-particles/src/curve.rs`: no field `length` on type `&Vec<f64>`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0608** `generated/candidates/flighthq-particles/src/curve.rs`: cannot index into a value of type `&mut SharedStructuralRecord3`
- **E0609** `generated/candidates/flighthq-particles/src/curve.rs`: no field `length` on type `&Vec<f64>`
- **E0277** `generated/candidates/flighthq-particles/src/particle_emitter_signals.rs`: the `?` operator can only be applied to values that implement `Try`
- **E0277** `generated/candidates/flighthq-particles/src/particle_emitter_signals.rs`: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
- **E0277** `generated/candidates/flighthq-particles/src/particle_emitter_signals.rs`: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
- **E0070** `generated/candidates/flighthq-particles/src/particle_emitter_signals.rs`: invalid left-hand side of assignment
- **E0308** `generated/candidates/flighthq-particles/src/particle_emitter_signals.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/particle_emitter_signals.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-particles/src/update_particle_objects.rs`: no field `length` on type `&Vec<f64>`
- **E0609** `generated/candidates/flighthq-particles/src/update_particle_objects.rs`: no field `length` on type `&Vec<f64>`
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0070** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: invalid left-hand side of assignment
- **E0608** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: cannot index into a value of type `&ParticleEmitterConfig`
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0425** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: cannot find function `string` in this scope
- **E0608** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: cannot index into a value of type `&ParticleEmitterConfig`
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: mismatched types
- **E0605** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: non-primitive cast: `std::string::String` as `usize`
- **E0605** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: non-primitive cast: `std::string::String` as `usize`
- **E0277** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: `ParticleEmitterConfig` doesn't implement `std::fmt::Display`
- **E0277** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: `ParticleEmitterConfig` doesn't implement `std::fmt::Display`
- **E0277** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: `ParticleEmitterConfig` doesn't implement `std::fmt::Display`
- **E0608** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: cannot index into a value of type `&ParticleEmitterConfig`
- **E0608** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: cannot index into a value of type `&ParticleEmitterConfig`
- **E0606** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: casting `&ParticleEmitterConfig` as `usize` is invalid
- **E0606** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: casting `&ParticleEmitterConfig` as `usize` is invalid
- **E0608** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: cannot index into a value of type `&ParticleEmitterConfig`
- **E0606** `generated/candidates/flighthq-particles/src/validate_particle_emitter_config.rs`: casting `&ParticleEmitterConfig` as `usize` is invalid

### `@flighthq/path-formats`

- **E0425** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot find value `number` in this scope
- **E0609** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: no field `char_code_at` on type `std::string::String`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0609** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: no field `slice` on type `std::string::String`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0308** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0308** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0308** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0308** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0308** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0308** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0308** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0308** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0277** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot add `Option<f64>` to `f64`
- **E0369** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot multiply `Option<f64>` by `f64`
- **E0308** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: mismatched types
- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0615** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: attempted to take value of method `join` on type `Vec<std::string::String>`
- **E0425** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot find function `string` in this scope
- **E0425** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: cannot find function `string` in this scope
- **E0609** `generated/candidates/flighthq-path-formats/src/svg_path_data.rs`: no field `index_of` on type `&'static str`

### `@flighthq/protocol`

- **E0308** `generated/candidates/flighthq-protocol/src/protocol.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-protocol/src/protocol.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `filter` on type `()`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `length` on type `OpaqueHostValue`
- **E0308** `generated/candidates/flighthq-protocol/src/protocol.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-protocol/src/protocol.rs`: `()` doesn't implement `std::fmt::Display`
- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-protocol/src/protocol.rs`: can't compare `OpaqueHostValue` with `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `index_of` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `slice` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `slice` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `index_of` on type `std::string::String`
- **E0070** `generated/candidates/flighthq-protocol/src/protocol.rs`: invalid left-hand side of assignment
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `slice` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `slice` on type `std::string::String`
- **E0070** `generated/candidates/flighthq-protocol/src/protocol.rs`: invalid left-hand side of assignment
- **E0308** `generated/candidates/flighthq-protocol/src/protocol.rs`: mismatched types

### `@flighthq/shell`

- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-shell/src/shell.rs`: no method named `as_ref` found for struct `LazyLock<std::sync::Mutex<Option<Vec<std::string::String>>>>` in the current scope
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-shell/src/shell.rs`: mismatched types

### `@flighthq/snapshot`

- **E0107** `generated/candidates/flighthq-snapshot/src/capture_snapshot.rs`: type alias takes 0 generic arguments but 1 generic argument was supplied
- **E0107** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: type alias takes 0 generic arguments but 1 generic argument was supplied
- **E0107** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: type alias takes 0 generic arguments but 1 generic argument was supplied
- **E0107** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: type alias takes 0 generic arguments but 1 generic argument was supplied
- **E0107** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: type alias takes 0 generic arguments but 1 generic argument was supplied
- **E0107** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: type alias takes 0 generic arguments but 1 generic argument was supplied
- **E0277** `generated/candidates/flighthq-snapshot/src/capture_snapshot.rs`: the trait bound `T: Default` is not satisfied
- **E0308** `generated/candidates/flighthq-snapshot/src/capture_snapshot.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-snapshot/src/capture_snapshot.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-snapshot/src/capture_snapshot.rs`: no method named `is_none` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/capture_snapshot.rs`: no method named `iter` found for unit type `()` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: no method named `is_none` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: no method named `is_none` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: no method named `len` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: no method named `len` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: no method named `len` found for enum `OpaqueHostValue` in the current scope
- **E0608** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: cannot index into a value of type `OpaqueHostValue`
- **E0608** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: cannot index into a value of type `OpaqueHostValue`
- **E0609** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: no field `length` on type `()`
- **E0609** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: no field `length` on type `()`
- **E0599** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: no method named `iter` found for unit type `()` in the current scope
- **E0600** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: cannot apply unary operator `!` to type `()`
- **E0277** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0277** `generated/candidates/flighthq-snapshot/src/equals_snapshot.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0599** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: no method named `is_none` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: no method named `is_none` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: no method named `is_none` found for type parameter `T` in the current scope
- **E0308** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: no method named `len` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: no method named `iter` found for unit type `()` in the current scope
- **E0277** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0277** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0277** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0308** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: mismatched types
- **E0070** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: invalid left-hand side of assignment
- **E0599** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: no method named `is_some` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: no method named `is_some` found for enum `OpaqueHostValue` in the current scope
- **E0277** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0277** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0070** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: invalid left-hand side of assignment
- **E0277** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0070** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: invalid left-hand side of assignment
- **E0609** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: no field `includes` on type `&Vec<std::string::String>`
- **E0599** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: no method named `is_some` found for enum `OpaqueHostValue` in the current scope
- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: no method named `is_none` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: no method named `is_none` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: no method named `is_none` found for type parameter `T` in the current scope
- **E0308** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: no method named `truncate` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: no method named `len` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: no method named `len` found for enum `OpaqueHostValue` in the current scope
- **E0608** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: cannot index into a value of type `OpaqueHostValue`
- **E0608** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: cannot index into a value of type `OpaqueHostValue`
- **E0599** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: no method named `len` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: no method named `push` found for enum `OpaqueHostValue` in the current scope
- **E0608** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: cannot index into a value of type `OpaqueHostValue`
- **E0599** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: no method named `iter` found for unit type `()` in the current scope
- **E0277** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0277** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0277** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0070** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: invalid left-hand side of assignment
- **E0599** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: no method named `is_none` found for enum `OpaqueHostValue` in the current scope
- **E0599** `generated/candidates/flighthq-snapshot/src/restore_snapshot.rs`: no method named `is_some` found for enum `OpaqueHostValue` in the current scope

### `@flighthq/socket`

- **E0599** `generated/candidates/flighthq-socket/src/socket.rs`: no method named `unwrap` found for reference `&Mutex<Box<dyn FnMut(Option<f64>, Option<String>) + Send>>` in the current scope
- **E0308** `generated/candidates/flighthq-socket/src/socket.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-socket/src/socket.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-socket/src/socket.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-socket/src/socket.rs`: mismatched types

### `@flighthq/textbidi`

- **E0369** `generated/candidates/flighthq-textbidi/src/bidi_class_backend.rs`: cannot subtract `f64` from `LazyLock<f64>`
- **E0609** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: no field `code_point_at` on type `std::string::String`
- **E0308** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: mismatched types
- **E0600** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: cannot apply unary operator `!` to type `f64`
- **E0382** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: use of moved value: `sos`
- **E0382** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: use of moved value: `sos`
- **E0382** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: borrow of moved value: `sos`
- **E0382** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: borrow of moved value: `eos`
- **E0382** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: use of moved value: `embedding_dir`
- **E0382** `generated/candidates/flighthq-textbidi/src/resolve_bidi_levels.rs`: use of moved value: `resolved`

### `@flighthq/timeline`

- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be applied to values that implement `Try`
- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
- **E0070** `generated/candidates/flighthq-timeline/src/timeline.rs`: invalid left-hand side of assignment
- **E0609** `generated/candidates/flighthq-timeline/src/timeline.rs`: no field `set` on type `Option<Vec<(f64, Arc<Mutex<Box<dyn FnMut(..., f64) + Send>>>)>>`
- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be applied to values that implement `Try`
- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
- **E0070** `generated/candidates/flighthq-timeline/src/timeline.rs`: invalid left-hand side of assignment
- **E0308** `generated/candidates/flighthq-timeline/src/timeline.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-timeline/src/timeline.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-timeline/src/timeline.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-timeline/src/timeline.rs`: no method named `unwrap` found for reference `&std::sync::Mutex<Box<(dyn FnMut(DisplayObject, f64) + Send + 'static)>>` in the current scope
- **E0308** `generated/candidates/flighthq-timeline/src/timeline.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-timeline/src/timeline.rs`: mismatched types
- **E0596** `generated/candidates/flighthq-timeline/src/timeline.rs`: cannot borrow `timeline.frame_scripts` as mutable, as it is behind a `&` reference

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

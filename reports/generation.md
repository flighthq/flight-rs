# Automatic Rust Generation

Upstream commit: `5d24729f7360475e28a105ae0caeeaa2e1328260`

| Metric | Count |
| --- | ---: |
| Inventoried packages | 131 |
| Default-generated packages | 125 |
| Emittable packages | 61 |
| Blocked packages | 64 |
| Compiled candidates | 27 |
| Compile-blocked candidates | 17 |
| Dependency-blocked candidates | 15 |
| Cultivated packages | 1 |
| Host-bound packages | 4 |
| Excluded packages | 1 |
| Source/package blockers | 355 |
| Eligible async scopes | 162 |
| Portable executable async scopes | 0 |
| Host-placeholder async scopes | 0 |
| Unsupported async scopes | 162 |
| Async scopes matching the legacy body-erasure path | 76 |
| Upstream conformance files translated and passing | 4/1166 |
| Generated conformance cases passing | 45/45 |

| Package | Disposition | Status | Candidate | Sources emitted/attempted | API generated/expected | Missing | Dependents direct/transitive | Opaque sources | Blockers | Target |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `@flighthq/accessibility` | generated | emittable | compile-blocked | 2/2 | 8/8 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/adjustments` | generated | emittable | compiled | 19/19 | 49/49 | 0 | 6/25 | 0 | 0 | no |
| `@flighthq/animation` | generated | emittable | compile-blocked | 4/4 | 18/18 | 0 | 3/7 | 1 | 0 | no |
| `@flighthq/app` | generated | emittable | compiled | 2/2 | 42/42 | 0 | 4/4 | 0 | 0 | no |
| `@flighthq/application` | generated | emittable | compiled | 3/3 | 83/83 | 0 | 3/3 | 2 | 0 | no |
| `@flighthq/assets` | generated | blocked | source-blocked | 1/2 | 0/10 | 10 | 1/1 | 0 | 2 | no |
| `@flighthq/audio` | generated | blocked | source-blocked | 3/4 | 13/20 | 7 | 2/2 | 1 | 2 | no |
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
| `@flighthq/collision` | generated | emittable | compile-blocked | 6/6 | 19/19 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/color` | generated | emittable | compiled | 10/10 | 32/32 | 0 | 10/29 | 0 | 0 | no |
| `@flighthq/connectivity` | generated | blocked | source-blocked | 1/2 | 0/14 | 14 | 2/2 | 0 | 2 | no |
| `@flighthq/debug` | generated | blocked | source-blocked | 2/3 | 4/9 | 5 | 1/1 | 1 | 2 | no |
| `@flighthq/device` | generated | emittable | compiled | 2/2 | 14/14 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/dialog` | generated | blocked | source-blocked | 1/2 | 0/15 | 15 | 5/5 | 0 | 2 | no |
| `@flighthq/displayobject` | generated | blocked | source-blocked | 1/8 | 46/46 | 0 | 14/24 | 0 | 7 | no |
| `@flighthq/displayobject-canvas` | host-backend | blocked | source-blocked | 19/31 | 59/94 | 35 | 5/5 | 17 | 13 | no |
| `@flighthq/displayobject-dom` | host-bound | host-bound | not-applicable | 0/0 | 0/58 | 58 | 1/1 | 0 | 0 | no |
| `@flighthq/displayobject-gl` | host-backend | blocked | source-blocked | 27/28 | 83/89 | 8 | 1/1 | 18 | 2 | no |
| `@flighthq/displayobject-wgpu` | host-backend | blocked | source-blocked | 28/29 | 88/95 | 8 | 1/1 | 19 | 2 | no |
| `@flighthq/easing` | generated | emittable | promoted | 20/20 | 48/48 | 0 | 2/3 | 0 | 0 | full |
| `@flighthq/effects` | generated | emittable | compile-blocked | 72/72 | 112/112 | 0 | 4/4 | 2 | 0 | no |
| `@flighthq/effects-canvas` | host-backend | blocked | source-blocked | 9/48 | 29/102 | 78 | 1/1 | 7 | 40 | no |
| `@flighthq/effects-gl` | host-backend | blocked | source-blocked | 11/58 | 31/135 | 104 | 1/1 | 8 | 48 | no |
| `@flighthq/effects-wgpu` | host-backend | blocked | source-blocked | 12/56 | 37/128 | 91 | 1/1 | 4 | 45 | no |
| `@flighthq/entity` | generated | emittable | compiled | 6/6 | 12/12 | 0 | 20/61 | 1 | 0 | no |
| `@flighthq/filesystem` | generated | blocked | source-blocked | 1/2 | 0/43 | 43 | 2/2 | 0 | 2 | no |
| `@flighthq/flow` | generated | emittable | compiled | 10/10 | 9/9 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/font` | generated | blocked | source-blocked | 5/8 | 5/15 | 10 | 1/1 | 0 | 4 | no |
| `@flighthq/geolocation` | generated | blocked | source-blocked | 1/2 | 0/12 | 12 | 2/2 | 0 | 2 | no |
| `@flighthq/geometry` | generated | emittable | compiled | 27/27 | 377/377 | 0 | 40/53 | 0 | 0 | no |
| `@flighthq/glyphatlas` | generated | emittable | dependency-blocked | 7/7 | 14/14 | 0 | 1/1 | 2 | 0 | no |
| `@flighthq/haptics` | generated | emittable | compiled | 2/2 | 13/13 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/host-capacitor` | host-bound | host-bound | not-applicable | 0/0 | 0/63 | 63 | 0/0 | 0 | 0 | no |
| `@flighthq/host-electron` | host-bound | host-bound | not-applicable | 0/0 | 0/57 | 57 | 0/0 | 0 | 0 | no |
| `@flighthq/host-tauri` | host-bound | host-bound | not-applicable | 0/0 | 0/51 | 51 | 0/0 | 0 | 0 | no |
| `@flighthq/image` | generated | blocked | source-blocked | 2/3 | 11/20 | 9 | 11/24 | 1 | 2 | partial |
| `@flighthq/image-codec` | generated | blocked | source-blocked | 4/8 | 11/16 | 5 | 3/26 | 0 | 5 | no |
| `@flighthq/input` | generated | emittable | compiled | 2/2 | 40/40 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/interaction` | generated | blocked | source-blocked | 1/16 | 71/83 | 12 | 1/1 | 0 | 16 | no |
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
| `@flighthq/node` | generated | blocked | source-blocked | 6/16 | 105/105 | 0 | 23/32 | 0 | 10 | no |
| `@flighthq/notification` | generated | blocked | source-blocked | 1/2 | 0/26 | 26 | 4/4 | 0 | 2 | no |
| `@flighthq/particleemitter` | generated | emittable | dependency-blocked | 11/11 | 51/51 | 0 | 1/1 | 10 | 0 | no |
| `@flighthq/particles` | generated | emittable | compile-blocked | 11/11 | 50/50 | 0 | 3/3 | 1 | 0 | no |
| `@flighthq/particles-formats` | generated | blocked | source-blocked | 9/21 | 62/79 | 17 | 1/1 | 0 | 13 | no |
| `@flighthq/path` | generated | emittable | compiled | 23/23 | 50/50 | 0 | 8/8 | 0 | 0 | no |
| `@flighthq/path-boolean` | generated | blocked | source-blocked | 7/8 | 12/12 | 1 | 1/1 | 0 | 2 | no |
| `@flighthq/path-formats` | generated | emittable | compile-blocked | 2/2 | 3/3 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/permissions` | generated | blocked | source-blocked | 1/2 | 0/5 | 5 | 1/1 | 0 | 2 | no |
| `@flighthq/picking` | generated | blocked | source-blocked | 1/2 | 6/6 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/platform` | generated | emittable | compiled | 2/2 | 16/16 | 0 | 3/3 | 1 | 0 | no |
| `@flighthq/power` | generated | emittable | compiled | 2/2 | 19/19 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/protocol` | generated | emittable | compile-blocked | 2/2 | 20/20 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/render` | generated | blocked | source-blocked | 4/17 | 63/63 | 0 | 9/13 | 0 | 13 | no |
| `@flighthq/render-gl` | host-backend | blocked | source-blocked | 23/24 | 79/75 | 5 | 4/4 | 16 | 2 | no |
| `@flighthq/render-wgpu` | host-backend | blocked | source-blocked | 15/18 | 58/68 | 10 | 5/5 | 12 | 4 | no |
| `@flighthq/scene` | generated | blocked | source-blocked | 2/14 | 43/43 | 0 | 6/6 | 0 | 12 | no |
| `@flighthq/scene-formats` | generated | blocked | source-blocked | 7/16 | 84/15 | 8 | 2/2 | 0 | 10 | no |
| `@flighthq/scene-gl` | host-backend | emittable | dependency-blocked | 53/53 | 187/184 | 0 | 1/1 | 26 | 0 | no |
| `@flighthq/scene-resources` | generated | blocked | source-blocked | 7/16 | 22/37 | 15 | 1/1 | 6 | 10 | no |
| `@flighthq/scene-wgpu` | host-backend | emittable | dependency-blocked | 42/42 | 142/140 | 0 | 1/1 | 30 | 0 | no |
| `@flighthq/screen` | generated | blocked | source-blocked | 1/2 | 0/31 | 31 | 2/2 | 0 | 2 | partial |
| `@flighthq/sdk` | generated | blocked | source-blocked | 14/14 | 0/5923 | 5923 | 0/0 | 0 | 1 | no |
| `@flighthq/sensors` | generated | blocked | source-blocked | 1/2 | 0/32 | 32 | 1/1 | 0 | 2 | no |
| `@flighthq/shading` | generated | blocked | source-blocked | 14/17 | 37/37 | 0 | 2/2 | 0 | 3 | no |
| `@flighthq/shape` | generated | blocked | source-blocked | 6/7 | 32/42 | 10 | 7/8 | 2 | 2 | no |
| `@flighthq/shape-formats` | generated | blocked | source-blocked | 1/2 | 0/5 | 5 | 1/1 | 0 | 2 | no |
| `@flighthq/share` | generated | blocked | source-blocked | 1/2 | 0/14 | 14 | 2/2 | 0 | 2 | no |
| `@flighthq/shell` | generated | blocked | source-blocked | 1/2 | 0/14 | 14 | 3/3 | 0 | 2 | no |
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
| `@flighthq/textshaper` | generated | blocked | source-blocked | 8/9 | 30/31 | 3 | 3/12 | 1 | 2 | no |
| `@flighthq/textshaper-canvas` | host-backend | emittable | dependency-blocked | 2/2 | 3/3 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/texture` | generated | emittable | compiled | 5/5 | 42/42 | 0 | 5/6 | 0 | 0 | no |
| `@flighthq/texture-formats` | generated | blocked | source-blocked | 8/9 | 8/6 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/textureatlas` | generated | blocked | source-blocked | 3/4 | 12/20 | 8 | 8/13 | 0 | 2 | no |
| `@flighthq/textureatlas-formats` | generated | emittable | dependency-blocked | 8/8 | 29/29 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/tilemap-formats` | generated | blocked | source-blocked | 5/9 | 16/16 | 0 | 1/1 | 0 | 4 | no |
| `@flighthq/tileset` | generated | blocked | source-blocked | 2/3 | 3/9 | 6 | 3/8 | 0 | 2 | no |
| `@flighthq/timeline` | generated | emittable | compile-blocked | 2/2 | 16/16 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/tool-capture` | excluded | excluded | not-applicable | 0/0 | 0/57 | 57 | 0/0 | 0 | 0 | no |
| `@flighthq/tray` | generated | blocked | source-blocked | 1/2 | 23/23 | 0 | 3/3 | 0 | 1 | no |
| `@flighthq/tween` | generated | blocked | source-blocked | 6/9 | 36/35 | 0 | 2/2 | 0 | 3 | no |
| `@flighthq/types` | generated | emittable | promoted | 590/590 | 1261/1261 | 0 | 129/129 | 76 | 0 | full |
| `@flighthq/updater` | generated | blocked | source-blocked | 1/2 | 23/23 | 0 | 2/2 | 0 | 1 | no |
| `@flighthq/useragent` | generated | emittable | compiled | 3/3 | 12/12 | 0 | 3/6 | 1 | 0 | no |
| `@flighthq/velocity` | generated | emittable | dependency-blocked | 4/4 | 20/20 | 0 | 3/3 | 2 | 0 | no |
| `@flighthq/video` | generated | blocked | source-blocked | 3/4 | 12/16 | 4 | 2/2 | 1 | 2 | no |
| `@flighthq/webcam` | generated | blocked | source-blocked | 1/3 | 0/10 | 10 | 1/1 | 0 | 3 | no |
| `@flighthq/xml` | generated | emittable | compile-blocked | 3/3 | 7/7 | 0 | 5/5 | 2 | 0 | no |

## Async tasks

Disposition partition: 162 eligible = 0 portable executable + 0 host placeholder + 162 unsupported.

| Operation | Count |
| --- | ---: |
| Await expressions | 190 |
| Async iterations | 3 |
| Promise.all | 0 |
| Promise.allSettled | 1 |
| Promise.resolve | 0 |
| Promise.reject | 0 |
| Promise.then | 0 |
| Promise.catch | 0 |
| Promise.finally | 0 |
| Void expressions | 5 |

| Package | Eligible | Portable executable | Host placeholder | Unsupported | Legacy erasure path |
| --- | ---: | ---: | ---: | ---: | ---: |
| `@flighthq/assets` | 1 | 0 | 0 | 1 | 1 |
| `@flighthq/audio` | 5 | 0 | 0 | 5 | 5 |
| `@flighthq/clipboard` | 22 | 0 | 0 | 22 | 1 |
| `@flighthq/connectivity` | 2 | 0 | 0 | 2 | 1 |
| `@flighthq/dialog` | 7 | 0 | 0 | 7 | 3 |
| `@flighthq/filesystem` | 38 | 0 | 0 | 38 | 12 |
| `@flighthq/font` | 9 | 0 | 0 | 9 | 9 |
| `@flighthq/geolocation` | 2 | 0 | 0 | 2 | 0 |
| `@flighthq/image` | 4 | 0 | 0 | 4 | 4 |
| `@flighthq/image-codec` | 5 | 0 | 0 | 5 | 3 |
| `@flighthq/loader` | 2 | 0 | 0 | 2 | 2 |
| `@flighthq/net` | 3 | 0 | 0 | 3 | 2 |
| `@flighthq/notification` | 16 | 0 | 0 | 16 | 0 |
| `@flighthq/permissions` | 5 | 0 | 0 | 5 | 5 |
| `@flighthq/render-wgpu` | 3 | 0 | 0 | 3 | 3 |
| `@flighthq/scene-resources` | 12 | 0 | 0 | 12 | 11 |
| `@flighthq/screen` | 2 | 0 | 0 | 2 | 2 |
| `@flighthq/sensors` | 2 | 0 | 0 | 2 | 1 |
| `@flighthq/share` | 3 | 0 | 0 | 3 | 1 |
| `@flighthq/shell` | 8 | 0 | 0 | 8 | 0 |
| `@flighthq/storage` | 1 | 0 | 0 | 1 | 1 |
| `@flighthq/textureatlas` | 4 | 0 | 0 | 4 | 4 |
| `@flighthq/tileset` | 4 | 0 | 0 | 4 | 4 |
| `@flighthq/video` | 1 | 0 | 0 | 1 | 1 |
| `@flighthq/webcam` | 1 | 0 | 0 | 1 | 0 |

### Unsupported async scopes

- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:103:1` `loadAssetGroup` (sha256:f3bdc028a58d1524f00b1b963fb051844bec376d8010a26492a62cbd6dacea3b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:24:1` `loadAudioResourceFromBase64` (sha256:f62565721d1676f6b065becc6e2abdd9e41b9d327d26c08664159eaf20e4e9fc): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:36:1` `loadAudioResourceFromBlob` (sha256:6f31ecce9820d5f8d705caee039a2579bafeb3dca51a82e47de6c043e688642e): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:48:1` `loadAudioResourceFromBytes` (sha256:d54dd5fbcc30d298fda7b249601018a99e88cb2048c29e17efd9bea7eb5110d5): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:60:1` `loadAudioResourceFromUrl` (sha256:0d18bb89086b9b42f5252d194365bd5f6bfd83020372e1b87f43a80ed4ce8cc4): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:75:1` `loadAudioResourceFromUrls` (sha256:f704e57788fe00ece2e8f7e0e8f89db8a63e3a17ca93c092fb6d1adf17df4c64): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:31:5` `createWebClipboardBackend.readFormat` (sha256:721317f182624551290a66cbbc88a82fb2c213c31c84039b3fbf7947bf397856): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:48:5` `createWebClipboardBackend.writeFormat` (sha256:06ec8c11ce5ed20c71af72322b5fe3d22c25749c4ee27b4fcffade76b202ef87): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:59:5` `createWebClipboardBackend.hasFormat` (sha256:0fe5c7789f000064e04648dbb2831f298a8726cae24bda2ef43f63b780a7eea6): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:63:5` `createWebClipboardBackend.getFormats` (sha256:ed75f7ccdf54735fec684a6a6ee69c47e91c5a0d3591169be8970cc8ad027266): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:79:5` `createWebClipboardBackend.writeItems` (sha256:bfe2ca1744d74e460bbb99f43395a8ad2eec6a0ed51212254ec322e9c93392e1): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:93:5` `createWebClipboardBackend.readItems` (sha256:04db96d1813aada66ce1c55a363f56cbd16734202877dcbaf231786cfe34c32b): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:112:5` `createWebClipboardBackend.readText` (sha256:1a5b9a58c3f16317df3446c1a66ef5114db31b5f6ee12f004359ae970ddb5f3e): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:121:5` `createWebClipboardBackend.writeText` (sha256:23eab1975987cd5de1f5dd408ae4fb2db6a4451dfebdc78c4ff5dcbf971da580): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:131:5` `createWebClipboardBackend.readHtml` (sha256:1a6c1714ae6b501d2713b397c6d79dc8049a767e5744a291718c2af37568fc6b): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:134:5` `createWebClipboardBackend.writeHtml` (sha256:5eb7aee33fabdabdf097fc74dbd79968d9b293cb97e42af1d1ea83d6c9a25060): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:137:5` `createWebClipboardBackend.hasText` (sha256:1d5b1f5d4457bd324170aa1e3866a3787d5c0b8719f36e18b9f20efb61c5411b): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:140:5` `createWebClipboardBackend.readImage` (sha256:51f9ad208bf3a315668eff8f6223c0c11e1b28431226790533ba02ed5d042d36): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:157:5` `createWebClipboardBackend.writeImage` (sha256:0d28b5751be206dc8174fd8d29ee8931217607ccc2bb598c881854d5574a5714): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:169:5` `createWebClipboardBackend.hasImage` (sha256:2a2c1863a814225bd6ae7583d0f099723f904138492d03cedea7727e7dc7e70e): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:172:5` `createWebClipboardBackend.readRTF` (sha256:a0f27e9f28118b12321c83624453b98f87dad2cfef71f630bb6c5e0c4131a165): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:175:5` `createWebClipboardBackend.writeRTF` (sha256:0e54cef02fdd6872e0a775fd27e741fc1bf97a54e7a9c7d8755a621a78e53a75): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:179:5` `createWebClipboardBackend.readBookmark` (sha256:78016ab7f76a0108d52c3f3ccee3c40d7161b80bb66cc8860faec4d5cc99f115): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:183:5` `createWebClipboardBackend.writeBookmark` (sha256:8cc982a2d45bee016f5baa8b6fa524db87eb0302c67f1a30e1fd4bbfc3ef66f4): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:187:5` `createWebClipboardBackend.readFiles` (sha256:540866183a941bfea5b304a70d7bff5ac1c2aa80da47e31cf2e894c29cde0ce6): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:191:5` `createWebClipboardBackend.writeFiles` (sha256:ff06a60769d4aece46e2761c73660884fed44831ad41a0b3903a1ba9f0b3c85d): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:194:5` `createWebClipboardBackend.clear` (sha256:34f107c7453a7d6158d4388d4f5c200844605acd824e1bc06de7f332dad9ee7c): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:368:1` `blobFromFormatData` (sha256:c1fb9e0ee718a360876a579fe9e1e5fdc16157f03db3dabc983b233c321466dd): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/connectivity` `upstream/packages/connectivity/src/connectivity.ts:82:5` `createWebConnectivityBackend.detectReachability` (sha256:694076bc4efb1a4cfa0e975d140e8cf6f2435a0fdd257300be3a958e3a2b5015): Portable task Rust lowering is not implemented.
- `@flighthq/connectivity` `upstream/packages/connectivity/src/connectivity.ts:138:1` `detectConnectivityReachability` (sha256:599d1031e281368c595d7c67f9094c5325c5caf880d596a64a3c5d592178e52b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:19:5` `createWebDialogBackend.confirm` (sha256:9e0d435b7b04504a82aec0618dbb15c78815dffab42978a68f29044f5e6fcf78): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:28:5` `createWebDialogBackend.message` (sha256:7c2e4513e6b2c9853b0c5bbf6a3f56fc25557127cde7c98c54a42e7b3820a77d): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:48:5` `createWebDialogBackend.prompt` (sha256:c2ef3f3e409d36b6f50d673164d87f3da10c9811b4725fa58da7f7886e5b23b3): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:56:5` `createWebDialogBackend.saveFile` (sha256:40414e87f9a0df2dbf9669cc34214ffee390112a4912d6d8a9648f7be2102d80): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:246:1` `openDirectoryPickerAccessApi` (sha256:b899e4b9db2555d6330dd9e3cdf86a011e79a2f0355b05f846f7c7204dbae732): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:319:1` `openFileSystemAccessPicker` (sha256:0a2092597843117736235fd619c2d4f395e5ff517d455c075d5948fee291c1cc): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:349:1` `saveWebFile` (sha256:e88949abb7c69f2698087c43c21543a34eaf2427cac71f67fccf397a0a8a7156): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:41:5` `createWebFileSystemBackend.readTextFile` (sha256:574171c1f78071a47b51fc4530a31a20df6cd2cef77438662dc248915ce18ff2): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:51:5` `createWebFileSystemBackend.writeTextFile` (sha256:eea5320063d5585ee7e7ec9354a30cd9217902159a9cfb617cfa9a2f088e6d9f): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:54:5` `createWebFileSystemBackend.readBinaryFile` (sha256:f8849adeb4063a8e366324baba2120d2e6789cec1ef89017cd25d86fe4063b0f): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:64:5` `createWebFileSystemBackend.readBinaryFileRange` (sha256:51679cf17403994237bec53b329aa9c371966174d4596140e957ed5656df9679): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:76:5` `createWebFileSystemBackend.writeBinaryFile` (sha256:a008f81798eccc1003aed0d9e903d42a763a22ea9cd9ea75ec66fef4e8c70011): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:81:5` `createWebFileSystemBackend.fileExists` (sha256:16edd4e2eeb397246605858dc6cc531b8e7375a6f7016a31bea39fdb86ed6f67): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:84:5` `createWebFileSystemBackend.directoryExists` (sha256:f46b834d739fc1ccecafc9af12e6d5f2ff98579406b192a7245d142e385fc659): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:89:5` `createWebFileSystemBackend.removeFile` (sha256:140b01563d3776aae563c30f2d6760391ae4a5e2a21c66732e9debd4e04740ac): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:92:5` `createWebFileSystemBackend.removeDirectory` (sha256:fdc2708c099a643b3d6cefa1ca4f560bcd3ca77f73e5351cb920d88a5f3889fb): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:107:5` `createWebFileSystemBackend.makeDirectory` (sha256:6f51bdc0b4150ea205900967a9b753947a8d07e2d2d5fe4cf4704de8ed954374): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:113:5` `createWebFileSystemBackend.readDirectory` (sha256:cf39347f679c2986d33af6374395320703ca8ec00f38d53648d13ef59b366b2a): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:130:5` `createWebFileSystemBackend.readDirectoryRecursive` (sha256:b9273fc96e399400fa069fdda02be9cdddc16bf6435aed16513e46684d4fcbd9): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:144:5` `createWebFileSystemBackend.statFile` (sha256:9f3ca243190b7dd20090bc636e833fcc518d4d27da3a62f50d9b0bf56343987c): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:167:5` `createWebFileSystemBackend.rename` (sha256:9e3228d2cb150076642cde946d5c8e0af12df36ddafd11d6f5d3a888fae1db64): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:172:5` `createWebFileSystemBackend.copy` (sha256:a8ab38a19dcb5e3fb20e8102c6231c614cbd4b237b975dc68a3094654c1c3ea8): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:185:5` `createWebFileSystemBackend.appendTextFile` (sha256:6b213fd0d23492209d078e56f40ac8ad6bb25bf23fdf0fcb61837aa18e7e5828): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:197:5` `createWebFileSystemBackend.openFileReadStream` (sha256:397d36c122ccf42160fd8c8d0b251d4e415f2caa9dc6a8e6d752f9d589e3b93a): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:207:5` `createWebFileSystemBackend.openFileWriteStream` (sha256:a1398c0c934e0b064752d3e88b8faf3fcceeb22c332ef48a527eebbcfe690500): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:216:5` `createWebFileSystemBackend.writeFileAtomic` (sha256:08338d01a7496bfb863bafb34ad143731f1825c05a980b1206a57d98ea8cb7a7): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:238:5` `createWebFileSystemBackend.createFileSymlink` (sha256:a7ec0965827a98018c26d6d389d77b43f2207b530026a721030f9711ce2fd2ba): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:242:5` `createWebFileSystemBackend.readFileSymlink` (sha256:f9aa63b08e2eb58e164feec37dc2c5e405773c1bfa1bc83cabe221db963a7c3e): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:246:5` `createWebFileSystemBackend.getFileRealPath` (sha256:c7c21015d70a979da0d7a21948807c11791069f7a6d31e5cedec80054c726aac): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:250:5` `createWebFileSystemBackend.getFilePermissions` (sha256:d5a528fd94e5ebdc23f72af0bda23fa3a3ff8f759a4785bf758cd588aae07224): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:254:5` `createWebFileSystemBackend.setFilePermissions` (sha256:a896a24d74069b5626a25ac32214db778d30f118359a34a1ccc8449cb63cf180): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:258:5` `createWebFileSystemBackend.canAccessFile` (sha256:044040117b7c5b060026f24b144f5a0799780f74f04d04c05e0fa674b7aea53c): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:278:5` `createWebFileSystemBackend.getFileSystemUsage` (sha256:415ecf42bf14245ab608395d0bc204940d14442436e0f4a855c24c92e1669a52): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:318:1` `findFiles` (sha256:086ee8f33aa2fc15cd2242768128432c33d546e7e1e07f6f180f639c3e741a06): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:437:1` `readDialogHandleBinaryFile` (sha256:168933b0fd5d152de69e33d8106ef967a6b88f971892b802c6682be962f5a682): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:459:1` `readDialogHandleTextFile` (sha256:ef1c5426a4503d00e890c0f0fa60cbb140d257532ba380cb12fc5086ed838cbd): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:548:1` `writeBinaryFileChunks` (sha256:3edaf0a0922e0acc7f59a89e6ff05bd4c376e657535b29c66f6e6b1607afc8df): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:571:1` `writeDialogHandleBinaryFile` (sha256:80e53fef3f1f97ef52a3ea87e4d071bda65a76fe2579d937716cab0aa01db411): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:594:1` `writeDialogHandleTextFile` (sha256:9080a9844f1f96cb71579d9cd83e47aff5e9c4ea4e145b7aed073f0411e8a076): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:625:1` `getWebRoot` (sha256:ef7f3d7015038a12b48081190128011dea066ed8d640e163d354dccba07a461f): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:636:1` `getWebDirectoryHandle` (sha256:7f0c9697a8fda8bf983894519de724aca66a1a9c03e449ffe09ddd4ae9c63f92): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:652:1` `getWebFileHandle` (sha256:81a545005c91ec665cdc44f2beb861a1922b104d011fcedd756fb8897428ae65): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:668:1` `walkWebDirectory` (sha256:6d1216dc9c5354ba256755b0451e1d3789557508dda88892e626011b93807eb4): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:687:1` `writeWebRemove` (sha256:1841fd4e7778e7dd8472360d0552f739a00f488b0961c21213eab5deafe197cf): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:707:1` `writeWebFile` (sha256:6e15f5efb9909a3d0e7c991013ac6cdfd764a902ed4e16f226efb7f32676e172): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/fontFrom.ts:7:1` `loadFontFromBytes` (sha256:f9c00f5f77010ba5fa7520c54e0f11bbd8b80a062e9cec2c0fd72b2d4df3c58f): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/fontFrom.ts:17:1` `loadFontFromName` (sha256:8c363bc40b1a4857571ae0a41e34757a523d789db59a76da91c10bcf425b9674): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/fontFrom.ts:22:1` `loadFontFromUrl` (sha256:53cbfccb56798f4aadb7562e6b83fa1f92bf7eccc05b8be04540142731cab6c6): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/fontFrom.ts:29:1` `loadFontFromUrls` (sha256:387f9e908381971d8780b4b4922a60ec9720966855b03befa295b3ab5430401f): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/fontResourceFrom.ts:6:1` `loadFontResourceFromBytes` (sha256:5bab0dfa03b6499764dc9fffab4e003cc96bbee7fac9e310d28a1f1198443d0f): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/fontResourceFrom.ts:17:1` `loadFontResourceFromName` (sha256:ca1a52299ca0e5197e44590ecce19f51dfdafb360c5d0d56ab601ed117e6484d): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/fontResourceFrom.ts:23:1` `loadFontResourceFromUrl` (sha256:442dc5b4effdd4fe6a53955dbd6fe687453982e638e8cc3e0e2c50ca770461dd): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/fontResourceFrom.ts:31:1` `loadFontResourceFromUrls` (sha256:448720b08044d844f77ccca5c73c1d49ac59b3d2d41d1fa4f0c958e27420b526): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/fontStatus.ts:7:1` `whenFontsReady` (sha256:a9a632e81b2b401d128497ca7e36252f671e86a84e7380d7ffa2915b10717660): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/geolocation` `upstream/packages/geolocation/src/geolocation.ts:80:5` `createWebGeolocationBackend.getPermission` (sha256:589424ae8a82b30bc0a05473e652fce3ea1e3d8dbf9d62047dec8eea751d9252): Portable task Rust lowering is not implemented.
- `@flighthq/geolocation` `upstream/packages/geolocation/src/geolocation.ts:92:5` `createWebGeolocationBackend.requestPermission` (sha256:c1d7c01868b81a6d6a24327b828b629c16b4760c6743b681bf2244d5fc64db43): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:70:1` `loadImageResourceFromBase64` (sha256:9766d6f1f595df39b0431b4aeebaf140d9cddbf13b4f4fb7212ce5980b7ace63): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:78:1` `loadImageResourceFromBlob` (sha256:789b648cfba73930add0a4f3468f72b94ce411b39d6860c22c04ab05e33590ee): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:87:1` `loadImageResourceFromBytes` (sha256:67e113685cfd158a0c49cf1757b7761013665ff96a364e34cf11a1e250391c42): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:100:1` `loadImageResourceFromUrl` (sha256:ff3642966e0506d5d9c68236e6fa44abfd7fa6721ee45db95b1683464c41b591): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image-codec` `upstream/packages/image-codec/src/decodeImage.ts:9:1` `decodeImage` (sha256:835898f04f93b4828dfcfab5782814216b29704acbd682d2d83bad27eef1d24d): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image-codec` `upstream/packages/image-codec/src/decodeImage.ts:17:1` `decodeImagePremultiplied` (sha256:d62260b672e380df16a5e004e25f9ef3df3e3d04c75d43d54b7ea1ce60d37b77): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image-codec` `upstream/packages/image-codec/src/encodeImage.ts:7:1` `encodeImage` (sha256:281f0916aa8be200aef442b2004055ce963218f0743820416b04577092053f4f): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image-codec` `upstream/packages/image-codec/src/registerWebImageDecoders.ts:18:45` `decodeImageWithCanvas` (sha256:6c70f189d2b99fef85a6c33991e3174b6e7cfc147486162cdd1abee7f40fc943): Portable task Rust lowering is not implemented.
- `@flighthq/image-codec` `upstream/packages/image-codec/src/registerWebImageEncoders.ts:16:10` `createCanvasImageEncoder.anonymous:7c4dbd1c1e56` (sha256:7c4dbd1c1e56dfb5d704a95587bc335ef9c42e20340743d59daacfb0eda450d3): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:417:1` `drainQueue` (sha256:88082b65b0b72d5a7051045ab407875a77a7430ece259e4bf911600ef6289ed7): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:467:1` `runEntry` (sha256:0c2b39c567b706e46e9bf125fdf9a0804cfd3aa6296a892957ac630951e7b438): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/net` `upstream/packages/net/src/net.ts:21:5` `createWebNetBackend.sendNetRequest` (sha256:95718bdeab605b5c428496aed0ad940a09e148c6ffd1b690723a621d3089c453): Portable task Rust lowering is not implemented.
- `@flighthq/net` `upstream/packages/net/src/net.ts:110:1` `_readNetResponseBody` (sha256:a65ec98271b1315b2d9f92bf7f23f3862fc0a2eefa19665fa73e7818af9bfc60): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/net` `upstream/packages/net/src/net.ts:133:1` `_readNetResponseWithProgress` (sha256:b695935644b73d37aea7c2af6c5ee8129d40d1a01cb334357c027ceadca38081): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:82:3` `createServiceWorkerNotificationBackend._show` (sha256:e00200853c5c516d6fb8f492b4d4a4936ad578f62c3c4c064339bd5fb6a4d072): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:120:5` `createServiceWorkerNotificationBackend.backend.closeAllNotifications` (sha256:52fa18d11686282d426bb5c4c47630bb80135c1b9e903717735b1eed8307c577): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:131:5` `createServiceWorkerNotificationBackend.backend.closeNotification` (sha256:6517e4505bd2c041ac811f8a05ba8353749846f946f30ac5ddfb2f22f7c88f0b): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:154:5` `createServiceWorkerNotificationBackend.backend.getLaunchNotification` (sha256:4f0b27a04899099e894c2a0d6f32f11acbeee1e340c8a13101f890b1fe7d68da): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:158:5` `createServiceWorkerNotificationBackend.backend.getActiveNotifications` (sha256:32b8787da01a2c016eff0505a0bdf6b9ea6a12c6ca23630e1755eeb54f07faa9): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:167:5` `createServiceWorkerNotificationBackend.backend.getPendingNotifications` (sha256:79e6ad239be0efd8c25c370a015d5579c20541e383a2e86b2c3b27bf5fd41d1e): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:182:5` `createServiceWorkerNotificationBackend.backend.requestPermission` (sha256:89dcfcb71df004f311498505a5c80f20a9a5338e1423d8e1f5dd2c482d61ce35): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:191:5` `createServiceWorkerNotificationBackend.backend.scheduleNotification` (sha256:13b016bea73a7da58bff9a988cede7621d76ce2183c21b6256e4dfd275bce1ab): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:249:5` `createServiceWorkerNotificationBackend.backend.updateNotification` (sha256:2796036fd76096b61157f020fcadbd3802fec28074380a61396a15221be36cd1): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:321:3` `createWebNotificationBackend._notify` (sha256:97462152a66b1c944e8a436b884c50a63ba0ba99f09da7b43c20889914a939bf): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:413:5` `createWebNotificationBackend.getLaunchNotification` (sha256:4f0b27a04899099e894c2a0d6f32f11acbeee1e340c8a13101f890b1fe7d68da): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:418:5` `createWebNotificationBackend.getActiveNotifications` (sha256:8fe4f259597f86d6f3ae693264b2764a1f2b204003b086bc88b1a2fa6f5d464c): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:422:5` `createWebNotificationBackend.getPendingNotifications` (sha256:79e6ad239be0efd8c25c370a015d5579c20541e383a2e86b2c3b27bf5fd41d1e): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:439:5` `createWebNotificationBackend.requestPermission` (sha256:50dc273ab49890f2d8ba5fce2bf96ff794063eaaf5f1fe32c5099b352c3cdb09): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:449:5` `createWebNotificationBackend.scheduleNotification` (sha256:837768d3e66af3adc2253422191d0a326d394f56b5590736e751f45e52532a00): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:510:5` `createWebNotificationBackend.updateNotification` (sha256:2a79742c4469437d9f0d3ea7fce31a4fa5562509765377722fa95700acbaa0eb): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:62:1` `readWebPermissionState` (sha256:92394438c09fe3ef27b9a9b1257f3a329ac1210b3a160a6e9d268193e7aec017): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:98:1` `requestWebMediaPermission` (sha256:f3ac94fd061a45ce63c7fd5c2f4f40b37199f1707959250ad5ad32eaf96908ac): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:110:1` `requestWebNotificationPermission` (sha256:7aaaa2831c88d31ed4fe57eab0b336f5d289962a28a0a65e1f1ae9372e67a59e): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:121:1` `requestWebPersistentStoragePermission` (sha256:b2c3b770762132c11047395b178d9ae70f9fdf81ef00291c52f1611c72fafd8b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:131:1` `requestWebPermission` (sha256:4fed2d962ccdc737c9b377f468a05cb53fac3631c1e6d1b8adecfd8c6a8952eb): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuRenderState.ts:16:1` `createWgpuRenderState` (sha256:f307b9234dd0ff4f3ad7ab31be9b51f27d12ccdca81862275abe1da0f7c54223): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuSurface.ts:42:1` `createSurfaceFromWgpuRenderState` (sha256:e104a245694e6ad30130cf83e51abcdd5ae2b9cee9c725396f948fc6b64ca994): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuTestHelper.ts:126:1` `createWgpuRenderStateForTest` (sha256:f9405bf588a520e76d9533461a24c2d89bdfa42ddb1e5a5481805c3e94c03d5a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/load3ds.ts:10:1` `loadSceneFrom3ds` (sha256:62b2b634bde65ac6779580bd1b61fe8e1792569b5483643ec23d59e6c9db6627): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/loadGltf.ts:15:1` `loadSceneFromGlb` (sha256:400507a514200080fcd85991bae9532f72a886de3a18b18bf97b65045bdbfcf3): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/loadGltf.ts:26:1` `loadSceneFromGltf` (sha256:e4e580f330d14c68b6e578d1c2b7d14d96ad54d0aa0bf1068fcbb0e0f4730700): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/loadGltf.ts:38:1` `loadScenesFromGlb` (sha256:a7a46d3671b3a3c6947a7d384f38c5192143f8a4c78213a457472d19b767e382): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/loadGltf.ts:49:1` `loadScenesFromGltf` (sha256:052e1ec05844e29919fc2d2f791c7fdd8a2c34fd3504133bc60f77cead54386a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/loadMd2.ts:10:1` `loadSceneFromMd2` (sha256:ba309ecdccc4a085abc0de9298b5f85dce37a22e04ba3f5487e5706922da335a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/loadMd5.ts:11:1` `loadSceneFromMd5Mesh` (sha256:78391bcc994260537e9fb1818bfb806390d32eae296f4e31d85e7e05eb42b346): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/loadObj.ts:11:1` `loadSceneFromObj` (sha256:02a9d0f93f83eaadeb6bd8fc039e36ffbe9c17e4777f31b13d2dad7c8201e494): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/loadSceneFromAwd.ts:11:1` `loadSceneFromAwd` (sha256:abb1d93ffbf72185621a1b9a65e7e14d47b57d4544431a40b37f1e012b6eb0c9): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/loadSceneOptions.ts:18:1` `resolveScenesWithOptions` (sha256:d497924e671808a5c6e0ea05abf7606e99edd904c0621946c20c2196dc7282f8): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/resolveSceneResourcesAndWait.ts:11:1` `resolveSceneResourcesAndWait` (sha256:01d4fb1834e22cdd854c87bb0f29be00c98c500fffead4ca98d313698e08ac4e): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene-resources` `upstream/packages/scene-resources/src/sceneResourceFetch.ts:16:10` `createWebSceneResourceFetch.anonymous:9f4b5cebcd53` (sha256:9f4b5cebcd5355501bc1019c4824f842685156f59193b13a88232009afe2adc2): Portable task Rust lowering is not implemented.
- `@flighthq/screen` `upstream/packages/screen/src/screen.ts:530:1` `getScreenDetailPermission` (sha256:d8cf40fc979d5a3f700e818acbaf83c4a8782cf0e2f81dbb95cae4928a164ea2): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/screen` `upstream/packages/screen/src/screen.ts:718:1` `requestScreenDetails` (sha256:b0810a06330b382391847f094e14db0df4a90f6bac18db9fac03a59be8a83ff4): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/sensors` `upstream/packages/sensors/src/sensors.ts:347:5` `createWebSensorsBackend.requestPermission` (sha256:a6b3f78082562059ef4444a5887aa326b0045614089c9750aee6061cbb35e0b1): Portable task Rust lowering is not implemented.
- `@flighthq/sensors` `upstream/packages/sensors/src/sensors.ts:734:1` `getWebSensorsPermissionState` (sha256:866595badad9bbaa0cb39468bdb48194a2b9d981244cb4e6c606ff225e7b0b56): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/share` `upstream/packages/share/src/share.ts:46:5` `createWebShareBackend.share` (sha256:4728f5341fa2d2ba2f77e4c2f40d7dec9804c802797d5c0dea14c9c428d2f5d3): Portable task Rust lowering is not implemented.
- `@flighthq/share` `upstream/packages/share/src/share.ts:59:5` `createWebShareBackend.shareWithResult` (sha256:d92540756a8e0651e19dc368a8386c5e2cc648380251118f8741018a71be5c0b): Portable task Rust lowering is not implemented.
- `@flighthq/share` `upstream/packages/share/src/share.ts:139:1` `shareContentWithResult` (sha256:ba76d368b1f5ebbefb52b037c617fa083585461ece2d83dddd0202f8f742a613): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:17:5` `createWebShellBackend.moveItemsToTrash` (sha256:de9261ef6b76c66d51f7895db3dfa0166804fa12b1736d08906a77f5182c1b87): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:21:5` `createWebShellBackend.moveToTrash` (sha256:d49f2ab1646329f228fd4dbcae8c7a83f7680f02e37bbc9a7634a08e37d42bb2): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:25:5` `createWebShellBackend.openExternal` (sha256:84df5b77ebd9b072e2cd26f7bff3a72cca839047849058809b421e0a81065f02): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:35:5` `createWebShellBackend.openPath` (sha256:717eb0cd4068de15dd3ed0c325d82ec9dfed0853bb8e991f90076d7c75a7d2da): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:39:5` `createWebShellBackend.openPathResult` (sha256:bdb09806443cc7816c45a1fec4a7f4e82c8492dc705a8ad8e5a4fb9311cc42af): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:43:5` `createWebShellBackend.readShortcutLink` (sha256:28ee120b9dd12b1d1bc09290ffe3c3841168d6a5fd616a57d0742ccd122fcdba): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:47:5` `createWebShellBackend.showItemInFolder` (sha256:37927761ccb0dd5a1883348de59ab8661eeee77fab2399832f77c3e33bcceb82): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:51:5` `createWebShellBackend.writeShortcutLink` (sha256:96c84cc788fa871527bacbf8f9a95bdb0218b2c55f5bca0890761378620eb3c6): Portable task Rust lowering is not implemented.
- `@flighthq/storage` `upstream/packages/storage/src/storage.ts:309:1` `getStorageQuotaEstimate` (sha256:bb6e6e61ce277610234cf0eb3e4794ff7a72f5694637f41d5f9169471ac85241): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:30:1` `loadTextureAtlasFromBase64` (sha256:c3b8831bbb577f82b4ecd77ec700c8eb9c03989c66a46b55c9a8dc00a57228c4): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:38:1` `loadTextureAtlasFromBlob` (sha256:7f84b1d162a9f648364d02ce2871669e570676045f31e732203694a2ca574321): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:42:1` `loadTextureAtlasFromBytes` (sha256:6ac6dc7033bfcd7d150920334b8cc158aa2fd830661a0084495ba72fdc98574b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:50:1` `loadTextureAtlasFromUrl` (sha256:e7a6e6d985a77feeffa76d7f124f2033cf98759af424baec6ff02000912842ad): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tileset` `upstream/packages/tileset/src/tilesetFrom.ts:42:1` `loadTilesetFromBase64` (sha256:ace18fa5f17b2f42f62ad5efc04ce0728152e40de50ca09ad6d2f4e5fe2e982b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tileset` `upstream/packages/tileset/src/tilesetFrom.ts:60:1` `loadTilesetFromBlob` (sha256:36a767bf7e046eb72890abb39757391b694908b4556fb161934e8c759b4b8fa7): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tileset` `upstream/packages/tileset/src/tilesetFrom.ts:77:1` `loadTilesetFromBytes` (sha256:3977b4718275eb6dd4585d28eebd7a18346623d8f300e108fbc55429e97473b1): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tileset` `upstream/packages/tileset/src/tilesetFrom.ts:95:1` `loadTilesetFromUrl` (sha256:a92858f561134da3392997d41a495f068693bf0f134207e5a37005b85e48d375): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/video` `upstream/packages/video/src/videoResourceFrom.ts:16:1` `loadVideoResourceFromBlob` (sha256:cf1a5afd32e8d470ddd095cd5892c72e20beabfedc58404d1535e49d2fffad9b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/webcam` `upstream/packages/webcam/src/webcam.ts:78:5` `createWebWebcamBackend.requestPermission` (sha256:9b066b5933be33c0c78cc297fd5270583ed6782749630182731a3b9979cd16cd): Portable task Rust lowering is not implemented.

## Generated upstream conformance

| Package | Files translated/passing/in scope | Cases translated/passing | Unsupported files |
| --- | ---: | ---: | ---: |
| `@flighthq/math` | 3/3/15 | 39/39 | 12 |
| `@flighthq/color` | 1/1/9 | 6/6 | 8 |

### Unsupported in-scope upstream test files

- `upstream/packages/math/src/angle.test.ts` (0/20 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/hash.test.ts` (0/16 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/interpolation.test.ts` (0/24 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/interpolationAdvanced.test.ts` (0/28 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/nextPowerOfTwo.test.ts` (0/17 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/numberTheory.test.ts` (0/21 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/random.test.ts` (0/4 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/randomDistributions.test.ts` (0/43 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/randomRange.test.ts` (0/14 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/rounding.test.ts` (0/21 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/scalar.test.ts` (0/13 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/statistics.test.ts` (0/18 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/colorFromKelvin.test.ts` (0/6 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/hslColor.test.ts` (0/9 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/hsvColor.test.ts` (0/8 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/lerpColor.test.ts` (0/9 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/luminance.test.ts` (0/9 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/oklab.test.ts` (0/4 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/packColor.test.ts` (0/23 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/premultiplyColorAlpha.test.ts` (0/7 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.

## Blockers

### `@flighthq/assets`

- **package** `upstream/packages/assets/src`: Generated crate is missing 10 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/assets/src/assetLibrary.ts`: loadAssetGroup: upstream/packages/assets/src/assetLibrary.ts:103:1: portableTask loadAssetGroup: Portable task Rust lowering is not implemented.

### `@flighthq/audio`

- **package** `upstream/packages/audio/src`: Generated crate is missing 7 of 20 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/audio/src/audioResourceFrom.ts`: loadAudioResourceFromBase64: upstream/packages/audio/src/audioResourceFrom.ts:24:1: portableTask loadAudioResourceFromBase64: Portable task Rust lowering is not implemented.

### `@flighthq/binpack`

- **package** `upstream/packages/binpack/src`: Generated crate is missing 1 of 1 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/binpack/src/packRectangles.ts`: compareRectangleId: typeof operand has no inferred Rust type: {"kind":"identifier","name":"a"}

### `@flighthq/capture`

- **package** `upstream/packages/capture/src`: Generated crate is missing 5 of 10 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/capture/src/captureBaseline.ts`: formatCaptureBaseline: JSON.stringify requires a portable scalar or structural array

### `@flighthq/clipboard`

- **package** `upstream/packages/clipboard/src`: Generated crate is missing 32 of 32 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/clipboard/src/clipboard.ts`: createWebClipboardBackend: upstream/packages/clipboard/src/clipboard.ts:31:5: portableTask createWebClipboardBackend.readFormat: Portable task Rust lowering is not implemented.

### `@flighthq/connectivity`

- **package** `upstream/packages/connectivity/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/connectivity/src/connectivity.ts`: createWebConnectivityBackend: upstream/packages/connectivity/src/connectivity.ts:82:5: portableTask createWebConnectivityBackend.detectReachability: Portable task Rust lowering is not implemented.

### `@flighthq/debug`

- **package** `upstream/packages/debug/src`: Generated crate is missing 5 of 9 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/debug/src/debug.ts`: _collectDebugChannels: spread Rust lowering is not implemented

### `@flighthq/dialog`

- **package** `upstream/packages/dialog/src`: Generated crate is missing 15 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/dialog/src/dialog.ts`: createWebDialogBackend: upstream/packages/dialog/src/dialog.ts:19:5: portableTask createWebDialogBackend.confirm: Portable task Rust lowering is not implemented.

### `@flighthq/displayobject`

- **emission** `upstream/packages/displayobject/src/bitmap.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/displayobject/src/displayContainer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/displayobject/src/displayObject.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/displayobject/src/htmlView.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/displayobject/src/renderView.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/displayobject/src/stage.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/displayobject/src/video.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/displayobject-canvas`

- **package** `upstream/packages/displayobject-canvas/src`: Generated crate is missing 35 of 94 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-canvas/src/canvasBitmap.ts`: drawCanvasBitmap: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"state"},"optional":false}
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

- **package** `upstream/packages/displayobject-gl/src`: Generated crate is missing 8 of 89 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/displayobject-gl/src/glVelocity.ts`: defaultGlDisplayObjectVelocityWriter: upstream/packages/displayobject-gl/src/glVelocity.ts: cannot infer return type for defaultGlDisplayObjectVelocityWriter

### `@flighthq/displayobject-wgpu`

- **package** `upstream/packages/displayobject-wgpu/src`: Generated crate is missing 8 of 95 upstream exports; re-export or declaration synthesis is required.
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
- **emission** `upstream/packages/effects-gl/src/glBevelEffect.ts`: defaultGlBevelEffectRunner: upstream/packages/effects-gl/src/glBevelEffect.ts: cannot infer return type for defaultGlBevelEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBlendEffect.ts`: defaultGlBlendEffectRunner: upstream/packages/effects-gl/src/glBlendEffect.ts: cannot infer return type for defaultGlBlendEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBloomEffect.ts`: defaultGlBloomEffectRunner: upstream/packages/effects-gl/src/glBloomEffect.ts: cannot infer return type for defaultGlBloomEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBlurEffect.ts`: defaultGlBlurEffectRunner: upstream/packages/effects-gl/src/glBlurEffect.ts: cannot infer return type for defaultGlBlurEffectRunner
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
- **emission** `upstream/packages/effects-gl/src/glDropShadowEffect.ts`: defaultGlDropShadowEffectRunner: upstream/packages/effects-gl/src/glDropShadowEffect.ts: cannot infer return type for defaultGlDropShadowEffectRunner
- **emission** `upstream/packages/effects-gl/src/glFilmGrainEffect.ts`: defaultGlFilmGrainEffectRunner: upstream/packages/effects-gl/src/glFilmGrainEffect.ts: cannot infer return type for defaultGlFilmGrainEffectRunner
- **emission** `upstream/packages/effects-gl/src/glFxaaEffect.ts`: defaultGlFxaaEffectRunner: upstream/packages/effects-gl/src/glFxaaEffect.ts: cannot infer return type for defaultGlFxaaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGlitchEffect.ts`: defaultGlGlitchEffectRunner: upstream/packages/effects-gl/src/glGlitchEffect.ts: cannot infer return type for defaultGlGlitchEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGodRaysEffect.ts`: defaultGlGodRaysEffectRunner: upstream/packages/effects-gl/src/glGodRaysEffect.ts: cannot infer return type for defaultGlGodRaysEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGradientBevelEffect.ts`: defaultGlGradientBevelEffectRunner: upstream/packages/effects-gl/src/glGradientBevelEffect.ts: cannot infer return type for defaultGlGradientBevelEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGradientGlowEffect.ts`: defaultGlGradientGlowEffectRunner: upstream/packages/effects-gl/src/glGradientGlowEffect.ts: cannot infer return type for defaultGlGradientGlowEffectRunner
- **emission** `upstream/packages/effects-gl/src/glHalftoneEffect.ts`: defaultGlHalftoneEffectRunner: upstream/packages/effects-gl/src/glHalftoneEffect.ts: cannot infer return type for defaultGlHalftoneEffectRunner
- **emission** `upstream/packages/effects-gl/src/glInnerGlowEffect.ts`: defaultGlInnerGlowEffectRunner: upstream/packages/effects-gl/src/glInnerGlowEffect.ts: cannot infer return type for defaultGlInnerGlowEffectRunner
- **emission** `upstream/packages/effects-gl/src/glInnerShadowEffect.ts`: defaultGlInnerShadowEffectRunner: upstream/packages/effects-gl/src/glInnerShadowEffect.ts: cannot infer return type for defaultGlInnerShadowEffectRunner
- **emission** `upstream/packages/effects-gl/src/glKuwaharaEffect.ts`: defaultGlKuwaharaEffectRunner: upstream/packages/effects-gl/src/glKuwaharaEffect.ts: cannot infer return type for defaultGlKuwaharaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensDirtEffect.ts`: defaultGlLensDirtEffectRunner: upstream/packages/effects-gl/src/glLensDirtEffect.ts: cannot infer return type for defaultGlLensDirtEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensDistortionEffect.ts`: defaultGlLensDistortionEffectRunner: upstream/packages/effects-gl/src/glLensDistortionEffect.ts: cannot infer return type for defaultGlLensDistortionEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensFlareEffect.ts`: defaultGlLensFlareEffectRunner: upstream/packages/effects-gl/src/glLensFlareEffect.ts: cannot infer return type for defaultGlLensFlareEffectRunner
- **emission** `upstream/packages/effects-gl/src/glMedianEffect.ts`: defaultGlMedianEffectRunner: upstream/packages/effects-gl/src/glMedianEffect.ts: cannot infer return type for defaultGlMedianEffectRunner
- **emission** `upstream/packages/effects-gl/src/glMotionBlurEffect.ts`: defaultGlMotionBlurEffectRunner: upstream/packages/effects-gl/src/glMotionBlurEffect.ts: cannot infer return type for defaultGlMotionBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glOuterGlowEffect.ts`: defaultGlOuterGlowEffectRunner: upstream/packages/effects-gl/src/glOuterGlowEffect.ts: cannot infer return type for defaultGlOuterGlowEffectRunner
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

- **package** `upstream/packages/effects-wgpu/src`: Generated crate is missing 91 of 128 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-wgpu/src/wgpuBevelEffect.ts`: defaultWgpuBevelEffectRunner: upstream/packages/effects-wgpu/src/wgpuBevelEffect.ts: cannot infer return type for defaultWgpuBevelEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuBloomEffect.ts`: defaultWgpuBloomEffectRunner: upstream/packages/effects-wgpu/src/wgpuBloomEffect.ts: cannot infer return type for defaultWgpuBloomEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuBlurEffect.ts`: defaultWgpuBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuBlurEffect.ts: cannot infer return type for defaultWgpuBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuBokehDepthOfFieldEffect.ts`: defaultWgpuBokehDepthOfFieldEffectRunner: upstream/packages/effects-wgpu/src/wgpuBokehDepthOfFieldEffect.ts: cannot infer return type for defaultWgpuBokehDepthOfFieldEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCameraMotionBlurEffect.ts`: defaultWgpuCameraMotionBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuCameraMotionBlurEffect.ts: cannot infer return type for defaultWgpuCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuChromaticAberrationEffect.ts`: defaultWgpuChromaticAberrationEffectRunner: upstream/packages/effects-wgpu/src/wgpuChromaticAberrationEffect.ts: cannot infer return type for defaultWgpuChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuConvolutionEffect.ts`: defaultWgpuConvolutionEffectRunner: upstream/packages/effects-wgpu/src/wgpuConvolutionEffect.ts: cannot infer return type for defaultWgpuConvolutionEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCrtEffect.ts`: defaultWgpuCrtEffectRunner: upstream/packages/effects-wgpu/src/wgpuCrtEffect.ts: cannot infer return type for defaultWgpuCrtEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDirectionalBlurEffect.ts`: defaultWgpuDirectionalBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuDirectionalBlurEffect.ts: cannot infer return type for defaultWgpuDirectionalBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDisplacementEffect.ts`: defaultWgpuDisplacementEffectRunner: upstream/packages/effects-wgpu/src/wgpuDisplacementEffect.ts: cannot infer return type for defaultWgpuDisplacementEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDitherEffect.ts`: defaultWgpuDitherEffectRunner: upstream/packages/effects-wgpu/src/wgpuDitherEffect.ts: cannot infer return type for defaultWgpuDitherEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDropShadowEffect.ts`: defaultWgpuDropShadowEffectRunner: upstream/packages/effects-wgpu/src/wgpuDropShadowEffect.ts: cannot infer return type for defaultWgpuDropShadowEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuFilmGrainEffect.ts`: defaultWgpuFilmGrainEffectRunner: upstream/packages/effects-wgpu/src/wgpuFilmGrainEffect.ts: cannot infer return type for defaultWgpuFilmGrainEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuFxaaEffect.ts`: defaultWgpuFxaaEffectRunner: upstream/packages/effects-wgpu/src/wgpuFxaaEffect.ts: cannot infer return type for defaultWgpuFxaaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGlitchEffect.ts`: defaultWgpuGlitchEffectRunner: upstream/packages/effects-wgpu/src/wgpuGlitchEffect.ts: cannot infer return type for defaultWgpuGlitchEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGodRaysEffect.ts`: defaultWgpuGodRaysEffectRunner: upstream/packages/effects-wgpu/src/wgpuGodRaysEffect.ts: cannot infer return type for defaultWgpuGodRaysEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGradientBevelEffect.ts`: defaultWgpuGradientBevelEffectRunner: upstream/packages/effects-wgpu/src/wgpuGradientBevelEffect.ts: cannot infer return type for defaultWgpuGradientBevelEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGradientGlowEffect.ts`: defaultWgpuGradientGlowEffectRunner: upstream/packages/effects-wgpu/src/wgpuGradientGlowEffect.ts: cannot infer return type for defaultWgpuGradientGlowEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuHalftoneEffect.ts`: defaultWgpuHalftoneEffectRunner: upstream/packages/effects-wgpu/src/wgpuHalftoneEffect.ts: cannot infer return type for defaultWgpuHalftoneEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuInnerGlowEffect.ts`: defaultWgpuInnerGlowEffectRunner: upstream/packages/effects-wgpu/src/wgpuInnerGlowEffect.ts: cannot infer return type for defaultWgpuInnerGlowEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuInnerShadowEffect.ts`: defaultWgpuInnerShadowEffectRunner: upstream/packages/effects-wgpu/src/wgpuInnerShadowEffect.ts: cannot infer return type for defaultWgpuInnerShadowEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuKuwaharaEffect.ts`: defaultWgpuKuwaharaEffectRunner: upstream/packages/effects-wgpu/src/wgpuKuwaharaEffect.ts: cannot infer return type for defaultWgpuKuwaharaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensDirtEffect.ts`: defaultWgpuLensDirtEffectRunner: upstream/packages/effects-wgpu/src/wgpuLensDirtEffect.ts: cannot infer return type for defaultWgpuLensDirtEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensDistortionEffect.ts`: defaultWgpuLensDistortionEffectRunner: upstream/packages/effects-wgpu/src/wgpuLensDistortionEffect.ts: cannot infer return type for defaultWgpuLensDistortionEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensFlareEffect.ts`: defaultWgpuLensFlareEffectRunner: upstream/packages/effects-wgpu/src/wgpuLensFlareEffect.ts: cannot infer return type for defaultWgpuLensFlareEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuMedianEffect.ts`: defaultWgpuMedianEffectRunner: upstream/packages/effects-wgpu/src/wgpuMedianEffect.ts: cannot infer return type for defaultWgpuMedianEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuMotionBlurEffect.ts`: defaultWgpuMotionBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuMotionBlurEffect.ts: cannot infer return type for defaultWgpuMotionBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuOuterGlowEffect.ts`: defaultWgpuOuterGlowEffectRunner: upstream/packages/effects-wgpu/src/wgpuOuterGlowEffect.ts: cannot infer return type for defaultWgpuOuterGlowEffectRunner
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
- **emission** `upstream/packages/filesystem/src/filesystem.ts`: createWebFileSystemBackend: upstream/packages/filesystem/src/filesystem.ts:41:5: portableTask createWebFileSystemBackend.readTextFile: Portable task Rust lowering is not implemented.

### `@flighthq/font`

- **package** `upstream/packages/font/src`: Generated crate is missing 10 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/font/src/fontFrom.ts`: loadFontFromBytes: upstream/packages/font/src/fontFrom.ts:7:1: portableTask loadFontFromBytes: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/font/src/fontResourceFrom.ts`: loadFontResourceFromBytes: upstream/packages/font/src/fontResourceFrom.ts:6:1: portableTask loadFontResourceFromBytes: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/font/src/fontStatus.ts`: whenFontsReady: upstream/packages/font/src/fontStatus.ts:7:1: portableTask whenFontsReady: Portable task Rust lowering is not implemented.

### `@flighthq/geolocation`

- **package** `upstream/packages/geolocation/src`: Generated crate is missing 12 of 12 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/geolocation/src/geolocation.ts`: createWebGeolocationBackend: typeof operand has no inferred Rust type: {"kind":"property","name":"clearWatch","object":{"kind":"identifier","name":"geo"},"optional":false}

### `@flighthq/image`

- **package** `upstream/packages/image/src`: Generated crate is missing 9 of 20 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/image/src/imageResourceFrom.ts`: loadImageResourceFromBase64: upstream/packages/image/src/imageResourceFrom.ts:70:1: portableTask loadImageResourceFromBase64: Portable task Rust lowering is not implemented.

### `@flighthq/image-codec`

- **package** `upstream/packages/image-codec/src`: Generated crate is missing 5 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/image-codec/src/decodeImage.ts`: decodeImage: upstream/packages/image-codec/src/decodeImage.ts:9:1: portableTask decodeImage: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/image-codec/src/encodeImage.ts`: encodeImage: upstream/packages/image-codec/src/encodeImage.ts:7:1: portableTask encodeImage: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/image-codec/src/registerWebImageDecoders.ts`: decodeImageWithCanvas: upstream/packages/image-codec/src/registerWebImageDecoders.ts:18:45: portableTask decodeImageWithCanvas: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/image-codec/src/registerWebImageEncoders.ts`: createCanvasImageEncoder: upstream/packages/image-codec/src/registerWebImageEncoders.ts:16:10: portableTask createCanvasImageEncoder.anonymous:7c4dbd1c1e56: Portable task Rust lowering is not implemented.

### `@flighthq/interaction`

- **package** `upstream/packages/interaction/src`: Generated crate is missing 12 of 83 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/interaction/src/cursorBackend.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/displayHitTests.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/displayObjectOverlap.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/enableInteractionGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/focusManager.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/hitTests.ts`: hitAreaContainsPoint: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/interaction/src/interactionManager.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/interactionSpatialIndex.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/nodeInteractionState.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/registerBitmapHitTest.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/registerDefaultHitTests.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/registerShapeHitTest.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/registerTextHitTest.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/spatialQuery.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/spriteHitTests.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/intl`

- **package** `upstream/packages/intl/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/intl/src/cache.ts`: getCacheKey: typeof operand has no inferred Rust type: {"kind":"identifier","name":"locale"}
- **emission** `upstream/packages/intl/src/collator.ts`: getCollator: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.Collator")
- **emission** `upstream/packages/intl/src/datetime.ts`: formatDateValue: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.DateTimeFormat")
- **emission** `upstream/packages/intl/src/list.ts`: formatList: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.ListFormat")
- **emission** `upstream/packages/intl/src/number.ts`: formatCompactNumber: object literal requires an inferred structural type (target=unknown, properties=notation,spread)
- **emission** `upstream/packages/intl/src/plural.ts`: selectOrdinalCategory: object literal requires an inferred structural type (target=unknown, properties=type,spread)
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
- **emission** `upstream/packages/net/src/net.ts`: createWebNetBackend: upstream/packages/net/src/net.ts:21:5: portableTask createWebNetBackend.sendNetRequest: Portable task Rust lowering is not implemented.

### `@flighthq/node`

- **emission** `upstream/packages/node/src/boundsRectangle.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (10 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/hasBoundsRectangle.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (10 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/hasTransform2d.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (10 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/hierarchy.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (10 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/node.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (10 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/revision.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (10 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/transform2d.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (10 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/transform3d.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (10 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/traversal.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (10 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/viewport.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (10 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/notification`

- **package** `upstream/packages/notification/src`: Generated crate is missing 26 of 26 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/notification/src/notification.ts`: createServiceWorkerNotificationBackend: upstream/packages/notification/src/notification.ts:82:3: portableTask createServiceWorkerNotificationBackend._show: Portable task Rust lowering is not implemented.

### `@flighthq/particles-formats`

- **package** `upstream/packages/particles-formats/src`: Generated crate is missing 17 of 79 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/particles-formats/src/detect.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/formatRegistry.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/libgdxParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/libgdxSerialize.ts`: documentToText: spread Rust lowering is not implemented
- **emission** `upstream/packages/particles-formats/src/parseParticleConfig.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/particleDesignerParse.ts`: num: typeof operand has no inferred Rust type: {"kind":"identifier","name":"v"}
- **emission** `upstream/packages/particles-formats/src/pixiParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/spineParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/spineSerialize.ts`: serializeSpineParticle: JSON.stringify requires a portable scalar or structural array
- **emission** `upstream/packages/particles-formats/src/starlingPexParse.ts`: extractAttr: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/particles-formats/src/unityParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (7 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/unitySerialize.ts`: serializeUnityParticle: JSON.stringify requires a portable scalar or structural array

### `@flighthq/path-boolean`

- **package** `upstream/packages/path-boolean/src`: Generated crate is missing 1 of 12 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/path-boolean/src/martinezKernel.ts`: buildArrangement: new-expression Rust lowering is not implemented: event_heap

### `@flighthq/permissions`

- **package** `upstream/packages/permissions/src`: Generated crate is missing 5 of 5 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/permissions/src/permission.ts`: readWebPermissionState: upstream/packages/permissions/src/permission.ts:62:1: portableTask readWebPermissionState: Portable task Rust lowering is not implemented.

### `@flighthq/picking`

- **emission** `upstream/packages/picking/src/pickScene.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/render`

- **emission** `upstream/packages/render/src/enableColorAdjustmentGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/explainDisplayObjectRender.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderAppearance.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderCache.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderColorTransform.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderProxy.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderProxyAdapter.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderQueue.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderTarget.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderTransform2d.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderViewport.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/sceneRender.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (13 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/render-gl`

- **package** `upstream/packages/render-gl/src`: Generated crate is missing 5 of 75 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-gl/src/glRenderState.ts`: createGlRenderState: object literal requires an inferred structural type (target=unknown, properties=alpha,antialias,powerPreference,stencil,spread)

### `@flighthq/render-wgpu`

- **package** `upstream/packages/render-wgpu/src`: Generated crate is missing 10 of 68 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-wgpu/src/wgpuRenderState.ts`: createWgpuRenderState: upstream/packages/render-wgpu/src/wgpuRenderState.ts:16:1: portableTask createWgpuRenderState: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/render-wgpu/src/wgpuSurface.ts`: createSurfaceFromWgpuRenderState: upstream/packages/render-wgpu/src/wgpuSurface.ts:42:1: portableTask createSurfaceFromWgpuRenderState: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/render-wgpu/src/wgpuTestHelper.ts`: createWgpuRenderStateForTest: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:126:1: portableTask createWgpuRenderStateForTest: Portable task Rust lowering is not implemented.

### `@flighthq/scene`

- **emission** `upstream/packages/scene/src/billboard.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/billboardCamera.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/mesh.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/scene.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/sceneAnimation.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/sceneMaterial.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/sceneNode.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/sceneNodeAppearance.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/sceneNodeBounds.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/sceneNodeCulling.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/sceneNodeDispose.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene/src/sceneNodeTransform.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 7); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/scene-formats`

- **package** `upstream/packages/scene-formats/src`: Generated crate is missing 8 of 15 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-formats/src/awdParse.ts`: createSceneFromAwd: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene-formats/src/gltfParse.ts`: readAccessor: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene-formats/src/gltfSchema.ts`: GltfNormalTextureInfo: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"KHR_texture_transform","optional":true,"type":{"arguments":[],"kind":"named","name":"GltfTextureTransform"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene-formats/src/md2Parse.ts`: createSceneFromMd2: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene-formats/src/md5AnimParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene-formats/src/md5Parse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene-formats/src/objParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene-formats/src/shared.ts`: findSceneSkeletonJoints: spread Rust lowering is not implemented
- **emission** `upstream/packages/scene-formats/src/threeDsParse.ts`: createSceneFrom3ds: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/scene-resources`

- **package** `upstream/packages/scene-resources/src`: Generated crate is missing 15 of 37 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-resources/src/load3ds.ts`: loadSceneFrom3ds: upstream/packages/scene-resources/src/load3ds.ts:10:1: portableTask loadSceneFrom3ds: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/scene-resources/src/loadGltf.ts`: loadSceneFromGlb: upstream/packages/scene-resources/src/loadGltf.ts:15:1: portableTask loadSceneFromGlb: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/scene-resources/src/loadMd2.ts`: loadSceneFromMd2: upstream/packages/scene-resources/src/loadMd2.ts:10:1: portableTask loadSceneFromMd2: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/scene-resources/src/loadMd5.ts`: loadSceneFromMd5Mesh: upstream/packages/scene-resources/src/loadMd5.ts:11:1: portableTask loadSceneFromMd5Mesh: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/scene-resources/src/loadObj.ts`: loadSceneFromObj: upstream/packages/scene-resources/src/loadObj.ts:11:1: portableTask loadSceneFromObj: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/scene-resources/src/loadSceneFromAwd.ts`: loadSceneFromAwd: upstream/packages/scene-resources/src/loadSceneFromAwd.ts:11:1: portableTask loadSceneFromAwd: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/scene-resources/src/loadSceneOptions.ts`: resolveScenesWithOptions: upstream/packages/scene-resources/src/loadSceneOptions.ts:18:1: portableTask resolveScenesWithOptions: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/scene-resources/src/resolveSceneResourcesAndWait.ts`: resolveSceneResourcesAndWait: upstream/packages/scene-resources/src/resolveSceneResourcesAndWait.ts:11:1: portableTask resolveSceneResourcesAndWait: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/scene-resources/src/sceneResourceFetch.ts`: createWebSceneResourceFetch: upstream/packages/scene-resources/src/sceneResourceFetch.ts:16:10: portableTask createWebSceneResourceFetch.anonymous:9f4b5cebcd53: Portable task Rust lowering is not implemented.

### `@flighthq/screen`

- **package** `upstream/packages/screen/src`: Generated crate is missing 31 of 31 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/screen/src/screen.ts`: getScreenDetailPermission: upstream/packages/screen/src/screen.ts:530:1: portableTask getScreenDetailPermission: Portable task Rust lowering is not implemented.

### `@flighthq/sdk`

- **package** `upstream/packages/sdk/src`: Generated crate is missing 5923 of 5923 upstream exports; re-export or declaration synthesis is required.

### `@flighthq/sensors`

- **package** `upstream/packages/sensors/src`: Generated crate is missing 32 of 32 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/sensors/src/sensors.ts`: createWebSensorsBackend: upstream/packages/sensors/src/sensors.ts:347:5: portableTask createWebSensorsBackend.requestPermission: Portable task Rust lowering is not implemented.

### `@flighthq/shading`

- **emission** `upstream/packages/shading/src/modifierRegistry.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shading/src/orderModifierStack.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shading/src/registerBuiltInModifiers.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/shape`

- **package** `upstream/packages/shape/src`: Generated crate is missing 10 of 42 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shape/src/shape.ts`: copyShapeCommands: spread Rust lowering is not implemented

### `@flighthq/shape-formats`

- **package** `upstream/packages/shape-formats/src`: Generated crate is missing 5 of 5 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shape-formats/src/shapeJson.ts`: formatShapeJson: JSON.stringify requires a portable scalar or structural array

### `@flighthq/share`

- **package** `upstream/packages/share/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/share/src/share.ts`: createWebShareBackend: upstream/packages/share/src/share.ts:46:5: portableTask createWebShareBackend.share: Portable task Rust lowering is not implemented.

### `@flighthq/shell`

- **package** `upstream/packages/shell/src`: Generated crate is missing 14 of 14 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shell/src/shell.ts`: createWebShellBackend: upstream/packages/shell/src/shell.ts:17:5: portableTask createWebShellBackend.moveItemsToTrash: Portable task Rust lowering is not implemented.

### `@flighthq/shortcut`

- **package** `upstream/packages/shortcut/src`: Generated crate is missing 26 of 26 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shortcut/src/shortcut.ts`: parseAcceleratorDetailed: in-operator requires a static property name or an opaque host receiver

### `@flighthq/spritesheet-formats`

- **package** `upstream/packages/spritesheet-formats/src`: Generated crate is missing 4 of 55 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/spritesheet-formats/src/asepriteSerialize.ts`: serializeAsepriteSpritesheet: JSON.stringify requires a portable scalar or structural array
- **emission** `upstream/packages/spritesheet-formats/src/texturePackerSerialize.ts`: serializeTexturePackerSpritesheet: JSON.stringify requires a portable scalar or structural array

### `@flighthq/statusbar`

- **package** `upstream/packages/statusbar/src`: Generated crate is missing 16 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/statusbar/src/statusbar.ts`: _styleStack: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"handle","optional":false,"type":{"arguments":[],"kind":"named","name":"StatusBarStyleEntryHandle"}},{"name":"entry","optional":false,"type":{"arguments":[],"kind":"named","name":"StatusBarStyleEntry"}}],"kind":"anonymous"}

### `@flighthq/storage`

- **package** `upstream/packages/storage/src`: Generated crate is missing 39 of 39 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/storage/src/storage.ts`: getStorageQuotaEstimate: upstream/packages/storage/src/storage.ts:309:1: portableTask getStorageQuotaEstimate: Portable task Rust lowering is not implemented.

### `@flighthq/text`

- **package** `upstream/packages/text/src`: Generated crate is missing 16 of 86 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/text/src/nativeText.ts`: patchNativeTextStyle: multiple object spreads require ordered Rust lowering

### `@flighthq/text-markup`

- **package** `upstream/packages/text-markup/src`: Generated crate is missing 2 of 8 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/text-markup/src/textMarkup.ts`: handleMarkupToken: multiple object spreads require ordered Rust lowering

### `@flighthq/textshaper`

- **package** `upstream/packages/textshaper/src`: Generated crate is missing 3 of 31 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textshaper/src/textShaper.ts`: setTextShaperBackend: optional call requires an inferred nullable function: {"kind":"identifier","name":"_textShaperBackendHook"}

### `@flighthq/texture-formats`

- **emission** `upstream/packages/texture-formats/src/byteReader.ts`: createByteReader: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/textureatlas`

- **package** `upstream/packages/textureatlas/src`: Generated crate is missing 8 of 20 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textureatlas/src/textureAtlasFrom.ts`: loadTextureAtlasFromBase64: upstream/packages/textureatlas/src/textureAtlasFrom.ts:30:1: portableTask loadTextureAtlasFromBase64: Portable task Rust lowering is not implemented.

### `@flighthq/tilemap-formats`

- **emission** `upstream/packages/tilemap-formats/src/tiledJsonParse.ts`: boolField: typeof operand has no inferred Rust type: {"kind":"identifier","name":"value"}
- **emission** `upstream/packages/tilemap-formats/src/tiledProject.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tilemap-formats/src/tiledTmxFormat.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tilemap-formats/src/tiledXmlParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/tileset`

- **package** `upstream/packages/tileset/src`: Generated crate is missing 6 of 9 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tileset/src/tilesetFrom.ts`: loadTilesetFromBase64: upstream/packages/tileset/src/tilesetFrom.ts:42:1: portableTask loadTilesetFromBase64: Portable task Rust lowering is not implemented.

### `@flighthq/tray`

- **emission** `upstream/packages/tray/src/tray.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/tween`

- **emission** `upstream/packages/tween/src/timer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tween/src/tween.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tween/src/tweenProgress.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/updater`

- **emission** `upstream/packages/updater/src/updater.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/video`

- **package** `upstream/packages/video/src`: Generated crate is missing 4 of 16 upstream exports; re-export or declaration synthesis is required.
- **emission** `upstream/packages/video/src/videoResourceFrom.ts`: loadVideoResourceFromBlob: upstream/packages/video/src/videoResourceFrom.ts:16:1: portableTask loadVideoResourceFromBlob: Portable task Rust lowering is not implemented.

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

### `@flighthq/collision`

- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `x` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `y` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `radius` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `radius` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `min_x` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `max_x` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `min_y` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `max_y` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `rotation` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `rotation` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `x` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `y` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `half_w` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `half_h` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `points` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `points` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `x1` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `x0` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `y1` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `y0` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `x0` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `y0` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `x0` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `y0` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `x` on type `&CollisionShape`
- **E0609** `generated/candidates/flighthq-collision/src/point_containment.rs`: no field `y` on type `&CollisionShape`
- **E0308** `generated/candidates/flighthq-collision/src/test_collision.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-collision/src/test_collision.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-collision/src/test_collision.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-collision/src/test_collision.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-collision/src/test_collision.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-collision/src/test_collision.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-collision/src/test_collision.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-collision/src/test_collision.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-collision/src/test_collision.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-collision/src/test_collision.rs`: arguments to this function are incorrect

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
- **E0308** `generated/candidates/flighthq-effects/src/custom_shader_effect.rs`: mismatched types
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
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: no field `webgl_data` on type `std::sync::MutexGuard<'_, flighthq_types::EntityRuntimeStorage>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: no field `webgpu_data` on type `std::sync::MutexGuard<'_, flighthq_types::EntityRuntimeStorage>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: no field `morph_bind_pose` on type `std::sync::MutexGuard<'_, flighthq_types::EntityRuntimeStorage>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: no field `skin_bind_pose` on type `std::sync::MutexGuard<'_, flighthq_types::EntityRuntimeStorage>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: no field `morph_bind_pose` on type `std::sync::MutexGuard<'_, flighthq_types::EntityRuntimeStorage>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: no field `skin_bind_pose` on type `std::sync::MutexGuard<'_, flighthq_types::EntityRuntimeStorage>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: no field `morph_bind_pose` on type `std::sync::MutexGuard<'_, flighthq_types::EntityRuntimeStorage>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: no field `skin_bind_pose` on type `std::sync::MutexGuard<'_, flighthq_types::EntityRuntimeStorage>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: no field `webgl_data` on type `std::sync::MutexGuard<'_, flighthq_types::EntityRuntimeStorage>`
- **E0609** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: no field `webgpu_data` on type `std::sync::MutexGuard<'_, flighthq_types::EntityRuntimeStorage>`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry.rs`: mismatched types
- **E0615** `generated/candidates/flighthq-mesh/src/mesh_geometry_attributes.rs`: attempted to take value of method `starts_with` on type `std::string::String`
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: arguments to this function are incorrect
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: arguments to this function are incorrect
- **E0282** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: type annotations needed
- **E0308** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: arguments to this function are incorrect
- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: a value of type `Vec<Vec<f64>>` cannot be built from an iterator over elements of type `OpaqueHostValue`
- **E0308** `<rustc>/library/alloc/src/macros.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-mesh/src/mesh_geometry_builders.rs`: a value of type `Vec<Vec<f64>>` cannot be built from an iterator over elements of type `OpaqueHostValue`
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
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `to_lower_case` on type `std::string::String`
- **E0615** `generated/candidates/flighthq-protocol/src/protocol.rs`: attempted to take value of method `starts_with` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `slice` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `index_of` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `index_of` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `length` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `slice` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `slice` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `index_of` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `slice` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `slice` on type `std::string::String`
- **E0609** `generated/candidates/flighthq-protocol/src/protocol.rs`: no field `index_of` on type `std::string::String`
- **E0070** `generated/candidates/flighthq-protocol/src/protocol.rs`: invalid left-hand side of assignment
- **E0070** `generated/candidates/flighthq-protocol/src/protocol.rs`: invalid left-hand side of assignment
- **E0308** `generated/candidates/flighthq-protocol/src/protocol.rs`: mismatched types

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
- **E0308** `generated/candidates/flighthq-snapshot/src/interpolate_snapshots.rs`: mismatched types
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
- **E0600** `generated/candidates/flighthq-xml/src/xml_parse.rs`: cannot apply unary operator `!` to type `String`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0615** `generated/candidates/flighthq-xml/src/xml_parse.rs`: attempted to take value of method `trim` on type `String`
- **E0368** `generated/candidates/flighthq-xml/src/xml_parse.rs`: binary assignment operation `+=` cannot be applied to type `&str`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0599** `generated/candidates/flighthq-xml/src/xml_query.rs`: no method named `is_some` found for struct `String` in the current scope
- **E0599** `generated/candidates/flighthq-xml/src/xml_query.rs`: no method named `is_none` found for struct `String` in the current scope
- **E0425** `generated/candidates/flighthq-xml/src/xml_query.rs`: cannot find function `number` in this scope
- **E0609** `generated/candidates/flighthq-xml/src/xml_query.rs`: no field `filter` on type `Vec<xml_parse::XmlElement>`
- **E0308** `generated/candidates/flighthq-xml/src/xml_query.rs`: mismatched types

# Automatic Rust Generation

Upstream commit: `cad72aa3ea4e6e76a050918a403dcb10efdfcb0d`

| Metric | Count |
| --- | ---: |
| Inventoried packages | 143 |
| Default-generated packages | 137 |
| Emittable packages | 43 |
| Blocked packages | 94 |
| Compiled candidates | 15 |
| Compile-blocked candidates | 7 |
| Dependency-blocked candidates | 19 |
| Cultivated packages | 1 |
| Host-bound packages | 4 |
| Excluded packages | 1 |
| Source/package blockers | 460 |
| Eligible task constructions | 225 |
| Portable executable task constructions | 19 |
| Host-placeholder task constructions | 0 |
| Unsupported task constructions | 206 |
| Eligible async scopes | 173 |
| Portable executable async scopes | 13 |
| Host-placeholder async scopes | 0 |
| Unsupported async scopes | 160 |
| Async scopes matching the legacy body-erasure path | 86 |
| Upstream conformance files translated and passing | 4/1419 |
| Generated conformance cases passing | 45/45 |

| Package | Disposition | Status | Candidate | Sources emitted/attempted | API generated/expected | Missing | Dependents direct/transitive | Opaque sources | Blockers | Target |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `@flighthq/abc` | generated | blocked | source-blocked | 3/4 | 2/2 | 0 | 2/2 | 0 | 1 | no |
| `@flighthq/accessibility` | generated | emittable | compile-blocked | 3/3 | 8/8 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/adjustments` | generated | emittable | compiled | 21/21 | 52/52 | 0 | 9/42 | 0 | 0 | no |
| `@flighthq/animation` | generated | blocked | source-blocked | 7/13 | 54/52 | 0 | 8/36 | 0 | 6 | no |
| `@flighthq/app` | generated | blocked | source-blocked | 2/3 | 39/42 | 3 | 4/4 | 0 | 2 | no |
| `@flighthq/application` | generated | blocked | source-blocked | 4/5 | 87/87 | 0 | 4/4 | 2 | 1 | partial |
| `@flighthq/application-gl` | host-backend | emittable | dependency-blocked | 3/3 | 2/2 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/assets` | generated | blocked | source-blocked | 4/5 | 17/18 | 1 | 1/1 | 1 | 2 | no |
| `@flighthq/audio` | generated | blocked | source-blocked | 5/7 | 34/34 | 0 | 4/4 | 1 | 2 | no |
| `@flighthq/binpack` | generated | blocked | source-blocked | 2/4 | 3/4 | 1 | 1/1 | 0 | 3 | no |
| `@flighthq/bitmap` | cultivated | cultivated | not-applicable | 0/0 | 0/109 | 109 | 7/10 | 0 | 0 | partial |
| `@flighthq/bitmapfont` | generated | blocked | source-blocked | 7/8 | 16/16 | 0 | 2/2 | 0 | 1 | no |
| `@flighthq/bitmapfont-formats` | generated | emittable | dependency-blocked | 6/6 | 5/4 | 0 | 1/1 | 3 | 0 | no |
| `@flighthq/bitmaptext` | generated | emittable | dependency-blocked | 4/4 | 14/14 | 0 | 1/1 | 2 | 0 | no |
| `@flighthq/camera` | generated | emittable | dependency-blocked | 17/17 | 38/38 | 0 | 5/5 | 0 | 0 | no |
| `@flighthq/camera-controls` | generated | blocked | source-blocked | 5/6 | 22/22 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/capture` | generated | blocked | source-blocked | 3/4 | 10/10 | 0 | 2/2 | 1 | 1 | no |
| `@flighthq/clip` | generated | blocked | source-blocked | 2/4 | 26/26 | 0 | 3/4 | 0 | 2 | no |
| `@flighthq/clipboard` | generated | blocked | source-blocked | 2/3 | 29/32 | 3 | 4/4 | 0 | 2 | no |
| `@flighthq/clock` | generated | emittable | compile-blocked | 4/4 | 14/14 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/collision` | generated | blocked | source-blocked | 15/17 | 47/41 | 0 | 2/2 | 0 | 2 | no |
| `@flighthq/color` | generated | emittable | compiled | 11/11 | 30/30 | 0 | 14/44 | 0 | 0 | no |
| `@flighthq/compression` | generated | blocked | source-blocked | 3/4 | 6/6 | 0 | 3/4 | 0 | 1 | no |
| `@flighthq/connectivity` | generated | blocked | source-blocked | 2/3 | 10/14 | 4 | 2/2 | 0 | 2 | no |
| `@flighthq/debug` | generated | blocked | source-blocked | 3/4 | 10/10 | 0 | 1/1 | 1 | 1 | no |
| `@flighthq/device` | generated | emittable | compiled | 3/3 | 14/14 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/dialog` | generated | blocked | source-blocked | 2/3 | 12/15 | 3 | 5/5 | 0 | 2 | no |
| `@flighthq/easing` | generated | emittable | promoted | 22/23 | 52/54 | 2 | 4/7 | 0 | 0 | full |
| `@flighthq/effects` | generated | emittable | dependency-blocked | 74/74 | 153/153 | 0 | 5/5 | 2 | 0 | no |
| `@flighthq/effects-canvas` | host-backend | blocked | source-blocked | 11/26 | 63/73 | 15 | 1/1 | 5 | 16 | no |
| `@flighthq/effects-gl` | host-backend | blocked | source-blocked | 13/59 | 120/176 | 56 | 1/1 | 10 | 47 | no |
| `@flighthq/effects-wgpu` | host-backend | blocked | source-blocked | 13/57 | 116/169 | 53 | 1/1 | 5 | 45 | no |
| `@flighthq/entity` | generated | blocked | source-blocked | 5/8 | 12/15 | 3 | 30/74 | 0 | 4 | no |
| `@flighthq/filesystem` | generated | blocked | source-blocked | 2/3 | 40/43 | 3 | 2/2 | 0 | 2 | no |
| `@flighthq/flow` | generated | emittable | compiled | 3/3 | 9/9 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/font` | generated | blocked | source-blocked | 9/11 | 16/16 | 0 | 1/1 | 0 | 2 | no |
| `@flighthq/geolocation` | generated | blocked | source-blocked | 2/3 | 9/12 | 3 | 2/2 | 0 | 2 | no |
| `@flighthq/geometry` | generated | emittable | dependency-blocked | 28/28 | 377/377 | 0 | 44/62 | 0 | 0 | no |
| `@flighthq/glyphatlas` | generated | blocked | source-blocked | 7/10 | 18/18 | 0 | 1/1 | 0 | 3 | no |
| `@flighthq/haptics` | generated | emittable | compiled | 3/3 | 13/13 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/host-capacitor` | host-bound | host-bound | not-applicable | 0/0 | 0/13 | 13 | 0/0 | 0 | 0 | no |
| `@flighthq/host-electron` | host-bound | host-bound | not-applicable | 0/0 | 0/20 | 20 | 0/0 | 0 | 0 | no |
| `@flighthq/host-tauri` | host-bound | host-bound | not-applicable | 0/0 | 0/11 | 11 | 0/0 | 0 | 0 | no |
| `@flighthq/image` | generated | blocked | source-blocked | 3/5 | 20/20 | 0 | 14/28 | 1 | 2 | partial |
| `@flighthq/image-codec` | generated | blocked | source-blocked | 8/11 | 20/20 | 0 | 3/29 | 0 | 3 | no |
| `@flighthq/importdiagnostics` | generated | emittable | compile-blocked | 4/4 | 3/3 | 0 | 5/8 | 0 | 0 | no |
| `@flighthq/input` | generated | emittable | compile-blocked | 3/3 | 40/40 | 0 | 1/1 | 1 | 0 | partial |
| `@flighthq/interaction` | generated | blocked | source-blocked | 2/17 | 78/78 | 0 | 1/1 | 0 | 15 | no |
| `@flighthq/intl` | generated | blocked | source-blocked | 2/9 | 14/14 | 0 | 1/1 | 0 | 7 | no |
| `@flighthq/ipc` | generated | blocked | source-blocked | 2/3 | 13/17 | 4 | 2/2 | 0 | 2 | no |
| `@flighthq/keyboard` | generated | emittable | compiled | 3/3 | 20/20 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/layout` | generated | blocked | source-blocked | 5/8 | 8/8 | 0 | 1/1 | 0 | 3 | no |
| `@flighthq/lifecycle` | generated | emittable | compiled | 3/3 | 13/13 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/lighting` | generated | emittable | dependency-blocked | 13/13 | 31/31 | 0 | 5/8 | 0 | 0 | no |
| `@flighthq/loader` | generated | blocked | source-blocked | 2/3 | 15/15 | 0 | 3/3 | 0 | 1 | no |
| `@flighthq/log` | generated | blocked | source-blocked | 2/3 | 59/62 | 3 | 33/88 | 0 | 2 | no |
| `@flighthq/materials` | generated | emittable | dependency-blocked | 22/22 | 79/79 | 0 | 8/42 | 1 | 0 | no |
| `@flighthq/math` | generated | emittable | compiled | 17/17 | 73/73 | 0 | 9/12 | 0 | 0 | no |
| `@flighthq/media` | generated | blocked | source-blocked | 2/6 | 45/45 | 0 | 1/1 | 0 | 4 | no |
| `@flighthq/mediasession` | generated | blocked | source-blocked | 2/3 | 7/10 | 3 | 1/1 | 0 | 2 | no |
| `@flighthq/menu` | generated | blocked | source-blocked | 3/4 | 14/17 | 3 | 3/3 | 0 | 2 | no |
| `@flighthq/mesh` | generated | blocked | source-blocked | 12/15 | 83/83 | 0 | 7/24 | 1 | 3 | no |
| `@flighthq/motionpath` | generated | emittable | dependency-blocked | 3/3 | 7/7 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/movieclip` | generated | blocked | source-blocked | 2/5 | 28/28 | 0 | 2/2 | 0 | 3 | no |
| `@flighthq/net` | generated | blocked | source-blocked | 2/3 | 1/4 | 3 | 2/2 | 0 | 2 | no |
| `@flighthq/node` | generated | blocked | source-blocked | 8/20 | 125/125 | 0 | 29/41 | 0 | 12 | no |
| `@flighthq/notification` | generated | blocked | source-blocked | 2/3 | 21/26 | 5 | 4/4 | 0 | 2 | no |
| `@flighthq/particleemitter` | generated | emittable | dependency-blocked | 12/12 | 52/52 | 0 | 1/1 | 10 | 0 | no |
| `@flighthq/particles` | generated | emittable | dependency-blocked | 12/12 | 28/28 | 0 | 3/3 | 1 | 0 | no |
| `@flighthq/particles-formats` | generated | blocked | source-blocked | 4/16 | 31/31 | 0 | 1/1 | 0 | 12 | no |
| `@flighthq/path` | generated | blocked | source-blocked | 28/30 | 73/60 | 0 | 13/19 | 0 | 2 | no |
| `@flighthq/path-boolean` | generated | blocked | source-blocked | 8/9 | 12/12 | 1 | 2/4 | 0 | 2 | no |
| `@flighthq/path-formats` | generated | emittable | dependency-blocked | 3/3 | 3/3 | 0 | 2/4 | 0 | 0 | no |
| `@flighthq/permissions` | generated | blocked | source-blocked | 2/4 | 7/11 | 4 | 1/1 | 0 | 3 | no |
| `@flighthq/physics2d` | generated | blocked | source-blocked | 15/21 | 96/83 | 0 | 1/1 | 0 | 6 | no |
| `@flighthq/picking` | generated | blocked | source-blocked | 2/4 | 11/11 | 0 | 1/1 | 0 | 2 | no |
| `@flighthq/platform` | generated | emittable | compiled | 3/3 | 16/16 | 0 | 3/3 | 1 | 0 | no |
| `@flighthq/power` | generated | blocked | source-blocked | 2/3 | 14/19 | 5 | 2/2 | 0 | 2 | partial |
| `@flighthq/protocol` | generated | emittable | compile-blocked | 3/3 | 20/20 | 0 | 2/2 | 1 | 0 | no |
| `@flighthq/quadbatch` | generated | blocked | source-blocked | 2/3 | 31/31 | 0 | 2/3 | 0 | 1 | no |
| `@flighthq/render` | generated | blocked | source-blocked | 7/23 | 61/73 | 12 | 11/17 | 0 | 17 | no |
| `@flighthq/render-gl` | host-backend | blocked | source-blocked | 28/32 | 113/115 | 6 | 5/5 | 22 | 5 | no |
| `@flighthq/render-wgpu` | host-backend | blocked | source-blocked | 21/27 | 99/108 | 9 | 5/5 | 16 | 7 | no |
| `@flighthq/scene2d` | generated | blocked | source-blocked | 2/9 | 25/33 | 8 | 18/29 | 0 | 8 | no |
| `@flighthq/scene2d-canvas` | host-backend | blocked | source-blocked | 37/41 | 109/119 | 10 | 5/5 | 29 | 5 | no |
| `@flighthq/scene2d-dom` | host-bound | host-bound | not-applicable | 0/0 | 0/61 | 61 | 1/1 | 0 | 0 | no |
| `@flighthq/scene2d-formats` | generated | blocked | source-blocked | 6/23 | 16/11 | 2 | 2/3 | 0 | 18 | no |
| `@flighthq/scene2d-gl` | host-backend | blocked | source-blocked | 27/32 | 90/97 | 9 | 1/1 | 17 | 6 | no |
| `@flighthq/scene2d-resources` | generated | blocked | source-blocked | 3/10 | 14/14 | 0 | 2/2 | 0 | 7 | no |
| `@flighthq/scene2d-wgpu` | host-backend | blocked | source-blocked | 27/32 | 91/99 | 9 | 1/1 | 18 | 6 | no |
| `@flighthq/scene3d` | generated | blocked | source-blocked | 3/18 | 40/40 | 0 | 7/7 | 0 | 15 | no |
| `@flighthq/scene3d-formats` | generated | blocked | source-blocked | 19/27 | 143/39 | 0 | 2/2 | 0 | 8 | no |
| `@flighthq/scene3d-gl` | host-backend | emittable | dependency-blocked | 65/65 | 182/176 | 0 | 1/1 | 60 | 0 | no |
| `@flighthq/scene3d-resources` | generated | blocked | source-blocked | 14/28 | 49/49 | 0 | 1/1 | 0 | 14 | no |
| `@flighthq/scene3d-wgpu` | host-backend | blocked | source-blocked | 46/49 | 129/157 | 32 | 1/1 | 36 | 4 | no |
| `@flighthq/screen` | generated | emittable | compiled | 3/3 | 31/31 | 0 | 2/2 | 0 | 0 | partial |
| `@flighthq/sdk` | generated | blocked | source-blocked | 15/15 | 0/6295 | 6295 | 0/0 | 0 | 1 | no |
| `@flighthq/sensors` | generated | blocked | source-blocked | 2/3 | 22/32 | 10 | 1/1 | 0 | 2 | no |
| `@flighthq/shading` | generated | blocked | source-blocked | 15/18 | 26/26 | 0 | 4/5 | 0 | 3 | no |
| `@flighthq/shape` | generated | blocked | source-blocked | 9/15 | 68/63 | 0 | 9/11 | 0 | 6 | no |
| `@flighthq/shape-formats` | generated | blocked | source-blocked | 2/3 | 2/2 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/share` | generated | blocked | source-blocked | 2/3 | 12/15 | 3 | 2/2 | 0 | 2 | no |
| `@flighthq/shell` | generated | blocked | source-blocked | 2/3 | 11/14 | 3 | 3/3 | 0 | 2 | no |
| `@flighthq/shortcut` | generated | blocked | source-blocked | 2/5 | 29/33 | 4 | 3/3 | 0 | 4 | no |
| `@flighthq/signals` | generated | emittable | compiled | 7/7 | 12/11 | 0 | 50/110 | 0 | 0 | partial |
| `@flighthq/skeleton2d` | generated | blocked | source-blocked | 17/23 | 55/55 | 0 | 3/5 | 0 | 6 | no |
| `@flighthq/skeleton2d-formats` | generated | blocked | source-blocked | 4/8 | 7/7 | 1 | 1/1 | 0 | 5 | no |
| `@flighthq/skeleton3d` | generated | blocked | source-blocked | 5/9 | 21/21 | 0 | 2/3 | 0 | 4 | no |
| `@flighthq/snapshot` | generated | blocked | source-blocked | 6/7 | 7/7 | 0 | 1/1 | 4 | 1 | no |
| `@flighthq/socket` | generated | blocked | source-blocked | 3/5 | 16/16 | 0 | 1/1 | 0 | 2 | no |
| `@flighthq/spatial` | generated | emittable | dependency-blocked | 6/6 | 14/14 | 0 | 3/3 | 0 | 0 | no |
| `@flighthq/spring` | generated | emittable | compiled | 6/6 | 12/12 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/spritesheet` | generated | emittable | dependency-blocked | 9/9 | 31/31 | 0 | 2/2 | 4 | 0 | no |
| `@flighthq/spritesheet-formats` | generated | blocked | source-blocked | 10/12 | 17/17 | 0 | 1/1 | 6 | 2 | no |
| `@flighthq/statechart` | generated | blocked | source-blocked | 3/5 | 15/15 | 0 | 1/1 | 0 | 2 | no |
| `@flighthq/statusbar` | generated | blocked | source-blocked | 2/3 | 14/18 | 4 | 2/2 | 0 | 2 | no |
| `@flighthq/storage` | generated | blocked | source-blocked | 2/3 | 36/39 | 3 | 2/2 | 0 | 2 | no |
| `@flighthq/swf` | generated | blocked | source-blocked | 3/13 | 18/11 | 0 | 1/1 | 0 | 10 | no |
| `@flighthq/text` | generated | blocked | source-blocked | 6/7 | 83/86 | 3 | 10/12 | 3 | 2 | no |
| `@flighthq/text-markup` | generated | blocked | source-blocked | 5/6 | 8/8 | 0 | 2/2 | 1 | 1 | no |
| `@flighthq/textbidi` | generated | emittable | compiled | 6/6 | 6/6 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/textinput` | generated | emittable | dependency-blocked | 6/6 | 55/55 | 0 | 5/6 | 4 | 0 | no |
| `@flighthq/textlayout` | generated | emittable | dependency-blocked | 14/14 | 47/47 | 0 | 10/14 | 1 | 0 | no |
| `@flighthq/textsegment` | generated | emittable | compiled | 5/5 | 11/11 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/textshaper` | generated | blocked | source-blocked | 7/11 | 33/33 | 2 | 3/15 | 0 | 5 | no |
| `@flighthq/textshaper-canvas` | host-backend | emittable | dependency-blocked | 3/3 | 2/2 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/texture` | generated | blocked | source-blocked | 6/9 | 52/52 | 0 | 18/38 | 0 | 3 | no |
| `@flighthq/texture-formats` | generated | blocked | source-blocked | 5/11 | 13/9 | 0 | 1/1 | 0 | 6 | no |
| `@flighthq/textureatlas` | generated | blocked | source-blocked | 6/7 | 37/37 | 0 | 8/14 | 1 | 1 | no |
| `@flighthq/textureatlas-formats` | generated | blocked | source-blocked | 5/7 | 10/10 | 0 | 2/2 | 1 | 2 | no |
| `@flighthq/tilemap` | generated | blocked | source-blocked | 2/3 | 22/22 | 0 | 2/2 | 0 | 1 | no |
| `@flighthq/tilemap-formats` | generated | blocked | source-blocked | 5/9 | 12/12 | 0 | 1/1 | 0 | 4 | no |
| `@flighthq/timeline` | generated | emittable | compile-blocked | 3/3 | 19/19 | 0 | 2/3 | 0 | 0 | no |
| `@flighthq/tool-capture` | excluded | excluded | not-applicable | 0/0 | 0/132 | 132 | 0/0 | 0 | 0 | no |
| `@flighthq/tray` | generated | blocked | source-blocked | 2/4 | 28/28 | 0 | 3/3 | 0 | 2 | no |
| `@flighthq/tween` | generated | blocked | source-blocked | 7/10 | 29/28 | 0 | 2/2 | 0 | 3 | no |
| `@flighthq/types` | generated | emittable | promoted | 726/860 | 2260/2260 | 0 | 142/142 | 0 | 0 | full |
| `@flighthq/updater` | generated | blocked | source-blocked | 2/3 | 23/23 | 0 | 2/2 | 0 | 1 | no |
| `@flighthq/useragent` | generated | emittable | compiled | 4/4 | 12/12 | 0 | 3/6 | 1 | 0 | no |
| `@flighthq/velocity` | generated | emittable | dependency-blocked | 5/5 | 20/20 | 0 | 3/3 | 2 | 0 | no |
| `@flighthq/video` | generated | blocked | source-blocked | 4/5 | 16/16 | 0 | 2/2 | 1 | 1 | no |
| `@flighthq/webcam` | generated | blocked | source-blocked | 2/4 | 5/9 | 4 | 1/1 | 0 | 3 | no |
| `@flighthq/xml` | generated | emittable | compile-blocked | 4/4 | 6/6 | 0 | 6/8 | 2 | 0 | no |

## Async tasks

Construction disposition partition: 225 eligible = 19 portable executable + 0 host placeholder + 206 unsupported.

Disposition partition: 173 eligible = 13 portable executable + 0 host placeholder + 160 unsupported.

| Operation | Count |
| --- | ---: |
| Await expressions | 205 |
| Async iterations | 3 |
| Promise.all | 3 |
| Promise.allSettled | 3 |
| Promise.resolve | 0 |
| Promise.reject | 0 |
| Promise.then | 1 |
| Promise.catch | 1 |
| Promise.finally | 0 |
| Void expressions | 4 |

| Package | Constructions eligible/executable/host/unsupported | Scopes eligible/executable/host/unsupported | Legacy erasure path |
| --- | ---: | ---: | ---: |
| `@flighthq/app` | 2/0/0/2 | 0/0/0/0 | 0 |
| `@flighthq/application` | 4/4/0/0 | 0/0/0/0 | 0 |
| `@flighthq/assets` | 6/0/0/6 | 1/0/0/1 | 1 |
| `@flighthq/audio` | 7/0/0/7 | 7/0/0/7 | 7 |
| `@flighthq/clipboard` | 22/0/0/22 | 22/0/0/22 | 1 |
| `@flighthq/connectivity` | 2/0/0/2 | 2/0/0/2 | 1 |
| `@flighthq/dialog` | 9/0/0/9 | 7/0/0/7 | 3 |
| `@flighthq/filesystem` | 38/0/0/38 | 38/0/0/38 | 12 |
| `@flighthq/font` | 11/8/0/3 | 11/8/0/3 | 11 |
| `@flighthq/geolocation` | 4/0/0/4 | 2/0/0/2 | 0 |
| `@flighthq/image` | 5/0/0/5 | 5/0/0/5 | 5 |
| `@flighthq/image-codec` | 5/3/0/2 | 5/3/0/2 | 3 |
| `@flighthq/input` | 3/2/0/1 | 0/0/0/0 | 0 |
| `@flighthq/ipc` | 2/0/0/2 | 0/0/0/0 | 0 |
| `@flighthq/loader` | 4/0/0/4 | 2/0/0/2 | 2 |
| `@flighthq/media` | 2/0/0/2 | 0/0/0/0 | 0 |
| `@flighthq/menu` | 1/0/0/1 | 0/0/0/0 | 0 |
| `@flighthq/net` | 3/0/0/3 | 3/0/0/3 | 2 |
| `@flighthq/notification` | 16/0/0/16 | 16/0/0/16 | 0 |
| `@flighthq/permissions` | 9/0/0/9 | 8/0/0/8 | 8 |
| `@flighthq/power` | 7/0/0/7 | 0/0/0/0 | 0 |
| `@flighthq/render-wgpu` | 7/0/0/7 | 4/0/0/4 | 4 |
| `@flighthq/scene2d-resources` | 9/0/0/9 | 5/0/0/5 | 3 |
| `@flighthq/scene3d-resources` | 19/0/0/19 | 13/0/0/13 | 13 |
| `@flighthq/screen` | 4/2/0/2 | 2/2/0/0 | 2 |
| `@flighthq/sensors` | 2/0/0/2 | 2/0/0/2 | 1 |
| `@flighthq/share` | 4/0/0/4 | 3/0/0/3 | 1 |
| `@flighthq/shell` | 9/0/0/9 | 8/0/0/8 | 0 |
| `@flighthq/storage` | 1/0/0/1 | 1/0/0/1 | 1 |
| `@flighthq/textureatlas` | 4/0/0/4 | 4/0/0/4 | 4 |
| `@flighthq/video` | 3/0/0/3 | 1/0/0/1 | 1 |
| `@flighthq/webcam` | 1/0/0/1 | 1/0/0/1 | 0 |

### Unsupported task constructions

- `@flighthq/app` `upstream/packages/app/src/app.ts:252:18` `createWebAppBackend.subscribeReady.id.then:252:18:1252317c8253` `then` (sha256:1252317c8253b97431cdd80643b0e94cc96694f61629466c7069f3a23557cd44): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/app` `upstream/packages/app/src/app.ts:252:18` `createWebAppBackend.subscribeReady.id.ready:252:18:9b8874eb7ffe` `ready` (sha256:9b8874eb7ffebcf300217391f3fb4a9d53d078bccf4ed67e3253de0dd1f14116): Portable task Rust lowering is not implemented.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:32:12` `acquireAsset.reject:32:12:c1bd098683aa` `reject` (sha256:c1bd098683aa6463478400329d6117bcc8118e92b9673a7ad2fc8f550a8626ce): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:37:12` `acquireAsset.reject:37:12:dd550e8fb233` `reject` (sha256:dd550e8fb2330f0c8ec7878760e1315348dd41dc602a767f968ff211b05d3361): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:43:35` `acquireAsset.ready:43:35:214b55bb04b3` `ready` (sha256:214b55bb04b3e86f9178fbdccfafd0397bff3632f9f1d66fb98aa49194e1b6d8): Portable task Rust lowering is not implemented.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:51:23` `acquireAsset.loadPromise.then:51:23:b3c9a19b076b` `then` (sha256:b3c9a19b076b818a81e48bea3b5d68375f1f009765468e3247eb00997296e799): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:141:1` `loadAssetGroup` `async-scope` (sha256:188d51024cb73e45ac54868edd4ae13e87859c658cbe0a8476dbe34b01d289ec): Portable task Rust lowering is not implemented.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:178:23` `loadAssetGroup.settlements.join-all-settled:178:23:0292ceef1e91` `join-all-settled` (sha256:0292ceef1e91d5acf9fc744e917e3cb8249d560087fdc92c345aa882b5feacbd): taskAllSettled Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:24:1` `loadAudioResourceFromBase64` `async-scope` (sha256:f62565721d1676f6b065becc6e2abdd9e41b9d327d26c08664159eaf20e4e9fc): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:36:1` `loadAudioResourceFromBlob` `async-scope` (sha256:6f31ecce9820d5f8d705caee039a2579bafeb3dca51a82e47de6c043e688642e): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:48:1` `loadAudioResourceFromBytes` `async-scope` (sha256:4ab485113a101f105f72060f521838ed60247dc4537ffb1961fe8b48664b6f1a): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:66:1` `loadAudioResourceFromUrl` `async-scope` (sha256:9afb9a9b90a12c12d7e56e320707532002a913be2e722b67c214e9da4246faa9): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:88:1` `loadAudioResourceFromUrls` `async-scope` (sha256:f704e57788fe00ece2e8f7e0e8f89db8a63e3a17ca93c092fb6d1adf17df4c64): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceReference.ts:110:1` `resolveAudioResourceReference` `async-scope` (sha256:427972d435dc38d25124a3ebca0e20385d9e78482f6e48c5352604cd213443e2): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceReference.ts:144:1` `decodeAudioResourceBytes` `async-scope` (sha256:3fb0a202462a960a2221ff6707ce03dc96eccc7968aa21b762d78bb652c31f2e): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:31:5` `createWebClipboardBackend.readFormat` `async-scope` (sha256:721317f182624551290a66cbbc88a82fb2c213c31c84039b3fbf7947bf397856): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:48:5` `createWebClipboardBackend.writeFormat` `async-scope` (sha256:06ec8c11ce5ed20c71af72322b5fe3d22c25749c4ee27b4fcffade76b202ef87): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:59:5` `createWebClipboardBackend.hasFormat` `async-scope` (sha256:0fe5c7789f000064e04648dbb2831f298a8726cae24bda2ef43f63b780a7eea6): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:63:5` `createWebClipboardBackend.getFormats` `async-scope` (sha256:ed75f7ccdf54735fec684a6a6ee69c47e91c5a0d3591169be8970cc8ad027266): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:79:5` `createWebClipboardBackend.writeItems` `async-scope` (sha256:bfe2ca1744d74e460bbb99f43395a8ad2eec6a0ed51212254ec322e9c93392e1): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:93:5` `createWebClipboardBackend.readItems` `async-scope` (sha256:04db96d1813aada66ce1c55a363f56cbd16734202877dcbaf231786cfe34c32b): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:112:5` `createWebClipboardBackend.readText` `async-scope` (sha256:1a5b9a58c3f16317df3446c1a66ef5114db31b5f6ee12f004359ae970ddb5f3e): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:121:5` `createWebClipboardBackend.writeText` `async-scope` (sha256:23eab1975987cd5de1f5dd408ae4fb2db6a4451dfebdc78c4ff5dcbf971da580): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:131:5` `createWebClipboardBackend.readHtml` `async-scope` (sha256:1a6c1714ae6b501d2713b397c6d79dc8049a767e5744a291718c2af37568fc6b): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:134:5` `createWebClipboardBackend.writeHtml` `async-scope` (sha256:5eb7aee33fabdabdf097fc74dbd79968d9b293cb97e42af1d1ea83d6c9a25060): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:137:5` `createWebClipboardBackend.hasText` `async-scope` (sha256:1d5b1f5d4457bd324170aa1e3866a3787d5c0b8719f36e18b9f20efb61c5411b): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:140:5` `createWebClipboardBackend.readImage` `async-scope` (sha256:51f9ad208bf3a315668eff8f6223c0c11e1b28431226790533ba02ed5d042d36): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:157:5` `createWebClipboardBackend.writeImage` `async-scope` (sha256:0d28b5751be206dc8174fd8d29ee8931217607ccc2bb598c881854d5574a5714): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:169:5` `createWebClipboardBackend.hasImage` `async-scope` (sha256:2a2c1863a814225bd6ae7583d0f099723f904138492d03cedea7727e7dc7e70e): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:172:5` `createWebClipboardBackend.readRTF` `async-scope` (sha256:a0f27e9f28118b12321c83624453b98f87dad2cfef71f630bb6c5e0c4131a165): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:175:5` `createWebClipboardBackend.writeRTF` `async-scope` (sha256:0e54cef02fdd6872e0a775fd27e741fc1bf97a54e7a9c7d8755a621a78e53a75): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:179:5` `createWebClipboardBackend.readBookmark` `async-scope` (sha256:78016ab7f76a0108d52c3f3ccee3c40d7161b80bb66cc8860faec4d5cc99f115): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:183:5` `createWebClipboardBackend.writeBookmark` `async-scope` (sha256:8cc982a2d45bee016f5baa8b6fa524db87eb0302c67f1a30e1fd4bbfc3ef66f4): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:187:5` `createWebClipboardBackend.readFiles` `async-scope` (sha256:540866183a941bfea5b304a70d7bff5ac1c2aa80da47e31cf2e894c29cde0ce6): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:191:5` `createWebClipboardBackend.writeFiles` `async-scope` (sha256:ff06a60769d4aece46e2761c73660884fed44831ad41a0b3903a1ba9f0b3c85d): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:194:5` `createWebClipboardBackend.clear` `async-scope` (sha256:34f107c7453a7d6158d4388d4f5c200844605acd824e1bc06de7f332dad9ee7c): Portable task Rust lowering is not implemented.
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:364:1` `blobFromFormatData` `async-scope` (sha256:c1fb9e0ee718a360876a579fe9e1e5fdc16157f03db3dabc983b233c321466dd): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/connectivity` `upstream/packages/connectivity/src/connectivity.ts:82:5` `createWebConnectivityBackend.detectReachability` `async-scope` (sha256:694076bc4efb1a4cfa0e975d140e8cf6f2435a0fdd257300be3a958e3a2b5015): Portable task Rust lowering is not implemented.
- `@flighthq/connectivity` `upstream/packages/connectivity/src/connectivity.ts:138:1` `detectConnectivityReachability` `async-scope` (sha256:599d1031e281368c595d7c67f9094c5325c5caf880d596a64a3c5d592178e52b): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:19:5` `createWebDialogBackend.confirm` `async-scope` (sha256:9e0d435b7b04504a82aec0618dbb15c78815dffab42978a68f29044f5e6fcf78): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:28:5` `createWebDialogBackend.message` `async-scope` (sha256:7c2e4513e6b2c9853b0c5bbf6a3f56fc25557127cde7c98c54a42e7b3820a77d): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:48:5` `createWebDialogBackend.prompt` `async-scope` (sha256:c2ef3f3e409d36b6f50d673164d87f3da10c9811b4725fa58da7f7886e5b23b3): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:56:5` `createWebDialogBackend.saveFile` `async-scope` (sha256:40414e87f9a0df2dbf9669cc34214ffee390112a4912d6d8a9648f7be2102d80): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:210:12` `openWebDirectoryDialog.ready:210:12:24ebc2b22456` `ready` (sha256:24ebc2b22456a00e7226f082338fe750ae6c68db4f80640c991ac703374932d3): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:251:1` `openDirectoryPickerAccessApi` `async-scope` (sha256:b899e4b9db2555d6330dd9e3cdf86a011e79a2f0355b05f846f7c7204dbae732): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:289:12` `openWebFileDialog.ready:289:12:24ebc2b22456` `ready` (sha256:24ebc2b22456a00e7226f082338fe750ae6c68db4f80640c991ac703374932d3): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:324:1` `openFileSystemAccessPicker` `async-scope` (sha256:0a2092597843117736235fd619c2d4f395e5ff517d455c075d5948fee291c1cc): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:354:1` `saveWebFile` `async-scope` (sha256:e88949abb7c69f2698087c43c21543a34eaf2427cac71f67fccf397a0a8a7156): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:41:5` `createWebFileSystemBackend.readTextFile` `async-scope` (sha256:574171c1f78071a47b51fc4530a31a20df6cd2cef77438662dc248915ce18ff2): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:51:5` `createWebFileSystemBackend.writeTextFile` `async-scope` (sha256:eea5320063d5585ee7e7ec9354a30cd9217902159a9cfb617cfa9a2f088e6d9f): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:54:5` `createWebFileSystemBackend.readBinaryFile` `async-scope` (sha256:f8849adeb4063a8e366324baba2120d2e6789cec1ef89017cd25d86fe4063b0f): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:64:5` `createWebFileSystemBackend.readBinaryFileRange` `async-scope` (sha256:51679cf17403994237bec53b329aa9c371966174d4596140e957ed5656df9679): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:76:5` `createWebFileSystemBackend.writeBinaryFile` `async-scope` (sha256:a008f81798eccc1003aed0d9e903d42a763a22ea9cd9ea75ec66fef4e8c70011): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:81:5` `createWebFileSystemBackend.fileExists` `async-scope` (sha256:16edd4e2eeb397246605858dc6cc531b8e7375a6f7016a31bea39fdb86ed6f67): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:84:5` `createWebFileSystemBackend.directoryExists` `async-scope` (sha256:f46b834d739fc1ccecafc9af12e6d5f2ff98579406b192a7245d142e385fc659): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:89:5` `createWebFileSystemBackend.removeFile` `async-scope` (sha256:140b01563d3776aae563c30f2d6760391ae4a5e2a21c66732e9debd4e04740ac): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:92:5` `createWebFileSystemBackend.removeDirectory` `async-scope` (sha256:fdc2708c099a643b3d6cefa1ca4f560bcd3ca77f73e5351cb920d88a5f3889fb): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:107:5` `createWebFileSystemBackend.makeDirectory` `async-scope` (sha256:6f51bdc0b4150ea205900967a9b753947a8d07e2d2d5fe4cf4704de8ed954374): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:113:5` `createWebFileSystemBackend.readDirectory` `async-scope` (sha256:cf39347f679c2986d33af6374395320703ca8ec00f38d53648d13ef59b366b2a): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:130:5` `createWebFileSystemBackend.readDirectoryRecursive` `async-scope` (sha256:b9273fc96e399400fa069fdda02be9cdddc16bf6435aed16513e46684d4fcbd9): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:144:5` `createWebFileSystemBackend.statFile` `async-scope` (sha256:9f3ca243190b7dd20090bc636e833fcc518d4d27da3a62f50d9b0bf56343987c): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:167:5` `createWebFileSystemBackend.rename` `async-scope` (sha256:9e3228d2cb150076642cde946d5c8e0af12df36ddafd11d6f5d3a888fae1db64): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:172:5` `createWebFileSystemBackend.copy` `async-scope` (sha256:a8ab38a19dcb5e3fb20e8102c6231c614cbd4b237b975dc68a3094654c1c3ea8): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:185:5` `createWebFileSystemBackend.appendTextFile` `async-scope` (sha256:6b213fd0d23492209d078e56f40ac8ad6bb25bf23fdf0fcb61837aa18e7e5828): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:197:5` `createWebFileSystemBackend.openFileReadStream` `async-scope` (sha256:397d36c122ccf42160fd8c8d0b251d4e415f2caa9dc6a8e6d752f9d589e3b93a): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:207:5` `createWebFileSystemBackend.openFileWriteStream` `async-scope` (sha256:a1398c0c934e0b064752d3e88b8faf3fcceeb22c332ef48a527eebbcfe690500): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:216:5` `createWebFileSystemBackend.writeFileAtomic` `async-scope` (sha256:08338d01a7496bfb863bafb34ad143731f1825c05a980b1206a57d98ea8cb7a7): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:238:5` `createWebFileSystemBackend.createFileSymlink` `async-scope` (sha256:a7ec0965827a98018c26d6d389d77b43f2207b530026a721030f9711ce2fd2ba): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:242:5` `createWebFileSystemBackend.readFileSymlink` `async-scope` (sha256:f9aa63b08e2eb58e164feec37dc2c5e405773c1bfa1bc83cabe221db963a7c3e): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:246:5` `createWebFileSystemBackend.getFileRealPath` `async-scope` (sha256:c7c21015d70a979da0d7a21948807c11791069f7a6d31e5cedec80054c726aac): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:250:5` `createWebFileSystemBackend.getFilePermissions` `async-scope` (sha256:d5a528fd94e5ebdc23f72af0bda23fa3a3ff8f759a4785bf758cd588aae07224): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:254:5` `createWebFileSystemBackend.setFilePermissions` `async-scope` (sha256:a896a24d74069b5626a25ac32214db778d30f118359a34a1ccc8449cb63cf180): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:258:5` `createWebFileSystemBackend.canAccessFile` `async-scope` (sha256:044040117b7c5b060026f24b144f5a0799780f74f04d04c05e0fa674b7aea53c): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:278:5` `createWebFileSystemBackend.getFileSystemUsage` `async-scope` (sha256:415ecf42bf14245ab608395d0bc204940d14442436e0f4a855c24c92e1669a52): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:318:1` `findFiles` `async-scope` (sha256:086ee8f33aa2fc15cd2242768128432c33d546e7e1e07f6f180f639c3e741a06): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:437:1` `readDialogHandleBinaryFile` `async-scope` (sha256:168933b0fd5d152de69e33d8106ef967a6b88f971892b802c6682be962f5a682): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:459:1` `readDialogHandleTextFile` `async-scope` (sha256:ef1c5426a4503d00e890c0f0fa60cbb140d257532ba380cb12fc5086ed838cbd): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:548:1` `writeBinaryFileChunks` `async-scope` (sha256:3edaf0a0922e0acc7f59a89e6ff05bd4c376e657535b29c66f6e6b1607afc8df): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:571:1` `writeDialogHandleBinaryFile` `async-scope` (sha256:80e53fef3f1f97ef52a3ea87e4d071bda65a76fe2579d937716cab0aa01db411): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:594:1` `writeDialogHandleTextFile` `async-scope` (sha256:9080a9844f1f96cb71579d9cd83e47aff5e9c4ea4e145b7aed073f0411e8a076): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:625:1` `getWebRoot` `async-scope` (sha256:ef7f3d7015038a12b48081190128011dea066ed8d640e163d354dccba07a461f): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:636:1` `getWebDirectoryHandle` `async-scope` (sha256:7f0c9697a8fda8bf983894519de724aca66a1a9c03e449ffe09ddd4ae9c63f92): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:652:1` `getWebFileHandle` `async-scope` (sha256:81a545005c91ec665cdc44f2beb861a1922b104d011fcedd756fb8897428ae65): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:668:1` `walkWebDirectory` `async-scope` (sha256:6d1216dc9c5354ba256755b0451e1d3789557508dda88892e626011b93807eb4): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:687:1` `writeWebRemove` `async-scope` (sha256:1841fd4e7778e7dd8472360d0552f739a00f488b0961c21213eab5deafe197cf): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:707:1` `writeWebFile` `async-scope` (sha256:6e15f5efb9909a3d0e7c991013ac6cdfd764a902ed4e16f226efb7f32676e172): Portable task Rust lowering is not implemented.
- `@flighthq/font` `upstream/packages/font/src/_fontFaceLoad.ts:6:1` `_loadFontFaceFromBytes` `async-scope` (sha256:1c96fe99227e32ac4c4cb8b9fe7ba660176b30f1b73e3fced9750ce6c9208f93): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/font` `upstream/packages/font/src/_fontFaceLoad.ts:29:1` `loadAndRegisterFontFace` `async-scope` (sha256:9146827c6b34222cda06746f591036a43d1cc4fde6318dc70d182b561931028d): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/font` `upstream/packages/font/src/fontStatus.ts:7:1` `whenFontsReady` `async-scope` (sha256:a9a632e81b2b401d128497ca7e36252f671e86a84e7380d7ffa2915b10717660): Portable task Rust lowering is not implemented.
- `@flighthq/geolocation` `upstream/packages/geolocation/src/geolocation.ts:80:5` `createWebGeolocationBackend.getPermission` `async-scope` (sha256:589424ae8a82b30bc0a05473e652fce3ea1e3d8dbf9d62047dec8eea751d9252): Portable task Rust lowering is not implemented.
- `@flighthq/geolocation` `upstream/packages/geolocation/src/geolocation.ts:92:5` `createWebGeolocationBackend.requestPermission` `async-scope` (sha256:c1d7c01868b81a6d6a24327b828b629c16b4760c6743b681bf2244d5fc64db43): Portable task Rust lowering is not implemented.
- `@flighthq/geolocation` `upstream/packages/geolocation/src/geolocation.ts:109:7` `createWebGeolocationBackend.subscribePermission.catch:109:7:1912fe908b14` `catch` (sha256:1912fe908b144e830d91a5460e49b68782f2d05cc475f706f410f2f64d8cfd4b): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/geolocation` `upstream/packages/geolocation/src/geolocation.ts:109:7` `createWebGeolocationBackend.subscribePermission.then:109:7:94366eb68bcc` `then` (sha256:94366eb68bccc368aac0b7d29a15142f8993f5806fe9d17ea272a0f8f2be0ca2): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:65:1` `loadImageResourceFromBase64` `async-scope` (sha256:a73bde2432a402dd8c65c2d52b8994241855cc78ee83d75be142db0c50287d8a): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:73:1` `loadImageResourceFromBlob` `async-scope` (sha256:97f7ece284c5aa823b9978fb24baf949ec2100662fcec48c12d8ccb2fe3260c5): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:82:1` `loadImageResourceFromBytes` `async-scope` (sha256:f2c96b3490cc8b7d2a8435a1ac35303a3806e4548d2edb8856c7f7eb6e6fef3a): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:95:1` `loadImageResourceFromUrl` `async-scope` (sha256:71b688be69653eee059a72041402f11b74e14997cd4fab3120696e48a52ae882): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceReference.ts:89:1` `resolveImageResourceReference` `async-scope` (sha256:7596b9859812ac08a756c394cc2cab4b589bc0542bc55822e22a5077a562f410): Portable task Rust lowering is not implemented.
- `@flighthq/image-codec` `upstream/packages/image-codec/src/registerWebImageDecoders.ts:18:45` `decodeImageWithCanvas` `async-scope` (sha256:6c70f189d2b99fef85a6c33991e3174b6e7cfc147486162cdd1abee7f40fc943): Portable task Rust lowering is not implemented.
- `@flighthq/image-codec` `upstream/packages/image-codec/src/registerWebImageEncoders.ts:16:10` `createCanvasImageEncoder.anonymous:7c4dbd1c1e56` `async-scope` (sha256:7c4dbd1c1e56dfb5d704a95587bc335ef9c42e20340743d59daacfb0eda450d3): Portable task Rust lowering is not implemented.
- `@flighthq/input` `upstream/packages/input/src/inputManager.ts:698:14` `requestInputPointerLock.then:698:14:979622d7dcaf` `then` (sha256:979622d7dcaf403ca7e08ea1127034ff6852f8056b49a75c65fba7a4f4149e07): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/ipc` `upstream/packages/ipc/src/ipc.ts:59:14` `createWebIpcBackend.invoke.ready:59:14:1c5c5ba35766` `ready` (sha256:1c5c5ba357669af58be4ccb70277a838ecedb8770bd77407ef56bfc2cdaf8630): Portable task Rust lowering is not implemented.
- `@flighthq/ipc` `upstream/packages/ipc/src/ipc.ts:124:5` `invokeIpcWithTimeout.timeout.then:124:5:e07d69e12e57` `then` (sha256:e07d69e12e57d3a3c25f40b1e1f795590bc5e444c9675b158a9bdaafb6f323eb): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:78:10` `_noopLoad.ready:78:10:1c5c5ba35766` `ready` (sha256:1c5c5ba357669af58be4ccb70277a838ecedb8770bd77407ef56bfc2cdaf8630): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:502:1` `drainQueue` `async-scope` (sha256:88082b65b0b72d5a7051045ab407875a77a7430ece259e4bf911600ef6289ed7): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:552:1` `runEntry` `async-scope` (sha256:0cc4fef1fe859a3eba70abead965e6eb2ab15fc8f1671ae50442eeba76acdea5): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:772:23` `delay.ready:772:23:9b8874eb7ffe` `ready` (sha256:9b8874eb7ffebcf300217391f3fb4a9d53d078bccf4ed67e3253de0dd1f14116): Portable task Rust lowering is not implemented.
- `@flighthq/media` `upstream/packages/media/src/audioChannel.ts:184:5` `startAudioChannel.catch:184:5:4bba768dc182` `catch` (sha256:4bba768dc1829657001ce75f0cd08a229575a08088e325f7fd2ff2a2580bcbf5): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/media` `upstream/packages/media/src/videoChannel.ts:143:3` `startVideoChannel.catch:143:3:54047f8e30cd` `catch` (sha256:54047f8e30cd1c74054508775a0324f1c77b7c4ef152a3f775dc36ae697630b4): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/menu` `upstream/packages/menu/src/menu.ts:103:10` `showContextMenu.then:103:10:99862c2ab561` `then` (sha256:99862c2ab561446ce8c047db7fb8d540a8e7ad1c9d5c6c984f01c4eb7aa0faeb): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/net` `upstream/packages/net/src/net.ts:21:5` `createWebNetBackend.sendNetRequest` `async-scope` (sha256:95718bdeab605b5c428496aed0ad940a09e148c6ffd1b690723a621d3089c453): Portable task Rust lowering is not implemented.
- `@flighthq/net` `upstream/packages/net/src/net.ts:110:1` `_readNetResponseBody` `async-scope` (sha256:a65ec98271b1315b2d9f92bf7f23f3862fc0a2eefa19665fa73e7818af9bfc60): Portable task Rust lowering is not implemented.
- `@flighthq/net` `upstream/packages/net/src/net.ts:133:1` `_readNetResponseWithProgress` `async-scope` (sha256:b695935644b73d37aea7c2af6c5ee8129d40d1a01cb334357c027ceadca38081): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:81:3` `createServiceWorkerNotificationBackend._show` `async-scope` (sha256:e00200853c5c516d6fb8f492b4d4a4936ad578f62c3c4c064339bd5fb6a4d072): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:119:5` `createServiceWorkerNotificationBackend.backend.closeAllNotifications` `async-scope` (sha256:52fa18d11686282d426bb5c4c47630bb80135c1b9e903717735b1eed8307c577): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:130:5` `createServiceWorkerNotificationBackend.backend.closeNotification` `async-scope` (sha256:6517e4505bd2c041ac811f8a05ba8353749846f946f30ac5ddfb2f22f7c88f0b): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:153:5` `createServiceWorkerNotificationBackend.backend.getLaunchNotification` `async-scope` (sha256:4f0b27a04899099e894c2a0d6f32f11acbeee1e340c8a13101f890b1fe7d68da): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:157:5` `createServiceWorkerNotificationBackend.backend.getActiveNotifications` `async-scope` (sha256:32b8787da01a2c016eff0505a0bdf6b9ea6a12c6ca23630e1755eeb54f07faa9): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:166:5` `createServiceWorkerNotificationBackend.backend.getPendingNotifications` `async-scope` (sha256:79e6ad239be0efd8c25c370a015d5579c20541e383a2e86b2c3b27bf5fd41d1e): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:181:5` `createServiceWorkerNotificationBackend.backend.requestPermission` `async-scope` (sha256:89dcfcb71df004f311498505a5c80f20a9a5338e1423d8e1f5dd2c482d61ce35): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:190:5` `createServiceWorkerNotificationBackend.backend.scheduleNotification` `async-scope` (sha256:13b016bea73a7da58bff9a988cede7621d76ce2183c21b6256e4dfd275bce1ab): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:248:5` `createServiceWorkerNotificationBackend.backend.updateNotification` `async-scope` (sha256:2796036fd76096b61157f020fcadbd3802fec28074380a61396a15221be36cd1): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:320:3` `createWebNotificationBackend._notify` `async-scope` (sha256:97462152a66b1c944e8a436b884c50a63ba0ba99f09da7b43c20889914a939bf): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:412:5` `createWebNotificationBackend.getLaunchNotification` `async-scope` (sha256:4f0b27a04899099e894c2a0d6f32f11acbeee1e340c8a13101f890b1fe7d68da): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:417:5` `createWebNotificationBackend.getActiveNotifications` `async-scope` (sha256:8fe4f259597f86d6f3ae693264b2764a1f2b204003b086bc88b1a2fa6f5d464c): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:421:5` `createWebNotificationBackend.getPendingNotifications` `async-scope` (sha256:79e6ad239be0efd8c25c370a015d5579c20541e383a2e86b2c3b27bf5fd41d1e): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:438:5` `createWebNotificationBackend.requestPermission` `async-scope` (sha256:50dc273ab49890f2d8ba5fce2bf96ff794063eaaf5f1fe32c5099b352c3cdb09): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:448:5` `createWebNotificationBackend.scheduleNotification` `async-scope` (sha256:837768d3e66af3adc2253422191d0a326d394f56b5590736e751f45e52532a00): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:509:5` `createWebNotificationBackend.updateNotification` `async-scope` (sha256:2a79742c4469437d9f0d3ea7fce31a4fa5562509765377722fa95700acbaa0eb): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:33:1` `explainPermissionState` `async-scope` (sha256:54c256b4f968a4ead0eb0729e6835c0093771b0ed0307918cbe2e29c676bd987): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:72:10` `getPermissionStates.join-all:72:10:6274d378044a` `join-all` (sha256:6274d378044af956d2ca53607e2b0ca7bab26a946ae52c4217e7bc1efbd6df24): taskAll Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:111:1` `readWebPermissionState` `async-scope` (sha256:b9688bad64f8e2870b249aaeb461d21482fe90c57d287df9924a7a9ab9f96110): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:145:1` `requestWebMediaPermission` `async-scope` (sha256:f3ac94fd061a45ce63c7fd5c2f4f40b37199f1707959250ad5ad32eaf96908ac): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:157:1` `requestWebNotificationPermission` `async-scope` (sha256:7aaaa2831c88d31ed4fe57eab0b336f5d289962a28a0a65e1f1ae9372e67a59e): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:171:1` `requestWebMidiPermission` `async-scope` (sha256:43e4d3682da72f40b376aa0722fd95a550bdf45386e3d4c6707cd7efa36916da): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:183:1` `requestWebPersistentStoragePermission` `async-scope` (sha256:b2c3b770762132c11047395b178d9ae70f9fdf81ef00291c52f1611c72fafd8b): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:196:1` `requestWebScreenWakeLockPermission` `async-scope` (sha256:b01a0e91c69a8f57895d21ade49a5f75504c34175c25464e9efe5a98d8649e4a): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:210:1` `requestWebPermission` `async-scope` (sha256:918492b95b5843a3fa84a3b29f8365df27d3cc905a7cd4604075444f3bf99e35): Portable task Rust lowering is not implemented.
- `@flighthq/power` `upstream/packages/power/src/power.ts:171:11` `createWebPowerBackend.setKeepAwake.catch:171:11:7e74b1d0f24a` `catch` (sha256:7e74b1d0f24a40e776a00b50cb9c5feeffee9b0c1c70ad7a9add4a53d5103c49): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/power` `upstream/packages/power/src/power.ts:175:9` `createWebPowerBackend.setKeepAwake.catch:175:9:f421e0e4c8db` `catch` (sha256:f421e0e4c8db3f465f3b2cd7b30b7be1bd06d4deb829186814e5c86b02b00f76): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/power` `upstream/packages/power/src/power.ts:175:9` `createWebPowerBackend.setKeepAwake.then:175:9:b0876493542e` `then` (sha256:b0876493542e4286c60b68e9330940be179f1d24c8d239d70a11adc88a20f20c): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/power` `upstream/packages/power/src/power.ts:183:17` `createWebPowerBackend.setKeepAwake.catch:183:17:55372bd53d79` `catch` (sha256:55372bd53d79841d64c76981d54c8b06bfc06341d2fa50c50eda51523c935c33): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/power` `upstream/packages/power/src/power.ts:183:17` `createWebPowerBackend.setKeepAwake.then:183:17:239c8a8483be` `then` (sha256:239c8a8483be286a24b3a480f268de52fcc63af5fb844a98c5587ba2914fbde2): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/power` `upstream/packages/power/src/power.ts:225:7` `createWebPowerBackend.subscribe.catch:225:7:d037e8389b31` `catch` (sha256:d037e8389b31a850e8d20e180eed2d8a94b6aa5aea2552d21de6f362e557fe3c): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/power` `upstream/packages/power/src/power.ts:225:7` `createWebPowerBackend.subscribe.then:225:7:80f76a3c57af` `then` (sha256:80f76a3c57af4e029b93f32b9b7f98a01243a77b1ee100a7be9e17c10f0355d1): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuRenderState.ts:76:1` `createWgpuRenderState` `async-scope` (sha256:e95a40cec48bb7b8d8acfb9094f04131fc308b6ee363ed06b03db04cf27b904a): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuSurface.ts:51:1` `createBitmapFromWgpuRenderState` `async-scope` (sha256:cc505770a7382fe57dbeb5478cf4671da8a3b2ce4d6a852c196e8b22c3179e5f): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuSurface.ts:154:1` `mapWgpuCaptureBuffer` `async-scope` (sha256:fcaccd53a6cd81d03cc2abab5c75eda0e3c98b0cca905c0f83194eaaf1a443c6): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuSurface.ts:159:3` `mapWgpuCaptureBuffer.catch:159:3:19fd6dcb960f` `catch` (sha256:19fd6dcb960fe21c3868f30a0efba0d1921c36f73e9fcf597006528705966c92): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuTestHelper.ts:124:26` `makeAdapter.requestDevice.ready:124:26:1032cf04da7d` `ready` (sha256:1032cf04da7dc0f8b6cc484070458a19db13b6febfcc3bcf01cc29e523535688): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuTestHelper.ts:143:1` `createWgpuRenderStateForTest` `async-scope` (sha256:f9405bf588a520e76d9533461a24c2d89bdfa42ddb1e5a5481805c3e94c03d5a): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuTestHelper.ts:154:27` `installWgpuMock.gpu.requestAdapter.ready:154:27:a9139df35f60` `ready` (sha256:a9139df35f600afd1271b7134f0310444b168a68bbfd801dc3ba3df003bf654b): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:15:1` `loadScene2DAudioResources` `async-scope` (sha256:f33713d7305f67083cc8b732de17975e0388fcd06a538b97921a6a530e4e8a82): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:27:27` `loadScene2DAudioResources.resources.join-all:27:27:d1870fb70e09` `join-all` (sha256:d1870fb70e098069d4819448f26073165126d70d45db24126e85ed34b1e97ff9): taskAll Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:28:18` `loadScene2DAudioResources.resources.anonymous:cb7146d78e6f` `async-scope` (sha256:cb7146d78e6f662b1e6c95053a1be90327458d35dc6d610c2fe7f6f6d7ab931f): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:51:63` `rejectExternalAudioResource.ready:51:63:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:16:1` `loadScene2DImageResources` `async-scope` (sha256:b340f57960862d0e1315b72c161b6880f0414629ee010674d191c2668a0714cf): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:27:24` `loadScene2DImageResources.images.join-all:27:24:4f7020c549f2` `join-all` (sha256:4f7020c549f26101c5ead9a31dabb64498fbf719badbbec7fddf949da786553f): taskAll Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:28:18` `loadScene2DImageResources.images.anonymous:b0e62b448f3a` `async-scope` (sha256:b0e62b448f3a3ffa3af9aa49b2b3097af58ce31969d710ecf8a70aa7354af3e2): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:65:63` `rejectExternalImageResource.ready:65:63:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/scene2DDocumentSource.ts:10:1` `loadScene2DDocumentFromUrl` `async-scope` (sha256:2e215fa8ee7a8f5657888c37e2298c8e5cc8ece188c0ff78f5a5da14e270fcc5): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/awd2Load.ts:11:1` `loadScene3DDocumentFromAwd2Url` `async-scope` (sha256:2ea92a2f432233f7da2a8941b6bf4357d21195130f9a799d3acea740ca53f34c): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:13:1` `loadScene3DDocumentFromGlbUrl` `async-scope` (sha256:7a93eafb613d3d2fbaf9df0b001c83350a79e233eb366aaec41c7661e619541a): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:26:1` `loadScene3DDocumentFromGltfUrl` `async-scope` (sha256:2e0c432f307b3d55b7fdba7dd5449268ef1134af9ada24322968a1fbac2fe52b): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:47:1` `loadGltfExternalBuffers` `async-scope` (sha256:cf1a4f7e4632c2bbce3fa3c1db0c3a494be71acfac02c51cd1a288b162cae6c2): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:60:23` `loadGltfExternalBuffers.bytes.join-all:60:23:231f9377cfef` `join-all` (sha256:231f9377cfef204e6b106b47848f683ef44f9376edd5785c8c2573fc25ff2ac3): taskAll Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/imageResourceFetch.ts:7:1` `fetchWebImageResource` `async-scope` (sha256:32c9f2af1f89f24c9dd0da6c377a64100ce4fc23b28f3448978fd12113d4fdb8): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:17:1` `loadScene3DResources` `async-scope` (sha256:4e95008fd62274016bb3c60408c0dbc4aaeab8d5dc55a1863dfffcaf3aff4cb4): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:39:7` `loadScene3DResources.then:39:7:de0441a97802` `then` (sha256:de0441a97802facc88067100be36082051bfc2680abffdca2cd195677dd0e2e9): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:46:9` `loadScene3DResources.join-all-settled:46:9:9a6c49538a0d` `join-all-settled` (sha256:9a6c49538a0dd0d43196b4381c10243649d88fdecf2303e3b624df43dd3d2ee8): taskAllSettled Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:51:1` `waitForScene3DResourceResolver` `async-scope` (sha256:540bcecd394ba445846abe250e2e046d49a3a3bb48264903efcc52f50f838cff): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:55:9` `waitForScene3DResourceResolver.join-all-settled:55:9:fbfa931863ef` `join-all-settled` (sha256:fbfa931863ef9e1b6320bb9ae20c8b9b8e7a620041f8d2a78ea4183d4c360d2f): taskAllSettled Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/md2Load.ts:10:1` `loadScene3DDocumentFromMd2Url` `async-scope` (sha256:1f1c06e1b2f6415e1a9c3096a51d2d60dac0d97ee5232697345b8d6f34df3a3e): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/md5Load.ts:10:1` `loadScene3DDocumentFromMd5MeshUrl` `async-scope` (sha256:88a8e12a510971c6dc2cc55312b1e0c856acead071bebf7aef4eb9d631ac168d): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/objLoad.ts:11:1` `loadScene3DDocumentFromObjUrl` `async-scope` (sha256:abc05b22e7407c4a12d72403e7e4ca806603fa706c8cfd4d177e1141fafc96a9): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/resolveScene3DResources.ts:248:21` `requestWorkingResolutions.then:248:21:378b7cc5cb4c` `then` (sha256:378b7cc5cb4ca1b91d8369041da9d2be92549bdc9453f01b659961dbf820520a): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/resolveScene3DResources.ts:256:38` `_resolvedVoid.ready:256:38:9b8874eb7ffe` `ready` (sha256:9b8874eb7ffebcf300217391f3fb4a9d53d078bccf4ed67e3253de0dd1f14116): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/sceneDocumentSource.ts:22:1` `loadScene3DDocumentBytesFromUrl` `async-scope` (sha256:497f04f3b9283b9149918a9735e0462f2c1bd0a5c6f2fb20f1c0507d388c5295): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/sceneDocumentSource.ts:37:1` `loadScene3DDocumentTextFromUrl` `async-scope` (sha256:b7af38e5cfdddaa074ae4da9a5d83d6844e0ec33f63a09791ca4c05237970016): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/threeDsLoad.ts:9:1` `loadScene3DDocumentFrom3dsUrl` `async-scope` (sha256:e1d0c6bd60275a86470b2660f6807042d3591ae50f5eacc49a362a681a9b7ba4): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/screen` `upstream/packages/screen/src/screen.ts:686:3` `onScreenDetailPermissionChange.catch:686:3:f9391dc21e33` `catch` (sha256:f9391dc21e337fc5b3e7f4e0ffa76f5007e9b8712530e7626e328e2074aef4f2): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/screen` `upstream/packages/screen/src/screen.ts:686:3` `onScreenDetailPermissionChange.then:686:3:8eeabb315a5a` `then` (sha256:8eeabb315a5a5409b03e7b97f684d1b41aae5a6844b0c58aaf724a8c4731fb83): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/sensors` `upstream/packages/sensors/src/sensors.ts:347:5` `createWebSensorsBackend.requestPermission` `async-scope` (sha256:a6b3f78082562059ef4444a5887aa326b0045614089c9750aee6061cbb35e0b1): Portable task Rust lowering is not implemented.
- `@flighthq/sensors` `upstream/packages/sensors/src/sensors.ts:734:1` `getWebSensorsPermissionState` `async-scope` (sha256:866595badad9bbaa0cb39468bdb48194a2b9d981244cb4e6c606ff225e7b0b56): Portable task Rust lowering is not implemented.
- `@flighthq/share` `upstream/packages/share/src/share.ts:52:5` `createWebShareBackend.share` `async-scope` (sha256:4728f5341fa2d2ba2f77e4c2f40d7dec9804c802797d5c0dea14c9c428d2f5d3): Portable task Rust lowering is not implemented.
- `@flighthq/share` `upstream/packages/share/src/share.ts:65:5` `createWebShareBackend.shareWithResult` `async-scope` (sha256:d92540756a8e0651e19dc368a8386c5e2cc648380251118f8741018a71be5c0b): Portable task Rust lowering is not implemented.
- `@flighthq/share` `upstream/packages/share/src/share.ts:137:47` `shareContent.ready:137:47:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/share` `upstream/packages/share/src/share.ts:145:1` `shareContentWithResult` `async-scope` (sha256:3b6bb869bc443762e19ff6042773430d02f24c8f9065e29fd5e3ae5db79d54f8): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:17:5` `createWebShellBackend.moveItemsToTrash` `async-scope` (sha256:de9261ef6b76c66d51f7895db3dfa0166804fa12b1736d08906a77f5182c1b87): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:21:5` `createWebShellBackend.moveToTrash` `async-scope` (sha256:d49f2ab1646329f228fd4dbcae8c7a83f7680f02e37bbc9a7634a08e37d42bb2): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:25:5` `createWebShellBackend.openExternal` `async-scope` (sha256:84df5b77ebd9b072e2cd26f7bff3a72cca839047849058809b421e0a81065f02): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:35:5` `createWebShellBackend.openPath` `async-scope` (sha256:717eb0cd4068de15dd3ed0c325d82ec9dfed0853bb8e991f90076d7c75a7d2da): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:39:5` `createWebShellBackend.openPathResult` `async-scope` (sha256:bdb09806443cc7816c45a1fec4a7f4e82c8492dc705a8ad8e5a4fb9311cc42af): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:43:5` `createWebShellBackend.readShortcutLink` `async-scope` (sha256:28ee120b9dd12b1d1bc09290ffe3c3841168d6a5fd616a57d0742ccd122fcdba): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:47:5` `createWebShellBackend.showItemInFolder` `async-scope` (sha256:37927761ccb0dd5a1883348de59ab8661eeee77fab2399832f77c3e33bcceb82): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:51:5` `createWebShellBackend.writeShortcutLink` `async-scope` (sha256:96c84cc788fa871527bacbf8f9a95bdb0218b2c55f5bca0890761378620eb3c6): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:94:39` `openShellExternalUrl.ready:94:39:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/storage` `upstream/packages/storage/src/storage.ts:309:1` `getStorageQuotaEstimate` `async-scope` (sha256:bb6e6e61ce277610234cf0eb3e4794ff7a72f5694637f41d5f9169471ac85241): Portable task Rust lowering is not implemented.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:37:1` `loadTextureAtlasFromBase64` `async-scope` (sha256:c3b8831bbb577f82b4ecd77ec700c8eb9c03989c66a46b55c9a8dc00a57228c4): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:45:1` `loadTextureAtlasFromBlob` `async-scope` (sha256:7f84b1d162a9f648364d02ce2871669e570676045f31e732203694a2ca574321): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:49:1` `loadTextureAtlasFromBytes` `async-scope` (sha256:6ac6dc7033bfcd7d150920334b8cc158aa2fd830661a0084495ba72fdc98574b): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:57:1` `loadTextureAtlasFromUrl` `async-scope` (sha256:fdebcaac0c0dc1938188c4a1a9a307778df89815dcdfc5041931ce2e4f35933a): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/video` `upstream/packages/video/src/videoResourceFrom.ts:22:1` `loadVideoResourceFromBlob` `async-scope` (sha256:49dac0e39380b6e5dc17131cf81972d032ee3bcd1ad012f4cf3cfdd7eb3c6112): Portable task Rust lowering is not implemented.
- `@flighthq/video` `upstream/packages/video/src/videoResourceFrom.ts:44:31` `loadVideoResourceFromUrl.reject:44:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/video` `upstream/packages/video/src/videoResourceFrom.ts:95:33` `loadVideoResourceFromUrls.ready:95:33:8dc66a891dc0` `ready` (sha256:8dc66a891dc00c6da20c45026f4d2735b90d17e2fd3226f4db31818040badc46): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/webcam` `upstream/packages/webcam/src/webcam.ts:78:5` `createWebWebcamBackend.requestPermission` `async-scope` (sha256:9b066b5933be33c0c78cc297fd5270583ed6782749630182731a3b9979cd16cd): Portable task Rust lowering is not implemented.

### Unsupported async scopes

- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:141:1` `loadAssetGroup` (sha256:188d51024cb73e45ac54868edd4ae13e87859c658cbe0a8476dbe34b01d289ec): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:24:1` `loadAudioResourceFromBase64` (sha256:f62565721d1676f6b065becc6e2abdd9e41b9d327d26c08664159eaf20e4e9fc): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:36:1` `loadAudioResourceFromBlob` (sha256:6f31ecce9820d5f8d705caee039a2579bafeb3dca51a82e47de6c043e688642e): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:48:1` `loadAudioResourceFromBytes` (sha256:4ab485113a101f105f72060f521838ed60247dc4537ffb1961fe8b48664b6f1a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:66:1` `loadAudioResourceFromUrl` (sha256:9afb9a9b90a12c12d7e56e320707532002a913be2e722b67c214e9da4246faa9): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:88:1` `loadAudioResourceFromUrls` (sha256:f704e57788fe00ece2e8f7e0e8f89db8a63e3a17ca93c092fb6d1adf17df4c64): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceReference.ts:110:1` `resolveAudioResourceReference` (sha256:427972d435dc38d25124a3ebca0e20385d9e78482f6e48c5352604cd213443e2): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceReference.ts:144:1` `decodeAudioResourceBytes` (sha256:3fb0a202462a960a2221ff6707ce03dc96eccc7968aa21b762d78bb652c31f2e): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
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
- `@flighthq/clipboard` `upstream/packages/clipboard/src/clipboard.ts:364:1` `blobFromFormatData` (sha256:c1fb9e0ee718a360876a579fe9e1e5fdc16157f03db3dabc983b233c321466dd): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/connectivity` `upstream/packages/connectivity/src/connectivity.ts:82:5` `createWebConnectivityBackend.detectReachability` (sha256:694076bc4efb1a4cfa0e975d140e8cf6f2435a0fdd257300be3a958e3a2b5015): Portable task Rust lowering is not implemented.
- `@flighthq/connectivity` `upstream/packages/connectivity/src/connectivity.ts:138:1` `detectConnectivityReachability` (sha256:599d1031e281368c595d7c67f9094c5325c5caf880d596a64a3c5d592178e52b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:19:5` `createWebDialogBackend.confirm` (sha256:9e0d435b7b04504a82aec0618dbb15c78815dffab42978a68f29044f5e6fcf78): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:28:5` `createWebDialogBackend.message` (sha256:7c2e4513e6b2c9853b0c5bbf6a3f56fc25557127cde7c98c54a42e7b3820a77d): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:48:5` `createWebDialogBackend.prompt` (sha256:c2ef3f3e409d36b6f50d673164d87f3da10c9811b4725fa58da7f7886e5b23b3): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:56:5` `createWebDialogBackend.saveFile` (sha256:40414e87f9a0df2dbf9669cc34214ffee390112a4912d6d8a9648f7be2102d80): Portable task Rust lowering is not implemented.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:251:1` `openDirectoryPickerAccessApi` (sha256:b899e4b9db2555d6330dd9e3cdf86a011e79a2f0355b05f846f7c7204dbae732): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:324:1` `openFileSystemAccessPicker` (sha256:0a2092597843117736235fd619c2d4f395e5ff517d455c075d5948fee291c1cc): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/dialog` `upstream/packages/dialog/src/dialog.ts:354:1` `saveWebFile` (sha256:e88949abb7c69f2698087c43c21543a34eaf2427cac71f67fccf397a0a8a7156): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
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
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:197:5` `createWebFileSystemBackend.openFileReadStream` (sha256:397d36c122ccf42160fd8c8d0b251d4e415f2caa9dc6a8e6d752f9d589e3b93a): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:207:5` `createWebFileSystemBackend.openFileWriteStream` (sha256:a1398c0c934e0b064752d3e88b8faf3fcceeb22c332ef48a527eebbcfe690500): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
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
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:625:1` `getWebRoot` (sha256:ef7f3d7015038a12b48081190128011dea066ed8d640e163d354dccba07a461f): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:636:1` `getWebDirectoryHandle` (sha256:7f0c9697a8fda8bf983894519de724aca66a1a9c03e449ffe09ddd4ae9c63f92): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:652:1` `getWebFileHandle` (sha256:81a545005c91ec665cdc44f2beb861a1922b104d011fcedd756fb8897428ae65): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:668:1` `walkWebDirectory` (sha256:6d1216dc9c5354ba256755b0451e1d3789557508dda88892e626011b93807eb4): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:687:1` `writeWebRemove` (sha256:1841fd4e7778e7dd8472360d0552f739a00f488b0961c21213eab5deafe197cf): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:707:1` `writeWebFile` (sha256:6e15f5efb9909a3d0e7c991013ac6cdfd764a902ed4e16f226efb7f32676e172): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/_fontFaceLoad.ts:6:1` `_loadFontFaceFromBytes` (sha256:1c96fe99227e32ac4c4cb8b9fe7ba660176b30f1b73e3fced9750ce6c9208f93): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/_fontFaceLoad.ts:29:1` `loadAndRegisterFontFace` (sha256:9146827c6b34222cda06746f591036a43d1cc4fde6318dc70d182b561931028d): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/fontStatus.ts:7:1` `whenFontsReady` (sha256:a9a632e81b2b401d128497ca7e36252f671e86a84e7380d7ffa2915b10717660): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/geolocation` `upstream/packages/geolocation/src/geolocation.ts:80:5` `createWebGeolocationBackend.getPermission` (sha256:589424ae8a82b30bc0a05473e652fce3ea1e3d8dbf9d62047dec8eea751d9252): Portable task Rust lowering is not implemented.
- `@flighthq/geolocation` `upstream/packages/geolocation/src/geolocation.ts:92:5` `createWebGeolocationBackend.requestPermission` (sha256:c1d7c01868b81a6d6a24327b828b629c16b4760c6743b681bf2244d5fc64db43): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:65:1` `loadImageResourceFromBase64` (sha256:a73bde2432a402dd8c65c2d52b8994241855cc78ee83d75be142db0c50287d8a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:73:1` `loadImageResourceFromBlob` (sha256:97f7ece284c5aa823b9978fb24baf949ec2100662fcec48c12d8ccb2fe3260c5): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:82:1` `loadImageResourceFromBytes` (sha256:f2c96b3490cc8b7d2a8435a1ac35303a3806e4548d2edb8856c7f7eb6e6fef3a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:95:1` `loadImageResourceFromUrl` (sha256:71b688be69653eee059a72041402f11b74e14997cd4fab3120696e48a52ae882): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceReference.ts:89:1` `resolveImageResourceReference` (sha256:7596b9859812ac08a756c394cc2cab4b589bc0542bc55822e22a5077a562f410): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image-codec` `upstream/packages/image-codec/src/registerWebImageDecoders.ts:18:45` `decodeImageWithCanvas` (sha256:6c70f189d2b99fef85a6c33991e3174b6e7cfc147486162cdd1abee7f40fc943): Portable task Rust lowering is not implemented.
- `@flighthq/image-codec` `upstream/packages/image-codec/src/registerWebImageEncoders.ts:16:10` `createCanvasImageEncoder.anonymous:7c4dbd1c1e56` (sha256:7c4dbd1c1e56dfb5d704a95587bc335ef9c42e20340743d59daacfb0eda450d3): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:502:1` `drainQueue` (sha256:88082b65b0b72d5a7051045ab407875a77a7430ece259e4bf911600ef6289ed7): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:552:1` `runEntry` (sha256:0cc4fef1fe859a3eba70abead965e6eb2ab15fc8f1671ae50442eeba76acdea5): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/net` `upstream/packages/net/src/net.ts:21:5` `createWebNetBackend.sendNetRequest` (sha256:95718bdeab605b5c428496aed0ad940a09e148c6ffd1b690723a621d3089c453): Portable task Rust lowering is not implemented.
- `@flighthq/net` `upstream/packages/net/src/net.ts:110:1` `_readNetResponseBody` (sha256:a65ec98271b1315b2d9f92bf7f23f3862fc0a2eefa19665fa73e7818af9bfc60): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/net` `upstream/packages/net/src/net.ts:133:1` `_readNetResponseWithProgress` (sha256:b695935644b73d37aea7c2af6c5ee8129d40d1a01cb334357c027ceadca38081): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:81:3` `createServiceWorkerNotificationBackend._show` (sha256:e00200853c5c516d6fb8f492b4d4a4936ad578f62c3c4c064339bd5fb6a4d072): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:119:5` `createServiceWorkerNotificationBackend.backend.closeAllNotifications` (sha256:52fa18d11686282d426bb5c4c47630bb80135c1b9e903717735b1eed8307c577): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:130:5` `createServiceWorkerNotificationBackend.backend.closeNotification` (sha256:6517e4505bd2c041ac811f8a05ba8353749846f946f30ac5ddfb2f22f7c88f0b): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:153:5` `createServiceWorkerNotificationBackend.backend.getLaunchNotification` (sha256:4f0b27a04899099e894c2a0d6f32f11acbeee1e340c8a13101f890b1fe7d68da): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:157:5` `createServiceWorkerNotificationBackend.backend.getActiveNotifications` (sha256:32b8787da01a2c016eff0505a0bdf6b9ea6a12c6ca23630e1755eeb54f07faa9): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:166:5` `createServiceWorkerNotificationBackend.backend.getPendingNotifications` (sha256:79e6ad239be0efd8c25c370a015d5579c20541e383a2e86b2c3b27bf5fd41d1e): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:181:5` `createServiceWorkerNotificationBackend.backend.requestPermission` (sha256:89dcfcb71df004f311498505a5c80f20a9a5338e1423d8e1f5dd2c482d61ce35): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:190:5` `createServiceWorkerNotificationBackend.backend.scheduleNotification` (sha256:13b016bea73a7da58bff9a988cede7621d76ce2183c21b6256e4dfd275bce1ab): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:248:5` `createServiceWorkerNotificationBackend.backend.updateNotification` (sha256:2796036fd76096b61157f020fcadbd3802fec28074380a61396a15221be36cd1): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:320:3` `createWebNotificationBackend._notify` (sha256:97462152a66b1c944e8a436b884c50a63ba0ba99f09da7b43c20889914a939bf): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:412:5` `createWebNotificationBackend.getLaunchNotification` (sha256:4f0b27a04899099e894c2a0d6f32f11acbeee1e340c8a13101f890b1fe7d68da): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:417:5` `createWebNotificationBackend.getActiveNotifications` (sha256:8fe4f259597f86d6f3ae693264b2764a1f2b204003b086bc88b1a2fa6f5d464c): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:421:5` `createWebNotificationBackend.getPendingNotifications` (sha256:79e6ad239be0efd8c25c370a015d5579c20541e383a2e86b2c3b27bf5fd41d1e): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:438:5` `createWebNotificationBackend.requestPermission` (sha256:50dc273ab49890f2d8ba5fce2bf96ff794063eaaf5f1fe32c5099b352c3cdb09): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:448:5` `createWebNotificationBackend.scheduleNotification` (sha256:837768d3e66af3adc2253422191d0a326d394f56b5590736e751f45e52532a00): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:509:5` `createWebNotificationBackend.updateNotification` (sha256:2a79742c4469437d9f0d3ea7fce31a4fa5562509765377722fa95700acbaa0eb): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:33:1` `explainPermissionState` (sha256:54c256b4f968a4ead0eb0729e6835c0093771b0ed0307918cbe2e29c676bd987): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:111:1` `readWebPermissionState` (sha256:b9688bad64f8e2870b249aaeb461d21482fe90c57d287df9924a7a9ab9f96110): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:145:1` `requestWebMediaPermission` (sha256:f3ac94fd061a45ce63c7fd5c2f4f40b37199f1707959250ad5ad32eaf96908ac): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:157:1` `requestWebNotificationPermission` (sha256:7aaaa2831c88d31ed4fe57eab0b336f5d289962a28a0a65e1f1ae9372e67a59e): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:171:1` `requestWebMidiPermission` (sha256:43e4d3682da72f40b376aa0722fd95a550bdf45386e3d4c6707cd7efa36916da): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:183:1` `requestWebPersistentStoragePermission` (sha256:b2c3b770762132c11047395b178d9ae70f9fdf81ef00291c52f1611c72fafd8b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:196:1` `requestWebScreenWakeLockPermission` (sha256:b01a0e91c69a8f57895d21ade49a5f75504c34175c25464e9efe5a98d8649e4a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:210:1` `requestWebPermission` (sha256:918492b95b5843a3fa84a3b29f8365df27d3cc905a7cd4604075444f3bf99e35): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuRenderState.ts:76:1` `createWgpuRenderState` (sha256:e95a40cec48bb7b8d8acfb9094f04131fc308b6ee363ed06b03db04cf27b904a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuSurface.ts:51:1` `createBitmapFromWgpuRenderState` (sha256:cc505770a7382fe57dbeb5478cf4671da8a3b2ce4d6a852c196e8b22c3179e5f): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuSurface.ts:154:1` `mapWgpuCaptureBuffer` (sha256:fcaccd53a6cd81d03cc2abab5c75eda0e3c98b0cca905c0f83194eaaf1a443c6): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuTestHelper.ts:143:1` `createWgpuRenderStateForTest` (sha256:f9405bf588a520e76d9533461a24c2d89bdfa42ddb1e5a5481805c3e94c03d5a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:15:1` `loadScene2DAudioResources` (sha256:f33713d7305f67083cc8b732de17975e0388fcd06a538b97921a6a530e4e8a82): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:28:18` `loadScene2DAudioResources.resources.anonymous:cb7146d78e6f` (sha256:cb7146d78e6f662b1e6c95053a1be90327458d35dc6d610c2fe7f6f6d7ab931f): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:16:1` `loadScene2DImageResources` (sha256:b340f57960862d0e1315b72c161b6880f0414629ee010674d191c2668a0714cf): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:28:18` `loadScene2DImageResources.images.anonymous:b0e62b448f3a` (sha256:b0e62b448f3a3ffa3af9aa49b2b3097af58ce31969d710ecf8a70aa7354af3e2): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/scene2DDocumentSource.ts:10:1` `loadScene2DDocumentFromUrl` (sha256:2e215fa8ee7a8f5657888c37e2298c8e5cc8ece188c0ff78f5a5da14e270fcc5): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/awd2Load.ts:11:1` `loadScene3DDocumentFromAwd2Url` (sha256:2ea92a2f432233f7da2a8941b6bf4357d21195130f9a799d3acea740ca53f34c): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:13:1` `loadScene3DDocumentFromGlbUrl` (sha256:7a93eafb613d3d2fbaf9df0b001c83350a79e233eb366aaec41c7661e619541a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:26:1` `loadScene3DDocumentFromGltfUrl` (sha256:2e0c432f307b3d55b7fdba7dd5449268ef1134af9ada24322968a1fbac2fe52b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:47:1` `loadGltfExternalBuffers` (sha256:cf1a4f7e4632c2bbce3fa3c1db0c3a494be71acfac02c51cd1a288b162cae6c2): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/imageResourceFetch.ts:7:1` `fetchWebImageResource` (sha256:32c9f2af1f89f24c9dd0da6c377a64100ce4fc23b28f3448978fd12113d4fdb8): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:17:1` `loadScene3DResources` (sha256:4e95008fd62274016bb3c60408c0dbc4aaeab8d5dc55a1863dfffcaf3aff4cb4): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:51:1` `waitForScene3DResourceResolver` (sha256:540bcecd394ba445846abe250e2e046d49a3a3bb48264903efcc52f50f838cff): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/md2Load.ts:10:1` `loadScene3DDocumentFromMd2Url` (sha256:1f1c06e1b2f6415e1a9c3096a51d2d60dac0d97ee5232697345b8d6f34df3a3e): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/md5Load.ts:10:1` `loadScene3DDocumentFromMd5MeshUrl` (sha256:88a8e12a510971c6dc2cc55312b1e0c856acead071bebf7aef4eb9d631ac168d): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/objLoad.ts:11:1` `loadScene3DDocumentFromObjUrl` (sha256:abc05b22e7407c4a12d72403e7e4ca806603fa706c8cfd4d177e1141fafc96a9): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/sceneDocumentSource.ts:22:1` `loadScene3DDocumentBytesFromUrl` (sha256:497f04f3b9283b9149918a9735e0462f2c1bd0a5c6f2fb20f1c0507d388c5295): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/sceneDocumentSource.ts:37:1` `loadScene3DDocumentTextFromUrl` (sha256:b7af38e5cfdddaa074ae4da9a5d83d6844e0ec33f63a09791ca4c05237970016): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/threeDsLoad.ts:9:1` `loadScene3DDocumentFrom3dsUrl` (sha256:e1d0c6bd60275a86470b2660f6807042d3591ae50f5eacc49a362a681a9b7ba4): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/sensors` `upstream/packages/sensors/src/sensors.ts:347:5` `createWebSensorsBackend.requestPermission` (sha256:a6b3f78082562059ef4444a5887aa326b0045614089c9750aee6061cbb35e0b1): Portable task Rust lowering is not implemented.
- `@flighthq/sensors` `upstream/packages/sensors/src/sensors.ts:734:1` `getWebSensorsPermissionState` (sha256:866595badad9bbaa0cb39468bdb48194a2b9d981244cb4e6c606ff225e7b0b56): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/share` `upstream/packages/share/src/share.ts:52:5` `createWebShareBackend.share` (sha256:4728f5341fa2d2ba2f77e4c2f40d7dec9804c802797d5c0dea14c9c428d2f5d3): Portable task Rust lowering is not implemented.
- `@flighthq/share` `upstream/packages/share/src/share.ts:65:5` `createWebShareBackend.shareWithResult` (sha256:d92540756a8e0651e19dc368a8386c5e2cc648380251118f8741018a71be5c0b): Portable task Rust lowering is not implemented.
- `@flighthq/share` `upstream/packages/share/src/share.ts:145:1` `shareContentWithResult` (sha256:3b6bb869bc443762e19ff6042773430d02f24c8f9065e29fd5e3ae5db79d54f8): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:17:5` `createWebShellBackend.moveItemsToTrash` (sha256:de9261ef6b76c66d51f7895db3dfa0166804fa12b1736d08906a77f5182c1b87): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:21:5` `createWebShellBackend.moveToTrash` (sha256:d49f2ab1646329f228fd4dbcae8c7a83f7680f02e37bbc9a7634a08e37d42bb2): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:25:5` `createWebShellBackend.openExternal` (sha256:84df5b77ebd9b072e2cd26f7bff3a72cca839047849058809b421e0a81065f02): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:35:5` `createWebShellBackend.openPath` (sha256:717eb0cd4068de15dd3ed0c325d82ec9dfed0853bb8e991f90076d7c75a7d2da): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:39:5` `createWebShellBackend.openPathResult` (sha256:bdb09806443cc7816c45a1fec4a7f4e82c8492dc705a8ad8e5a4fb9311cc42af): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:43:5` `createWebShellBackend.readShortcutLink` (sha256:28ee120b9dd12b1d1bc09290ffe3c3841168d6a5fd616a57d0742ccd122fcdba): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:47:5` `createWebShellBackend.showItemInFolder` (sha256:37927761ccb0dd5a1883348de59ab8661eeee77fab2399832f77c3e33bcceb82): Portable task Rust lowering is not implemented.
- `@flighthq/shell` `upstream/packages/shell/src/shell.ts:51:5` `createWebShellBackend.writeShortcutLink` (sha256:96c84cc788fa871527bacbf8f9a95bdb0218b2c55f5bca0890761378620eb3c6): Portable task Rust lowering is not implemented.
- `@flighthq/storage` `upstream/packages/storage/src/storage.ts:309:1` `getStorageQuotaEstimate` (sha256:bb6e6e61ce277610234cf0eb3e4794ff7a72f5694637f41d5f9169471ac85241): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:37:1` `loadTextureAtlasFromBase64` (sha256:c3b8831bbb577f82b4ecd77ec700c8eb9c03989c66a46b55c9a8dc00a57228c4): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:45:1` `loadTextureAtlasFromBlob` (sha256:7f84b1d162a9f648364d02ce2871669e570676045f31e732203694a2ca574321): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:49:1` `loadTextureAtlasFromBytes` (sha256:6ac6dc7033bfcd7d150920334b8cc158aa2fd830661a0084495ba72fdc98574b): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:57:1` `loadTextureAtlasFromUrl` (sha256:fdebcaac0c0dc1938188c4a1a9a307778df89815dcdfc5041931ce2e4f35933a): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/video` `upstream/packages/video/src/videoResourceFrom.ts:22:1` `loadVideoResourceFromBlob` (sha256:49dac0e39380b6e5dc17131cf81972d032ee3bcd1ad012f4cf3cfdd7eb3c6112): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
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
- `upstream/packages/color/src/oklab.test.ts` (0/6 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/packColor.test.ts` (0/23 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/premultiplyColorAlpha.test.ts` (0/7 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.

## Blockers

### `@flighthq/abc`

- **emission** `upstream/packages/abc/src/abcFile.ts`: readAbcFile: new-expression Rust lowering is not implemented: abc_reader

### `@flighthq/animation`

- **emission** `upstream/packages/animation/src/animationBlendTree.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/animation/src/animationClip.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/animation/src/animationCrossfade.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/animation/src/animationLayerStack.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/animation/src/animationStateMachine.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/animation/src/animationTrack.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/app`

- **package** `upstream/packages/app/src`: Generated crate is missing 3 of 42 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/app/src/app.ts`: createWebAppBackend: taskThen Rust lowering is reserved for Pass 27 Stage 4

### `@flighthq/application`

- **emission** `upstream/packages/application/src/applicationRenderView.ts`: ApplicationRenderViewRuntime: aggregate native entity runtime closure is unavailable: imported EntityRuntime aggregate cannot acquire package-local storage fields: ApplicationRenderViewRuntime.attached, ApplicationRenderViewRuntime.resize, ApplicationRenderViewRuntime.synchronize

### `@flighthq/assets`

- **package** `upstream/packages/assets/src`: Generated crate is missing 1 of 18 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/assets/src/assetLibrary.ts`: acquireAsset: taskThen Rust lowering is reserved for Pass 27 Stage 4

### `@flighthq/audio`

- **emission** `upstream/packages/audio/src/audioResourceFrom.ts`: loadAudioResourceFromBlob: upstream/packages/audio/src/audioResourceFrom.ts:41:23: await value type is not recovered
- **emission** `upstream/packages/audio/src/audioResourceReference.ts`: createAudioResourceFailure: instanceof Rust lowering requires a portable typed-array constructor

### `@flighthq/binpack`

- **package** `upstream/packages/binpack/src`: Generated crate is missing 1 of 4 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/binpack/src/explainUnpackedRectangles.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/binpack/src/packRectangles.ts`: compareRectangleId: typeof operand has no inferred Rust type: {"kind":"identifier","name":"a"}

### `@flighthq/bitmapfont`

- **emission** `upstream/packages/bitmapfont/src/enableBitmapFontGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/camera-controls`

- **emission** `upstream/packages/camera-controls/src/framing.ts`: getPerspectiveProjectionFrameDistanceToSphere: Math.atan Rust lowering is not implemented

### `@flighthq/capture`

- **emission** `upstream/packages/capture/src/captureBaseline.ts`: formatCaptureBaseline: JSON.stringify requires a portable scalar or structural array

### `@flighthq/clip`

- **emission** `upstream/packages/clip/src/clipRegion.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/clip/src/enableClipGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/clipboard`

- **package** `upstream/packages/clipboard/src`: Generated crate is missing 3 of 32 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/clipboard/src/clipboard.ts`: createWebClipboardBackend: upstream/packages/clipboard/src/clipboard.ts:31:5: portableTask createWebClipboardBackend.readFormat: Portable task Rust lowering is not implemented.

### `@flighthq/collision`

- **emission** `upstream/packages/collision/src/enableCollisionGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/collision/src/raycastCollisionShape.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/compression`

- **emission** `upstream/packages/compression/src/deflate.ts`: inflateDeflate: upstream/packages/compression/src/deflate.ts: cannot infer return type for inflateDeflate

### `@flighthq/connectivity`

- **package** `upstream/packages/connectivity/src`: Generated crate is missing 4 of 14 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/connectivity/src/connectivity.ts`: createWebConnectivityBackend: upstream/packages/connectivity/src/connectivity.ts:82:5: portableTask createWebConnectivityBackend.detectReachability: Portable task Rust lowering is not implemented.

### `@flighthq/debug`

- **emission** `upstream/packages/debug/src/debug.ts`: _collectDebugChannels: spread Rust lowering is not implemented

### `@flighthq/dialog`

- **package** `upstream/packages/dialog/src`: Generated crate is missing 3 of 15 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/dialog/src/dialog.ts`: createWebDialogBackend: upstream/packages/dialog/src/dialog.ts:19:5: portableTask createWebDialogBackend.confirm: Portable task Rust lowering is not implemented.

### `@flighthq/effects-canvas`

- **package** `upstream/packages/effects-canvas/src`: Generated crate is missing 15 of 73 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-canvas/src/canvasBevelEffect.ts`: defaultCanvasBevelEffectRunner: upstream/packages/effects-canvas/src/canvasBevelEffect.ts: cannot infer return type for defaultCanvasBevelEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasBlendEffect.ts`: defaultCanvasBlendEffectRunner: upstream/packages/effects-canvas/src/canvasBlendEffect.ts: cannot infer return type for defaultCanvasBlendEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasBloomEffect.ts`: defaultCanvasBloomEffectRunner: upstream/packages/effects-canvas/src/canvasBloomEffect.ts: cannot infer return type for defaultCanvasBloomEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasBlurEffect.ts`: defaultCanvasBlurEffectRunner: upstream/packages/effects-canvas/src/canvasBlurEffect.ts: cannot infer return type for defaultCanvasBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasCompositeEffect.ts`: defaultCanvasCompositeEffectRunner: upstream/packages/effects-canvas/src/canvasCompositeEffect.ts: cannot infer return type for defaultCanvasCompositeEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasDropShadowEffect.ts`: defaultCanvasDropShadowEffectRunner: upstream/packages/effects-canvas/src/canvasDropShadowEffect.ts: cannot infer return type for defaultCanvasDropShadowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasFilmGrainEffect.ts`: defaultCanvasFilmGrainEffectRunner: upstream/packages/effects-canvas/src/canvasFilmGrainEffect.ts: cannot infer return type for defaultCanvasFilmGrainEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasGradientBevelEffect.ts`: defaultCanvasGradientBevelEffectRunner: upstream/packages/effects-canvas/src/canvasGradientBevelEffect.ts: cannot infer return type for defaultCanvasGradientBevelEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasGradientGlowEffect.ts`: defaultCanvasGradientGlowEffectRunner: upstream/packages/effects-canvas/src/canvasGradientGlowEffect.ts: cannot infer return type for defaultCanvasGradientGlowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasInnerGlowEffect.ts`: defaultCanvasInnerGlowEffectRunner: upstream/packages/effects-canvas/src/canvasInnerGlowEffect.ts: cannot infer return type for defaultCanvasInnerGlowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasInnerShadowEffect.ts`: defaultCanvasInnerShadowEffectRunner: upstream/packages/effects-canvas/src/canvasInnerShadowEffect.ts: cannot infer return type for defaultCanvasInnerShadowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasOuterGlowEffect.ts`: defaultCanvasOuterGlowEffectRunner: upstream/packages/effects-canvas/src/canvasOuterGlowEffect.ts: cannot infer return type for defaultCanvasOuterGlowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasPixelateEffect.ts`: defaultCanvasPixelateEffectRunner: upstream/packages/effects-canvas/src/canvasPixelateEffect.ts: cannot infer return type for defaultCanvasPixelateEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasScanlinesEffect.ts`: defaultCanvasScanlinesEffectRunner: upstream/packages/effects-canvas/src/canvasScanlinesEffect.ts: cannot infer return type for defaultCanvasScanlinesEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasVignetteEffect.ts`: defaultCanvasVignetteEffectRunner: upstream/packages/effects-canvas/src/canvasVignetteEffect.ts: cannot infer return type for defaultCanvasVignetteEffectRunner

### `@flighthq/effects-gl`

- **package** `upstream/packages/effects-gl/src`: Generated crate is missing 56 of 176 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-gl/src/glBevelEffect.ts`: defaultGlBevelEffectRunner: upstream/packages/effects-gl/src/glBevelEffect.ts: cannot infer return type for defaultGlBevelEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBlendEffect.ts`: defaultGlBlendEffectRunner: upstream/packages/effects-gl/src/glBlendEffect.ts: cannot infer return type for defaultGlBlendEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBloomEffect.ts`: defaultGlBloomEffectRunner: upstream/packages/effects-gl/src/glBloomEffect.ts: cannot infer return type for defaultGlBloomEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBlurEffect.ts`: defaultGlBlurEffectRunner: upstream/packages/effects-gl/src/glBlurEffect.ts: cannot infer return type for defaultGlBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBokehDepthOfFieldEffect.ts`: defaultGlBokehDepthOfFieldEffectRunner: upstream/packages/effects-gl/src/glBokehDepthOfFieldEffect.ts: cannot infer return type for defaultGlBokehDepthOfFieldEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCameraMotionBlurEffect.ts`: defaultGlCameraMotionBlurEffectRunner: upstream/packages/effects-gl/src/glCameraMotionBlurEffect.ts: cannot infer return type for defaultGlCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glChromaticAberrationEffect.ts`: defaultGlChromaticAberrationEffectRunner: upstream/packages/effects-gl/src/glChromaticAberrationEffect.ts: cannot infer return type for defaultGlChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCompositeEffect.ts`: defaultGlCompositeEffectRunner: upstream/packages/effects-gl/src/glCompositeEffect.ts: cannot infer return type for defaultGlCompositeEffectRunner
- **emission** `upstream/packages/effects-gl/src/glContactShadowsEffect.ts`: defaultGlContactShadowsEffectRunner: upstream/packages/effects-gl/src/glContactShadowsEffect.ts: cannot infer return type for defaultGlContactShadowsEffectRunner
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
- **emission** `upstream/packages/effects-gl/src/glTiltShiftEffect.ts`: defaultGlTiltShiftEffectRunner: upstream/packages/effects-gl/src/glTiltShiftEffect.ts: cannot infer return type for defaultGlTiltShiftEffectRunner
- **emission** `upstream/packages/effects-gl/src/glToneMapEffect.ts`: defaultGlToneMapEffectRunner: upstream/packages/effects-gl/src/glToneMapEffect.ts: cannot infer return type for defaultGlToneMapEffectRunner
- **emission** `upstream/packages/effects-gl/src/glVignetteEffect.ts`: defaultGlVignetteEffectRunner: upstream/packages/effects-gl/src/glVignetteEffect.ts: cannot infer return type for defaultGlVignetteEffectRunner
- **emission** `upstream/packages/effects-gl/src/glWhiteBalanceEffect.ts`: defaultGlWhiteBalanceEffectRunner: upstream/packages/effects-gl/src/glWhiteBalanceEffect.ts: cannot infer return type for defaultGlWhiteBalanceEffectRunner

### `@flighthq/effects-wgpu`

- **package** `upstream/packages/effects-wgpu/src`: Generated crate is missing 53 of 169 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-wgpu/src/wgpuBevelEffect.ts`: defaultWgpuBevelEffectRunner: upstream/packages/effects-wgpu/src/wgpuBevelEffect.ts: cannot infer return type for defaultWgpuBevelEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuBlendEffect.ts`: defaultWgpuBlendEffectRunner: upstream/packages/effects-wgpu/src/wgpuBlendEffect.ts: cannot infer return type for defaultWgpuBlendEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuBloomEffect.ts`: defaultWgpuBloomEffectRunner: upstream/packages/effects-wgpu/src/wgpuBloomEffect.ts: cannot infer return type for defaultWgpuBloomEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuBlurEffect.ts`: defaultWgpuBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuBlurEffect.ts: cannot infer return type for defaultWgpuBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCameraMotionBlurEffect.ts`: defaultWgpuCameraMotionBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuCameraMotionBlurEffect.ts: cannot infer return type for defaultWgpuCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuChromaticAberrationEffect.ts`: defaultWgpuChromaticAberrationEffectRunner: upstream/packages/effects-wgpu/src/wgpuChromaticAberrationEffect.ts: cannot infer return type for defaultWgpuChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCompositeEffect.ts`: defaultWgpuCompositeEffectRunner: upstream/packages/effects-wgpu/src/wgpuCompositeEffect.ts: cannot infer return type for defaultWgpuCompositeEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuContactShadowsEffect.ts`: defaultWgpuContactShadowsEffectRunner: upstream/packages/effects-wgpu/src/wgpuContactShadowsEffect.ts: cannot infer return type for defaultWgpuContactShadowsEffectRunner
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
- **emission** `upstream/packages/effects-wgpu/src/wgpuTiltShiftEffect.ts`: defaultWgpuTiltShiftEffectRunner: upstream/packages/effects-wgpu/src/wgpuTiltShiftEffect.ts: cannot infer return type for defaultWgpuTiltShiftEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuToneMapEffect.ts`: defaultWgpuToneMapEffectRunner: upstream/packages/effects-wgpu/src/wgpuToneMapEffect.ts: cannot infer return type for defaultWgpuToneMapEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuVignetteEffect.ts`: defaultWgpuVignetteEffectRunner: upstream/packages/effects-wgpu/src/wgpuVignetteEffect.ts: cannot infer return type for defaultWgpuVignetteEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuWhiteBalanceEffect.ts`: defaultWgpuWhiteBalanceEffectRunner: upstream/packages/effects-wgpu/src/wgpuWhiteBalanceEffect.ts: cannot infer return type for defaultWgpuWhiteBalanceEffectRunner

### `@flighthq/entity`

- **package** `upstream/packages/entity/src`: Generated crate is missing 3 of 15 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/entity/src/binding.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/entity/src/enableEntityRuntimeGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/entity/src/runtime.ts`: createEntityRuntime: entity runtime field binding is not in the source closure

### `@flighthq/filesystem`

- **package** `upstream/packages/filesystem/src`: Generated crate is missing 3 of 43 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/filesystem/src/filesystem.ts`: createWebFileSystemBackend: upstream/packages/filesystem/src/filesystem.ts:41:5: portableTask createWebFileSystemBackend.readTextFile: Portable task Rust lowering is not implemented.

### `@flighthq/font`

- **emission** `upstream/packages/font/src/_fontFaceLoad.ts`: _loadFontFaceFromBytes: upstream/packages/font/src/_fontFaceLoad.ts:6:1: portableTask _loadFontFaceFromBytes: async output type is not recovered
- **emission** `upstream/packages/font/src/fontStatus.ts`: whenFontsReady: upstream/packages/font/src/fontStatus.ts:8:3: await value type is not recovered

### `@flighthq/geolocation`

- **package** `upstream/packages/geolocation/src`: Generated crate is missing 3 of 12 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/geolocation/src/geolocation.ts`: createWebGeolocationBackend: typeof operand has no inferred Rust type: {"kind":"property","name":"clearWatch","object":{"kind":"identifier","name":"geo"},"optional":false}

### `@flighthq/glyphatlas`

- **emission** `upstream/packages/glyphatlas/src/enableGlyphAtlasGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/glyphatlas/src/glyphAtlasEntry.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/image`

- **emission** `upstream/packages/image/src/imageResourceFrom.ts`: loadImageResourceFromBlob: loadImageResourceFromBlob: portable task has a non-void output without a guaranteed return
- **emission** `upstream/packages/image/src/imageResourceReference.ts`: createImageResourceFailure: instanceof Rust lowering requires a portable typed-array constructor

### `@flighthq/image-codec`

- **emission** `upstream/packages/image-codec/src/explainImageDecodeFailure.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/image-codec/src/registerWebImageDecoders.ts`: decodeImageWithCanvas: upstream/packages/image-codec/src/registerWebImageDecoders.ts:18:45: portableTask decodeImageWithCanvas: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/image-codec/src/registerWebImageEncoders.ts`: createCanvasImageEncoder: upstream/packages/image-codec/src/registerWebImageEncoders.ts:16:10: portableTask createCanvasImageEncoder.anonymous:7c4dbd1c1e56: Portable task Rust lowering is not implemented.

### `@flighthq/interaction`

- **emission** `upstream/packages/interaction/src/cursorBackend.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/displayHitTests.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/displayObjectOverlap.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/enableInteractionGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/focusManager.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/hitTests.ts`: hitAreaContainsPoint: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/interaction/src/interactionManager.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/interactionSpatialIndex.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/nodeInteractionState.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/registerDefaultHitTests.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/registerShapeHitTest.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/registerSpriteHitTest.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/registerTextHitTest.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/spatialQuery.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/interaction/src/spriteHitTests.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (14 opaque sources exceeds the approved baseline of 9); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/intl`

- **emission** `upstream/packages/intl/src/cache.ts`: getCacheKey: typeof operand has no inferred Rust type: {"kind":"identifier","name":"locale"}
- **emission** `upstream/packages/intl/src/collator.ts`: getCollator: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.Collator")
- **emission** `upstream/packages/intl/src/datetime.ts`: formatDateValue: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.DateTimeFormat")
- **emission** `upstream/packages/intl/src/list.ts`: formatList: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.ListFormat")
- **emission** `upstream/packages/intl/src/number.ts`: formatCompactNumber: object literal requires an inferred structural type (target=unknown, properties=notation,spread)
- **emission** `upstream/packages/intl/src/plural.ts`: selectOrdinalCategory: object literal requires an inferred structural type (target=unknown, properties=type,spread)
- **emission** `upstream/packages/intl/src/relativeTime.ts`: formatRelativeTime: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.RelativeTimeFormat")

### `@flighthq/ipc`

- **package** `upstream/packages/ipc/src`: Generated crate is missing 4 of 17 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/ipc/src/ipc.ts`: createWebIpcBackend: upstream/packages/ipc/src/ipc.ts:59:14: taskReady output type is not recovered

### `@flighthq/layout`

- **emission** `upstream/packages/layout/src/anchorLayout.ts`: anchorLayoutResolver: upstream/packages/layout/src/anchorLayout.ts: cannot infer return type for anchorLayoutResolver
- **emission** `upstream/packages/layout/src/flexLayout.ts`: flexLayoutResolver: upstream/packages/layout/src/flexLayout.ts: cannot infer return type for flexLayoutResolver
- **emission** `upstream/packages/layout/src/gridLayout.ts`: gridLayoutResolver: upstream/packages/layout/src/gridLayout.ts: cannot infer return type for gridLayoutResolver

### `@flighthq/loader`

- **emission** `upstream/packages/loader/src/resourceLoader.ts`: _noopLoad: upstream/packages/loader/src/resourceLoader.ts:78:10: taskReady output type is not recovered

### `@flighthq/log`

- **package** `upstream/packages/log/src`: Generated crate is missing 3 of 62 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/log/src/log.ts`: createChildLogContext: object literal requires an inferred structural type (target=unknown, properties=spread,spread)

### `@flighthq/media`

- **emission** `upstream/packages/media/src/audioChannel.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 3); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/media/src/audioMixer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 3); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/media/src/enableAudioMixerGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 3); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/media/src/videoChannel.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 3); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/mediasession`

- **package** `upstream/packages/mediasession/src`: Generated crate is missing 3 of 10 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/mediasession/src/mediasession.ts`: createWebMediaSessionBackend: typeof operand has no inferred Rust type: {"kind":"property","name":"setPositionState","object":{"kind":"identifier","name":"session"},"optional":false}

### `@flighthq/menu`

- **package** `upstream/packages/menu/src`: Generated crate is missing 3 of 17 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/menu/src/menu.ts`: showContextMenu: taskThen Rust lowering is reserved for Pass 27 Stage 4

### `@flighthq/mesh`

- **emission** `upstream/packages/mesh/src/meshGeometry.ts`: createMeshGeometryRuntime: entity runtime field binding is not in the source closure
- **emission** `upstream/packages/mesh/src/meshGeometryAttributes.ts`: getAttributeByteLocation: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/mesh/src/meshGeometryLayout.ts`: convertMeshGeometryLayout: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/movieclip`

- **emission** `upstream/packages/movieclip/src/enableMovieClipGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/movieclip/src/movieClip.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/movieclip/src/spritesheetTimelineSource.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/net`

- **package** `upstream/packages/net/src`: Generated crate is missing 3 of 4 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/net/src/net.ts`: createWebNetBackend: upstream/packages/net/src/net.ts:21:5: portableTask createWebNetBackend.sendNetRequest: Portable task Rust lowering is not implemented.

### `@flighthq/node`

- **emission** `upstream/packages/node/src/boundsRectangle.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/hasBoundsRectangle.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/hasTransform2d.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/hierarchy.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/node.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/nodeColorAdjustment.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/nodeOrderList.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/nodeTransform2d.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/nodeTransform3d.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/revision.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/stageFit.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/node/src/traversal.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (12 opaque sources exceeds the approved baseline of 6); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/notification`

- **package** `upstream/packages/notification/src`: Generated crate is missing 5 of 26 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/notification/src/notification.ts`: createServiceWorkerNotificationBackend: upstream/packages/notification/src/notification.ts:81:3: portableTask createServiceWorkerNotificationBackend._show: Portable task Rust lowering is not implemented.

### `@flighthq/particles-formats`

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

### `@flighthq/path`

- **emission** `upstream/packages/path/src/pathMorphGeometry.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/path/src/tessellateStrokePath.ts`: appendRoundCap: spread Rust lowering is not implemented

### `@flighthq/path-boolean`

- **package** `upstream/packages/path-boolean/src`: Generated crate is missing 1 of 12 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/path-boolean/src/martinezKernel.ts`: buildArrangement: new-expression Rust lowering is not implemented: event_heap

### `@flighthq/permissions`

- **package** `upstream/packages/permissions/src`: Generated crate is missing 4 of 11 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/permissions/src/enablePermissionGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/permissions/src/permission.ts`: explainPermissionState: portable task try/catch lowering is reserved for Pass 27 Stage 4

### `@flighthq/physics2d`

- **emission** `upstream/packages/physics2d/src/colliderTransform.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics2d/src/joints.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics2d/src/step.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics2d/src/stepValidation.ts`: isPhysics2DContactValid: dynamic for-in Rust enumeration is not implemented
- **emission** `upstream/packages/physics2d/src/world.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics2d/src/worldQueries.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/picking`

- **emission** `upstream/packages/picking/src/pickScene3D.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/picking/src/sceneHitAttributes.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/power`

- **package** `upstream/packages/power/src`: Generated crate is missing 5 of 19 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/power/src/power.ts`: createWebPowerBackend: taskThen Rust lowering is reserved for Pass 27 Stage 4

### `@flighthq/quadbatch`

- **emission** `upstream/packages/quadbatch/src/quadBatch.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/render`

- **package** `upstream/packages/render/src`: Generated crate is missing 12 of 73 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render/src/enableColorAdjustmentGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/enableColorAdjustments.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/enableSceneRenderGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/explainScene2DRender.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderAppearance.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderCache.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderProxy.ts`: resolveRenderProxyRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/render/src/renderProxyAdapter.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderQueue.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderRegistryGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderRegistrySignals.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderTarget.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderTransform2d.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/renderViewport.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/render/src/sceneRender.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 10); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/render-gl`

- **package** `upstream/packages/render-gl/src`: Generated crate is missing 6 of 115 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-gl/src/glMaterialRegistry.ts`: resolveGlMaterialRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/render-gl/src/glRenderPass.ts`: beginGlRenderPass: optional property at has no inferred receiver field
- **emission** `upstream/packages/render-gl/src/glRenderState.ts`: createGlRenderState: object literal requires an inferred structural type (target=unknown, properties=alpha,antialias,powerPreference,stencil,spread)
- **emission** `upstream/packages/render-gl/src/glTextureResolver.ts`: resolveGlTexture: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}

### `@flighthq/render-wgpu`

- **package** `upstream/packages/render-wgpu/src`: Generated crate is missing 9 of 108 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-wgpu/src/wgpuExternalImageSource.ts`: isWgpuExternalImageSourceUnavailableError: instanceof Rust lowering requires a portable typed-array constructor
- **emission** `upstream/packages/render-wgpu/src/wgpuMaterialRegistry.ts`: resolveWgpuMaterialRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/render-wgpu/src/wgpuRenderState.ts`: createWgpuRenderState: portable task throw/rejection lowering is reserved for Pass 27 Stage 4
- **emission** `upstream/packages/render-wgpu/src/wgpuSurface.ts`: createBitmapFromWgpuRenderState: portable task throw/rejection lowering is reserved for Pass 27 Stage 4
- **emission** `upstream/packages/render-wgpu/src/wgpuTestHelper.ts`: makeAdapter: upstream/packages/render-wgpu/src/wgpuTestHelper.ts:124:26: taskReady output type is not recovered
- **emission** `upstream/packages/render-wgpu/src/wgpuTextureResolver.ts`: resolveWgpuTexture: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}

### `@flighthq/scene2d`

- **package** `upstream/packages/scene2d/src`: Generated crate is missing 8 of 33 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d/src/displayContainer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d/src/displayObject.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d/src/displayObjectAnimation.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d/src/htmlView.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d/src/scene2d.ts`: createScene2DRuntime: entity runtime field binding is not in the source closure
- **emission** `upstream/packages/scene2d/src/sceneKindUsage.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d/src/sprite.ts`: computeSpriteLocalBoundsRectangle: optional property version has no inferred receiver field

### `@flighthq/scene2d-canvas`

- **package** `upstream/packages/scene2d-canvas/src`: Generated crate is missing 10 of 119 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d-canvas/src/canvasRenderState.ts`: createCanvasRenderState: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasRenderTarget.ts`: beginCanvasRenderPass: optional element access requires an inferred nullable collection
- **emission** `upstream/packages/scene2d-canvas/src/canvasShape.ts`: renderCanvasShapeCommands: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"arguments":[{"kind":"identifier","name":"state"}],"callee":{"kind":"identifier","name":"getRenderStateRuntime"},"kind":"call","optional":false,"typeArguments":[]},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasTextureResolver.ts`: connectCanvasTextureResolverMisses: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}

### `@flighthq/scene2d-formats`

- **package** `upstream/packages/scene2d-formats/src`: Generated crate is missing 2 of 11 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d-formats/src/lottieDocument.ts`: createLottieTrack: spread Rust lowering is not implemented
- **emission** `upstream/packages/scene2d-formats/src/riveAnimation.ts`: createRiveTypedChannel: spread Rust lowering is not implemented
- **emission** `upstream/packages/scene2d-formats/src/riveAssets.ts`: readRiveNumber: typeof operand has no inferred Rust type: {"kind":"property","name":"value","object":{"kind":"identifier","name":"property"},"optional":false}
- **emission** `upstream/packages/scene2d-formats/src/riveClipping.ts`: readRiveNumber: typeof operand has no inferred Rust type: {"kind":"property","name":"value","object":{"kind":"identifier","name":"property"},"optional":false}
- **emission** `upstream/packages/scene2d-formats/src/riveDocument.ts`: _floatView: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene2d-formats/src/riveDrawOrder.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/riveLayout.ts`: findRiveLayoutComponents: typeof operand has no inferred Rust type: {"kind":"property","name":"value","object":{"kind":"identifier","name":"styleProperty"},"optional":false}
- **emission** `upstream/packages/scene2d-formats/src/riveScene2D.ts`: readRiveNumber: typeof operand has no inferred Rust type: {"kind":"property","name":"value","object":{"kind":"identifier","name":"property"},"optional":false}
- **emission** `upstream/packages/scene2d-formats/src/riveScene2DDocument.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/riveShapePaint.ts`: trimRivePaths: spread Rust lowering is not implemented
- **emission** `upstream/packages/scene2d-formats/src/riveShapePath.ts`: readRiveDouble: typeof operand has no inferred Rust type: {"kind":"property","name":"value","object":{"kind":"identifier","name":"property"},"optional":false}
- **emission** `upstream/packages/scene2d-formats/src/riveSkeleton.ts`: readRiveNumber: typeof operand has no inferred Rust type: {"kind":"property","name":"value","object":{"kind":"identifier","name":"property"},"optional":false}
- **emission** `upstream/packages/scene2d-formats/src/riveSkin.ts`: readRiveNumber: typeof operand has no inferred Rust type: {"kind":"property","name":"value","object":{"kind":"identifier","name":"property"},"optional":false}
- **emission** `upstream/packages/scene2d-formats/src/riveSolo.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/riveStateMachine.ts`: readRiveNumber: typeof operand has no inferred Rust type: {"kind":"property","name":"value","object":{"kind":"identifier","name":"property"},"optional":false}
- **emission** `upstream/packages/scene2d-formats/src/riveText.ts`: readRiveNumber: typeof operand has no inferred Rust type: {"kind":"property","name":"value","object":{"kind":"identifier","name":"property"},"optional":false}
- **emission** `upstream/packages/scene2d-formats/src/svgDocument.ts`: createSvgTextNode: object field enabled is not initialized by its structural spreads

### `@flighthq/scene2d-gl`

- **package** `upstream/packages/scene2d-gl/src`: Generated crate is missing 9 of 97 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d-gl/src/glColorAdjustmentMaterialFeature.ts`: isTintMaterialData: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/scene2d-gl/src/glMeshShapeRenderer.ts`: defaultGlMeshShapeRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"arguments":[{"kind":"identifier","name":"state"}],"callee":{"kind":"identifier","name":"getGlRenderStateRuntime"},"kind":"call","optional":false,"typeArguments":[]},"optional":false}
- **emission** `upstream/packages/scene2d-gl/src/glRasterShapeRenderer.ts`: drawGlRasterShape: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/scene2d-gl/src/glScale9Shape.ts`: drawGlScale9Shape: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"arguments":[{"kind":"identifier","name":"state"}],"callee":{"kind":"identifier","name":"getGlRenderStateRuntime"},"kind":"call","optional":false,"typeArguments":[]},"optional":false}
- **emission** `upstream/packages/scene2d-gl/src/glVelocity.ts`: defaultGlNode2DVelocityWriter: upstream/packages/scene2d-gl/src/glVelocity.ts: cannot infer return type for defaultGlNode2DVelocityWriter

### `@flighthq/scene2d-resources`

- **emission** `upstream/packages/scene2d-resources/src/builtInScene2DDocumentImporters.ts`: _decoder: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts`: loadScene2DAudioResources: upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:27:21: await value type is not recovered
- **emission** `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts`: loadScene2DImageResources: upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:27:18: await value type is not recovered
- **emission** `upstream/packages/scene2d-resources/src/resolveScene2DResources.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-resources/src/scene2DDocumentImporterRegistry.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-resources/src/scene2DDocumentSource.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- **emission** `upstream/packages/scene2d-resources/src/scene2DSlotReference.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/scene2d-wgpu`

- **package** `upstream/packages/scene2d-wgpu/src`: Generated crate is missing 9 of 99 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuColorAdjustmentMaterialFeature.ts`: isTintMaterialData: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuMeshShapeRenderer.ts`: defaultWgpuMeshShapeRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"arguments":[{"kind":"identifier","name":"state"}],"callee":{"kind":"identifier","name":"getWgpuRenderStateRuntime"},"kind":"call","optional":false,"typeArguments":[]},"optional":false}
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuRasterShapeRenderer.ts`: drawWgpuRasterShape: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuScale9Shape.ts`: drawWgpuScale9Shape: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"arguments":[{"kind":"identifier","name":"state"}],"callee":{"kind":"identifier","name":"getWgpuRenderStateRuntime"},"kind":"call","optional":false,"typeArguments":[]},"optional":false}
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuVelocity.ts`: defaultWgpuNode2DVelocityWriter: upstream/packages/scene2d-wgpu/src/wgpuVelocity.ts: cannot infer return type for defaultWgpuNode2DVelocityWriter

### `@flighthq/scene3d`

- **emission** `upstream/packages/scene3d/src/billboard.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/billboardCamera.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/mesh.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/prepareScene3DMorph.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/scene.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/sceneAnimation.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/sceneDocument.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/sceneKindUsage.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/sceneMaterial.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/sceneNode.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/sceneNodeAppearance.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/sceneNodeBounds.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/sceneNodeCulling.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/sceneNodeDispose.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/sceneNodeTransform.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/scene3d-formats`

- **emission** `upstream/packages/scene3d-formats/src/awd2Parse.ts`: parseAwd2: spread Rust lowering is not implemented
- **emission** `upstream/packages/scene3d-formats/src/gltfParse.ts`: readAccessor: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/md2Parse.ts`: parseMd2: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/md5AnimParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/md5Parse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/objParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/sceneSkeleton.ts`: findScene3DSkeletonJoints: spread Rust lowering is not implemented
- **emission** `upstream/packages/scene3d-formats/src/threeDsParse.ts`: parse3ds: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/scene3d-resources`

- **emission** `upstream/packages/scene3d-resources/src/awd2Load.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- **emission** `upstream/packages/scene3d-resources/src/enableScene3DResourceFailureGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-resources/src/gltfLoad.ts`: loadScene3DDocumentFromGltfUrl: portable task try/catch lowering is reserved for Pass 27 Stage 4
- **emission** `upstream/packages/scene3d-resources/src/imageResourceFetch.ts`: fetchWebImageResource: portable task try/catch lowering is reserved for Pass 27 Stage 4
- **emission** `upstream/packages/scene3d-resources/src/loadScene3DResources.ts`: loadScene3DResources: upstream/packages/scene3d-resources/src/loadScene3DResources.ts:46:3: await value type is not recovered
- **emission** `upstream/packages/scene3d-resources/src/md2Load.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- **emission** `upstream/packages/scene3d-resources/src/md5Load.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- **emission** `upstream/packages/scene3d-resources/src/objLoad.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- **emission** `upstream/packages/scene3d-resources/src/resolveScene3DResources.ts`: requestWorkingResolutions: taskThen Rust lowering is reserved for Pass 27 Stage 4
- **emission** `upstream/packages/scene3d-resources/src/revealScene3DResourcesOnResolve.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-resources/src/sceneDocumentSource.ts`: loadScene3DDocumentBytesFromUrl: instanceof Rust lowering requires a portable typed-array constructor
- **emission** `upstream/packages/scene3d-resources/src/sceneMaterialTextureRegistry.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-resources/src/sceneResourceResolver.ts`: createScene3DResourceResolver: object literal requires an inferred structural type (target=unknown, properties=fetch,registry,computedProperty)
- **emission** `upstream/packages/scene3d-resources/src/threeDsLoad.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.

### `@flighthq/scene3d-wgpu`

- **package** `upstream/packages/scene3d-wgpu/src`: Generated crate is missing 32 of 157 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene3d-wgpu/src/customShaderWgpuMeshMaterialRenderer.ts`: ensureCustomTextureBindGroup: dynamic for-in Rust enumeration is not implemented
- **emission** `upstream/packages/scene3d-wgpu/src/enableWgpuScene3DCustomShaderGuards.ts`: hasBinding: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-wgpu/src/wgpuMeshPipeline.ts`: stashWgpuUvTransform: in-operator requires a static property name or an opaque host receiver

### `@flighthq/sdk`

- **package** `upstream/packages/sdk/src`: Generated crate is missing 6295 of 6295 upstream exports across 15 manifest lanes; re-export or declaration synthesis is required.

### `@flighthq/sensors`

- **package** `upstream/packages/sensors/src`: Generated crate is missing 10 of 32 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/sensors/src/sensors.ts`: createWebSensorsBackend: upstream/packages/sensors/src/sensors.ts:347:5: portableTask createWebSensorsBackend.requestPermission: Portable task Rust lowering is not implemented.

### `@flighthq/shading`

- **emission** `upstream/packages/shading/src/modifierRegistry.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shading/src/orderModifierStack.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shading/src/registerBuiltInModifiers.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/shape`

- **emission** `upstream/packages/shape/src/morphShape.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shape/src/morphShapeAnimation.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shape/src/morphShapePaint.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shape/src/scale9Shape.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shape/src/shape.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shape/src/shapeCommands.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/shape-formats`

- **emission** `upstream/packages/shape-formats/src/shapeJson.ts`: formatShapeJson: JSON.stringify requires a portable scalar or structural array

### `@flighthq/share`

- **package** `upstream/packages/share/src`: Generated crate is missing 3 of 15 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/share/src/share.ts`: createWebShareBackend: upstream/packages/share/src/share.ts:52:5: portableTask createWebShareBackend.share: Portable task Rust lowering is not implemented.

### `@flighthq/shell`

- **package** `upstream/packages/shell/src`: Generated crate is missing 3 of 14 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shell/src/shell.ts`: createWebShellBackend: upstream/packages/shell/src/shell.ts:17:5: portableTask createWebShellBackend.moveItemsToTrash: Portable task Rust lowering is not implemented.

### `@flighthq/shortcut`

- **package** `upstream/packages/shortcut/src`: Generated crate is missing 4 of 33 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shortcut/src/enableShortcutGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shortcut/src/explainGlobalShortcutRegistration.ts`: explainGlobalShortcutRegistration: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/shortcut/src/shortcut.ts`: parseAcceleratorDetailed: in-operator requires a static property name or an opaque host receiver

### `@flighthq/skeleton2d`

- **emission** `upstream/packages/skeleton2d/src/deformAnimationTarget2D.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d/src/enableSkeleton2DGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d/src/skeleton2d.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d/src/skeleton2dAnimationTarget.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d/src/skeleton2dDrawOrderTarget.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d/src/slotDeform2D.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/skeleton2d-formats`

- **package** `upstream/packages/skeleton2d-formats/src`: Generated crate is missing 1 of 7 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesParse.ts`: parseDragonBonesSkeleton: typeof operand has no inferred Rust type: {"kind":"identifier","name":"first"}
- **emission** `upstream/packages/skeleton2d-formats/src/spineBinaryParse.ts`: readSpineBinaryDrawOrderTimeline: spread Rust lowering is not implemented
- **emission** `upstream/packages/skeleton2d-formats/src/spineBinaryReader.ts`: createSpineBinaryReader: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/skeleton2d-formats/src/spineParse.ts`: parseSpineDrawOrderTimeline: typeof operand has no inferred Rust type: {"kind":"identifier","name":"frame"}

### `@flighthq/skeleton3d`

- **emission** `upstream/packages/skeleton3d/src/prepareScene3DSkinning.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton3d/src/skeleton3d.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton3d/src/updateMeshDeformation.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton3d/src/updateMeshSkin.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/snapshot`

- **emission** `upstream/packages/snapshot/src/enableSnapshotGuards.ts`: nonPlainSnapshotKind: instanceof Rust lowering requires a portable typed-array constructor

### `@flighthq/socket`

- **emission** `upstream/packages/socket/src/enableSocketGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/socket/src/socket.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/spritesheet-formats`

- **emission** `upstream/packages/spritesheet-formats/src/asepriteSerialize.ts`: serializeAsepriteSpritesheet: JSON.stringify requires a portable scalar or structural array
- **emission** `upstream/packages/spritesheet-formats/src/texturePackerSerialize.ts`: serializeTexturePackerSpritesheet: JSON.stringify requires a portable scalar or structural array

### `@flighthq/statechart`

- **emission** `upstream/packages/statechart/src/enableStatechartGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/statechart/src/statechart.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/statusbar`

- **package** `upstream/packages/statusbar/src`: Generated crate is missing 4 of 18 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/statusbar/src/statusbar.ts`: _styleStack: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"handle","optional":false,"type":{"arguments":[],"kind":"named","name":"StatusBarStyleEntryHandle"}},{"name":"entry","optional":false,"type":{"arguments":[],"kind":"named","name":"StatusBarStyleEntry"}}],"kind":"anonymous"}

### `@flighthq/storage`

- **package** `upstream/packages/storage/src`: Generated crate is missing 3 of 39 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/storage/src/storage.ts`: setStorageJSON: JSON.stringify requires a portable scalar or structural array

### `@flighthq/swf`

- **emission** `upstream/packages/swf/src/enableSwfGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/swf/src/swfDocument.ts`: readSwfFile: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfEditText.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/swf/src/swfFilter.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/swf/src/swfFrameAction.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/swf/src/swfMorphShape.ts`: createSwfMorphShape: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfReader.ts`: SwfReader: upstream/packages/swf/src/swfReader.ts:5: class methods and static fields are not implemented for SwfReader
- **emission** `upstream/packages/swf/src/swfShape.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/swf/src/swfShapeTestHelper.ts`: ShapeWriter: upstream/packages/swf/src/swfShapeTestHelper.ts:5: class methods and static fields are not implemented for ShapeWriter
- **emission** `upstream/packages/swf/src/swfText.ts`: readSwfFontGlyphOutlineSource: new-expression Rust lowering is not implemented: swf_reader

### `@flighthq/text`

- **package** `upstream/packages/text/src`: Generated crate is missing 3 of 86 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/text/src/nativeText.ts`: patchNativeTextStyle: multiple object spreads require ordered Rust lowering

### `@flighthq/text-markup`

- **emission** `upstream/packages/text-markup/src/textMarkup.ts`: handleMarkupToken: multiple object spreads require ordered Rust lowering

### `@flighthq/textshaper`

- **package** `upstream/packages/textshaper/src`: Generated crate is missing 2 of 33 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textshaper/src/enableTextShaperGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/textshaper/src/textShaper.ts`: setTextShaperBackend: optional call requires an inferred nullable function: {"kind":"identifier","name":"_textShaperBackendHook"}
- **emission** `upstream/packages/textshaper/src/textShaperCache.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/textshaper/src/textShaperItemize.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/texture`

- **emission** `upstream/packages/texture/src/cubeTexture.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/texture/src/texture.ts`: createTexture: optional property colorSpace has no inferred receiver field
- **emission** `upstream/packages/texture/src/videoTexture.ts`: createVideoTexture: object literal requires an inferred structural type (target=unknown, properties=spread,dimension,source,version)

### `@flighthq/texture-formats`

- **emission** `upstream/packages/texture-formats/src/byteReader.ts`: createByteReader: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/texture-formats/src/explainTextureContainerParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/texture-formats/src/parseAtf.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/texture-formats/src/parseBasis.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/texture-formats/src/parseDds.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/texture-formats/src/parseKtx2.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (5 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/textureatlas`

- **emission** `upstream/packages/textureatlas/src/textureAtlasFrom.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.

### `@flighthq/textureatlas-formats`

- **emission** `upstream/packages/textureatlas-formats/src/textureAtlasAsepriteParse.ts`: applyAsepriteFrame: typeof operand has no inferred Rust type: {"kind":"identifier","name":"frame"}
- **emission** `upstream/packages/textureatlas-formats/src/textureAtlasPackerParse.ts`: applyFrame: typeof operand has no inferred Rust type: {"kind":"identifier","name":"frame"}

### `@flighthq/tilemap`

- **emission** `upstream/packages/tilemap/src/tilemap.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/tilemap-formats`

- **emission** `upstream/packages/tilemap-formats/src/tiledJsonParse.ts`: boolField: typeof operand has no inferred Rust type: {"kind":"identifier","name":"value"}
- **emission** `upstream/packages/tilemap-formats/src/tiledProject.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tilemap-formats/src/tiledTmxFormat.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tilemap-formats/src/tiledXmlParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 2); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/tray`

- **emission** `upstream/packages/tray/src/enableTrayGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tray/src/tray.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/tween`

- **emission** `upstream/packages/tween/src/timer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tween/src/tween.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tween/src/tweenProgress.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/updater`

- **emission** `upstream/packages/updater/src/updater.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/video`

- **emission** `upstream/packages/video/src/videoResourceFrom.ts`: loadVideoResourceFromBlob: portable task try/catch lowering is reserved for Pass 27 Stage 4

### `@flighthq/webcam`

- **package** `upstream/packages/webcam/src`: Generated crate is missing 4 of 9 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/webcam/src/webcam.ts`: createWebWebcamBackend: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/webcam/src/webcamStream.ts`: createWebcamStreamEntity: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state

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

### `@flighthq/clock`

- **E0308** `generated/candidates/flighthq-clock/src/clock.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/clock.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/clock.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/clock.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/clock.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/clock.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-clock/src/clock.rs`: mismatched types

### `@flighthq/importdiagnostics`

- **E0308** `generated/candidates/flighthq-importdiagnostics/src/import_diagnostic_collector.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-importdiagnostics/src/import_diagnostic_text.rs`: no field `sort` on type `()`
- **E0277** `generated/candidates/flighthq-importdiagnostics/src/import_diagnostic_text.rs`: can't compare `std::string::String` with `&std::string::String`
- **E0277** `generated/candidates/flighthq-importdiagnostics/src/import_diagnostic_text.rs`: `FlightUnion2<bool, ...>` doesn't implement `std::fmt::Display`
- **E0368** `generated/candidates/flighthq-importdiagnostics/src/import_diagnostic_text.rs`: binary assignment operation `+=` cannot be applied to type `&str`
- **E0277** `generated/candidates/flighthq-importdiagnostics/src/import_diagnostic_text.rs`: `OpaqueHostValue` doesn't implement `std::fmt::Display`

### `@flighthq/input`

- **E0432** `generated/candidates/flighthq-input/src/input_manager.rs`: unresolved import `flighthq_host_signals`
- **E0277** `generated/candidates/flighthq-input/src/input_manager.rs`: the trait bound `FlightTask<bool>: Default` is not satisfied
- **E0308** `generated/candidates/flighthq-input/src/input_manager.rs`: mismatched types

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

### `@flighthq/timeline`

- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be applied to values that implement `Try`
- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
- **E0070** `generated/candidates/flighthq-timeline/src/timeline.rs`: invalid left-hand side of assignment
- **E0609** `generated/candidates/flighthq-timeline/src/timeline.rs`: no field `set` on type `Option<Vec<(f64, Arc<Mutex<Box<...>>>)>>`
- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be applied to values that implement `Try`
- **E0277** `generated/candidates/flighthq-timeline/src/timeline.rs`: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
- **E0070** `generated/candidates/flighthq-timeline/src/timeline.rs`: invalid left-hand side of assignment
- **E0308** `generated/candidates/flighthq-timeline/src/timeline.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-timeline/src/timeline.rs`: no method named `unwrap` found for reference `&Mutex<Box<dyn FnMut(..., f64) + Send>>` in the current scope
- **E0308** `generated/candidates/flighthq-timeline/src/timeline.rs`: mismatched types
- **E0596** `generated/candidates/flighthq-timeline/src/timeline.rs`: cannot borrow `timeline.frame_scripts` as mutable, as it is behind a `&` reference
- **E0596** `generated/candidates/flighthq-timeline/src/timeline.rs`: cannot borrow `timeline.frame_scripts` as mutable, as it is behind a `&` reference

### `@flighthq/xml`

- **E0425** `generated/candidates/flighthq-xml/src/xml_parse.rs`: cannot find value `string` in this scope
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0599** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no method named `is_some` found for enum `OpaqueHostValue` in the current scope
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: can't compare `std::string::String` with `OpaqueHostValue`
- **E0070** `generated/candidates/flighthq-xml/src/xml_parse.rs`: invalid left-hand side of assignment
- **E0599** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no method named `is_none` found for struct `std::string::String` in the current scope
- **E0599** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no method named `is_some` found for struct `std::string::String` in the current scope
- **E0599** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no method named `is_some` found for struct `std::string::String` in the current scope
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0609** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no field `index_of` on type `std::string::String`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0600** `generated/candidates/flighthq-xml/src/xml_parse.rs`: cannot apply unary operator `!` to type `std::string::String`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0368** `generated/candidates/flighthq-xml/src/xml_parse.rs`: binary assignment operation `+=` cannot be applied to type `&str`
- **E0609** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no field `index_of` on type `std::string::String`
- **E0615** `generated/candidates/flighthq-xml/src/xml_parse.rs`: attempted to take value of method `trim` on type `std::string::String`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0609** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no field `index_of` on type `std::string::String`
- **E0368** `generated/candidates/flighthq-xml/src/xml_parse.rs`: binary assignment operation `+=` cannot be applied to type `&str`
- **E0609** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no field `index_of` on type `std::string::String`
- **E0369** `generated/candidates/flighthq-xml/src/xml_parse.rs`: cannot add `std::string::String` to `&str`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0609** `generated/candidates/flighthq-xml/src/xml_parse.rs`: no field `to_lower_case` on type `std::string::String`
- **E0368** `generated/candidates/flighthq-xml/src/xml_parse.rs`: binary assignment operation `+=` cannot be applied to type `&str`
- **E0277** `generated/candidates/flighthq-xml/src/xml_parse.rs`: the type `str` cannot be indexed by `usize`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0369** `generated/candidates/flighthq-xml/src/xml_parse.rs`: cannot add `std::string::String` to `&str`
- **E0308** `generated/candidates/flighthq-xml/src/xml_parse.rs`: mismatched types
- **E0070** `generated/candidates/flighthq-xml/src/xml_parse.rs`: invalid left-hand side of assignment
- **E0599** `generated/candidates/flighthq-xml/src/xml_query.rs`: no method named `is_some` found for struct `std::string::String` in the current scope
- **E0599** `generated/candidates/flighthq-xml/src/xml_query.rs`: no method named `is_none` found for struct `std::string::String` in the current scope
- **E0425** `generated/candidates/flighthq-xml/src/xml_query.rs`: cannot find function `number` in this scope
- **E0609** `generated/candidates/flighthq-xml/src/xml_query.rs`: no field `filter` on type `Vec<XmlElement>`
- **E0308** `generated/candidates/flighthq-xml/src/xml_query.rs`: mismatched types

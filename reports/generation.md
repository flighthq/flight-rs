# Automatic Rust Generation

Upstream commit: `a62784923f3be814463286c9fb28edfe6ab15789`

| Metric | Count |
| --- | ---: |
| Inventoried packages | 163 |
| Default-generated packages | 157 |
| Emittable packages | 45 |
| Blocked packages | 112 |
| Compiled candidates | 9 |
| Compile-blocked candidates | 3 |
| Dependency-blocked candidates | 31 |
| Cultivated packages | 1 |
| Host-bound packages | 4 |
| Excluded packages | 1 |
| Source/package blockers | 679 |
| Eligible task constructions | 391 |
| Portable executable task constructions | 33 |
| Host-placeholder task constructions | 0 |
| Unsupported task constructions | 358 |
| Eligible async scopes | 263 |
| Portable executable async scopes | 29 |
| Host-placeholder async scopes | 0 |
| Unsupported async scopes | 234 |
| Async scopes matching the legacy body-erasure path | 165 |
| Upstream conformance files translated and passing | 3/2062 |
| Generated conformance cases passing | 45/45 |

| Package | Disposition | Status | Candidate | Sources emitted/attempted | API generated/expected | Missing | Dependents direct/transitive | Opaque sources | Blockers | Target |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `@flighthq/abc` | generated | blocked | source-blocked | 3/4 | 2/2 | 0 | 2/4 | 0 | 1 | no |
| `@flighthq/accessibility` | generated | emittable | compiled | 3/3 | 6/6 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/adjustments` | generated | emittable | dependency-blocked | 22/22 | 69/69 | 0 | 9/56 | 0 | 0 | no |
| `@flighthq/animation` | generated | emittable | dependency-blocked | 13/13 | 67/67 | 0 | 8/42 | 1 | 0 | no |
| `@flighthq/app` | generated | blocked | source-blocked | 4/6 | 126/127 | 1 | 5/6 | 1 | 3 | no |
| `@flighthq/assets` | generated | blocked | source-blocked | 4/5 | 18/19 | 1 | 1/1 | 0 | 2 | no |
| `@flighthq/audio` | generated | blocked | source-blocked | 5/8 | 25/35 | 10 | 5/11 | 1 | 4 | no |
| `@flighthq/binpack` | generated | blocked | source-blocked | 3/4 | 4/4 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/bitmap` | cultivated | cultivated | not-applicable | 0/0 | 0/112 | 112 | 8/18 | 0 | 0 | partial |
| `@flighthq/bitmapfont` | generated | blocked | source-blocked | 6/8 | 17/18 | 1 | 2/3 | 0 | 3 | no |
| `@flighthq/bitmapfont-formats` | generated | blocked | source-blocked | 9/11 | 20/22 | 2 | 2/2 | 1 | 3 | no |
| `@flighthq/bitmaptext` | generated | blocked | source-blocked | 4/6 | 22/26 | 4 | 1/1 | 0 | 3 | no |
| `@flighthq/camera` | generated | emittable | dependency-blocked | 21/21 | 55/55 | 0 | 9/30 | 0 | 0 | no |
| `@flighthq/camera-controls` | generated | blocked | source-blocked | 6/7 | 31/31 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/capture` | generated | blocked | source-blocked | 3/4 | 5/12 | 7 | 2/2 | 1 | 2 | no |
| `@flighthq/clip` | generated | emittable | dependency-blocked | 4/4 | 36/36 | 0 | 4/9 | 0 | 0 | no |
| `@flighthq/clipboard` | generated | emittable | dependency-blocked | 3/3 | 27/27 | 0 | 4/4 | 0 | 0 | no |
| `@flighthq/clock` | generated | emittable | dependency-blocked | 4/4 | 15/15 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/collision` | generated | blocked | source-blocked | 31/38 | 135/141 | 6 | 5/5 | 0 | 8 | no |
| `@flighthq/color` | generated | emittable | compiled | 11/11 | 31/31 | 0 | 20/59 | 0 | 0 | no |
| `@flighthq/command` | generated | blocked | source-blocked | 7/8 | 36/36 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/compression` | generated | blocked | source-blocked | 4/7 | 7/14 | 7 | 4/7 | 0 | 4 | no |
| `@flighthq/connectivity` | generated | emittable | dependency-blocked | 3/3 | 12/12 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/debug` | generated | emittable | dependency-blocked | 4/4 | 10/10 | 0 | 1/1 | 1 | 0 | no |
| `@flighthq/device` | generated | blocked | source-blocked | 2/3 | 10/10 | 0 | 3/6 | 0 | 1 | no |
| `@flighthq/dialog` | generated | blocked | source-blocked | 3/5 | 16/16 | 0 | 6/7 | 0 | 2 | no |
| `@flighthq/easing` | generated | emittable | promoted | 23/23 | 52/54 | 2 | 4/9 | 0 | 0 | full |
| `@flighthq/effects` | generated | blocked | source-blocked | 76/77 | 212/213 | 1 | 5/8 | 0 | 2 | no |
| `@flighthq/effects-canvas` | host-backend | blocked | source-blocked | 11/30 | 69/88 | 24 | 1/1 | 4 | 20 | no |
| `@flighthq/effects-gl` | host-backend | blocked | source-blocked | 12/61 | 129/187 | 58 | 1/1 | 7 | 50 | no |
| `@flighthq/effects-wgpu` | host-backend | blocked | source-blocked | 14/60 | 131/187 | 56 | 2/2 | 4 | 47 | no |
| `@flighthq/encoding` | generated | emittable | compiled | 3/3 | 2/2 | 0 | 4/6 | 0 | 0 | no |
| `@flighthq/entity` | generated | blocked | source-blocked | 9/10 | 20/22 | 2 | 119/141 | 1 | 2 | no |
| `@flighthq/filesystem` | generated | blocked | source-blocked | 2/3 | 40/40 | 0 | 3/6 | 0 | 1 | no |
| `@flighthq/flow` | generated | emittable | dependency-blocked | 4/4 | 13/13 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/font` | generated | blocked | source-blocked | 7/9 | 12/17 | 5 | 3/7 | 0 | 3 | no |
| `@flighthq/font-formats` | generated | blocked | source-blocked | 3/17 | 3/58 | 55 | 1/1 | 0 | 15 | no |
| `@flighthq/geolocation` | generated | emittable | dependency-blocked | 4/4 | 8/8 | 0 | 3/6 | 0 | 0 | no |
| `@flighthq/geometry` | generated | emittable | dependency-blocked | 30/30 | 408/408 | 0 | 47/75 | 0 | 0 | no |
| `@flighthq/gizmo` | generated | blocked | source-blocked | 3/5 | 18/18 | 0 | 1/1 | 0 | 2 | no |
| `@flighthq/glyphatlas` | generated | blocked | source-blocked | 9/10 | 18/19 | 1 | 2/6 | 2 | 2 | no |
| `@flighthq/gui` | generated | blocked | source-blocked | 4/20 | 104/106 | 2 | 1/1 | 0 | 17 | no |
| `@flighthq/haptics` | generated | emittable | compiled | 3/3 | 10/10 | 0 | 3/6 | 0 | 0 | no |
| `@flighthq/host` | generated | blocked | source-blocked | 5/6 | 59/67 | 8 | 5/6 | 0 | 2 | no |
| `@flighthq/host-capacitor` | host-bound | host-bound | not-applicable | 0/0 | 0/87 | 87 | 0/0 | 0 | 0 | no |
| `@flighthq/host-electron` | host-bound | host-bound | not-applicable | 0/0 | 0/159 | 159 | 0/0 | 0 | 0 | no |
| `@flighthq/host-tauri` | host-bound | host-bound | not-applicable | 0/0 | 0/84 | 84 | 0/0 | 0 | 0 | no |
| `@flighthq/host-web` | generated | blocked | source-blocked | 45/112 | 155/237 | 82 | 4/4 | 0 | 68 | no |
| `@flighthq/image` | generated | blocked | source-blocked | 5/7 | 19/28 | 9 | 15/36 | 0 | 3 | partial |
| `@flighthq/image-codec` | generated | emittable | compile-blocked | 5/5 | 4/4 | 0 | 5/37 | 0 | 0 | no |
| `@flighthq/importdiagnostics` | generated | emittable | compiled | 4/4 | 3/3 | 0 | 10/14 | 0 | 0 | no |
| `@flighthq/input` | generated | emittable | dependency-blocked | 3/3 | 34/34 | 0 | 2/6 | 0 | 0 | partial |
| `@flighthq/interaction` | generated | blocked | source-blocked | 17/19 | 76/92 | 16 | 3/3 | 2 | 3 | no |
| `@flighthq/intl` | generated | blocked | source-blocked | 2/9 | 2/16 | 14 | 1/1 | 0 | 8 | no |
| `@flighthq/ipc` | generated | emittable | compile-blocked | 3/3 | 6/6 | 0 | 2/2 | 0 | 0 | no |
| `@flighthq/keyboard` | generated | emittable | dependency-blocked | 3/3 | 14/14 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/layout` | generated | blocked | source-blocked | 5/8 | 9/9 | 0 | 1/1 | 0 | 3 | no |
| `@flighthq/lifecycle` | generated | blocked | source-blocked | 2/3 | 12/13 | 1 | 2/6 | 0 | 2 | no |
| `@flighthq/lighting` | generated | emittable | dependency-blocked | 14/14 | 49/49 | 0 | 6/12 | 0 | 0 | no |
| `@flighthq/loader` | generated | blocked | source-blocked | 2/4 | 17/18 | 1 | 3/3 | 0 | 3 | no |
| `@flighthq/log` | generated | blocked | source-blocked | 2/3 | 61/61 | 0 | 51/95 | 0 | 1 | no |
| `@flighthq/materials` | generated | emittable | dependency-blocked | 24/24 | 95/95 | 0 | 8/56 | 1 | 0 | no |
| `@flighthq/math` | generated | emittable | compiled | 17/17 | 74/74 | 0 | 19/85 | 0 | 0 | no |
| `@flighthq/media` | generated | blocked | source-blocked | 6/7 | 44/59 | 15 | 2/6 | 2 | 2 | no |
| `@flighthq/mediasession` | generated | blocked | source-blocked | 2/3 | 11/11 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/menu` | generated | blocked | source-blocked | 2/4 | 24/24 | 0 | 4/6 | 0 | 2 | no |
| `@flighthq/mesh` | generated | blocked | source-blocked | 12/16 | 50/89 | 39 | 7/31 | 1 | 5 | no |
| `@flighthq/midi` | generated | blocked | source-blocked | 3/7 | 28/37 | 9 | 2/6 | 0 | 5 | no |
| `@flighthq/motionpath` | generated | emittable | dependency-blocked | 3/3 | 10/10 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/movieclip` | generated | blocked | source-blocked | 4/5 | 26/30 | 4 | 2/4 | 0 | 2 | no |
| `@flighthq/net` | generated | blocked | source-blocked | 4/5 | 7/8 | 1 | 4/15 | 0 | 2 | no |
| `@flighthq/node` | generated | blocked | source-blocked | 19/21 | 118/134 | 16 | 37/55 | 5 | 3 | no |
| `@flighthq/notification` | generated | blocked | source-blocked | 2/3 | 32/36 | 4 | 5/6 | 0 | 2 | no |
| `@flighthq/particleemitter` | generated | blocked | source-blocked | 11/12 | 49/53 | 4 | 1/1 | 2 | 2 | no |
| `@flighthq/particles` | generated | blocked | source-blocked | 11/13 | 32/32 | 0 | 3/4 | 0 | 2 | no |
| `@flighthq/particles-formats` | generated | blocked | source-blocked | 7/18 | 41/41 | 0 | 2/2 | 0 | 11 | no |
| `@flighthq/path` | generated | blocked | source-blocked | 30/31 | 82/83 | 1 | 13/31 | 0 | 2 | no |
| `@flighthq/path-boolean` | generated | blocked | source-blocked | 9/10 | 14/15 | 1 | 2/6 | 0 | 2 | no |
| `@flighthq/path-formats` | generated | emittable | dependency-blocked | 3/3 | 4/4 | 0 | 2/6 | 0 | 0 | no |
| `@flighthq/permissions` | generated | blocked | source-blocked | 2/3 | 3/3 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/physics2d` | generated | blocked | source-blocked | 21/26 | 126/140 | 14 | 2/2 | 0 | 6 | no |
| `@flighthq/physics2d-abi` | generated | blocked | source-blocked | 4/8 | 61/82 | 21 | 1/1 | 0 | 5 | no |
| `@flighthq/physics3d` | generated | blocked | source-blocked | 25/36 | 209/233 | 24 | 2/2 | 0 | 12 | no |
| `@flighthq/physics3d-abi` | generated | blocked | source-blocked | 4/8 | 57/78 | 21 | 1/1 | 0 | 5 | no |
| `@flighthq/picking` | generated | emittable | dependency-blocked | 4/4 | 12/12 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/platform` | generated | emittable | dependency-blocked | 3/3 | 14/14 | 0 | 4/6 | 0 | 0 | no |
| `@flighthq/power` | generated | blocked | source-blocked | 2/3 | 19/19 | 0 | 3/6 | 0 | 1 | no |
| `@flighthq/preferences` | generated | blocked | source-blocked | 3/4 | 39/39 | 0 | 3/6 | 0 | 1 | no |
| `@flighthq/protocol` | generated | emittable | dependency-blocked | 3/3 | 18/18 | 0 | 3/6 | 0 | 0 | no |
| `@flighthq/quadbatch` | generated | blocked | source-blocked | 2/3 | 28/33 | 5 | 2/7 | 0 | 2 | no |
| `@flighthq/registry` | generated | blocked | source-blocked | 2/3 | 6/6 | 0 | 25/38 | 0 | 1 | no |
| `@flighthq/render` | generated | blocked | source-blocked | 23/26 | 64/83 | 19 | 13/22 | 5 | 4 | no |
| `@flighthq/render-gl` | host-backend | blocked | source-blocked | 26/32 | 102/144 | 45 | 5/9 | 13 | 7 | no |
| `@flighthq/render-wgpu` | host-backend | blocked | source-blocked | 24/33 | 111/156 | 45 | 6/9 | 16 | 10 | no |
| `@flighthq/requirement` | generated | blocked | source-blocked | 3/4 | 6/6 | 0 | 12/14 | 0 | 1 | no |
| `@flighthq/requirement-catalog` | generated | blocked | source-blocked | 3/4 | 4/9 | 5 | 4/5 | 0 | 2 | no |
| `@flighthq/requirement-codegen` | generated | emittable | dependency-blocked | 3/3 | 2/2 | 0 | 3/3 | 0 | 0 | no |
| `@flighthq/scene-document` | generated | blocked | source-blocked | 3/11 | 22/36 | 14 | 2/2 | 0 | 9 | no |
| `@flighthq/scene2d` | generated | blocked | source-blocked | 5/9 | 33/44 | 11 | 17/35 | 0 | 5 | no |
| `@flighthq/scene2d-canvas` | host-backend | blocked | source-blocked | 37/51 | 126/146 | 25 | 5/7 | 20 | 15 | no |
| `@flighthq/scene2d-dom` | host-bound | host-bound | not-applicable | 0/0 | 0/87 | 87 | 1/1 | 0 | 0 | no |
| `@flighthq/scene2d-formats` | generated | blocked | source-blocked | 52/80 | 152/215 | 63 | 3/5 | 0 | 29 | no |
| `@flighthq/scene2d-gl` | host-backend | blocked | source-blocked | 27/33 | 80/93 | 14 | 2/2 | 10 | 7 | no |
| `@flighthq/scene2d-resources` | generated | blocked | source-blocked | 7/12 | 16/22 | 6 | 2/4 | 0 | 6 | no |
| `@flighthq/scene2d-wgpu` | host-backend | blocked | source-blocked | 25/33 | 74/91 | 17 | 2/2 | 11 | 9 | no |
| `@flighthq/scene3d` | generated | blocked | source-blocked | 18/21 | 68/70 | 2 | 8/11 | 0 | 4 | no |
| `@flighthq/scene3d-formats` | generated | blocked | source-blocked | 73/111 | 270/344 | 74 | 4/4 | 0 | 39 | no |
| `@flighthq/scene3d-gl` | host-backend | blocked | source-blocked | 65/69 | 185/205 | 20 | 1/1 | 22 | 5 | no |
| `@flighthq/scene3d-resources` | generated | blocked | source-blocked | 21/29 | 45/54 | 9 | 1/1 | 0 | 9 | no |
| `@flighthq/scene3d-wgpu` | host-backend | blocked | source-blocked | 45/50 | 129/170 | 41 | 1/1 | 29 | 6 | no |
| `@flighthq/screen` | generated | emittable | dependency-blocked | 3/3 | 31/31 | 0 | 3/6 | 0 | 0 | no |
| `@flighthq/sdk` | generated | blocked | source-blocked | 156/156 | 3/9360 | 9357 | 0/0 | 0 | 1 | no |
| `@flighthq/selection` | generated | blocked | source-blocked | 3/6 | 27/28 | 1 | 2/2 | 0 | 4 | no |
| `@flighthq/sensors` | generated | emittable | dependency-blocked | 3/3 | 37/37 | 0 | 2/6 | 0 | 0 | no |
| `@flighthq/shading` | generated | blocked | source-blocked | 18/19 | 36/36 | 0 | 4/7 | 1 | 1 | no |
| `@flighthq/shape` | generated | blocked | source-blocked | 16/20 | 90/100 | 10 | 10/17 | 1 | 5 | no |
| `@flighthq/shape-formats` | generated | blocked | source-blocked | 2/4 | 3/3 | 0 | 1/1 | 0 | 2 | no |
| `@flighthq/share` | generated | emittable | dependency-blocked | 3/3 | 14/14 | 0 | 3/6 | 0 | 0 | no |
| `@flighthq/shell` | generated | emittable | compile-blocked | 3/3 | 10/10 | 0 | 3/3 | 0 | 0 | no |
| `@flighthq/shortcut` | generated | blocked | source-blocked | 2/4 | 20/21 | 1 | 3/3 | 0 | 3 | no |
| `@flighthq/signals` | generated | emittable | dependency-blocked | 10/10 | 21/21 | 0 | 56/117 | 0 | 0 | partial |
| `@flighthq/skeleton2d` | generated | blocked | source-blocked | 24/25 | 63/64 | 1 | 3/7 | 0 | 2 | no |
| `@flighthq/skeleton2d-formats` | generated | blocked | source-blocked | 33/59 | 181/209 | 33 | 2/2 | 0 | 27 | no |
| `@flighthq/skeleton3d` | generated | blocked | source-blocked | 9/10 | 25/27 | 2 | 2/5 | 0 | 2 | no |
| `@flighthq/snapshot` | generated | blocked | source-blocked | 6/7 | 7/7 | 0 | 1/1 | 2 | 1 | no |
| `@flighthq/socket` | generated | blocked | source-blocked | 4/5 | 14/14 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/spatial` | generated | blocked | source-blocked | 6/11 | 32/34 | 2 | 4/8 | 0 | 6 | no |
| `@flighthq/spring` | generated | blocked | source-blocked | 5/6 | 24/24 | 0 | 1/1 | 0 | 1 | no |
| `@flighthq/spritesheet` | generated | emittable | dependency-blocked | 9/9 | 38/38 | 0 | 2/2 | 2 | 0 | no |
| `@flighthq/spritesheet-formats` | generated | blocked | source-blocked | 11/13 | 27/27 | 0 | 1/1 | 4 | 2 | no |
| `@flighthq/statechart` | generated | blocked | source-blocked | 3/5 | 17/17 | 0 | 1/1 | 0 | 2 | no |
| `@flighthq/statusbar` | generated | blocked | source-blocked | 2/3 | 18/18 | 0 | 3/6 | 0 | 1 | no |
| `@flighthq/surface` | generated | blocked | source-blocked | 5/7 | 11/14 | 3 | 4/13 | 0 | 3 | no |
| `@flighthq/swf` | generated | blocked | source-blocked | 37/58 | 80/114 | 34 | 3/3 | 0 | 22 | no |
| `@flighthq/text` | generated | blocked | source-blocked | 6/9 | 83/93 | 10 | 11/22 | 0 | 4 | no |
| `@flighthq/text-markup` | generated | blocked | source-blocked | 6/8 | 13/13 | 0 | 2/4 | 0 | 2 | no |
| `@flighthq/textbidi` | generated | emittable | dependency-blocked | 8/8 | 11/11 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/textinput` | generated | emittable | dependency-blocked | 6/6 | 57/57 | 0 | 6/9 | 1 | 0 | no |
| `@flighthq/textlayout` | generated | emittable | dependency-blocked | 14/14 | 52/52 | 0 | 10/24 | 1 | 0 | no |
| `@flighthq/textsegment` | generated | blocked | source-blocked | 6/7 | 16/16 | 0 | 1/1 | 1 | 1 | no |
| `@flighthq/textshaper` | generated | blocked | source-blocked | 7/10 | 31/36 | 5 | 2/25 | 0 | 4 | no |
| `@flighthq/texture` | generated | blocked | source-blocked | 5/9 | 56/59 | 3 | 19/46 | 0 | 5 | no |
| `@flighthq/texture-formats` | generated | blocked | source-blocked | 6/11 | 13/23 | 10 | 1/1 | 0 | 6 | no |
| `@flighthq/textureatlas` | generated | blocked | source-blocked | 6/7 | 36/36 | 0 | 9/23 | 0 | 1 | no |
| `@flighthq/textureatlas-formats` | generated | blocked | source-blocked | 6/8 | 20/20 | 0 | 2/2 | 0 | 2 | no |
| `@flighthq/tilemap` | generated | blocked | source-blocked | 2/3 | 18/24 | 6 | 3/6 | 0 | 2 | no |
| `@flighthq/tilemap-formats` | generated | blocked | source-blocked | 11/13 | 38/38 | 0 | 2/2 | 2 | 2 | no |
| `@flighthq/timeline` | generated | emittable | dependency-blocked | 3/3 | 21/21 | 0 | 2/5 | 0 | 0 | no |
| `@flighthq/tokens` | generated | blocked | source-blocked | 2/5 | 9/10 | 1 | 1/1 | 0 | 4 | no |
| `@flighthq/tool-capture` | excluded | excluded | not-applicable | 0/0 | 0/201 | 201 | 0/0 | 0 | 0 | no |
| `@flighthq/tool-manifest` | generated | blocked | source-blocked | 3/5 | 0/7 | 7 | 0/0 | 0 | 3 | no |
| `@flighthq/tool-pipeline` | generated | blocked | source-blocked | 3/6 | 0/12 | 12 | 0/0 | 0 | 4 | no |
| `@flighthq/tool-registry` | generated | blocked | source-blocked | 3/4 | 0/2 | 2 | 0/0 | 0 | 2 | no |
| `@flighthq/tray` | generated | blocked | source-blocked | 3/4 | 26/30 | 4 | 3/3 | 0 | 2 | no |
| `@flighthq/tween` | generated | blocked | source-blocked | 6/10 | 29/33 | 4 | 2/2 | 0 | 5 | no |
| `@flighthq/types` | generated | emittable | promoted | 886/1026 | 3340/3341 | 1 | 159/160 | 0 | 0 | full |
| `@flighthq/updater` | generated | blocked | source-blocked | 2/3 | 3/3 | 0 | 2/2 | 0 | 1 | no |
| `@flighthq/useragent` | generated | emittable | compiled | 4/4 | 12/12 | 0 | 4/8 | 0 | 0 | no |
| `@flighthq/velocity` | generated | emittable | dependency-blocked | 5/5 | 22/22 | 0 | 3/5 | 1 | 0 | no |
| `@flighthq/video` | generated | blocked | source-blocked | 4/5 | 17/17 | 0 | 3/7 | 0 | 1 | no |
| `@flighthq/vite-plugin-manifest` | generated | blocked | source-blocked | 3/7 | 5/19 | 14 | 0/0 | 0 | 5 | no |
| `@flighthq/webcam` | generated | emittable | compiled | 2/2 | 0/0 | 0 | 1/1 | 0 | 0 | no |
| `@flighthq/xml` | generated | emittable | compiled | 4/4 | 6/6 | 0 | 8/12 | 0 | 0 | no |

## Async tasks

Construction disposition partition: 391 eligible = 33 portable executable + 0 host placeholder + 358 unsupported.

Disposition partition: 263 eligible = 29 portable executable + 0 host placeholder + 234 unsupported.

| Operation | Count |
| --- | ---: |
| Await expressions | 333 |
| Async iterations | 3 |
| Promise.all | 3 |
| Promise.allSettled | 3 |
| Promise.resolve | 2 |
| Promise.reject | 1 |
| Promise.then | 2 |
| Promise.catch | 10 |
| Promise.finally | 0 |
| Void expressions | 3 |

| Package | Constructions eligible/executable/host/unsupported | Scopes eligible/executable/host/unsupported | Legacy erasure path |
| --- | ---: | ---: | ---: |
| `@flighthq/app` | 2/2/0/0 | 2/2/0/0 | 2 |
| `@flighthq/assets` | 6/0/0/6 | 1/0/0/1 | 1 |
| `@flighthq/audio` | 8/0/0/8 | 8/0/0/8 | 8 |
| `@flighthq/clipboard` | 1/1/0/0 | 1/1/0/0 | 1 |
| `@flighthq/filesystem` | 47/0/0/47 | 6/0/0/6 | 6 |
| `@flighthq/font` | 7/5/0/2 | 7/5/0/2 | 7 |
| `@flighthq/geolocation` | 1/1/0/0 | 1/1/0/0 | 1 |
| `@flighthq/host-web` | 161/3/0/158 | 121/3/0/118 | 32 |
| `@flighthq/image` | 6/0/0/6 | 6/0/0/6 | 6 |
| `@flighthq/image-codec` | 3/3/0/0 | 3/3/0/0 | 3 |
| `@flighthq/keyboard` | 1/1/0/0 | 1/1/0/0 | 1 |
| `@flighthq/loader` | 6/0/0/6 | 4/0/0/4 | 4 |
| `@flighthq/log` | 2/0/0/2 | 2/0/0/2 | 2 |
| `@flighthq/media` | 1/0/0/1 | 0/0/0/0 | 0 |
| `@flighthq/menu` | 1/0/0/1 | 0/0/0/0 | 0 |
| `@flighthq/midi` | 20/1/0/19 | 13/1/0/12 | 13 |
| `@flighthq/net` | 1/0/0/1 | 0/0/0/0 | 0 |
| `@flighthq/notification` | 16/0/0/16 | 16/0/0/16 | 16 |
| `@flighthq/permissions` | 14/0/0/14 | 9/0/0/9 | 9 |
| `@flighthq/render-wgpu` | 9/0/0/9 | 5/0/0/5 | 4 |
| `@flighthq/scene2d-resources` | 9/0/0/9 | 5/0/0/5 | 3 |
| `@flighthq/scene3d-resources` | 21/7/0/14 | 15/7/0/8 | 14 |
| `@flighthq/share` | 3/3/0/0 | 1/1/0/0 | 1 |
| `@flighthq/shell` | 2/2/0/0 | 0/0/0/0 | 0 |
| `@flighthq/shortcut` | 5/0/0/5 | 5/0/0/5 | 5 |
| `@flighthq/surface` | 2/2/0/0 | 2/2/0/0 | 2 |
| `@flighthq/swf` | 2/2/0/0 | 2/2/0/0 | 2 |
| `@flighthq/textureatlas` | 4/0/0/4 | 4/0/0/4 | 4 |
| `@flighthq/tool-manifest` | 6/0/0/6 | 6/0/0/6 | 6 |
| `@flighthq/tool-pipeline` | 4/0/0/4 | 4/0/0/4 | 4 |
| `@flighthq/tray` | 12/0/0/12 | 8/0/0/8 | 5 |
| `@flighthq/updater` | 2/0/0/2 | 2/0/0/2 | 2 |
| `@flighthq/video` | 4/0/0/4 | 1/0/0/1 | 1 |
| `@flighthq/vite-plugin-manifest` | 2/0/0/2 | 2/0/0/2 | 0 |

### Unsupported task constructions

- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:34:12` `acquireAsset.reject:34:12:c1bd098683aa` `reject` (sha256:c1bd098683aa6463478400329d6117bcc8118e92b9673a7ad2fc8f550a8626ce): Portable task Rust lowering is not implemented.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:39:12` `acquireAsset.reject:39:12:dd550e8fb233` `reject` (sha256:dd550e8fb2330f0c8ec7878760e1315348dd41dc602a767f968ff211b05d3361): Portable task Rust lowering is not implemented.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:45:35` `acquireAsset.ready:45:35:214b55bb04b3` `ready` (sha256:214b55bb04b3e86f9178fbdccfafd0397bff3632f9f1d66fb98aa49194e1b6d8): Portable task Rust lowering is not implemented.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:53:23` `acquireAsset.loadPromise.then:53:23:b3c9a19b076b` `then` (sha256:b3c9a19b076b818a81e48bea3b5d68375f1f009765468e3247eb00997296e799): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:149:1` `loadAssetGroup` `async-scope` (sha256:188d51024cb73e45ac54868edd4ae13e87859c658cbe0a8476dbe34b01d289ec): Portable task Rust lowering is not implemented.
- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:186:23` `loadAssetGroup.settlements.join-all-settled:186:23:0292ceef1e91` `join-all-settled` (sha256:0292ceef1e91d5acf9fc744e917e3cb8249d560087fdc92c345aa882b5feacbd): taskAllSettled Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:33:1` `loadAudioResourceFromBase64` `async-scope` (sha256:86b5f46fce60b8506f3b9fb74b18b2e4b544c3413504a56a61e3ae05211bedf7): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:46:1` `loadAudioResourceFromBlob` `async-scope` (sha256:b9c4734232552c9160a8454b6dc2affdac69b54bd51bed26eb6cc45a03f686e5): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:59:1` `loadAudioResourceFromBytes` `async-scope` (sha256:fdaaaa08b20b3f3afcf1ebec521aed24b757857fce7b9ded6203ed6046ccd78d): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:77:1` `loadAudioResourceFromUrl` `async-scope` (sha256:5dbf1599d68fc7bda16ecec38433c7b3af6994a4b49f276066e7f3dd0a3a2bc2): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:87:1` `_loadAudioResourceFromUrl` `async-scope` (sha256:f003cb4174c556bed4c870c6e5c717f3bd49b58a546679efd5f6b193d27aae93): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:114:1` `loadAudioResourceFromUrls` `async-scope` (sha256:85566fe200667c48a3823a75bdc3b29642c1ee200ae9f6a33d8c5d79c9a2d710): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceReference.ts:141:1` `resolveAudioResourceReference` `async-scope` (sha256:c2351a145ce3eba30f060e44fb9c2a9c6c87622b6d01fda5f55a6d7f4899a501): Portable task Rust lowering is not implemented.
- `@flighthq/audio` `upstream/packages/audio/src/decodeAudioResourceBytes.ts:19:1` `decodeAudioResourceBytes` `async-scope` (sha256:6676e4daedc047fc7890deed29b85ee332aa21105b32f3ac92e8b6c6874750c0): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:20:31` `appendTextFile.reject:20:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:22:36` `appendTextFile.ready:22:36:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:31:56` `canAccessFile.ready:31:56:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:39:45` `copyFile.ready:39:45:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:44:10` `createFileSymlink.ready:44:10:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:48:52` `directoryExists.ready:48:52:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:52:47` `fileExists.ready:52:47:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:55:1` `findFiles` `async-scope` (sha256:f945c32cbca14094e2808e51af8cdd791f459d9d2ad11133a6dc1a27cfc07c5c): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:86:10` `getFilePermissions.ready:86:10:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:91:10` `getFileRealPath.ready:91:10:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:102:51` `getFileSystemUsage.ready:102:51:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:121:50` `makeDirectory.ready:121:50:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:146:31` `openFileReadStream.reject:146:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:148:34` `openFileReadStream.ready:148:34:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:157:31` `openFileWriteStream.reject:157:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:159:34` `openFileWriteStream.ready:159:34:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:168:31` `readBinaryFile.reject:168:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:170:34` `readBinaryFile.ready:170:34:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:181:31` `readBinaryFileRange.reject:181:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:183:34` `readBinaryFileRange.ready:183:34:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:187:1` `readDialogHandleBinaryFile` `async-scope` (sha256:aa86eef1938205026ca2e8df97ac880284c1638e6ab50f9ff35b71ded9e10b65): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:192:31` `readDialogHandleBinaryFile.reject:192:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:199:1` `readDialogHandleTextFile` `async-scope` (sha256:2d970d15beb55f8aac7eb6d37fe230687df534a06f9a1bb8a7ce9b05616d3cfa): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:216:31` `readDirectory.reject:216:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:218:34` `readDirectory.ready:218:34:24ebc2b22456` `ready` (sha256:24ebc2b22456a00e7226f082338fe750ae6c68db4f80640c991ac703374932d3): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:227:40` `readDirectoryRecursive.reject:227:40:326fef96ac74` `reject` (sha256:326fef96ac7409fdf9e33253fabb33a557f40ea80d136a3abccdf7ea826e4df7): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:229:34` `readDirectoryRecursive.ready:229:34:24ebc2b22456` `ready` (sha256:24ebc2b22456a00e7226f082338fe750ae6c68db4f80640c991ac703374932d3): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:235:10` `readFileSymlink.ready:235:10:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:243:31` `readTextFile.reject:243:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:245:34` `readTextFile.ready:245:34:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:254:63` `removeDirectory.ready:254:63:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:258:47` `removeFile.ready:258:47:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:266:47` `renameFile.ready:266:47:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:271:10` `setFilePermissions.ready:271:10:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:275:45` `statFile.ready:275:45:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:289:31` `writeBinaryFile.reject:289:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:291:35` `writeBinaryFile.ready:291:35:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:295:1` `writeBinaryFileChunks` `async-scope` (sha256:90b2a2b4cc4ce769c8e93c1a27d35d89eebbc50568cd946f8d9b24aa8fd13403): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:306:11` `writeBinaryFileChunks.catch:306:11:31d6c532211e` `catch` (sha256:31d6c532211e343b0774421d00b899a3f057c93e7d7e776b1f5214022afedeb4): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:313:20` `writeBinaryFileChunks.onAbort.catch:313:20:f660b2995a93` `catch` (sha256:f660b2995a93667808fa324cd77ed2c0d1139de7390a72826105614b6d8b0907): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:325:38` `writeBinaryFileChunks.catch:325:38:de15f8a5ff5c` `catch` (sha256:de15f8a5ff5c03565a1a62888d3db5cd37f4d9f7fd0aa734f1e19fe1b0c6ca7a): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:334:1` `writeDialogHandleBinaryFile` `async-scope` (sha256:87bae656f18496b1b4cbdad02935b19ace4046fe3c502fbdc69f5d16753601dd): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:347:1` `writeDialogHandleTextFile` `async-scope` (sha256:933771a09107b298e7725d790a669edba5f616c9a63428111846f9b29d6697fa): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:366:31` `writeFileAtomic.reject:366:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:368:35` `writeFileAtomic.ready:368:35:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:378:31` `writeTextFile.reject:378:31:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Portable task Rust lowering is not implemented.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:380:35` `writeTextFile.ready:380:35:a913f3cb1f97` `ready` (sha256:a913f3cb1f9734d159588904231d21b38dd02a4b990de3f76aaea042e535d166): Portable task Rust lowering is not implemented.
- `@flighthq/font` `upstream/packages/font/src/_fontFaceLoad.ts:6:1` `_loadFontFaceFromBytes` `async-scope` (sha256:7320850a3926a27d2228d4f2b0fce53c5157049dd15091f787a21afcdfd29c2e): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/font` `upstream/packages/font/src/_fontFaceLoad.ts:44:1` `loadAndRegisterFontFace` `async-scope` (sha256:1a006a9ec3a17aa2a4d351f916d68a6dfb10d45b0bf0a20f2c2e8f681002ea49): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webApp.ts:32:23` `initializeWebAppBadgeBackend.anonymous:7722fac082ee` `async-scope` (sha256:7722fac082ee02a5b7f71f81bee7c7ecf4ba4dd0c42377c40d4bd064ecc5d7a7): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webAudioDecodeHost.ts:9:3` `decodeWithWebAudio.decode` `async-scope` (sha256:eb1ba276bb31025fe36a4cd2ecd5612922dbe9ecea7039b5d57d620099d9b64c): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webAudioDevice.ts:131:5` `initializeWebAudioDeviceBackend.catch:131:5:680208b8c42c` `catch` (sha256:680208b8c42c737d43cd7428803e2770ff9641c1145165a592cfb07ae43cf9fb): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webAudioDevice.ts:196:32` `initializeWebAudioDeviceBackend.catch:196:32:c0ebd74250a5` `catch` (sha256:c0ebd74250a5ca10b6497c7a3c6088422b125bc4e467f4ee98c030cf63e7d06b): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:65:3` `initializeWebClipboardFormatsBackend.blobFromFormatData` `async-scope` (sha256:c1fb9e0ee718a360876a579fe9e1e5fdc16157f03db3dabc983b233c321466dd): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:72:3` `initializeWebClipboardFormatsBackend.writeFormat` `async-scope` (sha256:29328f33b90e3ead78ad617623031ae288c663d52699722b9018bc8509779dad): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:83:3` `initializeWebClipboardFormatsBackend.writeItems` `async-scope` (sha256:122e97ba7bfab133a56b3b699ebddbe1d3cf9dabdb17de93cbe5b3c4aaa4920d): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:99:19` `initializeWebClipboardFormatsBackend.anonymous:158d58a052a9` `async-scope` (sha256:158d58a052a95eaac54a01cd980088e8ec42a56f1c31ea2b5c92cd1d140cb28f): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:111:18` `initializeWebClipboardImageBackend.anonymous:c12cfbc1fff8` `async-scope` (sha256:c12cfbc1fff820cfbd900bf4dd255fc91ab2076be7482f694ef01125f709a1a1): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:113:20` `initializeWebClipboardImageBackend.anonymous:1d1422ff3263` `async-scope` (sha256:1d1422ff32639d98cd482f40476cf29ec20a55a359e17293c184f24f45142cf5): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:129:17` `initializeWebClipboardTextProvider.anonymous:298f67a20d28` `async-scope` (sha256:298f67a20d284821512fc3f6329ea4a8b42cf78df9eff9b8f343f0afd317d475): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:139:1` `getWebClipboardFormats` `async-scope` (sha256:e2f4ac97c33f4184f959c31f4387c3732a64d66ef02447a837ff28ee4970a2dc): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:181:1` `readWebClipboardFormat` `async-scope` (sha256:70231f4d129a64d2ded6eb37aff67d3c98c95ba59917afc11c1bb994295de6ff): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:195:1` `readWebClipboardImage` `async-scope` (sha256:7befe20cf0f6e9e0dbc6d7cb1e1679023711a9a82a9a01b92124cbbbfffb1556): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:208:1` `readWebClipboardItems` `async-scope` (sha256:cd9eca2adbc48dceb00bf290bdab241e43dbc8bbe30c6cb2d843c825009d232a): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:227:1` `readWebClipboardText` `async-scope` (sha256:e9b52a210afbafa718e4ef0c99e34e03db3e0859ab0352572eb021f00ef0b830): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:237:1` `writeWebClipboardText` `async-scope` (sha256:455c0ecd72d8eeadfd0d96bc216dc6a77d1de8ce3f3830f7bad5f6b53d95e6eb): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webConnectivity.ts:84:32` `initializeWebConnectivityReachabilityBackend.anonymous:2dc70fca6c7e` `async-scope` (sha256:2dc70fca6c7efc30cb41acbe7e3ca02e49e962616b6a88bf2a612ef8903083d1): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:46:17` `initializeWebMessageDialogBackend.anonymous:8f5fa360ad1b` `async-scope` (sha256:8f5fa360ad1bd77680e60f3e88c42e00f025909ac887d86735fde3d5855e04cf): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:55:17` `initializeWebMessageDialogBackend.anonymous:d7dee00bddb8` `async-scope` (sha256:d7dee00bddb85e5a455b5844b7aa02107da564afda145148fda619728768686f): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:75:16` `initializeWebPromptDialogBackend.anonymous:6eac38774a4f` `async-scope` (sha256:6eac38774a4f7efc8deaf411a56a1575fca0612eab87cb298440277da0fbd67d): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:138:1` `openDirectory` `async-scope` (sha256:9be8e96047dabae3a7ee8e9b813b355ea8be9fda492876df37757f59848def21): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:155:39` `openFile.ready:155:39:1199bc94c234` `ready` (sha256:1199bc94c234241ee2572ff8c292bcbdd45481820ee04e2374a75f5db3bc2f8f): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:163:1` `openFileSystemAccessPicker` `async-scope` (sha256:55e46c0f7864422cea17f4fb99b4b17769c8835091e61e9ad58b64ac91195f32): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:188:39` `openLegacyFilePicker.ready:188:39:1199bc94c234` `ready` (sha256:1199bc94c234241ee2572ff8c292bcbdd45481820ee04e2374a75f5db3bc2f8f): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:195:12` `openLegacyFilePicker.ready:195:12:2dc41488061f` `ready` (sha256:2dc41488061fdaad2cc94d680aa93e0989186df1efcd876066f03f38091e4762): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:259:1` `saveFile` `async-scope` (sha256:a5a5db15a6ceaa9384d3b0ac560920cf2d9defc55d0d33ff9c1e07f6676d0346): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:279:1` `openImage` `async-scope` (sha256:cd80d73ffa76c38aaa37d79867c34990a32a036141dab7a97cbf167179fc5b16): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:290:1` `capturePhoto` `async-scope` (sha256:faf58363764a48f8a2e92b1c589fda6c29894a5fe0f7b54e2bf69dbf37cde1a0): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:306:1` `captureVideo` `async-scope` (sha256:3f56640e626412ab5794052302468d1e846477b28c3e7e255857d43ea1ebe89f): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:332:31` `pickMediaFile.ready:332:31:1199bc94c234` `ready` (sha256:1199bc94c234241ee2572ff8c292bcbdd45481820ee04e2374a75f5db3bc2f8f): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:338:12` `pickMediaFile.ready:338:12:2dc41488061f` `ready` (sha256:2dc41488061fdaad2cc94d680aa93e0989186df1efcd876066f03f38091e4762): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:385:31` `readDataUrl.ready:385:31:0f59e6d7f003` `ready` (sha256:0f59e6d7f0038343cc34a42802cbf853e1465aab929eafbc3c9b152ebc84b99e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:386:49` `readDataUrl.ready:386:49:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:439:31` `decodeImage.ready:439:31:0f59e6d7f003` `ready` (sha256:0f59e6d7f0038343cc34a42802cbf853e1465aab929eafbc3c9b152ebc84b99e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:440:95` `decodeImage.ready:440:95:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:488:31` `decodeVideo.ready:488:31:0f59e6d7f003` `ready` (sha256:0f59e6d7f0038343cc34a42802cbf853e1465aab929eafbc3c9b152ebc84b99e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:489:95` `decodeVideo.ready:489:95:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:586:5` `retainedFileOperations.readBinary` `async-scope` (sha256:a7d067107188ae105abf5f81949745cb507a45aec1b4e047dac1f35fd3bcce7b): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:594:5` `retainedFileOperations.readText` `async-scope` (sha256:be47184491269aee86a401f9091cfff25f45fa254c6f4f29e214e18fd9e8a76c): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:607:5` `fileSystemHandleOperations.readBinary` `async-scope` (sha256:00814413734471d772248abbac2f8bdd9248239662c3ca99b9ccce64823bdae2): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:622:5` `fileSystemHandleOperations.readText` `async-scope` (sha256:ab160fc0443273980ba746d7bb6fa186de4fe8951815b9e8a38f952d093c185d): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:637:5` `fileSystemHandleOperations.writeBinary` `async-scope` (sha256:dab78eda45d46903147fa1063f38c950c9dfa6b311aea999cd8573bb39db2475): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:641:5` `fileSystemHandleOperations.writeText` `async-scope` (sha256:172ea3833e605d90447b67c01f801cfe980b0d4c71480cbdaa17ac56a1eb11c0): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:648:1` `writeFileSystemHandle` `async-scope` (sha256:18295f0d91e9d34dadbfc15499af211ebbc169d9080bb84ffb584b5b46d8fa62): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:661:11` `writeFileSystemHandle.catch:661:11:e721f7673201` `catch` (sha256:e721f7673201f13f3aab4a842bf4538100998d17316396a701831634bba366ee): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:668:20` `writeFileSystemHandle.onAbort.catch:668:20:a8b13093d9f1` `catch` (sha256:a8b13093d9f1267a353e1b37974b0ddb5bb0772945e3a9138d117d27ab7245c9): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:668:73` `writeFileSystemHandle.onAbort.ready:668:73:9b8874eb7ffe` `ready` (sha256:9b8874eb7ffebcf300217391f3fb4a9d53d078bccf4ed67e3253de0dd1f14116): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:680:38` `writeFileSystemHandle.catch:680:38:40add2412270` `catch` (sha256:40add24122700a0fb0f67a0cdac06dd37b12932c364c7bf8e0e03b499560096c): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:6:3` `webHostFileSystem.appendTextFile` `async-scope` (sha256:61f7174a25870e6bb51819eec7bb5bda8c299a9428137bd4824a94b56984cbcd): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:23:3` `webHostFileSystem.canAccessFile` `async-scope` (sha256:74a2f199f13e597db145981b88bfc0b4e3e7a92cfeefb845c64fe286b8e04ef0): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:41:3` `webHostFileSystem.copy` `async-scope` (sha256:e1e892418c0c5d49918483c740ad63791036835cd1417d63903cd2c0e37a804e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:51:3` `webHostFileSystem.directoryExists` `async-scope` (sha256:56d4ea2626bdcd31b546678a3acf52398dcc20bfb63a150cbc61ac02568f185c): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:55:3` `webHostFileSystem.fileExists` `async-scope` (sha256:a50063fec4839730a28a8f732afd1ccce90aff1eb8923b58fe7d40304c34af52): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:58:3` `webHostFileSystem.getFileSystemUsage` `async-scope` (sha256:5c331a37e8df7c5443b1649881acd1b5a0c8af50fadee4499702df7337c7e565): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:69:3` `webHostFileSystem.makeDirectory` `async-scope` (sha256:b47d09fb68f5f7cffeb77f07f43272329248f0b39d4092606a05ee2e7475e0a2): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:73:3` `webHostFileSystem.openFileReadStream` `async-scope` (sha256:0a2d3e03e87c35ca13b5eeecc3150a1666ac5c83484e688bba8e65e0e7635f82): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:85:3` `webHostFileSystem.openFileWriteStream` `async-scope` (sha256:e2101321c7b81f6fa6980281059d2cdae656a06be8cfe5fe8c1cf6cbfc1ecd2e): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:97:3` `webHostFileSystem.readBinaryFile` `async-scope` (sha256:10f7db5888fe084427466596e6abe2110f21343825be57b226941e0d901dc743): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:115:3` `webHostFileSystem.readBinaryFileRange` `async-scope` (sha256:448a06e36ff589824ad01d59ad93d802408619f160448942bec63401ca701402): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:134:3` `webHostFileSystem.readDirectory` `async-scope` (sha256:c8b9339953de1c244cabfa386687e130dce05940f646fda8494e9a76beaa7356): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:159:3` `webHostFileSystem.readDirectoryRecursive` `async-scope` (sha256:773df7a02fc33f283bad74c8a8b33c0790d85901f6340e7d53856a6eb33100ef): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:176:3` `webHostFileSystem.readTextFile` `async-scope` (sha256:a441f7e773723246a5e8b1ebe798f7b54a72cab3910b245012d698700da996c3): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:194:3` `webHostFileSystem.removeDirectory` `async-scope` (sha256:cdc1977fd620c25d864f59bd5f16869540e60e0a21df9c820ef7ead511a6bd7d): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:208:3` `webHostFileSystem.removeFile` `async-scope` (sha256:cdf596ec2cd0dac70862bb0c1bd10d7a44bd4b8ba84bcc992134b7d934b10b63): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:211:3` `webHostFileSystem.rename` `async-scope` (sha256:74e173b8d71d90660c74e82ea126f1632c4616f83a5984dc910d1f0c417764ed): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:215:3` `webHostFileSystem.statFile` `async-scope` (sha256:89681b0982792536e5dea921a570a4cedf0b8746b0ad75efb28179b456f60add): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:235:3` `webHostFileSystem.writeBinaryFile` `async-scope` (sha256:36b0a9115b4c35c47f193dd94072e442a7d19cc4605d9eb7472f91ae70eb6f9b): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:239:3` `webHostFileSystem.writeFileAtomic` `async-scope` (sha256:7c0161d85639e9ad87e893f4a7db5be7fb8174d9b22e255d1a4db2ea3eb04580): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:260:3` `webHostFileSystem.writeTextFile` `async-scope` (sha256:d27ff883d2ba5e33c3091f3f99d42bd84ab4f5093728626ebb42d8f4a71688bf): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:276:1` `getDirectoryHandle` `async-scope` (sha256:4be00ff9889ad9c9843e3f5c1e6975e1dd9e83f85dfdcd03c57f0e1f61f4526c): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:290:1` `getFileHandle` `async-scope` (sha256:b729a6f6c2badcfde84cdc259d610652496bfeb585bb2d28192b8cebd6fcb506): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:304:1` `getRoot` `async-scope` (sha256:0bae26de7124174f36022d3d2eced8575c8a426ef8bd22273906acbef7dee990): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:319:1` `removeFile` `async-scope` (sha256:ac4e32e9f1b9879492f7736ddd4e26517f9c121c04e39dd6dc7307f34c877f80): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:338:1` `walkDirectory` `async-scope` (sha256:8d7fffae00b2add1f0e47ec93773bc983a21e7a6d6cb4ff61d5a7ab6752cdfa6): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:357:1` `writeFile` `async-scope` (sha256:4b7b28650b9818f0bcdca434bd60565b91acc2de26ac3bb94418bc8ef2ccd75a): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:369:11` `writeFile.catch:369:11:dc9a01ed546b` `catch` (sha256:dc9a01ed546bf5f81f77ff871402e6f28c62bb990167176e4f69f1446367c175): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:376:20` `writeFile.onAbort.catch:376:20:52e7c067f6b1` `catch` (sha256:52e7c067f6b13fd48de489089c6b299860d2e04ae0f60b3767b9632c1dcce23f): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:388:38` `writeFile.catch:388:38:8c18b0647533` `catch` (sha256:8c18b06475338e5e27034d2180eee6b4aded072e80495159c196758c865d1cab): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFontLoading.ts:13:14` `webHostFontLoading.whenReady` `async-scope` (sha256:c2707a11ca2d3dd78ad32c6a3685218357fd1c0e5d548b2bdd4f10b1b6e737a9): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webHostWgpuContext.ts:13:17` `initializeWebHostWgpuContext.anonymous:5c77ab86f0db` `async-scope` (sha256:5c77ab86f0db4a52b4c96ad50b5e5e6f21fc2d167502abac40347ee3f766a3a5): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webImage.ts:20:21` `webHostImage.loadImageFromUrl` `async-scope` (sha256:5f99946cd87190e9f14dc9527e8b2afd64c4d8bcaa1d9fb120497dfeb8d76df0): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webImageBitmapComposition.ts:42:1` `resolveImageBitmapComposition` `async-scope` (sha256:6500eb8a3798aa9024d5063ea23996aa936647777c0a53aea75d7b046e367c7d): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/host-web` `upstream/packages/host-web/src/webImageDecodeHost.ts:9:3` `decodeWithCanvas.decode` `async-scope` (sha256:61dec2eca4383679897ff700592fa77bca87619b9d44122c69d447d83e30239e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webImageEncodeHost.ts:10:5` `createCanvasEncoder.encode` `async-scope` (sha256:d925295fa16f025170d731973c40c8d83434f5964d1a1eac3b5401bd5092c56c): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webInputTarget.ts:55:49` `webHostInputPointerLock.ready:55:49:0f06d185bddd` `ready` (sha256:0f06d185bddda5e12abbab9bb447adc3304469699cfcb372c391ec81db7b8349): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webInputTarget.ts:56:54` `webHostInputPointerLock.ready:56:54:d5b7fb62e27f` `ready` (sha256:d5b7fb62e27f836f434caaaf446ff79e046baa9ab3406d0846648ee026bcee59): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webInputTarget.ts:58:55` `webHostInputPointerLock.ready:58:55:0f06d185bddd` `ready` (sha256:0f06d185bddda5e12abbab9bb447adc3304469699cfcb372c391ec81db7b8349): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webInputTarget.ts:64:14` `webHostInputPointerLock.ready:64:14:a48a882ce409` `ready` (sha256:a48a882ce409e18b79ea2a9f8636eaa439c2bfd1d4be174f73fdbf0799f183f3): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webInputTarget.ts:68:14` `webHostInputPointerLock.ready:68:14:d5b7fb62e27f` `ready` (sha256:d5b7fb62e27f836f434caaaf446ff79e046baa9ab3406d0846648ee026bcee59): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webInputTarget.ts:74:39` `webHostInputPointerLock.ready:74:39:696646537544` `ready` (sha256:696646537544993f3d32c704e2932f8f1ad558e82a8944f14f44aa489e8a27c5): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webInputTarget.ts:76:58` `webHostInputPointerLock.ready:76:58:0f06d185bddd` `ready` (sha256:0f06d185bddda5e12abbab9bb447adc3304469699cfcb372c391ec81db7b8349): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webInputTarget.ts:84:14` `webHostInputPointerLock.ready:84:14:5fc08613ca5c` `ready` (sha256:5fc08613ca5c907ddf5c9f2e5aa6ae1e84e49be628fb68ac2e39057b0d8d4a54): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webInputTarget.ts:87:12` `webHostInputPointerLock.then:87:12:e11879ece0d1` `then` (sha256:e11879ece0d195dcea3525941a7e78fb6ba0334e42f930e29ba56d6e902b687f): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webInputTarget.ts:87:12` `webHostInputPointerLock.ready:87:12:6eaf6d04b61c` `ready` (sha256:6eaf6d04b61c5785edf4e55302fb7fa1bee6311c76be15aaa987058794f5f056): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webKeyboard.ts:11:14` `webHostSoftKeyboardChange.subscribe` `async-scope` (sha256:ae4ac32889b70faf712bb38fdb0ccbd2d0c02b59a6f374bea8454f2985ad757e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webKeyboard.ts:43:9` `webHostSoftKeyboardVisibility.hide` `async-scope` (sha256:244807309d2ec2b07bc08a7ee324984a1e78f345a405506b694e40e067aaa140): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webKeyboard.ts:49:9` `webHostSoftKeyboardVisibility.show` `async-scope` (sha256:b631ab134e72bb5524d521dcdd886a07befb9cabfd657e64a63cd595c8bc7f13): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:26:23` `initializeWebMidiAccessBackend.anonymous:755b5dc1a7e0` `async-scope` (sha256:755b5dc1a7e069c5c50c1e0b862940b8227190ca8c64572ea310899c8644a3c1): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:43:17` `initializeWebMidiEventAttachment.anonymous:cc858b57020d` `async-scope` (sha256:cc858b57020d9e8aae42c11611fbf459200fdb109d5282a8f4d5daa15aa1a5ae): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:77:14` `webMidiAccess.toInput.port.close` `async-scope` (sha256:43a702d4abfd706518f8c54a1b47664cf6a7049d5e6aa623a255eecc394f9e66): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:82:13` `webMidiAccess.toInput.port.open` `async-scope` (sha256:c0e355dd7eba9dd3c1150d04622aee0267459f4351482d8236e2245a2744b9af): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:95:14` `webMidiAccess.toOutput.port.close` `async-scope` (sha256:43a702d4abfd706518f8c54a1b47664cf6a7049d5e6aa623a255eecc394f9e66): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:100:13` `webMidiAccess.toOutput.port.open` `async-scope` (sha256:c0e355dd7eba9dd3c1150d04622aee0267459f4351482d8236e2245a2744b9af): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:155:12` `attachWebMidiEvent.ready:155:12:26d79da12bab` `ready` (sha256:26d79da12bab646f9a9a7c743cb0aecffab63b278d2391b01b8cec72a51dbd8c): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:163:10` `attachWebMidiEvent.ready:163:10:2266dc7742dd` `ready` (sha256:2266dc7742dd206ad1d767b9f1d97f1b845a433803ab3b7fb509e1c8c18a4eaa): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:178:1` `queryWebMidiPermission` `async-scope` (sha256:13aae848f5205dc28bc2d1550e1d3efe8fe8e015ddf532d8ae6bfa8696e6cd82): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNet.ts:19:24` `initializeWebNetBackend.anonymous:f4b378cb2e95` `async-scope` (sha256:f4b378cb2e9548bc3405de12c662eacdc48f901e262a3f0bd4546c2d75bcf4b8): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNet.ts:77:1` `_readNetResponseBody` `async-scope` (sha256:a65ec98271b1315b2d9f92bf7f23f3862fc0a2eefa19665fa73e7818af9bfc60): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNet.ts:98:1` `_readNetResponseWithProgress` `async-scope` (sha256:2fa2d01a1a3391986cc439013a763959207aaacc5a66efdf1c5f6ac215296646): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:28:3` `createWebPageNotificationCapabilities.closeOne` `async-scope` (sha256:1a977d98f99e5dc7c5eb53011950029669f9ac7c6da0ba2336269fe9bb2f121a): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:39:3` `createWebPageNotificationCapabilities.closeAll` `async-scope` (sha256:1ad18279f2ba368997c445404cf159d4049c51724357d7361b8726068aeb3b73): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:51:7` `createWebPageNotificationCapabilities.delivery.notify` `async-scope` (sha256:14fa545dd5dc83d61e295ab2856b8a05658cac4fadc571cca145d8385460f3b6): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:91:7` `createWebPageNotificationCapabilities.lifecycle.destroy` `async-scope` (sha256:9b61ba27cc64b102c5771b9181ead6b40cc010eebbccd50e4dedfa75cacb886f): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:109:5` `makeWebNotificationEventBackend.attach` `async-scope` (sha256:2ecb63ca714566e1666b725946e562b754557027b0fbdfed6fe38673ec4aacc6): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:115:11` `makeWebNotificationEventBackend.attach.attachment.release` `async-scope` (sha256:77f61cd1eacb8d04122c1c92e1b6a756c152baaf908c1c0088163a39409e3f54): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPermissions.ts:19:5` `initializeWebPermissionsBackend.getPermission` `async-scope` (sha256:eb2d7e7d7268d418c7cfffdbaf00d4c2690dd1fc42996b5b645e951346cf36cf): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPermissions.ts:27:5` `initializeWebPermissionsBackend.requestPermission` `async-scope` (sha256:7bb49dd65928a29b0cbbe037bd6884d9045c739202365c60411a286b7a19b792): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPermissions.ts:42:1` `queryWebPermission` `async-scope` (sha256:6403297b8e566b02211c0e1c488de42029e375393e0badaa693e5fcec16f972a): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPermissions.ts:60:1` `requestWebMediaAccess` `async-scope` (sha256:14dbf5a38b39ea8eadf1e11d48da9844ae0fc78cecad7f0f538cfe7a50d25ee7): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPermissions.ts:81:1` `requestWebWakeLock` `async-scope` (sha256:a26465753f49a80c8ecc71484a5d538000866c37fb603d99d3458a783c583051): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPower.ts:134:9` `createWebPowerReadings.change.subscribe.catch:134:9:d037e8389b31` `catch` (sha256:d037e8389b31a850e8d20e180eed2d8a94b6aa5aea2552d21de6f362e557fe3c): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPower.ts:134:9` `createWebPowerReadings.change.subscribe.then:134:9:80f76a3c57af` `then` (sha256:80f76a3c57af4e029b93f32b9b7f98a01243a77b1ee100a7be9e17c10f0355d1): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webScreen.ts:256:27` `createWebScreenCapabilities.detailsBackend.anonymous:89703d863b4e` `async-scope` (sha256:89703d863b4e7e30f14adec7cebe46181e6a1340fb1da9a614f371850516bc83): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webScreen.ts:265:19` `createWebScreenCapabilities.detailsBackend.anonymous:57575bfd7269` `async-scope` (sha256:57575bfd7269ee5afd44d863fd75b16f6c6831fafe598a7fe6a894bddbbf97d9): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webScreen.ts:293:12` `createWebScreenCapabilities.permissionChange.catch:293:12:e36607b17c17` `catch` (sha256:e36607b17c1797def648ce0eac0c584be87013a0a3f27fbad3c88d72f79e61a4): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webScreen.ts:293:12` `createWebScreenCapabilities.permissionChange.then:293:12:51f133a5d202` `then` (sha256:51f133a5d202da7046b6f734f0d9c76e4adb758d4306e058ca42d130d9d923b3): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webSensors.ts:71:27` `createWebSensorsBackend.anonymous:a957cbfc183a` `async-scope` (sha256:a957cbfc183aae75cc7d7b4e1d863f901079a58602a6dc4e85244734a7c27a2a): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webSensors.ts:371:1` `getWebSensorsPermissionState` `async-scope` (sha256:ae0c3340d40546bf3036b49fb90747d9572219eb5d78be5db02f27f01c5ab35a): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:31:3` `createWebServiceWorkerNotificationCapabilities.closeOne` `async-scope` (sha256:e2ca64e091727fbc0d4f3fb5b2203bd6da786ecc70aa04ea8304117935df94ec): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:49:3` `createWebServiceWorkerNotificationCapabilities.closeAll` `async-scope` (sha256:842a6eea49929ef76fba68cf638adbf004f3d5c9a0e8464d4688c523813f79f2): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:60:7` `createWebServiceWorkerNotificationCapabilities.capabilities.activeList.getActiveNotifications` `async-scope` (sha256:2d29e46e75c040af4f4cd2f9d2ce1da799844fde4fa45534b52c45501c77c3a2): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:78:7` `createWebServiceWorkerNotificationCapabilities.capabilities.delivery.notify` `async-scope` (sha256:4862385c636b740d46b7c6c13fe757ed603b954a76c416957a117e9003340080): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:99:7` `createWebServiceWorkerNotificationCapabilities.capabilities.lifecycle.destroy` `async-scope` (sha256:20d19c98239fbba7bb1b2b04a2321caf273b21bf4a06843deb9ebee75034d62a): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:111:7` `createWebServiceWorkerNotificationCapabilities.capabilities.permission.getPermission` `async-scope` (sha256:b88af037a878404b3d5af13d8f31b3eac4595c4d0165c0345733c035f81643f9): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:118:7` `createWebServiceWorkerNotificationCapabilities.capabilities.permission.requestPermission` `async-scope` (sha256:5582804874ee6c69b2f295cc70b4844c346db5cbac6a876a5ad62a49e1b77906): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:169:5` `makeWebServiceWorkerNotificationEventBackend.attach` `async-scope` (sha256:2ecb63ca714566e1666b725946e562b754557027b0fbdfed6fe38673ec4aacc6): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:175:11` `makeWebServiceWorkerNotificationEventBackend.attach.attachment.release` `async-scope` (sha256:77f61cd1eacb8d04122c1c92e1b6a756c152baaf908c1c0088163a39409e3f54): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:13:22` `initializeWebShareContentBackend.anonymous:446cdd4ed448` `async-scope` (sha256:446cdd4ed4482f254bde397f0e7e3e9c7a861ff8118e5489cb727106d32ea914): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:17:32` `initializeWebShareContentBackend.anonymous:55aeac6f5f71` `async-scope` (sha256:55aeac6f5f71b5d6a978ae206fac5241944c84d874937094c757fda62d088ff9): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:32:22` `initializeWebShareFilesBackend.anonymous:afbc5a443935` `async-scope` (sha256:afbc5a443935606a1cd655c54d3c39233874db4683b3829b8eaf754f33df46e9): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:40:32` `initializeWebShareFilesBackend.anonymous:26e8ec61137f` `async-scope` (sha256:26e8ec61137f6ec29262ba9c4a8c32f03da6545076dc9e3de5be26ee7f9b237b): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:98:1` `invokeNavigatorShare` `async-scope` (sha256:6fb73a5e473e993661139498c0a9ed46b5917eb24c888160c73103a876a661f7): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:108:1` `invokeNavigatorShareWithResult` `async-scope` (sha256:bc7bfe297293d8d1dc45dbd136a551007f761160be9579f975dc347166514a95): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShell.ts:4:14` `initializeWebShellExternalBackend.anonymous:fa929f9b108b` `async-scope` (sha256:fa929f9b108b26f3a563fb5a9e2247593ca6a18b630bfeb78557dd90d359bc96): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:18:7` `createWebWindowStoragePersistenceCapabilities.persistenceRequest.requestPersistence` `async-scope` (sha256:1b29a9d327ff84e9dd50dbfd011ab7073d0da35fc3a04b631431b0eb3d23de34): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:39:5` `createPersistenceQueryBackend.getPersistence` `async-scope` (sha256:a2dfb870f260b8080375e940c7b3d274cf9a741325ea7d6fe89f9df24e17899e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:48:3` `webWindowStoragePersistenceCapabilities.getPermissionState` `async-scope` (sha256:dbb32ac59265bc94471284f431a476051ddc90673ef34b1aa18bb8a1aeb62e01): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:52:3` `webWindowStoragePersistenceCapabilities.persist` `async-scope` (sha256:46e8f7cdee9608f80f9c9a3223f4540a8d57fa171f3a700ff39781197b0c4f31): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:55:3` `webWindowStoragePersistenceCapabilities.persisted` `async-scope` (sha256:e2e3f11c91ab6bda15834024257d83d029723db405b322eb5ca6e5f5f80b9cf7): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:63:1` `observePermissionState` `async-scope` (sha256:fa24d51d3c50a097a858b8195158146215903864f3a27d331ec6a36865868026): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:72:1` `observePersistenceOutcome` `async-scope` (sha256:82cf7b59567f1dbafbb31ca934363baac497fb69dfc28330d29e9d8b394959dd): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webVideoCapability.ts:49:33` `webHostVideo.loadUrl.reject:49:33:cfdfb9bf7a98` `reject` (sha256:cfdfb9bf7a9819f2a26b91a03eb09e7426a1ca56476ae2bae4e4888ce1be90bf): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webVideoCapability.ts:51:34` `webHostVideo.loadUrl.reject:51:34:5096577a4538` `reject` (sha256:5096577a453808af856af67953ff6d41236921117dbe9533e98a54c2613b5125): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webWindow.ts:31:9` `webHostFullscreen.exit` `async-scope` (sha256:ae9910143afed739b88eac0b75920a47d7033b0150bfdef22ac7836a42dc6c02): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webWindow.ts:40:12` `webHostFullscreen.request` `async-scope` (sha256:d1cc8422674333d68d56f518dce4aa9a22aee9b0ebed624d2ea2851b3f5f257d): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webWindow.ts:98:28` `webHostWindowFullscreen.setFullscreen.catch:98:28:aa181688e79a` `catch` (sha256:aa181688e79a0f3eef19961eb8ce7505e2746870d3f21286929ac03556f644c9): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/host-web` `upstream/packages/host-web/src/webWindow.ts:99:17` `webHostWindowFullscreen.setFullscreen.catch:99:17:ca41b61af2dc` `catch` (sha256:ca41b61af2dc09c447387498b1acbe71ca71ace3265d3e46e6288ff53b3a9cb4): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:22:1` `loadImageResourceFromBase64` `async-scope` (sha256:dbdcb8291bc16014b22358459a0dc39484688e9cb2d47bd2dcb46381082499b9): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:31:1` `loadImageResourceFromBlob` `async-scope` (sha256:498ba360a53d9ef9dcb3160a29e0035b3f8b5bd4035a4b086ecd414a31ba7b5f): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:44:1` `loadImageResourceFromBytes` `async-scope` (sha256:bc0df7b2796ca2e47a9565e4b8ce88966df7143013b0f218b941ad2d54198fba): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:58:1` `loadImageResourceFromUrl` `async-scope` (sha256:9416567113dd62bbb7f8084f0790fb235b5a3404cf7d2f8dc05cb2a622c63644): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceReference.ts:48:1` `decodeEmbeddedImageResourceReference` `async-scope` (sha256:a27cae9978b54042771849619a6657fa2568ba2d7be5e68bd3007015958e850c): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceReference.ts:168:1` `resolveImageResourceReference` `async-scope` (sha256:8abe34ac1a30f05db048471b3ab7d43f6e64d45a8fd4307ab3eec77d0a160c83): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/load.ts:18:1` `loadBytes` `async-scope` (sha256:dcae29752339f563801720a28084619b8ecd14d7ad3c7136195fe260e35efafc): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/load.ts:34:1` `loadText` `async-scope` (sha256:a77b3f353a88ea39979bc0daf5189b594c70c35a52fd81c4a2585041d641fb16): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:80:10` `_noopLoad.ready:80:10:1c5c5ba35766` `ready` (sha256:1c5c5ba357669af58be4ccb70277a838ecedb8770bd77407ef56bfc2cdaf8630): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:512:1` `drainQueue` `async-scope` (sha256:88082b65b0b72d5a7051045ab407875a77a7430ece259e4bf911600ef6289ed7): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:562:1` `runEntry` `async-scope` (sha256:0cc4fef1fe859a3eba70abead965e6eb2ab15fc8f1671ae50442eeba76acdea5): Portable task Rust lowering is not implemented.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:782:23` `delay.ready:782:23:9b8874eb7ffe` `ready` (sha256:9b8874eb7ffebcf300217391f3fb4a9d53d078bccf4ed67e3253de0dd1f14116): Portable task Rust lowering is not implemented.
- `@flighthq/log` `upstream/packages/log/src/log.ts:304:1` `destroyFileLogSink` `async-scope` (sha256:aaddc0709f7b943eaf89e4625f68abb2f26a6f28a658553afdf8e9c9793acd73): Portable task Rust lowering is not implemented.
- `@flighthq/log` `upstream/packages/log/src/log.ts:678:1` `_destroyFileLogSinkState` `async-scope` (sha256:0f542da762d0de88d6bc0e4a20ed1ce995f8823fd11e12036c649b4b9680ea72): Portable task Rust lowering is not implemented.
- `@flighthq/media` `upstream/packages/media/src/videoChannel.ts:204:3` `startVideoChannel.catch:204:3:a6d336b118e7` `catch` (sha256:a6d336b118e7c1b3057b721758bb6f72fcefc509c15fad3958f4793a4cad1287): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/menu` `upstream/packages/menu/src/menu.ts:175:30` `showContextMenu.then:175:30:99862c2ab561` `then` (sha256:99862c2ab561446ce8c047db7fb8d540a8e7ad1c9d5c6c984f01c4eb7aa0faeb): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/midi` `upstream/packages/midi/src/midiAccess.ts:31:61` `disposeMidiAccess.ready:31:61:f9ca10c9fd03` `ready` (sha256:f9ca10c9fd0304fa8d36ecdc17dde5fc02c348849e02f2d575d0614d99990605): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/midi` `upstream/packages/midi/src/midiAccess.ts:36:8` `disposeMidiAccess.finally:36:8:b1b00fb3a840` `finally` (sha256:b1b00fb3a840bc18b8c9a817f94ee5a32609cb417ade1c48e5125032c2ff7c66): taskFinally Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/midi` `upstream/packages/midi/src/midiAccess.ts:52:1` `requestMidiAccess` `async-scope` (sha256:b808895722ea45b12579bfb2cf623cbdc08bbfaf700e3d36baca2748b294433c): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/midi` `upstream/packages/midi/src/midiAccess.ts:62:1` `disposeMidiAccessKnownPorts` `async-scope` (sha256:0a498b917eda125e3a67e6f05872e6a1e925d01f0a045236738a2a3f4fd4f75f): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/midi` `upstream/packages/midi/src/midiPort.ts:31:1` `closeMidiPort` `async-scope` (sha256:bd4cf863b7e68276c4bef08184c81c7ec7c560647e392fe3a5448491c27b618a): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiPort.ts:73:61` `disposeMidiPort.ready:73:61:f9ca10c9fd03` `ready` (sha256:f9ca10c9fd0304fa8d36ecdc17dde5fc02c348849e02f2d575d0614d99990605): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiPort.ts:78:8` `disposeMidiPort.finally:78:8:b1b00fb3a840` `finally` (sha256:b1b00fb3a840bc18b8c9a817f94ee5a32609cb417ade1c48e5125032c2ff7c66): taskFinally Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/midi` `upstream/packages/midi/src/midiPort.ts:124:1` `openMidiPort` `async-scope` (sha256:697c7bbbec42915f1d576a4329dba07f17ce131ba107571abe325cf7ced67560): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiPort.ts:168:1` `disposeOwnedMidiPort` `async-scope` (sha256:06c53d301a72d3d58596586e20c1bcc16968e2398f743d924a35b270d65fb843): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:27:53` `attachMidiAccessStateSubscription.ready:27:53:98ce13c73b6d` `ready` (sha256:98ce13c73b6dab570ddf65a231a706c575418ec213873c28f22035fdb6484a8f): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:44:79` `attachMidiInputMessageSubscription.ready:44:79:98ce13c73b6d` `ready` (sha256:98ce13c73b6dab570ddf65a231a706c575418ec213873c28f22035fdb6484a8f): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:59:53` `attachMidiPortStateSubscription.ready:59:53:98ce13c73b6d` `ready` (sha256:98ce13c73b6dab570ddf65a231a706c575418ec213873c28f22035fdb6484a8f): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:160:1` `attachMidiSubscription` `async-scope` (sha256:b8c0577fde0f154df2b12d2ec5f0015fb9fec7cc0978096a278afa5800cee4b2): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:190:1` `performMidiAttach` `async-scope` (sha256:def29cf853ce63812f8c8c818d78934872d029085f1b48dcbc1779f0eb068fb6): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:217:1` `detachMidiSubscription` `async-scope` (sha256:a338a04f550381803c30a927d354e47096ab0cc813ccac04d97eae4b724b7ec3): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:236:1` `disposeMidiSubscription` `async-scope` (sha256:de01b0e5923187efb70b3331726b88d39217498be3c620ff5118b2093546fe2a): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:264:1` `releaseMidiAttachment` `async-scope` (sha256:8dece981a791ff2281b09e369af11927e7beeb935ed4b14f653381232d19b20b): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:272:1` `releaseTrackedMidiAttachment` `async-scope` (sha256:d524f4c0182d90196500cc1b81aa9db67d7fdf8fbbac5236fb190ceb1b6515e4): Portable task Rust lowering is not implemented.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:284:1` `settleMidiAttach` `async-scope` (sha256:59601c489eb9afece7e87a203acb11fd1fc41e26d9b7e39a8cf6171f8e131c91): Portable task Rust lowering is not implemented.
- `@flighthq/net` `upstream/packages/net/src/net.ts:56:10` `sendNetRequest.then:56:10:ba68ab3b247c` `then` (sha256:ba68ab3b247c5685ba556aaac0e4763082ff1956f441f5208bf6c18d898b898a): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:43:1` `attachNotificationActionSubscription` `async-scope` (sha256:ef0567517b8ad81167f34ae8b0e554e74693041ca162e320d0bfaa12509444e3): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:55:1` `attachNotificationClickSubscription` `async-scope` (sha256:6c730b2ab6b0c044ded291a138010ceca2af5b4629daef97ccbff4982a5b3275): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:66:1` `attachNotificationDismissSubscription` `async-scope` (sha256:538af16a57895fe0a4e39a2d268ac7fc966d4898d147183696fe93bf6b462d51): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:77:1` `attachNotificationReceivedSubscription` `async-scope` (sha256:35dbdacdb96e8b8568d1da2d4607c8fcb506474c23d6a9564e2ded4b7cd9ae63): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:88:1` `attachNotificationReplySubscription` `async-scope` (sha256:8a8acabe250c8dcf55edd59ff09aadc42fa3f1d755704e0ce9770043246fe2a3): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:117:1` `cancelScheduledNotification` `async-scope` (sha256:5d04c987ea0773647b5743796a025e3af6d197581f47568f6493cb51051a9625): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:142:1` `closeNotification` `async-scope` (sha256:dd6cc70df16c804640562c157075cf8087a8dc33dcc29abd910cc269d9b4b97c): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:241:1` `disposeNotificationActionSubscription` `async-scope` (sha256:c53638d28af6b747c2d9ec0ff39b8b18a0ef7538c4d7090294f7b3801b3ddb12): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:247:1` `disposeNotificationClickSubscription` `async-scope` (sha256:9c1ba368c798e786daf28a3887a2dce3811f422d7ec07da86689118b5a040c38): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:253:1` `disposeNotificationDismissSubscription` `async-scope` (sha256:e524271acda7d900ad96065155388e8c013e5d155561ea30c503ea0cef3f13cc): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:259:1` `disposeNotificationReceivedSubscription` `async-scope` (sha256:c94a25e6137e039d966eb05a6f3b2c9680ea762485f9f830d8f82a94d6d529bf): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:265:1` `disposeNotificationReplySubscription` `async-scope` (sha256:ee2380186aa1668a991dd373ce23d74083bb5e95c19fe3c020118ee121fa7eaf): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:362:1` `attachNotificationSubscription` `async-scope` (sha256:35b753a60bee5da14d884fca08076dd9b576bd4b3a737f2a05e2a60caa6676a8): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:423:1` `detachNotificationSubscription` `async-scope` (sha256:2176607c00b685e4a1d2432f5d68853e8dedc17c8d6e640bab7f2576b21201e0): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:433:1` `disposeNotificationSubscription` `async-scope` (sha256:1a533b52269f4566aaabd8eafbfbf679e9e86142fe00ed11334e8eaf50e30e69): Portable task Rust lowering is not implemented.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:463:1` `releaseNotificationAttachment` `async-scope` (sha256:7f76cffd99accc4917fa113b88a48436c5c41189f53854319af043c88322686b): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:37:34` `getPermissionStates.ready:37:34:24ebc2b22456` `ready` (sha256:24ebc2b22456a00e7226f082338fe750ae6c68db4f80640c991ac703374932d3): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:44:10` `getPermissionStates.join-all:44:10:7593981dd683` `join-all` (sha256:7593981dd68357dc21d82b36dfcebc0a075fd344ec0829b38087ce642f2f8a2c): taskAll Rust lowering is implemented, but its containing source did not emit.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:63:31` `requestPermission.ready:63:31:2fe7e707fcd0` `ready` (sha256:2fe7e707fcd0630f09fd21b08496d47ec97c0a4d6ac5c93e206f7e7a5c55a5de): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:75:14` `requestPermission.ready:75:14:2fe7e707fcd0` `ready` (sha256:2fe7e707fcd0630f09fd21b08496d47ec97c0a4d6ac5c93e206f7e7a5c55a5de): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:77:14` `requestPermission.ready:77:14:c70844c32420` `ready` (sha256:c70844c324209c27303c57ab4cfa5b9935986583a8a6a1807200e3c534c79f55): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:108:1` `queryPermissionState` `async-scope` (sha256:4356fd3c9a6367e21efb543eacbabd6547b4e4d2a55062f636a3ab09c1de9cb5): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:123:1` `queryMidiPermission` `async-scope` (sha256:345a5db1a95f39f93b02cd794851b947dbb2e47044b54dc32f72976cccef08b7): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:134:1` `queryStoragePersistencePermission` `async-scope` (sha256:4b2e25003aaa254a255a15afb97a04e9afc671c6e3a98728ddab8dd1f221b15c): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:145:1` `queryNotificationPermission` `async-scope` (sha256:9d4f9399f2729f3b72927ee8bdfdcdb9486b1d2081c333bb13fd140e29faf807): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:161:1` `requestNotificationPermission` `async-scope` (sha256:9d1a87d9d18597a67596a713ca78da1683f3a9798152a550196556295cbea234): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:182:1` `requestStoragePersistencePermission` `async-scope` (sha256:9d9a503e3f660197ffcffaaa2f051320799533ece572865e329a75003c52224a): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:215:1` `requestHostMediaAccess` `async-scope` (sha256:80741265b8ab9c8a896e20874943836755fc5102a6efc602a0b9b5c9736f7046): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:226:1` `requestHostWakeLock` `async-scope` (sha256:d62e90d4e68baae99f933c768082dd0f236a5f6a2ee62ad5f08b084cfd35f1f8): Portable task Rust lowering is not implemented.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:240:1` `requestGeolocationAccessPermission` `async-scope` (sha256:dd921bc8af05b5d0a86b9f882a5a72e13c81bbdaa5e02222a9fb2d08bbe4f6ff): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuDeviceLoss.ts:52:8` `observeWgpuDeviceLoss.then:52:8:8419e35cfa99` `then` (sha256:8419e35cfa995580d87af94c7afbbd15e847d81678d5f819f1f2008fbbf5b3a2): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuHost.ts:32:14` `initializeTestWgpuHost.acquire` `async-scope` (sha256:2a49b63d4ebdd5977978166004add73c3d3ccae8d3eb5d7f67e9bb3d87915597): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuRenderState.ts:66:1` `createWgpuAcquisition` `async-scope` (sha256:f1e27be72bbc2ac0726466122b6cdc162ababf4ab333dd31eee1c52cd3f6759b): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuScreenCapture.ts:15:1` `createBitmapFromWgpuScreenRenderTarget` `async-scope` (sha256:7e2096d69ea1036c01030577a575dca5b51b3a34de818d0dddc852d0b2ef7839): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuScreenCapture.ts:132:1` `mapWgpuCaptureBuffer` `async-scope` (sha256:1da4506af014127e275821281050ed573f0ae137559ba42e86832eae9bbfd76a): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuScreenCapture.ts:137:3` `mapWgpuCaptureBuffer.catch:137:3:19fd6dcb960f` `catch` (sha256:19fd6dcb960fe21c3868f30a0efba0d1921c36f73e9fcf597006528705966c92): taskCatch Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuTestHelper.ts:279:26` `makeAdapter.requestDevice.ready:279:26:1032cf04da7d` `ready` (sha256:1032cf04da7dc0f8b6cc484070458a19db13b6febfcc3bcf01cc29e523535688): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuTestHelper.ts:310:1` `createWgpuRenderStateForTest` `async-scope` (sha256:9d0e3eb6a99dacf88dc020fc1b677dbb115198e067c50954c2d61f4783c52404): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuTestHelper.ts:347:27` `installWgpuMock.gpu.requestAdapter.ready:347:27:a9139df35f60` `ready` (sha256:a9139df35f600afd1271b7134f0310444b168a68bbfd801dc3ba3df003bf654b): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:18:1` `loadScene2DAudioResources` `async-scope` (sha256:ef068d6bf25188d6c4b8e19131d569035ef9986a4252f22281d13d8e6f4460ec): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:30:27` `loadScene2DAudioResources.resources.join-all:30:27:94e42ed969f9` `join-all` (sha256:94e42ed969f9023c1558dc148ca869c0c6d8b5a526da93fad3ecd5ed7b8a5bc4): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:31:18` `loadScene2DAudioResources.resources.anonymous:014c4c09995b` `async-scope` (sha256:014c4c09995ba8c55e50daa2d96114d02e21b4bcd39ef0cb9a9d72f6024a8627): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:64:63` `rejectExternalAudioResource.ready:64:63:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:19:1` `loadScene2DImageResources` `async-scope` (sha256:3b04df14c63a67c991e14f483f948a0f02fd08797c0b5d72fa8e09ddca99c2ad): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:31:25` `loadScene2DImageResources.sources.join-all:31:25:77ec66e45e34` `join-all` (sha256:77ec66e45e34f1217bd0b676363732c8d1543239c41bf3e1cc4b496940615c0a): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:32:18` `loadScene2DImageResources.sources.anonymous:ed3b14b6c7cc` `async-scope` (sha256:ed3b14b6c7cc1b2df2a78b7b4f07f179828805d27c9409ef986c7ce67b0d7eb0): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:79:63` `rejectExternalImageResource.ready:79:63:37ba15596492` `ready` (sha256:37ba15596492a0af99e763cee16f8e12aa4baee725e18de171f9db8163f5e025): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/scene2DDocumentSource.ts:11:1` `loadScene2DDocumentFromUrl` `async-scope` (sha256:49dce597317731f52aab32f841b209b1d9d6423cf002c393d4a6c0bbef3338e2): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:17:1` `loadScene3DDocumentFromGlbUrlWithCoreFeatureHandlers` `async-scope` (sha256:2deb03f338abf9120b56d32ec38b535b10f8f9362a9f8d93c5fe5ef54dab13b7): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:31:1` `loadScene3DDocumentFromGltfUrlWithCoreFeatureHandlers` `async-scope` (sha256:e7b0f927a411ceb02aaf07ccda2527c4fcbce9c6b8c9e14f9194002c8d2ffe79): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:58:1` `loadGltfExternalBuffers` `async-scope` (sha256:29f43ed3dba8af8677c22997d34a8b8bad47e29d05478fd2282a6b337ddec8a5): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:72:23` `loadGltfExternalBuffers.bytes.join-all:72:23:8f89bfb8e583` `join-all` (sha256:8f89bfb8e5832f1c5ebb565167bd58d27a9e1f019b90445771e98f500cf93e29): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/imageResourceFetch.ts:5:10` `createWebImageResourceFetch.anonymous:a9c2a94a69cb` `async-scope` (sha256:a9c2a94a69cbd18729a9a36e737224372bfb14b95051916d4dbaf146458fc47b): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:16:1` `loadScene3DResources` `async-scope` (sha256:d05ea967b4337d34fcf6817d0084d4bfc0097f94e66ba818e0cccea9850440c2): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:38:7` `loadScene3DResources.then:38:7:de0441a97802` `then` (sha256:de0441a97802facc88067100be36082051bfc2680abffdca2cd195677dd0e2e9): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:45:9` `loadScene3DResources.join-all-settled:45:9:9a6c49538a0d` `join-all-settled` (sha256:9a6c49538a0dd0d43196b4381c10243649d88fdecf2303e3b624df43dd3d2ee8): taskAllSettled Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:50:1` `waitForScene3DResourceResolver` `async-scope` (sha256:630808eb313bd9009acacca48b178848df40388f8a0539152c1a0f73a0fa622e): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:56:9` `waitForScene3DResourceResolver.join-all-settled:56:9:fbfa931863ef` `join-all-settled` (sha256:fbfa931863ef9e1b6320bb9ae20c8b9b8e7a620041f8d2a78ea4183d4c360d2f): taskAllSettled Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/resolveScene3DResources.ts:259:21` `requestWorkingResolutions.then:259:21:0d8a23d00189` `then` (sha256:0d8a23d00189a921fed0db2720613476fa4d85496d6429a52da5293b071e2cf0): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/resolveScene3DResources.ts:267:38` `_resolvedVoid.ready:267:38:9b8874eb7ffe` `ready` (sha256:9b8874eb7ffebcf300217391f3fb4a9d53d078bccf4ed67e3253de0dd1f14116): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/sceneDocumentSource.ts:16:1` `loadScene3DDocumentBytesFromUrl` `async-scope` (sha256:9ba99bd9eb9329ed494489200ee4f19d343610bfcdd5f3cfc4d014f31c9ab9a6): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/sceneDocumentSource.ts:27:1` `loadScene3DDocumentTextFromUrl` `async-scope` (sha256:d5e02b1b2e55ab21340c1ea52e41dae5b80ea458c58e29ac6f77df6b06e27df0): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- `@flighthq/shortcut` `upstream/packages/shortcut/src/shortcutExplicitDependency.ts:44:1` `attachGlobalShortcut` `async-scope` (sha256:6e48a46ace84161493bee03227852f88afa4ca06d04ed5a482ada5681938e815): Portable task Rust lowering is not implemented.
- `@flighthq/shortcut` `upstream/packages/shortcut/src/shortcutExplicitDependency.ts:103:1` `detachGlobalShortcut` `async-scope` (sha256:665c451d1098500106fed076d932707a64b14a182e5597c10a5bd828e76e9825): Portable task Rust lowering is not implemented.
- `@flighthq/shortcut` `upstream/packages/shortcut/src/shortcutExplicitDependency.ts:126:1` `disposeGlobalShortcut` `async-scope` (sha256:2aa5541bd92b8562efa0a2118fa573e3199c5f9d4083cf615e5dd8e8149825f6): Portable task Rust lowering is not implemented.
- `@flighthq/shortcut` `upstream/packages/shortcut/src/shortcutExplicitDependency.ts:142:1` `queryGlobalShortcutConflict` `async-scope` (sha256:ac2870c15f6f732bbb726ab7082ea9bd56dc3436ddcc646975f39cbcf8881757): Portable task Rust lowering is not implemented.
- `@flighthq/shortcut` `upstream/packages/shortcut/src/shortcutExplicitDependency.ts:149:1` `queryGlobalShortcutRegistration` `async-scope` (sha256:52f55c55516304e13ebbee1f5e0e70ecb5607788270bdc011762f0cb21b646d0): Portable task Rust lowering is not implemented.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:16:1` `loadTextureAtlasFromBase64` `async-scope` (sha256:b79c097cd13f07be0ca0f69e0e76c9f30a138c974531c354ad9f413fd691c2df): Portable task Rust lowering is not implemented.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:25:1` `loadTextureAtlasFromBlob` `async-scope` (sha256:7739ccf8b4aa714a5da53bafd133148035dd43473405024fc9cf223890c38eb7): Portable task Rust lowering is not implemented.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:33:1` `loadTextureAtlasFromBytes` `async-scope` (sha256:2b354bb32a6f7df16288e45a846428f1dd37c66959a7a7d70c4d60616a66754e): Portable task Rust lowering is not implemented.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:42:1` `loadTextureAtlasFromUrl` `async-scope` (sha256:b6efeac87e8d6fbafabdcdb8e6b4646d0685e1846cd3464cc8fa9e08b3922bfb): Portable task Rust lowering is not implemented.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:41:1` `runManifestTool` `async-scope` (sha256:98791a63db250404b5b35e6ba583cb36345a2e18891a426387cb67afb74b32c5): Portable task Rust lowering is not implemented.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:97:1` `readSet` `async-scope` (sha256:728e4746a677db2a2a1c7c3c3e106fe897d4d6b527138d5737676afea3d9f461): Portable task Rust lowering is not implemented.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:103:1` `runDiff` `async-scope` (sha256:3942d404c241af65a0560df264ba5dbb36424116566f5f158a5daf903e34f8ae): Portable task Rust lowering is not implemented.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:121:1` `runMerge` `async-scope` (sha256:d42bea15425b9d4fb19b700bf65b2ec7701fa5a2961593f851ea856fac051dee): Portable task Rust lowering is not implemented.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:136:1` `runPlan` `async-scope` (sha256:2f8f2063693db0e9cd11edf677d7a10dda436b2d5f628c0c9feca91020b22fdb): Portable task Rust lowering is not implemented.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:176:1` `runScan` `async-scope` (sha256:bf434b366b3df4fa5f89f623314ef39f95b5171f471f984b826b5f853257ea37): Portable task Rust lowering is not implemented.
- `@flighthq/tool-pipeline` `upstream/packages/tool-pipeline/src/pipelineBuild.ts:37:1` `buildToolPipeline` `async-scope` (sha256:22f9541a877470380b7f36d06a74eadbd2eb3fe4f7d9ad989ae6ad4dfe2bdec0): Portable task Rust lowering is not implemented.
- `@flighthq/tool-pipeline` `upstream/packages/tool-pipeline/src/pipelineBuild.ts:98:1` `pathExists` `async-scope` (sha256:169284220a0f41c2b29faaa71f28e8599750db473261568a6468556dcb442175): Portable task Rust lowering is not implemented.
- `@flighthq/tool-pipeline` `upstream/packages/tool-pipeline/src/pipelineBuild.ts:108:1` `publishToolPipelineOutput` `async-scope` (sha256:703aa4f36977f70f39a747841f0f5fe8575e15bcfafcc750c606cf8194cacdec): Portable task Rust lowering is not implemented.
- `@flighthq/tool-pipeline` `upstream/packages/tool-pipeline/src/pipelineTool.ts:10:1` `runToolPipeline` `async-scope` (sha256:a9bf6cb14c196c84fecbc88b0b7c860f3d6301b2857090ef7d9693808e7b89d0): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:70:1` `createTrayIcon` `async-scope` (sha256:8e5a11c3a325f966acb1562fb9afe780a21355a8e1a2c51b42c609c172f0e979): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:92:32` `createTrayIcon.ready:92:32:9b8874eb7ffe` `ready` (sha256:9b8874eb7ffebcf300217391f3fb4a9d53d078bccf4ed67e3253de0dd1f14116): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:105:65` `destroyTrayIcon.ready:105:65:eb6d945a1c41` `ready` (sha256:eb6d945a1c41cd24efcae78c4420d7bf2fa6121a5097481418898451a220eb3c): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:107:28` `destroyTrayIcon.finally:107:28:fe4966478e8b` `finally` (sha256:fe4966478e8b850ab61f6761842d36e6252760201df4121c71c893e3fbcbaf23): taskFinally Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:128:1` `destroyTrayRuntime` `async-scope` (sha256:bf9ab5ba1cb7e8518118f26c0db17ae5e3056424e0bf4c6eef9367c72602a9f2): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:312:7` `attachTrayEvent.release` `async-scope` (sha256:2497ec1f7b535094570b205084c42e53315d7dd775983204df8ad6dfc1b473ee): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:351:1` `startTrayIconAnimation` `async-scope` (sha256:7f231c223ebf4f8b947d00496a5c0ba85e2c2ce4f6d0e94cedea841332c9e842): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:373:5` `startTrayIconAnimation.release` `async-scope` (sha256:16cd72440ffea00be0e2a01285fdf3e819c430130fad6977ac7e748364ad6ed4): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:383:1` `queueAnimationWrite` `async-scope` (sha256:6a56458894f87d39deb02bc52fe80c911a6354066d709f6e8d21de7caf1125e9): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:391:32` `queueAnimationWrite.then:391:32:d7e87b0a50eb` `then` (sha256:d7e87b0a50eba2acb3ffc0c514ad5d39f0dea9a8f843e3388dbe07ff3c772f0b): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:391:64` `queueAnimationWrite.anonymous:635632bebbd9` `async-scope` (sha256:635632bebbd9fcf299f8f91a9c13ab2672e8ebc0a35df4291e3dbcfc21a83e6d): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:423:1` `invokeUpdate` `async-scope` (sha256:1b318cf7e8e896c137e99f815636839768d29891a5e871ca4af8c7cb0b340926): Portable task Rust lowering is not implemented.
- `@flighthq/updater` `upstream/packages/updater/src/updater.ts:14:1` `checkForAppUpdate` `async-scope` (sha256:edc555a8b15bb82cefc36b9d338ae9bb2714c03df6663dfb6936404805eb8483): Portable task Rust lowering is not implemented.
- `@flighthq/updater` `upstream/packages/updater/src/updater.ts:45:1` `installDownloadedUpdate` `async-scope` (sha256:dfd0fe22e04ff0da48d9054725e9d3be041eafeecd3a1224166da93ac76b2554): Portable task Rust lowering is not implemented.
- `@flighthq/video` `upstream/packages/video/src/videoResourceFrom.ts:11:1` `loadVideoResourceFromBlob` `async-scope` (sha256:4444c971bb966d4df923ae0ac42a9e9c8dd9363cdf9407e4c1a1c30921027fdb): Portable task Rust lowering is not implemented.
- `@flighthq/video` `upstream/packages/video/src/videoResourceFrom.ts:38:47` `loadVideoResourceFromUrl.reject:38:47:19831d73ff8b` `reject` (sha256:19831d73ff8b183ff5d3551fe4fac61a288acd7483e9ae649e3a344f19c89f75): Portable task Rust lowering is not implemented.
- `@flighthq/video` `upstream/packages/video/src/videoResourceFrom.ts:39:10` `loadVideoResourceFromUrl.then:39:10:55b8f708c765` `then` (sha256:55b8f708c76578679049e393f4d076307990096e4936b200a27bdcef6403e476): taskThen Rust lowering is reserved for Pass 27 Stage 4.
- `@flighthq/video` `upstream/packages/video/src/videoResourceFrom.ts:49:33` `loadVideoResourceFromUrls.ready:49:33:8dc66a891dc0` `ready` (sha256:8dc66a891dc00c6da20c45026f4d2735b90d17e2fd3226f4db31818040badc46): Portable task Rust lowering is not implemented.
- `@flighthq/vite-plugin-manifest` `upstream/packages/vite-plugin-manifest/src/manifestPlugin.ts:82:3` `createManifestPlugin.build` `async-scope` (sha256:b89706c7e984370955dc017fbcbf5dc92276b16206303c58d3994ac0d2cc6afa): Portable task Rust lowering is not implemented.
- `@flighthq/vite-plugin-manifest` `upstream/packages/vite-plugin-manifest/src/manifestPlugin.ts:159:11` `createManifestPlugin.load` `async-scope` (sha256:c6251ff4d54c06b778e5de94dfd6843c492459f6b6f2cdc390805d4d92ca78b7): Task output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.

### Unsupported async scopes

- `@flighthq/assets` `upstream/packages/assets/src/assetLibrary.ts:149:1` `loadAssetGroup` (sha256:188d51024cb73e45ac54868edd4ae13e87859c658cbe0a8476dbe34b01d289ec): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:33:1` `loadAudioResourceFromBase64` (sha256:86b5f46fce60b8506f3b9fb74b18b2e4b544c3413504a56a61e3ae05211bedf7): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:46:1` `loadAudioResourceFromBlob` (sha256:b9c4734232552c9160a8454b6dc2affdac69b54bd51bed26eb6cc45a03f686e5): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:59:1` `loadAudioResourceFromBytes` (sha256:fdaaaa08b20b3f3afcf1ebec521aed24b757857fce7b9ded6203ed6046ccd78d): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:77:1` `loadAudioResourceFromUrl` (sha256:5dbf1599d68fc7bda16ecec38433c7b3af6994a4b49f276066e7f3dd0a3a2bc2): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:87:1` `_loadAudioResourceFromUrl` (sha256:f003cb4174c556bed4c870c6e5c717f3bd49b58a546679efd5f6b193d27aae93): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceFrom.ts:114:1` `loadAudioResourceFromUrls` (sha256:85566fe200667c48a3823a75bdc3b29642c1ee200ae9f6a33d8c5d79c9a2d710): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/audioResourceReference.ts:141:1` `resolveAudioResourceReference` (sha256:c2351a145ce3eba30f060e44fb9c2a9c6c87622b6d01fda5f55a6d7f4899a501): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/audio` `upstream/packages/audio/src/decodeAudioResourceBytes.ts:19:1` `decodeAudioResourceBytes` (sha256:6676e4daedc047fc7890deed29b85ee332aa21105b32f3ac92e8b6c6874750c0): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:55:1` `findFiles` (sha256:f945c32cbca14094e2808e51af8cdd791f459d9d2ad11133a6dc1a27cfc07c5c): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:187:1` `readDialogHandleBinaryFile` (sha256:aa86eef1938205026ca2e8df97ac880284c1638e6ab50f9ff35b71ded9e10b65): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:199:1` `readDialogHandleTextFile` (sha256:2d970d15beb55f8aac7eb6d37fe230687df534a06f9a1bb8a7ce9b05616d3cfa): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:295:1` `writeBinaryFileChunks` (sha256:90b2a2b4cc4ce769c8e93c1a27d35d89eebbc50568cd946f8d9b24aa8fd13403): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:334:1` `writeDialogHandleBinaryFile` (sha256:87bae656f18496b1b4cbdad02935b19ace4046fe3c502fbdc69f5d16753601dd): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/filesystem` `upstream/packages/filesystem/src/filesystem.ts:347:1` `writeDialogHandleTextFile` (sha256:933771a09107b298e7725d790a669edba5f616c9a63428111846f9b29d6697fa): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/_fontFaceLoad.ts:6:1` `_loadFontFaceFromBytes` (sha256:7320850a3926a27d2228d4f2b0fce53c5157049dd15091f787a21afcdfd29c2e): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/font` `upstream/packages/font/src/_fontFaceLoad.ts:44:1` `loadAndRegisterFontFace` (sha256:1a006a9ec3a17aa2a4d351f916d68a6dfb10d45b0bf0a20f2c2e8f681002ea49): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webApp.ts:32:23` `initializeWebAppBadgeBackend.anonymous:7722fac082ee` (sha256:7722fac082ee02a5b7f71f81bee7c7ecf4ba4dd0c42377c40d4bd064ecc5d7a7): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webAudioDecodeHost.ts:9:3` `decodeWithWebAudio.decode` (sha256:eb1ba276bb31025fe36a4cd2ecd5612922dbe9ecea7039b5d57d620099d9b64c): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:65:3` `initializeWebClipboardFormatsBackend.blobFromFormatData` (sha256:c1fb9e0ee718a360876a579fe9e1e5fdc16157f03db3dabc983b233c321466dd): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:72:3` `initializeWebClipboardFormatsBackend.writeFormat` (sha256:29328f33b90e3ead78ad617623031ae288c663d52699722b9018bc8509779dad): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:83:3` `initializeWebClipboardFormatsBackend.writeItems` (sha256:122e97ba7bfab133a56b3b699ebddbe1d3cf9dabdb17de93cbe5b3c4aaa4920d): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:99:19` `initializeWebClipboardFormatsBackend.anonymous:158d58a052a9` (sha256:158d58a052a95eaac54a01cd980088e8ec42a56f1c31ea2b5c92cd1d140cb28f): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:111:18` `initializeWebClipboardImageBackend.anonymous:c12cfbc1fff8` (sha256:c12cfbc1fff820cfbd900bf4dd255fc91ab2076be7482f694ef01125f709a1a1): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:113:20` `initializeWebClipboardImageBackend.anonymous:1d1422ff3263` (sha256:1d1422ff32639d98cd482f40476cf29ec20a55a359e17293c184f24f45142cf5): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:129:17` `initializeWebClipboardTextProvider.anonymous:298f67a20d28` (sha256:298f67a20d284821512fc3f6329ea4a8b42cf78df9eff9b8f343f0afd317d475): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:139:1` `getWebClipboardFormats` (sha256:e2f4ac97c33f4184f959c31f4387c3732a64d66ef02447a837ff28ee4970a2dc): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:181:1` `readWebClipboardFormat` (sha256:70231f4d129a64d2ded6eb37aff67d3c98c95ba59917afc11c1bb994295de6ff): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:195:1` `readWebClipboardImage` (sha256:7befe20cf0f6e9e0dbc6d7cb1e1679023711a9a82a9a01b92124cbbbfffb1556): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:208:1` `readWebClipboardItems` (sha256:cd9eca2adbc48dceb00bf290bdab241e43dbc8bbe30c6cb2d843c825009d232a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:227:1` `readWebClipboardText` (sha256:e9b52a210afbafa718e4ef0c99e34e03db3e0859ab0352572eb021f00ef0b830): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webClipboard.ts:237:1` `writeWebClipboardText` (sha256:455c0ecd72d8eeadfd0d96bc216dc6a77d1de8ce3f3830f7bad5f6b53d95e6eb): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webConnectivity.ts:84:32` `initializeWebConnectivityReachabilityBackend.anonymous:2dc70fca6c7e` (sha256:2dc70fca6c7efc30cb41acbe7e3ca02e49e962616b6a88bf2a612ef8903083d1): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:46:17` `initializeWebMessageDialogBackend.anonymous:8f5fa360ad1b` (sha256:8f5fa360ad1bd77680e60f3e88c42e00f025909ac887d86735fde3d5855e04cf): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:55:17` `initializeWebMessageDialogBackend.anonymous:d7dee00bddb8` (sha256:d7dee00bddb85e5a455b5844b7aa02107da564afda145148fda619728768686f): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:75:16` `initializeWebPromptDialogBackend.anonymous:6eac38774a4f` (sha256:6eac38774a4f7efc8deaf411a56a1575fca0612eab87cb298440277da0fbd67d): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:138:1` `openDirectory` (sha256:9be8e96047dabae3a7ee8e9b813b355ea8be9fda492876df37757f59848def21): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:163:1` `openFileSystemAccessPicker` (sha256:55e46c0f7864422cea17f4fb99b4b17769c8835091e61e9ad58b64ac91195f32): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:259:1` `saveFile` (sha256:a5a5db15a6ceaa9384d3b0ac560920cf2d9defc55d0d33ff9c1e07f6676d0346): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:279:1` `openImage` (sha256:cd80d73ffa76c38aaa37d79867c34990a32a036141dab7a97cbf167179fc5b16): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:290:1` `capturePhoto` (sha256:faf58363764a48f8a2e92b1c589fda6c29894a5fe0f7b54e2bf69dbf37cde1a0): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:306:1` `captureVideo` (sha256:3f56640e626412ab5794052302468d1e846477b28c3e7e255857d43ea1ebe89f): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:586:5` `retainedFileOperations.readBinary` (sha256:a7d067107188ae105abf5f81949745cb507a45aec1b4e047dac1f35fd3bcce7b): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:594:5` `retainedFileOperations.readText` (sha256:be47184491269aee86a401f9091cfff25f45fa254c6f4f29e214e18fd9e8a76c): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:607:5` `fileSystemHandleOperations.readBinary` (sha256:00814413734471d772248abbac2f8bdd9248239662c3ca99b9ccce64823bdae2): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:622:5` `fileSystemHandleOperations.readText` (sha256:ab160fc0443273980ba746d7bb6fa186de4fe8951815b9e8a38f952d093c185d): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:637:5` `fileSystemHandleOperations.writeBinary` (sha256:dab78eda45d46903147fa1063f38c950c9dfa6b311aea999cd8573bb39db2475): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:641:5` `fileSystemHandleOperations.writeText` (sha256:172ea3833e605d90447b67c01f801cfe980b0d4c71480cbdaa17ac56a1eb11c0): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webDialog.ts:648:1` `writeFileSystemHandle` (sha256:18295f0d91e9d34dadbfc15499af211ebbc169d9080bb84ffb584b5b46d8fa62): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:6:3` `webHostFileSystem.appendTextFile` (sha256:61f7174a25870e6bb51819eec7bb5bda8c299a9428137bd4824a94b56984cbcd): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:23:3` `webHostFileSystem.canAccessFile` (sha256:74a2f199f13e597db145981b88bfc0b4e3e7a92cfeefb845c64fe286b8e04ef0): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:41:3` `webHostFileSystem.copy` (sha256:e1e892418c0c5d49918483c740ad63791036835cd1417d63903cd2c0e37a804e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:51:3` `webHostFileSystem.directoryExists` (sha256:56d4ea2626bdcd31b546678a3acf52398dcc20bfb63a150cbc61ac02568f185c): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:55:3` `webHostFileSystem.fileExists` (sha256:a50063fec4839730a28a8f732afd1ccce90aff1eb8923b58fe7d40304c34af52): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:58:3` `webHostFileSystem.getFileSystemUsage` (sha256:5c331a37e8df7c5443b1649881acd1b5a0c8af50fadee4499702df7337c7e565): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:69:3` `webHostFileSystem.makeDirectory` (sha256:b47d09fb68f5f7cffeb77f07f43272329248f0b39d4092606a05ee2e7475e0a2): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:73:3` `webHostFileSystem.openFileReadStream` (sha256:0a2d3e03e87c35ca13b5eeecc3150a1666ac5c83484e688bba8e65e0e7635f82): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:85:3` `webHostFileSystem.openFileWriteStream` (sha256:e2101321c7b81f6fa6980281059d2cdae656a06be8cfe5fe8c1cf6cbfc1ecd2e): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:97:3` `webHostFileSystem.readBinaryFile` (sha256:10f7db5888fe084427466596e6abe2110f21343825be57b226941e0d901dc743): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:115:3` `webHostFileSystem.readBinaryFileRange` (sha256:448a06e36ff589824ad01d59ad93d802408619f160448942bec63401ca701402): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:134:3` `webHostFileSystem.readDirectory` (sha256:c8b9339953de1c244cabfa386687e130dce05940f646fda8494e9a76beaa7356): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:159:3` `webHostFileSystem.readDirectoryRecursive` (sha256:773df7a02fc33f283bad74c8a8b33c0790d85901f6340e7d53856a6eb33100ef): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:176:3` `webHostFileSystem.readTextFile` (sha256:a441f7e773723246a5e8b1ebe798f7b54a72cab3910b245012d698700da996c3): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:194:3` `webHostFileSystem.removeDirectory` (sha256:cdc1977fd620c25d864f59bd5f16869540e60e0a21df9c820ef7ead511a6bd7d): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:208:3` `webHostFileSystem.removeFile` (sha256:cdf596ec2cd0dac70862bb0c1bd10d7a44bd4b8ba84bcc992134b7d934b10b63): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:211:3` `webHostFileSystem.rename` (sha256:74e173b8d71d90660c74e82ea126f1632c4616f83a5984dc910d1f0c417764ed): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:215:3` `webHostFileSystem.statFile` (sha256:89681b0982792536e5dea921a570a4cedf0b8746b0ad75efb28179b456f60add): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:235:3` `webHostFileSystem.writeBinaryFile` (sha256:36b0a9115b4c35c47f193dd94072e442a7d19cc4605d9eb7472f91ae70eb6f9b): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:239:3` `webHostFileSystem.writeFileAtomic` (sha256:7c0161d85639e9ad87e893f4a7db5be7fb8174d9b22e255d1a4db2ea3eb04580): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:260:3` `webHostFileSystem.writeTextFile` (sha256:d27ff883d2ba5e33c3091f3f99d42bd84ab4f5093728626ebb42d8f4a71688bf): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:276:1` `getDirectoryHandle` (sha256:4be00ff9889ad9c9843e3f5c1e6975e1dd9e83f85dfdcd03c57f0e1f61f4526c): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:290:1` `getFileHandle` (sha256:b729a6f6c2badcfde84cdc259d610652496bfeb585bb2d28192b8cebd6fcb506): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:304:1` `getRoot` (sha256:0bae26de7124174f36022d3d2eced8575c8a426ef8bd22273906acbef7dee990): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:319:1` `removeFile` (sha256:ac4e32e9f1b9879492f7736ddd4e26517f9c121c04e39dd6dc7307f34c877f80): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:338:1` `walkDirectory` (sha256:8d7fffae00b2add1f0e47ec93773bc983a21e7a6d6cb4ff61d5a7ab6752cdfa6): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFilesystem.ts:357:1` `writeFile` (sha256:4b7b28650b9818f0bcdca434bd60565b91acc2de26ac3bb94418bc8ef2ccd75a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webFontLoading.ts:13:14` `webHostFontLoading.whenReady` (sha256:c2707a11ca2d3dd78ad32c6a3685218357fd1c0e5d548b2bdd4f10b1b6e737a9): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webHostWgpuContext.ts:13:17` `initializeWebHostWgpuContext.anonymous:5c77ab86f0db` (sha256:5c77ab86f0db4a52b4c96ad50b5e5e6f21fc2d167502abac40347ee3f766a3a5): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webImage.ts:20:21` `webHostImage.loadImageFromUrl` (sha256:5f99946cd87190e9f14dc9527e8b2afd64c4d8bcaa1d9fb120497dfeb8d76df0): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webImageBitmapComposition.ts:42:1` `resolveImageBitmapComposition` (sha256:6500eb8a3798aa9024d5063ea23996aa936647777c0a53aea75d7b046e367c7d): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webImageDecodeHost.ts:9:3` `decodeWithCanvas.decode` (sha256:61dec2eca4383679897ff700592fa77bca87619b9d44122c69d447d83e30239e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webImageEncodeHost.ts:10:5` `createCanvasEncoder.encode` (sha256:d925295fa16f025170d731973c40c8d83434f5964d1a1eac3b5401bd5092c56c): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webKeyboard.ts:11:14` `webHostSoftKeyboardChange.subscribe` (sha256:ae4ac32889b70faf712bb38fdb0ccbd2d0c02b59a6f374bea8454f2985ad757e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webKeyboard.ts:43:9` `webHostSoftKeyboardVisibility.hide` (sha256:244807309d2ec2b07bc08a7ee324984a1e78f345a405506b694e40e067aaa140): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webKeyboard.ts:49:9` `webHostSoftKeyboardVisibility.show` (sha256:b631ab134e72bb5524d521dcdd886a07befb9cabfd657e64a63cd595c8bc7f13): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:26:23` `initializeWebMidiAccessBackend.anonymous:755b5dc1a7e0` (sha256:755b5dc1a7e069c5c50c1e0b862940b8227190ca8c64572ea310899c8644a3c1): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:43:17` `initializeWebMidiEventAttachment.anonymous:cc858b57020d` (sha256:cc858b57020d9e8aae42c11611fbf459200fdb109d5282a8f4d5daa15aa1a5ae): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:77:14` `webMidiAccess.toInput.port.close` (sha256:43a702d4abfd706518f8c54a1b47664cf6a7049d5e6aa623a255eecc394f9e66): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:82:13` `webMidiAccess.toInput.port.open` (sha256:c0e355dd7eba9dd3c1150d04622aee0267459f4351482d8236e2245a2744b9af): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:95:14` `webMidiAccess.toOutput.port.close` (sha256:43a702d4abfd706518f8c54a1b47664cf6a7049d5e6aa623a255eecc394f9e66): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:100:13` `webMidiAccess.toOutput.port.open` (sha256:c0e355dd7eba9dd3c1150d04622aee0267459f4351482d8236e2245a2744b9af): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webMidi.ts:178:1` `queryWebMidiPermission` (sha256:13aae848f5205dc28bc2d1550e1d3efe8fe8e015ddf532d8ae6bfa8696e6cd82): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNet.ts:19:24` `initializeWebNetBackend.anonymous:f4b378cb2e95` (sha256:f4b378cb2e9548bc3405de12c662eacdc48f901e262a3f0bd4546c2d75bcf4b8): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNet.ts:77:1` `_readNetResponseBody` (sha256:a65ec98271b1315b2d9f92bf7f23f3862fc0a2eefa19665fa73e7818af9bfc60): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNet.ts:98:1` `_readNetResponseWithProgress` (sha256:2fa2d01a1a3391986cc439013a763959207aaacc5a66efdf1c5f6ac215296646): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:28:3` `createWebPageNotificationCapabilities.closeOne` (sha256:1a977d98f99e5dc7c5eb53011950029669f9ac7c6da0ba2336269fe9bb2f121a): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:39:3` `createWebPageNotificationCapabilities.closeAll` (sha256:1ad18279f2ba368997c445404cf159d4049c51724357d7361b8726068aeb3b73): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:51:7` `createWebPageNotificationCapabilities.delivery.notify` (sha256:14fa545dd5dc83d61e295ab2856b8a05658cac4fadc571cca145d8385460f3b6): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:91:7` `createWebPageNotificationCapabilities.lifecycle.destroy` (sha256:9b61ba27cc64b102c5771b9181ead6b40cc010eebbccd50e4dedfa75cacb886f): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:109:5` `makeWebNotificationEventBackend.attach` (sha256:2ecb63ca714566e1666b725946e562b754557027b0fbdfed6fe38673ec4aacc6): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webNotification.ts:115:11` `makeWebNotificationEventBackend.attach.attachment.release` (sha256:77f61cd1eacb8d04122c1c92e1b6a756c152baaf908c1c0088163a39409e3f54): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPermissions.ts:19:5` `initializeWebPermissionsBackend.getPermission` (sha256:eb2d7e7d7268d418c7cfffdbaf00d4c2690dd1fc42996b5b645e951346cf36cf): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPermissions.ts:27:5` `initializeWebPermissionsBackend.requestPermission` (sha256:7bb49dd65928a29b0cbbe037bd6884d9045c739202365c60411a286b7a19b792): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPermissions.ts:42:1` `queryWebPermission` (sha256:6403297b8e566b02211c0e1c488de42029e375393e0badaa693e5fcec16f972a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPermissions.ts:60:1` `requestWebMediaAccess` (sha256:14dbf5a38b39ea8eadf1e11d48da9844ae0fc78cecad7f0f538cfe7a50d25ee7): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webPermissions.ts:81:1` `requestWebWakeLock` (sha256:a26465753f49a80c8ecc71484a5d538000866c37fb603d99d3458a783c583051): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webScreen.ts:256:27` `createWebScreenCapabilities.detailsBackend.anonymous:89703d863b4e` (sha256:89703d863b4e7e30f14adec7cebe46181e6a1340fb1da9a614f371850516bc83): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webScreen.ts:265:19` `createWebScreenCapabilities.detailsBackend.anonymous:57575bfd7269` (sha256:57575bfd7269ee5afd44d863fd75b16f6c6831fafe598a7fe6a894bddbbf97d9): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webSensors.ts:71:27` `createWebSensorsBackend.anonymous:a957cbfc183a` (sha256:a957cbfc183aae75cc7d7b4e1d863f901079a58602a6dc4e85244734a7c27a2a): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webSensors.ts:371:1` `getWebSensorsPermissionState` (sha256:ae0c3340d40546bf3036b49fb90747d9572219eb5d78be5db02f27f01c5ab35a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:31:3` `createWebServiceWorkerNotificationCapabilities.closeOne` (sha256:e2ca64e091727fbc0d4f3fb5b2203bd6da786ecc70aa04ea8304117935df94ec): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:49:3` `createWebServiceWorkerNotificationCapabilities.closeAll` (sha256:842a6eea49929ef76fba68cf638adbf004f3d5c9a0e8464d4688c523813f79f2): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:60:7` `createWebServiceWorkerNotificationCapabilities.capabilities.activeList.getActiveNotifications` (sha256:2d29e46e75c040af4f4cd2f9d2ce1da799844fde4fa45534b52c45501c77c3a2): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:78:7` `createWebServiceWorkerNotificationCapabilities.capabilities.delivery.notify` (sha256:4862385c636b740d46b7c6c13fe757ed603b954a76c416957a117e9003340080): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:99:7` `createWebServiceWorkerNotificationCapabilities.capabilities.lifecycle.destroy` (sha256:20d19c98239fbba7bb1b2b04a2321caf273b21bf4a06843deb9ebee75034d62a): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:111:7` `createWebServiceWorkerNotificationCapabilities.capabilities.permission.getPermission` (sha256:b88af037a878404b3d5af13d8f31b3eac4595c4d0165c0345733c035f81643f9): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:118:7` `createWebServiceWorkerNotificationCapabilities.capabilities.permission.requestPermission` (sha256:5582804874ee6c69b2f295cc70b4844c346db5cbac6a876a5ad62a49e1b77906): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:169:5` `makeWebServiceWorkerNotificationEventBackend.attach` (sha256:2ecb63ca714566e1666b725946e562b754557027b0fbdfed6fe38673ec4aacc6): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webServiceWorkerNotification.ts:175:11` `makeWebServiceWorkerNotificationEventBackend.attach.attachment.release` (sha256:77f61cd1eacb8d04122c1c92e1b6a756c152baaf908c1c0088163a39409e3f54): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:13:22` `initializeWebShareContentBackend.anonymous:446cdd4ed448` (sha256:446cdd4ed4482f254bde397f0e7e3e9c7a861ff8118e5489cb727106d32ea914): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:17:32` `initializeWebShareContentBackend.anonymous:55aeac6f5f71` (sha256:55aeac6f5f71b5d6a978ae206fac5241944c84d874937094c757fda62d088ff9): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:32:22` `initializeWebShareFilesBackend.anonymous:afbc5a443935` (sha256:afbc5a443935606a1cd655c54d3c39233874db4683b3829b8eaf754f33df46e9): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:40:32` `initializeWebShareFilesBackend.anonymous:26e8ec61137f` (sha256:26e8ec61137f6ec29262ba9c4a8c32f03da6545076dc9e3de5be26ee7f9b237b): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:98:1` `invokeNavigatorShare` (sha256:6fb73a5e473e993661139498c0a9ed46b5917eb24c888160c73103a876a661f7): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShare.ts:108:1` `invokeNavigatorShareWithResult` (sha256:bc7bfe297293d8d1dc45dbd136a551007f761160be9579f975dc347166514a95): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webShell.ts:4:14` `initializeWebShellExternalBackend.anonymous:fa929f9b108b` (sha256:fa929f9b108b26f3a563fb5a9e2247593ca6a18b630bfeb78557dd90d359bc96): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:18:7` `createWebWindowStoragePersistenceCapabilities.persistenceRequest.requestPersistence` (sha256:1b29a9d327ff84e9dd50dbfd011ab7073d0da35fc3a04b631431b0eb3d23de34): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:39:5` `createPersistenceQueryBackend.getPersistence` (sha256:a2dfb870f260b8080375e940c7b3d274cf9a741325ea7d6fe89f9df24e17899e): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:48:3` `webWindowStoragePersistenceCapabilities.getPermissionState` (sha256:dbb32ac59265bc94471284f431a476051ddc90673ef34b1aa18bb8a1aeb62e01): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:52:3` `webWindowStoragePersistenceCapabilities.persist` (sha256:46e8f7cdee9608f80f9c9a3223f4540a8d57fa171f3a700ff39781197b0c4f31): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:55:3` `webWindowStoragePersistenceCapabilities.persisted` (sha256:e2e3f11c91ab6bda15834024257d83d029723db405b322eb5ca6e5f5f80b9cf7): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:63:1` `observePermissionState` (sha256:fa24d51d3c50a097a858b8195158146215903864f3a27d331ec6a36865868026): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webStoragePersistence.ts:72:1` `observePersistenceOutcome` (sha256:82cf7b59567f1dbafbb31ca934363baac497fb69dfc28330d29e9d8b394959dd): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/host-web` `upstream/packages/host-web/src/webWindow.ts:31:9` `webHostFullscreen.exit` (sha256:ae9910143afed739b88eac0b75920a47d7033b0150bfdef22ac7836a42dc6c02): Portable task Rust lowering is not implemented.
- `@flighthq/host-web` `upstream/packages/host-web/src/webWindow.ts:40:12` `webHostFullscreen.request` (sha256:d1cc8422674333d68d56f518dce4aa9a22aee9b0ebed624d2ea2851b3f5f257d): Portable task Rust lowering is not implemented.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:22:1` `loadImageResourceFromBase64` (sha256:dbdcb8291bc16014b22358459a0dc39484688e9cb2d47bd2dcb46381082499b9): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:31:1` `loadImageResourceFromBlob` (sha256:498ba360a53d9ef9dcb3160a29e0035b3f8b5bd4035a4b086ecd414a31ba7b5f): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:44:1` `loadImageResourceFromBytes` (sha256:bc0df7b2796ca2e47a9565e4b8ce88966df7143013b0f218b941ad2d54198fba): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceFrom.ts:58:1` `loadImageResourceFromUrl` (sha256:9416567113dd62bbb7f8084f0790fb235b5a3404cf7d2f8dc05cb2a622c63644): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceReference.ts:48:1` `decodeEmbeddedImageResourceReference` (sha256:a27cae9978b54042771849619a6657fa2568ba2d7be5e68bd3007015958e850c): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/image` `upstream/packages/image/src/imageResourceReference.ts:168:1` `resolveImageResourceReference` (sha256:8abe34ac1a30f05db048471b3ab7d43f6e64d45a8fd4307ab3eec77d0a160c83): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/loader` `upstream/packages/loader/src/load.ts:18:1` `loadBytes` (sha256:dcae29752339f563801720a28084619b8ecd14d7ad3c7136195fe260e35efafc): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/loader` `upstream/packages/loader/src/load.ts:34:1` `loadText` (sha256:a77b3f353a88ea39979bc0daf5189b594c70c35a52fd81c4a2585041d641fb16): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:512:1` `drainQueue` (sha256:88082b65b0b72d5a7051045ab407875a77a7430ece259e4bf911600ef6289ed7): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/loader` `upstream/packages/loader/src/resourceLoader.ts:562:1` `runEntry` (sha256:0cc4fef1fe859a3eba70abead965e6eb2ab15fc8f1671ae50442eeba76acdea5): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/log` `upstream/packages/log/src/log.ts:304:1` `destroyFileLogSink` (sha256:aaddc0709f7b943eaf89e4625f68abb2f26a6f28a658553afdf8e9c9793acd73): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/log` `upstream/packages/log/src/log.ts:678:1` `_destroyFileLogSinkState` (sha256:0f542da762d0de88d6bc0e4a20ed1ce995f8823fd11e12036c649b4b9680ea72): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiAccess.ts:52:1` `requestMidiAccess` (sha256:b808895722ea45b12579bfb2cf623cbdc08bbfaf700e3d36baca2748b294433c): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiAccess.ts:62:1` `disposeMidiAccessKnownPorts` (sha256:0a498b917eda125e3a67e6f05872e6a1e925d01f0a045236738a2a3f4fd4f75f): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiPort.ts:31:1` `closeMidiPort` (sha256:bd4cf863b7e68276c4bef08184c81c7ec7c560647e392fe3a5448491c27b618a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiPort.ts:124:1` `openMidiPort` (sha256:697c7bbbec42915f1d576a4329dba07f17ce131ba107571abe325cf7ced67560): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiPort.ts:168:1` `disposeOwnedMidiPort` (sha256:06c53d301a72d3d58596586e20c1bcc16968e2398f743d924a35b270d65fb843): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:160:1` `attachMidiSubscription` (sha256:b8c0577fde0f154df2b12d2ec5f0015fb9fec7cc0978096a278afa5800cee4b2): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:190:1` `performMidiAttach` (sha256:def29cf853ce63812f8c8c818d78934872d029085f1b48dcbc1779f0eb068fb6): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:217:1` `detachMidiSubscription` (sha256:a338a04f550381803c30a927d354e47096ab0cc813ccac04d97eae4b724b7ec3): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:236:1` `disposeMidiSubscription` (sha256:de01b0e5923187efb70b3331726b88d39217498be3c620ff5118b2093546fe2a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:264:1` `releaseMidiAttachment` (sha256:8dece981a791ff2281b09e369af11927e7beeb935ed4b14f653381232d19b20b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:272:1` `releaseTrackedMidiAttachment` (sha256:d524f4c0182d90196500cc1b81aa9db67d7fdf8fbbac5236fb190ceb1b6515e4): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/midi` `upstream/packages/midi/src/midiSubscription.ts:284:1` `settleMidiAttach` (sha256:59601c489eb9afece7e87a203acb11fd1fc41e26d9b7e39a8cf6171f8e131c91): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:43:1` `attachNotificationActionSubscription` (sha256:ef0567517b8ad81167f34ae8b0e554e74693041ca162e320d0bfaa12509444e3): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:55:1` `attachNotificationClickSubscription` (sha256:6c730b2ab6b0c044ded291a138010ceca2af5b4629daef97ccbff4982a5b3275): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:66:1` `attachNotificationDismissSubscription` (sha256:538af16a57895fe0a4e39a2d268ac7fc966d4898d147183696fe93bf6b462d51): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:77:1` `attachNotificationReceivedSubscription` (sha256:35dbdacdb96e8b8568d1da2d4607c8fcb506474c23d6a9564e2ded4b7cd9ae63): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:88:1` `attachNotificationReplySubscription` (sha256:8a8acabe250c8dcf55edd59ff09aadc42fa3f1d755704e0ce9770043246fe2a3): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:117:1` `cancelScheduledNotification` (sha256:5d04c987ea0773647b5743796a025e3af6d197581f47568f6493cb51051a9625): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:142:1` `closeNotification` (sha256:dd6cc70df16c804640562c157075cf8087a8dc33dcc29abd910cc269d9b4b97c): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:241:1` `disposeNotificationActionSubscription` (sha256:c53638d28af6b747c2d9ec0ff39b8b18a0ef7538c4d7090294f7b3801b3ddb12): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:247:1` `disposeNotificationClickSubscription` (sha256:9c1ba368c798e786daf28a3887a2dce3811f422d7ec07da86689118b5a040c38): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:253:1` `disposeNotificationDismissSubscription` (sha256:e524271acda7d900ad96065155388e8c013e5d155561ea30c503ea0cef3f13cc): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:259:1` `disposeNotificationReceivedSubscription` (sha256:c94a25e6137e039d966eb05a6f3b2c9680ea762485f9f830d8f82a94d6d529bf): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:265:1` `disposeNotificationReplySubscription` (sha256:ee2380186aa1668a991dd373ce23d74083bb5e95c19fe3c020118ee121fa7eaf): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:362:1` `attachNotificationSubscription` (sha256:35b753a60bee5da14d884fca08076dd9b576bd4b3a737f2a05e2a60caa6676a8): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:423:1` `detachNotificationSubscription` (sha256:2176607c00b685e4a1d2432f5d68853e8dedc17c8d6e640bab7f2576b21201e0): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:433:1` `disposeNotificationSubscription` (sha256:1a533b52269f4566aaabd8eafbfbf679e9e86142fe00ed11334e8eaf50e30e69): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/notification` `upstream/packages/notification/src/notification.ts:463:1` `releaseNotificationAttachment` (sha256:7f76cffd99accc4917fa113b88a48436c5c41189f53854319af043c88322686b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:108:1` `queryPermissionState` (sha256:4356fd3c9a6367e21efb543eacbabd6547b4e4d2a55062f636a3ab09c1de9cb5): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:123:1` `queryMidiPermission` (sha256:345a5db1a95f39f93b02cd794851b947dbb2e47044b54dc32f72976cccef08b7): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:134:1` `queryStoragePersistencePermission` (sha256:4b2e25003aaa254a255a15afb97a04e9afc671c6e3a98728ddab8dd1f221b15c): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:145:1` `queryNotificationPermission` (sha256:9d4f9399f2729f3b72927ee8bdfdcdb9486b1d2081c333bb13fd140e29faf807): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:161:1` `requestNotificationPermission` (sha256:9d1a87d9d18597a67596a713ca78da1683f3a9798152a550196556295cbea234): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:182:1` `requestStoragePersistencePermission` (sha256:9d9a503e3f660197ffcffaaa2f051320799533ece572865e329a75003c52224a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:215:1` `requestHostMediaAccess` (sha256:80741265b8ab9c8a896e20874943836755fc5102a6efc602a0b9b5c9736f7046): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:226:1` `requestHostWakeLock` (sha256:d62e90d4e68baae99f933c768082dd0f236a5f6a2ee62ad5f08b084cfd35f1f8): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/permissions` `upstream/packages/permissions/src/permission.ts:240:1` `requestGeolocationAccessPermission` (sha256:dd921bc8af05b5d0a86b9f882a5a72e13c81bbdaa5e02222a9fb2d08bbe4f6ff): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuHost.ts:32:14` `initializeTestWgpuHost.acquire` (sha256:2a49b63d4ebdd5977978166004add73c3d3ccae8d3eb5d7f67e9bb3d87915597): Portable task Rust lowering is not implemented.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuRenderState.ts:66:1` `createWgpuAcquisition` (sha256:f1e27be72bbc2ac0726466122b6cdc162ababf4ab333dd31eee1c52cd3f6759b): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuScreenCapture.ts:15:1` `createBitmapFromWgpuScreenRenderTarget` (sha256:7e2096d69ea1036c01030577a575dca5b51b3a34de818d0dddc852d0b2ef7839): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuScreenCapture.ts:132:1` `mapWgpuCaptureBuffer` (sha256:1da4506af014127e275821281050ed573f0ae137559ba42e86832eae9bbfd76a): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/render-wgpu` `upstream/packages/render-wgpu/src/wgpuTestHelper.ts:310:1` `createWgpuRenderStateForTest` (sha256:9d0e3eb6a99dacf88dc020fc1b677dbb115198e067c50954c2d61f4783c52404): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:18:1` `loadScene2DAudioResources` (sha256:ef068d6bf25188d6c4b8e19131d569035ef9986a4252f22281d13d8e6f4460ec): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:31:18` `loadScene2DAudioResources.resources.anonymous:014c4c09995b` (sha256:014c4c09995ba8c55e50daa2d96114d02e21b4bcd39ef0cb9a9d72f6024a8627): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:19:1` `loadScene2DImageResources` (sha256:3b04df14c63a67c991e14f483f948a0f02fd08797c0b5d72fa8e09ddca99c2ad): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:32:18` `loadScene2DImageResources.sources.anonymous:ed3b14b6c7cc` (sha256:ed3b14b6c7cc1b2df2a78b7b4f07f179828805d27c9409ef986c7ce67b0d7eb0): Portable task Rust lowering is not implemented.
- `@flighthq/scene2d-resources` `upstream/packages/scene2d-resources/src/scene2DDocumentSource.ts:11:1` `loadScene2DDocumentFromUrl` (sha256:49dce597317731f52aab32f841b209b1d9d6423cf002c393d4a6c0bbef3338e2): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:17:1` `loadScene3DDocumentFromGlbUrlWithCoreFeatureHandlers` (sha256:2deb03f338abf9120b56d32ec38b535b10f8f9362a9f8d93c5fe5ef54dab13b7): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:31:1` `loadScene3DDocumentFromGltfUrlWithCoreFeatureHandlers` (sha256:e7b0f927a411ceb02aaf07ccda2527c4fcbce9c6b8c9e14f9194002c8d2ffe79): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/gltfLoad.ts:58:1` `loadGltfExternalBuffers` (sha256:29f43ed3dba8af8677c22997d34a8b8bad47e29d05478fd2282a6b337ddec8a5): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/imageResourceFetch.ts:5:10` `createWebImageResourceFetch.anonymous:a9c2a94a69cb` (sha256:a9c2a94a69cbd18729a9a36e737224372bfb14b95051916d4dbaf146458fc47b): Portable task Rust lowering is not implemented.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:16:1` `loadScene3DResources` (sha256:d05ea967b4337d34fcf6817d0084d4bfc0097f94e66ba818e0cccea9850440c2): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/loadScene3DResources.ts:50:1` `waitForScene3DResourceResolver` (sha256:630808eb313bd9009acacca48b178848df40388f8a0539152c1a0f73a0fa622e): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/sceneDocumentSource.ts:16:1` `loadScene3DDocumentBytesFromUrl` (sha256:9ba99bd9eb9329ed494489200ee4f19d343610bfcdd5f3cfc4d014f31c9ab9a6): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/scene3d-resources` `upstream/packages/scene3d-resources/src/sceneDocumentSource.ts:27:1` `loadScene3DDocumentTextFromUrl` (sha256:d5e02b1b2e55ab21340c1ea52e41dae5b80ea458c58e29ac6f77df6b06e27df0): Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution. Matched the legacy body-erasure path.
- `@flighthq/shortcut` `upstream/packages/shortcut/src/shortcutExplicitDependency.ts:44:1` `attachGlobalShortcut` (sha256:6e48a46ace84161493bee03227852f88afa4ca06d04ed5a482ada5681938e815): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/shortcut` `upstream/packages/shortcut/src/shortcutExplicitDependency.ts:103:1` `detachGlobalShortcut` (sha256:665c451d1098500106fed076d932707a64b14a182e5597c10a5bd828e76e9825): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/shortcut` `upstream/packages/shortcut/src/shortcutExplicitDependency.ts:126:1` `disposeGlobalShortcut` (sha256:2aa5541bd92b8562efa0a2118fa573e3199c5f9d4083cf615e5dd8e8149825f6): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/shortcut` `upstream/packages/shortcut/src/shortcutExplicitDependency.ts:142:1` `queryGlobalShortcutConflict` (sha256:ac2870c15f6f732bbb726ab7082ea9bd56dc3436ddcc646975f39cbcf8881757): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/shortcut` `upstream/packages/shortcut/src/shortcutExplicitDependency.ts:149:1` `queryGlobalShortcutRegistration` (sha256:52f55c55516304e13ebbee1f5e0e70ecb5607788270bdc011762f0cb21b646d0): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:16:1` `loadTextureAtlasFromBase64` (sha256:b79c097cd13f07be0ca0f69e0e76c9f30a138c974531c354ad9f413fd691c2df): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:25:1` `loadTextureAtlasFromBlob` (sha256:7739ccf8b4aa714a5da53bafd133148035dd43473405024fc9cf223890c38eb7): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:33:1` `loadTextureAtlasFromBytes` (sha256:2b354bb32a6f7df16288e45a846428f1dd37c66959a7a7d70c4d60616a66754e): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/textureatlas` `upstream/packages/textureatlas/src/textureAtlasFrom.ts:42:1` `loadTextureAtlasFromUrl` (sha256:b6efeac87e8d6fbafabdcdb8e6b4646d0685e1846cd3464cc8fa9e08b3922bfb): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:41:1` `runManifestTool` (sha256:98791a63db250404b5b35e6ba583cb36345a2e18891a426387cb67afb74b32c5): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:97:1` `readSet` (sha256:728e4746a677db2a2a1c7c3c3e106fe897d4d6b527138d5737676afea3d9f461): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:103:1` `runDiff` (sha256:3942d404c241af65a0560df264ba5dbb36424116566f5f158a5daf903e34f8ae): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:121:1` `runMerge` (sha256:d42bea15425b9d4fb19b700bf65b2ec7701fa5a2961593f851ea856fac051dee): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:136:1` `runPlan` (sha256:2f8f2063693db0e9cd11edf677d7a10dda436b2d5f628c0c9feca91020b22fdb): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tool-manifest` `upstream/packages/tool-manifest/src/manifestTool.ts:176:1` `runScan` (sha256:bf434b366b3df4fa5f89f623314ef39f95b5171f471f984b826b5f853257ea37): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tool-pipeline` `upstream/packages/tool-pipeline/src/pipelineBuild.ts:37:1` `buildToolPipeline` (sha256:22f9541a877470380b7f36d06a74eadbd2eb3fe4f7d9ad989ae6ad4dfe2bdec0): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tool-pipeline` `upstream/packages/tool-pipeline/src/pipelineBuild.ts:98:1` `pathExists` (sha256:169284220a0f41c2b29faaa71f28e8599750db473261568a6468556dcb442175): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tool-pipeline` `upstream/packages/tool-pipeline/src/pipelineBuild.ts:108:1` `publishToolPipelineOutput` (sha256:703aa4f36977f70f39a747841f0f5fe8575e15bcfafcc750c606cf8194cacdec): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tool-pipeline` `upstream/packages/tool-pipeline/src/pipelineTool.ts:10:1` `runToolPipeline` (sha256:a9bf6cb14c196c84fecbc88b0b7c860f3d6301b2857090ef7d9693808e7b89d0): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:70:1` `createTrayIcon` (sha256:8e5a11c3a325f966acb1562fb9afe780a21355a8e1a2c51b42c609c172f0e979): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:128:1` `destroyTrayRuntime` (sha256:bf9ab5ba1cb7e8518118f26c0db17ae5e3056424e0bf4c6eef9367c72602a9f2): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:312:7` `attachTrayEvent.release` (sha256:2497ec1f7b535094570b205084c42e53315d7dd775983204df8ad6dfc1b473ee): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:351:1` `startTrayIconAnimation` (sha256:7f231c223ebf4f8b947d00496a5c0ba85e2c2ce4f6d0e94cedea841332c9e842): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:373:5` `startTrayIconAnimation.release` (sha256:16cd72440ffea00be0e2a01285fdf3e819c430130fad6977ac7e748364ad6ed4): Portable task Rust lowering is not implemented.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:383:1` `queueAnimationWrite` (sha256:6a56458894f87d39deb02bc52fe80c911a6354066d709f6e8d21de7caf1125e9): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:391:64` `queueAnimationWrite.anonymous:635632bebbd9` (sha256:635632bebbd9fcf299f8f91a9c13ab2672e8ebc0a35df4291e3dbcfc21a83e6d): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.
- `@flighthq/tray` `upstream/packages/tray/src/tray.ts:423:1` `invokeUpdate` (sha256:1b318cf7e8e896c137e99f815636839768d29891a5e871ca4af8c7cb0b340926): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/updater` `upstream/packages/updater/src/updater.ts:14:1` `checkForAppUpdate` (sha256:edc555a8b15bb82cefc36b9d338ae9bb2714c03df6663dfb6936404805eb8483): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/updater` `upstream/packages/updater/src/updater.ts:45:1` `installDownloadedUpdate` (sha256:dfd0fe22e04ff0da48d9054725e9d3be041eafeecd3a1224166da93ac76b2554): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/video` `upstream/packages/video/src/videoResourceFrom.ts:11:1` `loadVideoResourceFromBlob` (sha256:4444c971bb966d4df923ae0ac42a9e9c8dd9363cdf9407e4c1a1c30921027fdb): Portable task Rust lowering is not implemented. Matched the legacy body-erasure path.
- `@flighthq/vite-plugin-manifest` `upstream/packages/vite-plugin-manifest/src/manifestPlugin.ts:82:3` `createManifestPlugin.build` (sha256:b89706c7e984370955dc017fbcbf5dc92276b16206303c58d3994ac0d2cc6afa): Portable task Rust lowering is not implemented.
- `@flighthq/vite-plugin-manifest` `upstream/packages/vite-plugin-manifest/src/manifestPlugin.ts:159:11` `createManifestPlugin.load` (sha256:c6251ff4d54c06b778e5de94dfd6843c492459f6b6f2cdc390805d4d92ca78b7): Async output type is not recovered; portable tasks may not erase their output to OpaqueHostValue.

## Generated upstream conformance

| Package | Files translated/passing/in scope | Cases translated/passing | Unsupported files |
| --- | ---: | ---: | ---: |
| `@flighthq/math` | 2/2/15 | 39/39 | 13 |
| `@flighthq/color` | 1/1/9 | 6/6 | 8 |

### Unsupported in-scope upstream test files

- `upstream/packages/math/src/angle.test.ts` (0/21 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/constants.test.ts` (8/10 cases): constants > CIRCLE_KAPPA > equals four thirds of the square root of two less one: only direct function calls are supported; constants > CIRCLE_KAPPA > sweeps a unit quarter circle when used as the cubic control distance: unsupported test expression: ArrowFunction
- `upstream/packages/math/src/hash.test.ts` (0/17 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/interpolation.test.ts` (0/24 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/interpolationAdvanced.test.ts` (0/29 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/nextPowerOfTwo.test.ts` (0/18 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/numberTheory.test.ts` (0/23 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/random.test.ts` (0/5 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/randomDistributions.test.ts` (0/60 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/randomRange.test.ts` (0/18 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/rounding.test.ts` (0/22 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/scalar.test.ts` (0/13 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/math/src/statistics.test.ts` (0/34 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/colorFromKelvin.test.ts` (0/6 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/hslColor.test.ts` (0/12 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/hsvColor.test.ts` (0/8 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/lerpColor.test.ts` (0/9 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/luminance.test.ts` (0/9 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/oklab.test.ts` (0/6 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/packColor.test.ts` (0/24 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.
- `upstream/packages/color/src/premultiplyColorAlpha.test.ts` (0/7 cases): Outside the first pure scalar-expression harvest; translator support must be added before this upstream test file is admitted.

## Blockers

### `@flighthq/abc`

- **emission** `upstream/packages/abc/src/abcFile.ts`: readAbcFile: new-expression Rust lowering is not implemented: abc_reader

### `@flighthq/app`

- **package** `upstream/packages/app/src`: Generated crate is missing 1 of 127 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/app/src/app.ts`: detachAppEvents: optional call requires an inferred nullable function: {"kind":"identifier","name":"unsubscribe"}
- **emission** `upstream/packages/app/src/appRenderView.ts`: AppRenderViewRuntime: aggregate native entity runtime closure is unavailable: imported EntityRuntime aggregate cannot acquire package-local storage fields: AppRenderViewRuntime.attached, AppRenderViewRuntime.resize, AppRenderViewRuntime.synchronize

### `@flighthq/assets`

- **package** `upstream/packages/assets/src`: Generated crate is missing 1 of 19 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/assets/src/assetLibrary.ts`: acquireAsset: taskThen Rust lowering is reserved for Pass 27 Stage 4

### `@flighthq/audio`

- **package** `upstream/packages/audio/src`: Generated crate is missing 10 of 35 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/audio/src/audioResourceFrom.ts`: loadAudioResourceFromBlob: upstream/packages/audio/src/audioResourceFrom.ts:52:23: await value type is not recovered
- **emission** `upstream/packages/audio/src/audioResourceReference.ts`: resolveAudioResourceReference: upstream/packages/audio/src/audioResourceReference.ts:150:3: portable task catch bindings are not implemented
- **emission** `upstream/packages/audio/src/decodeAudioResourceBytes.ts`: decodeAudioResourceBytes: upstream/packages/audio/src/decodeAudioResourceBytes.ts:30:24: await value type is not recovered

### `@flighthq/binpack`

- **emission** `upstream/packages/binpack/src/packRectangles.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/bitmapfont`

- **package** `upstream/packages/bitmapfont/src`: Generated crate is missing 1 of 18 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/bitmapfont/src/bitmapFontFromGlyphAtlas.ts`: createBitmapFontFromGlyphAtlas: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"TextureCommon"}],"fields":[{"discriminantValue":"2d-array","name":"dimension","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"sources","optional":false,"type":{"element":{"inner":{"arguments":[],"kind":"named","name":"TextureSource"},"kind":"nullable"},"kind":"array"}}],"kind":"anonymous"}
- **emission** `upstream/packages/bitmapfont/src/bitmapFontGlyphSource.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/bitmapfont-formats`

- **package** `upstream/packages/bitmapfont-formats/src`: Generated crate is missing 2 of 22 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/bitmapfont-formats/src/bitmapFontBinary.ts`: parseBitmapFontBinaryRecord: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/bitmapfont-formats/src/explainBitmapFontParse.ts`: probeJson: typeof operand has no inferred Rust type: {"kind":"property","name":"id","object":{"kind":"identifier","name":"raw"},"optional":false}

### `@flighthq/bitmaptext`

- **package** `upstream/packages/bitmaptext/src`: Generated crate is missing 4 of 26 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/bitmaptext/src/bitmapText.ts`: createBitmapText: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}
- **lowering** `upstream/packages/bitmaptext/src/updateBitmapText.ts`: TypeScript lowering produced diagnostics.

### `@flighthq/camera-controls`

- **emission** `upstream/packages/camera-controls/src/framing.ts`: getPerspectiveProjectionFrameDistanceToSphere: Math.atan Rust lowering is not implemented

### `@flighthq/capture`

- **package** `upstream/packages/capture/src`: Generated crate is missing 7 of 12 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/capture/src/captureBaseline.ts`: formatCaptureBaseline: JSON.stringify replacer and spacing arguments are not implemented

### `@flighthq/collision`

- **package** `upstream/packages/collision/src`: Generated crate is missing 6 of 141 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/collision/src/collideContactManifold2D.ts`: collideContactManifold2D: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"CollisionCircle2D"}],"fields":[{"discriminantValue":"circle","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/collision/src/enableCollisionGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/collision/src/raycastCollisionShape2D.ts`: capsuleRectangleProbe: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"CollisionAabb2D"}],"fields":[{"discriminantValue":"aabb","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/collision/src/raycastCollisionShape3D.ts`: raycastCollisionShape3D: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"CollisionSphere3D"}],"fields":[{"discriminantValue":"sphere","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/collision/src/shapeCollision2D.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/collision/src/sweepCollisionShape2D.ts`: writeShapeVertices: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"CollisionCircle2D"}],"fields":[{"discriminantValue":"circle","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/collision/src/triangleMesh3D.ts`: scratchTriangle: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"CollisionAabb3D"}],"fields":[{"discriminantValue":"aabb","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}

### `@flighthq/command`

- **emission** `upstream/packages/command/src/commandBinding.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/compression`

- **package** `upstream/packages/compression/src`: Generated crate is missing 7 of 14 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/compression/src/compress.ts`: DeflateBitWriter: upstream/packages/compression/src/compress.ts:53: class methods and static fields are not implemented for DeflateBitWriter
- **emission** `upstream/packages/compression/src/deflate.ts`: decompressDeflate: upstream/packages/compression/src/deflate.ts: cannot infer return type for decompressDeflate
- **emission** `upstream/packages/compression/src/lzma.ts`: decompressLzma: upstream/packages/compression/src/lzma.ts: cannot infer return type for decompressLzma

### `@flighthq/device`

- **emission** `upstream/packages/device/src/device.ts`: createDeviceInfo: portable value conversion requires a statically recoverable source type

### `@flighthq/dialog`

- **emission** `upstream/packages/dialog/src/dialog.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/dialog/src/fileDialog.ts`: initializeFileDialogHandle: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state

### `@flighthq/effects`

- **package** `upstream/packages/effects/src`: Generated crate is missing 1 of 213 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects/src/effectInterpolation.ts`: lerpEffect: optional element access requires an inferred nullable collection

### `@flighthq/effects-canvas`

- **package** `upstream/packages/effects-canvas/src`: Generated crate is missing 24 of 88 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-canvas/src/canvasBevelEffect.ts`: canvasBevelEffectRunner: upstream/packages/effects-canvas/src/canvasBevelEffect.ts: cannot infer return type for canvasBevelEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasBlendEffect.ts`: canvasBlendEffectRunner: upstream/packages/effects-canvas/src/canvasBlendEffect.ts: cannot infer return type for canvasBlendEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasBloomEffect.ts`: canvasBloomEffectRunner: upstream/packages/effects-canvas/src/canvasBloomEffect.ts: cannot infer return type for canvasBloomEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasBlurEffect.ts`: canvasBlurEffectRunner: upstream/packages/effects-canvas/src/canvasBlurEffect.ts: cannot infer return type for canvasBlurEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasCompositeEffect.ts`: canvasCompositeEffectRunner: upstream/packages/effects-canvas/src/canvasCompositeEffect.ts: cannot infer return type for canvasCompositeEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasDropShadowEffect.ts`: canvasDropShadowEffectRunner: upstream/packages/effects-canvas/src/canvasDropShadowEffect.ts: cannot infer return type for canvasDropShadowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasEffectState.ts`: endCanvasEffectPass: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/effects-canvas/src/canvasFilmGrainEffect.ts`: canvasFilmGrainEffectRunner: upstream/packages/effects-canvas/src/canvasFilmGrainEffect.ts: cannot infer return type for canvasFilmGrainEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasGradientBevelEffect.ts`: canvasGradientBevelEffectRunner: upstream/packages/effects-canvas/src/canvasGradientBevelEffect.ts: cannot infer return type for canvasGradientBevelEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasGradientGlowEffect.ts`: canvasGradientGlowEffectRunner: upstream/packages/effects-canvas/src/canvasGradientGlowEffect.ts: cannot infer return type for canvasGradientGlowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasInnerGlowEffect.ts`: canvasInnerGlowEffectRunner: upstream/packages/effects-canvas/src/canvasInnerGlowEffect.ts: cannot infer return type for canvasInnerGlowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasInnerShadowEffect.ts`: canvasInnerShadowEffectRunner: upstream/packages/effects-canvas/src/canvasInnerShadowEffect.ts: cannot infer return type for canvasInnerShadowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasLensDistortionEffect.ts`: canvasLensDistortionEffectRunner: upstream/packages/effects-canvas/src/canvasLensDistortionEffect.ts: cannot infer return type for canvasLensDistortionEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasOuterGlowEffect.ts`: canvasOuterGlowEffectRunner: upstream/packages/effects-canvas/src/canvasOuterGlowEffect.ts: cannot infer return type for canvasOuterGlowEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasPixelateEffect.ts`: canvasPixelateEffectRunner: upstream/packages/effects-canvas/src/canvasPixelateEffect.ts: cannot infer return type for canvasPixelateEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasPosterizeEffect.ts`: canvasPosterizeEffectRunner: upstream/packages/effects-canvas/src/canvasPosterizeEffect.ts: cannot infer return type for canvasPosterizeEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasScanlinesEffect.ts`: canvasScanlinesEffectRunner: upstream/packages/effects-canvas/src/canvasScanlinesEffect.ts: cannot infer return type for canvasScanlinesEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasTiltShiftEffect.ts`: canvasTiltShiftEffectRunner: upstream/packages/effects-canvas/src/canvasTiltShiftEffect.ts: cannot infer return type for canvasTiltShiftEffectRunner
- **emission** `upstream/packages/effects-canvas/src/canvasVignetteEffect.ts`: canvasVignetteEffectRunner: upstream/packages/effects-canvas/src/canvasVignetteEffect.ts: cannot infer return type for canvasVignetteEffectRunner

### `@flighthq/effects-gl`

- **package** `upstream/packages/effects-gl/src`: Generated crate is missing 58 of 187 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-gl/src/glBevelEffect.ts`: glBevelEffectRunner: upstream/packages/effects-gl/src/glBevelEffect.ts: cannot infer return type for glBevelEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBitmapDisplacementEffect.ts`: glBitmapDisplacementEffectRunner: upstream/packages/effects-gl/src/glBitmapDisplacementEffect.ts: cannot infer return type for glBitmapDisplacementEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBlendEffect.ts`: glBlendEffectRunner: upstream/packages/effects-gl/src/glBlendEffect.ts: cannot infer return type for glBlendEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBloomEffect.ts`: glBloomEffectRunner: upstream/packages/effects-gl/src/glBloomEffect.ts: cannot infer return type for glBloomEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBlurEffect.ts`: glBlurEffectRunner: upstream/packages/effects-gl/src/glBlurEffect.ts: cannot infer return type for glBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glBokehDepthOfFieldEffect.ts`: glBokehDepthOfFieldEffectRunner: upstream/packages/effects-gl/src/glBokehDepthOfFieldEffect.ts: cannot infer return type for glBokehDepthOfFieldEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCameraMotionBlurEffect.ts`: glCameraMotionBlurEffectRunner: upstream/packages/effects-gl/src/glCameraMotionBlurEffect.ts: cannot infer return type for glCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glChromaticAberrationEffect.ts`: glChromaticAberrationEffectRunner: upstream/packages/effects-gl/src/glChromaticAberrationEffect.ts: cannot infer return type for glChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCompositeEffect.ts`: glCompositeEffectRunner: upstream/packages/effects-gl/src/glCompositeEffect.ts: cannot infer return type for glCompositeEffectRunner
- **emission** `upstream/packages/effects-gl/src/glContactShadowsEffect.ts`: glContactShadowsEffectRunner: upstream/packages/effects-gl/src/glContactShadowsEffect.ts: cannot infer return type for glContactShadowsEffectRunner
- **emission** `upstream/packages/effects-gl/src/glConvolutionEffect.ts`: glConvolutionEffectRunner: upstream/packages/effects-gl/src/glConvolutionEffect.ts: cannot infer return type for glConvolutionEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCrtEffect.ts`: glCrtEffectRunner: upstream/packages/effects-gl/src/glCrtEffect.ts: cannot infer return type for glCrtEffectRunner
- **emission** `upstream/packages/effects-gl/src/glCustomShaderEffect.ts`: glCustomShaderEffectRunner: upstream/packages/effects-gl/src/glCustomShaderEffect.ts: cannot infer return type for glCustomShaderEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDirectionalBlurEffect.ts`: glDirectionalBlurEffectRunner: upstream/packages/effects-gl/src/glDirectionalBlurEffect.ts: cannot infer return type for glDirectionalBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDisplacementEffect.ts`: glDisplacementEffectRunner: upstream/packages/effects-gl/src/glDisplacementEffect.ts: cannot infer return type for glDisplacementEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDitherEffect.ts`: glDitherEffectRunner: upstream/packages/effects-gl/src/glDitherEffect.ts: cannot infer return type for glDitherEffectRunner
- **emission** `upstream/packages/effects-gl/src/glDropShadowEffect.ts`: glDropShadowEffectRunner: upstream/packages/effects-gl/src/glDropShadowEffect.ts: cannot infer return type for glDropShadowEffectRunner
- **emission** `upstream/packages/effects-gl/src/glEffectState.ts`: endGlEffectPass: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/effects-gl/src/glFilmGrainEffect.ts`: glFilmGrainEffectRunner: upstream/packages/effects-gl/src/glFilmGrainEffect.ts: cannot infer return type for glFilmGrainEffectRunner
- **emission** `upstream/packages/effects-gl/src/glFxaaEffect.ts`: glFxaaEffectRunner: upstream/packages/effects-gl/src/glFxaaEffect.ts: cannot infer return type for glFxaaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGlitchEffect.ts`: glGlitchEffectRunner: upstream/packages/effects-gl/src/glGlitchEffect.ts: cannot infer return type for glGlitchEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGodRaysEffect.ts`: glGodRaysEffectRunner: upstream/packages/effects-gl/src/glGodRaysEffect.ts: cannot infer return type for glGodRaysEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGradientBevelEffect.ts`: glGradientBevelEffectRunner: upstream/packages/effects-gl/src/glGradientBevelEffect.ts: cannot infer return type for glGradientBevelEffectRunner
- **emission** `upstream/packages/effects-gl/src/glGradientGlowEffect.ts`: glGradientGlowEffectRunner: upstream/packages/effects-gl/src/glGradientGlowEffect.ts: cannot infer return type for glGradientGlowEffectRunner
- **emission** `upstream/packages/effects-gl/src/glHalftoneEffect.ts`: glHalftoneEffectRunner: upstream/packages/effects-gl/src/glHalftoneEffect.ts: cannot infer return type for glHalftoneEffectRunner
- **emission** `upstream/packages/effects-gl/src/glInnerGlowEffect.ts`: glInnerGlowEffectRunner: upstream/packages/effects-gl/src/glInnerGlowEffect.ts: cannot infer return type for glInnerGlowEffectRunner
- **emission** `upstream/packages/effects-gl/src/glInnerShadowEffect.ts`: glInnerShadowEffectRunner: upstream/packages/effects-gl/src/glInnerShadowEffect.ts: cannot infer return type for glInnerShadowEffectRunner
- **emission** `upstream/packages/effects-gl/src/glKuwaharaEffect.ts`: glKuwaharaEffectRunner: upstream/packages/effects-gl/src/glKuwaharaEffect.ts: cannot infer return type for glKuwaharaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensDirtEffect.ts`: glLensDirtEffectRunner: upstream/packages/effects-gl/src/glLensDirtEffect.ts: cannot infer return type for glLensDirtEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensDistortionEffect.ts`: glLensDistortionEffectRunner: upstream/packages/effects-gl/src/glLensDistortionEffect.ts: cannot infer return type for glLensDistortionEffectRunner
- **emission** `upstream/packages/effects-gl/src/glLensFlareEffect.ts`: glLensFlareEffectRunner: upstream/packages/effects-gl/src/glLensFlareEffect.ts: cannot infer return type for glLensFlareEffectRunner
- **emission** `upstream/packages/effects-gl/src/glMedianEffect.ts`: glMedianEffectRunner: upstream/packages/effects-gl/src/glMedianEffect.ts: cannot infer return type for glMedianEffectRunner
- **emission** `upstream/packages/effects-gl/src/glMotionBlurEffect.ts`: glMotionBlurEffectRunner: upstream/packages/effects-gl/src/glMotionBlurEffect.ts: cannot infer return type for glMotionBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glOuterGlowEffect.ts`: glOuterGlowEffectRunner: upstream/packages/effects-gl/src/glOuterGlowEffect.ts: cannot infer return type for glOuterGlowEffectRunner
- **emission** `upstream/packages/effects-gl/src/glOutlineEffect.ts`: glOutlineEffectRunner: upstream/packages/effects-gl/src/glOutlineEffect.ts: cannot infer return type for glOutlineEffectRunner
- **emission** `upstream/packages/effects-gl/src/glPixelateEffect.ts`: glPixelateEffectRunner: upstream/packages/effects-gl/src/glPixelateEffect.ts: cannot infer return type for glPixelateEffectRunner
- **emission** `upstream/packages/effects-gl/src/glPosterizeEffect.ts`: glPosterizeEffectRunner: upstream/packages/effects-gl/src/glPosterizeEffect.ts: cannot infer return type for glPosterizeEffectRunner
- **emission** `upstream/packages/effects-gl/src/glRadialBlurEffect.ts`: glRadialBlurEffectRunner: upstream/packages/effects-gl/src/glRadialBlurEffect.ts: cannot infer return type for glRadialBlurEffectRunner
- **emission** `upstream/packages/effects-gl/src/glScanlinesEffect.ts`: glScanlinesEffectRunner: upstream/packages/effects-gl/src/glScanlinesEffect.ts: cannot infer return type for glScanlinesEffectRunner
- **emission** `upstream/packages/effects-gl/src/glScreenSpaceFogEffect.ts`: glScreenSpaceFogEffectRunner: upstream/packages/effects-gl/src/glScreenSpaceFogEffect.ts: cannot infer return type for glScreenSpaceFogEffectRunner
- **emission** `upstream/packages/effects-gl/src/glShaderTestHelper.ts`: evaluateGlslScalarExpression: new-expression Rust lowering is not implemented: function
- **emission** `upstream/packages/effects-gl/src/glSharpenEffect.ts`: glSharpenEffectRunner: upstream/packages/effects-gl/src/glSharpenEffect.ts: cannot infer return type for glSharpenEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSketchEffect.ts`: glSketchEffectRunner: upstream/packages/effects-gl/src/glSketchEffect.ts: cannot infer return type for glSketchEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSmaaEffect.ts`: glSmaaEffectRunner: upstream/packages/effects-gl/src/glSmaaEffect.ts: cannot infer return type for glSmaaEffectRunner
- **emission** `upstream/packages/effects-gl/src/glSsaoEffect.ts`: glSsaoEffectRunner: upstream/packages/effects-gl/src/glSsaoEffect.ts: cannot infer return type for glSsaoEffectRunner
- **emission** `upstream/packages/effects-gl/src/glTiltShiftEffect.ts`: glTiltShiftEffectRunner: upstream/packages/effects-gl/src/glTiltShiftEffect.ts: cannot infer return type for glTiltShiftEffectRunner
- **emission** `upstream/packages/effects-gl/src/glToneMapEffect.ts`: glToneMapEffectRunner: upstream/packages/effects-gl/src/glToneMapEffect.ts: cannot infer return type for glToneMapEffectRunner
- **emission** `upstream/packages/effects-gl/src/glVignetteEffect.ts`: glVignetteEffectRunner: upstream/packages/effects-gl/src/glVignetteEffect.ts: cannot infer return type for glVignetteEffectRunner
- **emission** `upstream/packages/effects-gl/src/glWhiteBalanceEffect.ts`: glWhiteBalanceEffectRunner: upstream/packages/effects-gl/src/glWhiteBalanceEffect.ts: cannot infer return type for glWhiteBalanceEffectRunner

### `@flighthq/effects-wgpu`

- **package** `upstream/packages/effects-wgpu/src`: Generated crate is missing 56 of 187 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/effects-wgpu/src/wgpuBevelEffect.ts`: wgpuBevelEffectRunner: upstream/packages/effects-wgpu/src/wgpuBevelEffect.ts: cannot infer return type for wgpuBevelEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuBitmapDisplacementEffect.ts`: wgpuBitmapDisplacementEffectRunner: upstream/packages/effects-wgpu/src/wgpuBitmapDisplacementEffect.ts: cannot infer return type for wgpuBitmapDisplacementEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuBlendEffect.ts`: wgpuBlendEffectRunner: upstream/packages/effects-wgpu/src/wgpuBlendEffect.ts: cannot infer return type for wgpuBlendEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuBloomEffect.ts`: wgpuBloomEffectRunner: upstream/packages/effects-wgpu/src/wgpuBloomEffect.ts: cannot infer return type for wgpuBloomEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuBlurEffect.ts`: wgpuBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuBlurEffect.ts: cannot infer return type for wgpuBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCameraMotionBlurEffect.ts`: wgpuCameraMotionBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuCameraMotionBlurEffect.ts: cannot infer return type for wgpuCameraMotionBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuChromaticAberrationEffect.ts`: wgpuChromaticAberrationEffectRunner: upstream/packages/effects-wgpu/src/wgpuChromaticAberrationEffect.ts: cannot infer return type for wgpuChromaticAberrationEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCompositeEffect.ts`: wgpuCompositeEffectRunner: upstream/packages/effects-wgpu/src/wgpuCompositeEffect.ts: cannot infer return type for wgpuCompositeEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuContactShadowsEffect.ts`: wgpuContactShadowsEffectRunner: upstream/packages/effects-wgpu/src/wgpuContactShadowsEffect.ts: cannot infer return type for wgpuContactShadowsEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuConvolutionEffect.ts`: wgpuConvolutionEffectRunner: upstream/packages/effects-wgpu/src/wgpuConvolutionEffect.ts: cannot infer return type for wgpuConvolutionEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuCrtEffect.ts`: wgpuCrtEffectRunner: upstream/packages/effects-wgpu/src/wgpuCrtEffect.ts: cannot infer return type for wgpuCrtEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDirectionalBlurEffect.ts`: wgpuDirectionalBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuDirectionalBlurEffect.ts: cannot infer return type for wgpuDirectionalBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDisplacementEffect.ts`: wgpuDisplacementEffectRunner: upstream/packages/effects-wgpu/src/wgpuDisplacementEffect.ts: cannot infer return type for wgpuDisplacementEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDitherEffect.ts`: wgpuDitherEffectRunner: upstream/packages/effects-wgpu/src/wgpuDitherEffect.ts: cannot infer return type for wgpuDitherEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuDropShadowEffect.ts`: wgpuDropShadowEffectRunner: upstream/packages/effects-wgpu/src/wgpuDropShadowEffect.ts: cannot infer return type for wgpuDropShadowEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuEffectState.ts`: createWgpuEffectState: portable record conversion requires string map keys
- **emission** `upstream/packages/effects-wgpu/src/wgpuFilmGrainEffect.ts`: wgpuFilmGrainEffectRunner: upstream/packages/effects-wgpu/src/wgpuFilmGrainEffect.ts: cannot infer return type for wgpuFilmGrainEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuFxaaEffect.ts`: wgpuFxaaEffectRunner: upstream/packages/effects-wgpu/src/wgpuFxaaEffect.ts: cannot infer return type for wgpuFxaaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGlitchEffect.ts`: wgpuGlitchEffectRunner: upstream/packages/effects-wgpu/src/wgpuGlitchEffect.ts: cannot infer return type for wgpuGlitchEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGodRaysEffect.ts`: wgpuGodRaysEffectRunner: upstream/packages/effects-wgpu/src/wgpuGodRaysEffect.ts: cannot infer return type for wgpuGodRaysEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGradientBevelEffect.ts`: wgpuGradientBevelEffectRunner: upstream/packages/effects-wgpu/src/wgpuGradientBevelEffect.ts: cannot infer return type for wgpuGradientBevelEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuGradientGlowEffect.ts`: wgpuGradientGlowEffectRunner: upstream/packages/effects-wgpu/src/wgpuGradientGlowEffect.ts: cannot infer return type for wgpuGradientGlowEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuHalftoneEffect.ts`: wgpuHalftoneEffectRunner: upstream/packages/effects-wgpu/src/wgpuHalftoneEffect.ts: cannot infer return type for wgpuHalftoneEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuInnerGlowEffect.ts`: wgpuInnerGlowEffectRunner: upstream/packages/effects-wgpu/src/wgpuInnerGlowEffect.ts: cannot infer return type for wgpuInnerGlowEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuInnerShadowEffect.ts`: wgpuInnerShadowEffectRunner: upstream/packages/effects-wgpu/src/wgpuInnerShadowEffect.ts: cannot infer return type for wgpuInnerShadowEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuKuwaharaEffect.ts`: wgpuKuwaharaEffectRunner: upstream/packages/effects-wgpu/src/wgpuKuwaharaEffect.ts: cannot infer return type for wgpuKuwaharaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensDirtEffect.ts`: wgpuLensDirtEffectRunner: upstream/packages/effects-wgpu/src/wgpuLensDirtEffect.ts: cannot infer return type for wgpuLensDirtEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensDistortionEffect.ts`: wgpuLensDistortionEffectRunner: upstream/packages/effects-wgpu/src/wgpuLensDistortionEffect.ts: cannot infer return type for wgpuLensDistortionEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuLensFlareEffect.ts`: wgpuLensFlareEffectRunner: upstream/packages/effects-wgpu/src/wgpuLensFlareEffect.ts: cannot infer return type for wgpuLensFlareEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuMedianEffect.ts`: wgpuMedianEffectRunner: upstream/packages/effects-wgpu/src/wgpuMedianEffect.ts: cannot infer return type for wgpuMedianEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuMotionBlurEffect.ts`: wgpuMotionBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuMotionBlurEffect.ts: cannot infer return type for wgpuMotionBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuOuterGlowEffect.ts`: wgpuOuterGlowEffectRunner: upstream/packages/effects-wgpu/src/wgpuOuterGlowEffect.ts: cannot infer return type for wgpuOuterGlowEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuOutlineEffect.ts`: wgpuOutlineEffectRunner: upstream/packages/effects-wgpu/src/wgpuOutlineEffect.ts: cannot infer return type for wgpuOutlineEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuPixelateEffect.ts`: wgpuPixelateEffectRunner: upstream/packages/effects-wgpu/src/wgpuPixelateEffect.ts: cannot infer return type for wgpuPixelateEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuPosterizeEffect.ts`: wgpuPosterizeEffectRunner: upstream/packages/effects-wgpu/src/wgpuPosterizeEffect.ts: cannot infer return type for wgpuPosterizeEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuRadialBlurEffect.ts`: wgpuRadialBlurEffectRunner: upstream/packages/effects-wgpu/src/wgpuRadialBlurEffect.ts: cannot infer return type for wgpuRadialBlurEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuScanlinesEffect.ts`: wgpuScanlinesEffectRunner: upstream/packages/effects-wgpu/src/wgpuScanlinesEffect.ts: cannot infer return type for wgpuScanlinesEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuScreenSpaceFogEffect.ts`: wgpuScreenSpaceFogEffectRunner: upstream/packages/effects-wgpu/src/wgpuScreenSpaceFogEffect.ts: cannot infer return type for wgpuScreenSpaceFogEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSharpenEffect.ts`: wgpuSharpenEffectRunner: upstream/packages/effects-wgpu/src/wgpuSharpenEffect.ts: cannot infer return type for wgpuSharpenEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSketchEffect.ts`: wgpuSketchEffectRunner: upstream/packages/effects-wgpu/src/wgpuSketchEffect.ts: cannot infer return type for wgpuSketchEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSmaaEffect.ts`: wgpuSmaaEffectRunner: upstream/packages/effects-wgpu/src/wgpuSmaaEffect.ts: cannot infer return type for wgpuSmaaEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuSsaoEffect.ts`: wgpuSsaoEffectRunner: upstream/packages/effects-wgpu/src/wgpuSsaoEffect.ts: cannot infer return type for wgpuSsaoEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuTiltShiftEffect.ts`: wgpuTiltShiftEffectRunner: upstream/packages/effects-wgpu/src/wgpuTiltShiftEffect.ts: cannot infer return type for wgpuTiltShiftEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuToneMapEffect.ts`: wgpuToneMapEffectRunner: upstream/packages/effects-wgpu/src/wgpuToneMapEffect.ts: cannot infer return type for wgpuToneMapEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuVignetteEffect.ts`: wgpuVignetteEffectRunner: upstream/packages/effects-wgpu/src/wgpuVignetteEffect.ts: cannot infer return type for wgpuVignetteEffectRunner
- **emission** `upstream/packages/effects-wgpu/src/wgpuWhiteBalanceEffect.ts`: wgpuWhiteBalanceEffectRunner: upstream/packages/effects-wgpu/src/wgpuWhiteBalanceEffect.ts: cannot infer return type for wgpuWhiteBalanceEffectRunner

### `@flighthq/entity`

- **package** `upstream/packages/entity/src`: Generated crate is missing 2 of 22 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/entity/src/entity.ts`: allocateEntity: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state

### `@flighthq/filesystem`

- **emission** `upstream/packages/filesystem/src/filesystem.ts`: appendTextFile: taskReject requires a typed null, boolean, number, string, or Error rejection

### `@flighthq/font`

- **package** `upstream/packages/font/src`: Generated crate is missing 5 of 17 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/font/src/_fontFaceLoad.ts`: _loadFontFaceFromBytes: upstream/packages/font/src/_fontFaceLoad.ts:6:1: portableTask _loadFontFaceFromBytes: async output type is not recovered
- **emission** `upstream/packages/font/src/glyphOutlineSource.ts`: allocateGlyphRasterizerBackendFromGlyphOutlineSource: object field rasterize is not initialized and has no Rust default

### `@flighthq/font-formats`

- **package** `upstream/packages/font-formats/src`: Generated crate is missing 55 of 58 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/font-formats/src/cffCharstring.ts`: runCffCharstring: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/cffDict.ts`: readCffDict: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/cffFdSelect.ts`: readCffFdSelect: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/cffIndex.ts`: readCffIndex: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/cffTable.ts`: readCffTable: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/openTypeCmap.ts`: readOpenTypeCodepointMap: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/openTypeGlyf.ts`: readOpenTypeGlyphOutline: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/openTypeMetrics.ts`: readOpenTypeAdvances: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/openTypeTestHelper.ts`: encodeSyntheticCff: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/sfntAssembly.ts`: assembleSfntFont: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/sfntTableDirectory.ts`: readSfntTableDirectory: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/woff2Font.ts`: readWoff2Font: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/woff2GlyfTransform.ts`: readWoff2GlyfStreams: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/font-formats/src/woffFont.ts`: readWoffChecksumMismatches: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/gizmo`

- **emission** `upstream/packages/gizmo/src/gizmoState.ts`: GizmoRuntime: aggregate native entity runtime closure is unavailable: imported EntityRuntime aggregate cannot acquire package-local storage fields: GizmoRuntime.bounds, GizmoRuntime.camera, GizmoRuntime.cleanups, GizmoRuntime.customPivotX, GizmoRuntime.customPivotY, GizmoRuntime.disposed, GizmoRuntime.drag, GizmoRuntime.features, GizmoRuntime.handleRoot, GizmoRuntime.handles, GizmoRuntime.nodeBounds, GizmoRuntime.outline, GizmoRuntime.outlineColor, GizmoRuntime.outlineEnabled, GizmoRuntime.outlinePoints, GizmoRuntime.overlay, GizmoRuntime.overlayRoot, GizmoRuntime.mode, GizmoRuntime.viewportHeight, GizmoRuntime.viewportWidth, GizmoRuntime.pivot, GizmoRuntime.pivotScreen, GizmoRuntime.pivotWorld, GizmoRuntime.snapRotation, GizmoRuntime.snapScale, GizmoRuntime.snapTranslate, GizmoRuntime.space, GizmoRuntime.scratchPoint, GizmoRuntime.selection, GizmoRuntime.signals
- **emission** `upstream/packages/gizmo/src/node2dGizmoFeatures.ts`: createNode2DGizmoFeatures: portable value conversion requires a statically recoverable source type

### `@flighthq/glyphatlas`

- **package** `upstream/packages/glyphatlas/src`: Generated crate is missing 1 of 19 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/glyphatlas/src/glyphRasterizerBackend.ts`: allocateStubGlyphRasterizerBackend: object field rasterize is not initialized and has no Rust default

### `@flighthq/gui`

- **package** `upstream/packages/gui/src`: Generated crate is missing 2 of 106 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/gui/src/buttonController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/comboBoxController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/guiController.ts`: GuiControllerRuntime: aggregate native entity runtime closure is unavailable: imported EntityRuntime aggregate cannot acquire package-local storage fields: GuiControllerRuntime.cleanups, GuiControllerRuntime.disposed, GuiControllerRuntime.hitStates, GuiControllerRuntime.transition
- **emission** `upstream/packages/gui/src/guiDialog.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/listController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/progressBarController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/radioGroupController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/scrollBarController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/scrollViewController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/sliderController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/splitPaneController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/tabBarController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/textInputController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/toggleController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/tooltipController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/gui/src/treeViewController.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (15 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/host`

- **package** `upstream/packages/host/src`: Generated crate is missing 8 of 67 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/host/src/hostExplain.ts`: _COVERAGE: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"capability","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"group","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"slot","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}

### `@flighthq/host-web`

- **package** `upstream/packages/host-web/src`: Generated crate is missing 82 of 237 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/host-web/src/webAccessibility.ts`: createWebAccessibilityBackend: object field announce is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webApp.ts`: initializeWebAppBadgeBackend: upstream/packages/host-web/src/webApp.ts:32:23: portableTask initializeWebAppBadgeBackend.anonymous:7722fac082ee: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webAudio.ts`: webHostAudio: object field canPlayType is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webAudioDecodeHost.ts`: decodeWithWebAudio: upstream/packages/host-web/src/webAudioDecodeHost.ts:9:3: portableTask decodeWithWebAudio.decode: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webAudioDevice.ts`: initializeWebAudioDeviceBackend: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/host-web/src/webAudioMixer.ts`: createWebAudioMixerBackend: object field createMixerGraph is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webBitmapDraw.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webBitmapFrom.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webBitmapReadback.ts`: isExpectedSourceRefusal: instanceof Rust lowering requires a portable typed-array constructor
- **emission** `upstream/packages/host-web/src/webCanvasHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webClipboard.ts`: initializeWebClipboardBackend: upstream/packages/host-web/src/webClipboard.ts:65:3: portableTask initializeWebClipboardFormatsBackend.blobFromFormatData: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webConnectivity.ts`: createWebConnectivityChangeProvider: object field destroy is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webCursor.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webDevice.ts`: createWebDeviceBackend: object field getCapabilities is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webDeviceHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webDialog.ts`: initializeWebMessageDialogBackend: upstream/packages/host-web/src/webDialog.ts:46:17: portableTask initializeWebMessageDialogBackend.anonymous:8f5fa360ad1b: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webFilesystem.ts`: webHostFileSystem: upstream/packages/host-web/src/webFilesystem.ts:6:3: portableTask webHostFileSystem.appendTextFile: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webFontHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webFontLoading.ts`: webHostFontLoading: upstream/packages/host-web/src/webFontLoading.ts:13:14: portableTask webHostFontLoading.whenReady: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webGeolocation.ts`: createWebGeolocationBackend: object field getCurrentPosition is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webGlContext.ts`: getWebGlContext: object literal requires an inferred structural type (target=unknown, properties=alpha,antialias,powerPreference,stencil,spread)
- **emission** `upstream/packages/host-web/src/webGlHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webGlyphRasterizer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webHaptics.ts`: webHostHaptics: object field cancel is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webHostCanvas.ts`: createWebHostCanvas: object field acquire is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webHostGl.ts`: createWebHostGl: object field acquire is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webHostSurface.ts`: webHostSurfaceDisplay: object field setDisplaySize is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webHostWgpuContext.ts`: createWebHostWgpuContext: object field acquire is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webImage.ts`: webHostImage: upstream/packages/host-web/src/webImage.ts:20:21: portableTask webHostImage.loadImageFromUrl: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webImageBitmapComposition.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- **emission** `upstream/packages/host-web/src/webImageDecodeHost.ts`: decodeWithCanvas: upstream/packages/host-web/src/webImageDecodeHost.ts:9:3: portableTask decodeWithCanvas.decode: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webImageEncodeHost.ts`: createCanvasEncoder: upstream/packages/host-web/src/webImageEncodeHost.ts:10:5: portableTask createCanvasEncoder.encode: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webImageHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webImageResource.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webInputIngress.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webInputTarget.ts`: webHostInputDropFile: object field subscribe is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webKeyboard.ts`: webHostSoftKeyboardChange: upstream/packages/host-web/src/webKeyboard.ts:11:14: portableTask webHostSoftKeyboardChange.subscribe: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webLifecycle.ts`: createWebLifecycleBackend: object field getState is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webMediasession.ts`: createWebMediaSessionActionBackend: object field destroy is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webMenu.ts`: webHostMenuHighlight: object field subscribe is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webMidi.ts`: initializeWebMidiAccessBackend: upstream/packages/host-web/src/webMidi.ts:26:23: portableTask initializeWebMidiAccessBackend.anonymous:755b5dc1a7e0: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webNet.ts`: createWebNetBackend: object field sendNetRequest is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webNotification.ts`: createWebPageNotificationCapabilities: upstream/packages/host-web/src/webNotification.ts:28:3: portableTask createWebPageNotificationCapabilities.closeOne: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webPermissions.ts`: createWebPermissionsBackend: object field notification is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webPermissionsHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webPlatform.ts`: createWebPlatformBackend: object field getInfo is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webPlatformHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webPower.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webPowerHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webProtocol.ts`: createWebProtocolCapabilities: object field getLaunchUrl is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webScreen.ts`: createWebScreenCapabilities: object field getScreens is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webSensors.ts`: createWebSensorsBackend: object field getPermissionState is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webSensorsHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webServiceWorkerNotification.ts`: createWebServiceWorkerNotificationCapabilities: upstream/packages/host-web/src/webServiceWorkerNotification.ts:31:3: portableTask createWebServiceWorkerNotificationCapabilities.closeOne: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webShare.ts`: initializeWebShareContentBackend: upstream/packages/host-web/src/webShare.ts:13:22: portableTask initializeWebShareContentBackend.anonymous:446cdd4ed448: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webShell.ts`: initializeWebShellExternalBackend: upstream/packages/host-web/src/webShell.ts:4:14: portableTask initializeWebShellExternalBackend.anonymous:fa929f9b108b: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webStorage.ts`: createWebStorageChangeProvider: object field destroy is not initialized and has no Rust default
- **emission** `upstream/packages/host-web/src/webStoragePersistence.ts`: createPersistenceQueryBackend: upstream/packages/host-web/src/webStoragePersistence.ts:39:5: portableTask createPersistenceQueryBackend.getPersistence: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/host-web/src/webSurfaceHandle.ts`: getWebSurfaceElementHandle: instanceof Rust lowering requires a portable typed-array constructor
- **emission** `upstream/packages/host-web/src/webSurfacePresentation.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webTextShaper.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webTextureAtlas.ts`: createWebTextureAtlasFromCanvas: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"TextureCommon"}],"fields":[{"discriminantValue":"2d-array","name":"dimension","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"sources","optional":false,"type":{"element":{"inner":{"arguments":[],"kind":"named","name":"TextureSource"},"kind":"nullable"},"kind":"array"}}],"kind":"anonymous"}
- **emission** `upstream/packages/host-web/src/webVideoCapability.ts`: webHostVideo: taskReject requires a typed null, boolean, number, string, or Error rejection
- **emission** `upstream/packages/host-web/src/webVideoResource.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webWgpuHost.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (21 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/host-web/src/webWindow.ts`: webHostFullscreen: upstream/packages/host-web/src/webWindow.ts:31:9: portableTask webHostFullscreen.exit: Portable task Rust lowering is not implemented.

### `@flighthq/image`

- **package** `upstream/packages/image/src`: Generated crate is missing 9 of 28 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/image/src/imageResourceFrom.ts`: loadImageResourceFromBlob: loadImageResourceFromBlob: portable task has a non-void output without a guaranteed return
- **emission** `upstream/packages/image/src/imageResourceReference.ts`: resolveImageResourceReference: upstream/packages/image/src/imageResourceReference.ts:177:3: portable task catch bindings are not implemented

### `@flighthq/interaction`

- **package** `upstream/packages/interaction/src`: Generated crate is missing 16 of 92 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/interaction/src/hitTests.ts`: hitAreaContainsPoint: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/interaction/src/nodeInteractiveStateBinding.ts`: InteractiveStateRuntime: aggregate native entity runtime closure is unavailable: imported EntityRuntime aggregate cannot acquire package-local storage fields: InteractiveStateRuntime.base, InteractiveStateRuntime.extensions, InteractiveStateRuntime.flags, InteractiveStateRuntime.node, InteractiveStateRuntime.states, InteractiveStateRuntime.transition

### `@flighthq/intl`

- **package** `upstream/packages/intl/src`: Generated crate is missing 14 of 16 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/intl/src/cache.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/intl/src/collator.ts`: getCollator: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.Collator")
- **emission** `upstream/packages/intl/src/datetime.ts`: formatDateValue: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.DateTimeFormat")
- **emission** `upstream/packages/intl/src/list.ts`: formatList: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.ListFormat")
- **emission** `upstream/packages/intl/src/number.ts`: formatCompactNumber: portable object spread 1 requires a structural source
- **emission** `upstream/packages/intl/src/plural.ts`: selectOrdinalCategory: portable object spread 1 requires a structural source
- **emission** `upstream/packages/intl/src/relativeTime.ts`: formatRelativeTime: new-expression Rust lowering is not implemented: crate::host_value::<crate::OpaqueHostValue>("host.RelativeTimeFormat")

### `@flighthq/layout`

- **emission** `upstream/packages/layout/src/anchorLayout.ts`: anchorLayoutResolver: upstream/packages/layout/src/anchorLayout.ts: cannot infer return type for anchorLayoutResolver
- **emission** `upstream/packages/layout/src/flexLayout.ts`: flexLayoutResolver: upstream/packages/layout/src/flexLayout.ts: cannot infer return type for flexLayoutResolver
- **emission** `upstream/packages/layout/src/gridLayout.ts`: gridLayoutResolver: upstream/packages/layout/src/gridLayout.ts: cannot infer return type for gridLayoutResolver

### `@flighthq/lifecycle`

- **package** `upstream/packages/lifecycle/src`: Generated crate is missing 1 of 13 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/lifecycle/src/lifecycle.ts`: explainLifecycleOperation: typeof operand has no inferred Rust type: {"index":{"kind":"identifier","name":"operation"},"kind":"element","object":{"kind":"identifier","name":"hostLifecycle"},"optional":false}

### `@flighthq/loader`

- **package** `upstream/packages/loader/src`: Generated crate is missing 1 of 18 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/loader/src/load.ts`: loadBytes: instanceof Rust lowering requires a portable typed-array constructor
- **emission** `upstream/packages/loader/src/resourceLoader.ts`: cancelResourceLoad: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/log`

- **emission** `upstream/packages/log/src/log.ts`: _destroyFileLogSinkState: upstream/packages/log/src/log.ts:683:3: portable task catch bindings are not implemented

### `@flighthq/media`

- **package** `upstream/packages/media/src`: Generated crate is missing 15 of 59 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/media/src/videoChannel.ts`: startVideoChannel: taskCatch Rust lowering is reserved for Pass 27 Stage 4

### `@flighthq/mediasession`

- **emission** `upstream/packages/mediasession/src/mediasession.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/menu`

- **emission** `upstream/packages/menu/src/menu-templates.ts`: createDefaultAppMenuTemplate: portable value conversion requires a statically recoverable source type
- **emission** `upstream/packages/menu/src/menu.ts`: destroyAppMenu: try/catch with escaping loop control is not implemented

### `@flighthq/mesh`

- **package** `upstream/packages/mesh/src`: Generated crate is missing 39 of 89 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/mesh/src/meshGeometry.ts`: createMeshGeometryRuntime: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state
- **emission** `upstream/packages/mesh/src/meshGeometryAttributes.ts`: getAttributeDataView: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/mesh/src/meshGeometryBuilders.ts`: faceSphericalU: spread Rust lowering is not implemented
- **emission** `upstream/packages/mesh/src/meshGeometryLayout.ts`: convertMeshGeometryLayout: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/midi`

- **package** `upstream/packages/midi/src`: Generated crate is missing 9 of 37 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/midi/src/midiAccess.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- **emission** `upstream/packages/midi/src/midiPort.ts`: closeMidiPort: upstream/packages/midi/src/midiPort.ts:39:5: await value type is not recovered
- **emission** `upstream/packages/midi/src/midiResource.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/midi/src/midiSubscription.ts`: attachMidiSubscription: attachMidiSubscription: generic portable task lowering is not implemented

### `@flighthq/movieclip`

- **package** `upstream/packages/movieclip/src`: Generated crate is missing 4 of 30 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/movieclip/src/movieClip.ts`: createMovieClip: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}

### `@flighthq/net`

- **package** `upstream/packages/net/src`: Generated crate is missing 1 of 8 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/net/src/net.ts`: sendNetRequest: taskThen Rust lowering is reserved for Pass 27 Stage 4

### `@flighthq/node`

- **package** `upstream/packages/node/src`: Generated crate is missing 16 of 134 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/node/src/boundsRectangle.ts`: ensureNodeWorldBoundsRectangle: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[{"arguments":[],"kind":"named","name":"Traits"}],"kind":"named","name":"HasBoundsRectangleRuntime"},{"arguments":[],"kind":"named","name":"HasTransform2DRuntime"}],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/node/src/node.ts`: initializeNode: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state

### `@flighthq/notification`

- **package** `upstream/packages/notification/src`: Generated crate is missing 4 of 36 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/notification/src/notification.ts`: attachNotificationSubscription: attachNotificationSubscription: generic portable task lowering is not implemented

### `@flighthq/particleemitter`

- **package** `upstream/packages/particleemitter/src`: Generated crate is missing 4 of 53 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/particleemitter/src/particleEmitter.ts`: createParticleEmitter2D: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}

### `@flighthq/particles`

- **emission** `upstream/packages/particles/src/particleEmitterSignals.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles/src/validateParticleEmitterConfig.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/particles-formats`

- **emission** `upstream/packages/particles-formats/src/formatRegistry.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/libgdxParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/libgdxSerialize.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/parseParticleConfig.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/particleDesignerParse.ts`: num: typeof operand has no inferred Rust type: {"kind":"identifier","name":"v"}
- **emission** `upstream/packages/particles-formats/src/pixiParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/spineParse.ts`: readSpineImages: typeof operand has no inferred Rust type: {"kind":"identifier","name":"image"}
- **emission** `upstream/packages/particles-formats/src/spineSerialize.ts`: serializeSpineParticle: JSON.stringify replacer and spacing arguments are not implemented
- **emission** `upstream/packages/particles-formats/src/starlingPexParse.ts`: extractAttr: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/particles-formats/src/unityParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 5); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/particles-formats/src/unitySerialize.ts`: serializeUnityParticle: JSON.stringify replacer and spacing arguments are not implemented

### `@flighthq/path`

- **package** `upstream/packages/path/src`: Generated crate is missing 1 of 83 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/path/src/forEachPathSegment.ts`: forEachPathSegment: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"discriminantValue":"quadraticCurveTo","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"controlX","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"controlY","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"x","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"y","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}

### `@flighthq/path-boolean`

- **package** `upstream/packages/path-boolean/src`: Generated crate is missing 1 of 15 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/path-boolean/src/martinezKernel.ts`: buildArrangement: new-expression Rust lowering is not implemented: event_heap

### `@flighthq/permissions`

- **emission** `upstream/packages/permissions/src/permission.ts`: requestNotificationPermission: requestNotificationPermission: portable task has a non-void output without a guaranteed return

### `@flighthq/physics2d`

- **package** `upstream/packages/physics2d/src`: Generated crate is missing 14 of 140 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/physics2d/src/joints.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics2d/src/step.ts`: testPhysics2DSegmentOverlap: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"CollisionCircle2D"}],"fields":[{"discriminantValue":"circle","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/physics2d/src/stepValidation.ts`: isPhysics2DJointValid: dynamic for-in Rust enumeration is not implemented
- **emission** `upstream/packages/physics2d/src/world.ts`: createPhysics2DCollider: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"categoryBits","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"groupIndex","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"maskBits","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}
- **emission** `upstream/packages/physics2d/src/worldQueries.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/physics2d-abi`

- **package** `upstream/packages/physics2d-abi/src`: Generated crate is missing 21 of 82 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/physics2d-abi/src/physics2DAbiBuffer.ts`: clearPhysics2DAbiCommandBuffer: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/physics2d-abi/src/physics2DAbiCommand.ts`: beginCommand: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/physics2d-abi/src/physics2DAbiQuery.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics2d-abi/src/referencePhysics2DAbi.ts`: executeCommands: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/physics3d`

- **package** `upstream/packages/physics3d/src`: Generated crate is missing 24 of 233 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/physics3d/src/colliderCollision.ts`: collidePhysics3DColliderShapes: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"CollisionSphere3D"}],"fields":[{"discriminantValue":"sphere","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/physics3d/src/colliderTransform.ts`: writePhysics3DColliderBounds: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"CollisionSphere3D"}],"fields":[{"discriminantValue":"sphere","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/physics3d/src/contactIntake.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics3d/src/continuous.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics3d/src/debugGeometry.ts`: writeCollider: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"CollisionSphere3D"}],"fields":[{"discriminantValue":"sphere","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/physics3d/src/physics3DBroadphasePublication.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics3d/src/solver.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics3d/src/step.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics3d/src/stepValidation.ts`: isPhysics3DJointValid: dynamic for-in Rust enumeration is not implemented
- **emission** `upstream/packages/physics3d/src/world.ts`: createPhysics3DWorld: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"SpatialIndexBackend3D"},{"arguments":[],"kind":"named","name":"Entity"}],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/physics3d/src/worldQueries.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (6 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/physics3d-abi`

- **package** `upstream/packages/physics3d-abi/src`: Generated crate is missing 21 of 78 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/physics3d-abi/src/physics3DAbiBuffer.ts`: clearPhysics3DAbiCommandBuffer: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/physics3d-abi/src/physics3DAbiCommand.ts`: writePhysics3DAbiSetJointCommand: optional property localRotationAX has no inferred receiver field
- **emission** `upstream/packages/physics3d-abi/src/physics3DAbiQuery.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/physics3d-abi/src/referencePhysics3DAbi.ts`: executeCommands: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/power`

- **emission** `upstream/packages/power/src/power.ts`: destroyPowerKeepAwake: try/catch with escaping loop control is not implemented

### `@flighthq/preferences`

- **emission** `upstream/packages/preferences/src/storage.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/quadbatch`

- **package** `upstream/packages/quadbatch/src`: Generated crate is missing 5 of 33 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/quadbatch/src/quadBatch.ts`: createQuadBatch: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}

### `@flighthq/registry`

- **emission** `upstream/packages/registry/src/registryTable.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/render`

- **package** `upstream/packages/render/src`: Generated crate is missing 19 of 83 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render/src/explainScene3DPipelineCoverage.ts`: collectUsed3DKinds: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/render/src/renderProxy.ts`: resolveRenderProxyRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/render/src/renderState.ts`: initializeRenderState: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state

### `@flighthq/render-gl`

- **package** `upstream/packages/render-gl/src`: Generated crate is missing 45 of 144 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-gl/src/glDraw.ts`: bindGlTextureSourceTexture: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"texture","optional":false,"type":{"kind":"dynamic"}},{"name":"version","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}
- **emission** `upstream/packages/render-gl/src/glQuadMaterialRegistry.ts`: resolveGlQuadMaterialRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/render-gl/src/glRenderPass.ts`: beginGlRenderPass: optional property at has no inferred receiver field
- **emission** `upstream/packages/render-gl/src/glRenderState.ts`: initializeGlContextState: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"texture","optional":false,"type":{"kind":"dynamic"}},{"name":"version","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}
- **emission** `upstream/packages/render-gl/src/glRenderTarget.ts`: declareGlRenderTargetColorSpace: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/render-gl/src/glTextureResolver.ts`: resolveGlTexture: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}

### `@flighthq/render-wgpu`

- **package** `upstream/packages/render-wgpu/src`: Generated crate is missing 45 of 156 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/render-wgpu/src/wgpuCompressedTexture.ts`: uploadWgpuCompressedImage: portable record conversion requires string map keys
- **emission** `upstream/packages/render-wgpu/src/wgpuExternalImageSource.ts`: isWgpuExternalImageSourceUnavailableError: instanceof Rust lowering requires a portable typed-array constructor
- **emission** `upstream/packages/render-wgpu/src/wgpuHost.ts`: createTestWgpuSurface: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state
- **emission** `upstream/packages/render-wgpu/src/wgpuQuadMaterialRegistry.ts`: resolveWgpuQuadMaterialRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/render-wgpu/src/wgpuRenderState.ts`: createWgpuDeviceState: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state
- **emission** `upstream/packages/render-wgpu/src/wgpuRenderTexture.ts`: getWgpuRenderTextureTarget: portable record conversion requires string map keys
- **emission** `upstream/packages/render-wgpu/src/wgpuScreenCapture.ts`: mapWgpuCaptureBuffer: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/render-wgpu/src/wgpuTestHelper.ts`: validateWriteBufferDestination: instanceof Rust lowering requires a portable typed-array constructor
- **emission** `upstream/packages/render-wgpu/src/wgpuTextureResolver.ts`: resolveWgpuTexture: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}

### `@flighthq/requirement`

- **emission** `upstream/packages/requirement/src/requirementSet.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/requirement-catalog`

- **package** `upstream/packages/requirement-catalog/src`: Generated crate is missing 5 of 9 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/requirement-catalog/src/requirementCatalog.ts`: createRequirementCatalog: portable value conversion requires a statically recoverable source type

### `@flighthq/scene-document`

- **package** `upstream/packages/scene-document/src`: Generated crate is missing 14 of 36 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene-document/src/flightDocumentText.ts`: readCamera: typeof operand has no inferred Rust type: {"kind":"identifier","name":"far"}
- **emission** `upstream/packages/scene-document/src/sceneDocumentInteractiveStateBindings.ts`: elideDefaultFields: delete Rust lowering is not implemented
- **emission** `upstream/packages/scene-document/src/sceneDocumentLayoutBindings.ts`: writeFlightDocumentLayoutBindings: optional element access requires an inferred nullable collection
- **emission** `upstream/packages/scene-document/src/sceneDocumentMaterializationSelection.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene-document/src/sceneDocumentRefusal.ts`: checkInteractiveState: typeof operand has no inferred Rust type: {"kind":"identifier","name":"value"}
- **emission** `upstream/packages/scene-document/src/sceneDocumentScene2DMaterialization.ts`: writeFieldsWithDefaults: delete Rust lowering is not implemented
- **emission** `upstream/packages/scene-document/src/sceneDocumentScene3DMaterialization.ts`: writeFieldsWithDefaults: delete Rust lowering is not implemented
- **emission** `upstream/packages/scene-document/src/sceneDocumentYamlSubset.ts`: parseSceneDocumentYamlSubset: new-expression Rust lowering is not implemented: scene_document_yaml_subset_lexer

### `@flighthq/scene2d`

- **package** `upstream/packages/scene2d/src`: Generated crate is missing 11 of 44 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d/src/displayObject.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d/src/htmlView.ts`: createHtmlView: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/scene2d/src/scale9Sprite.ts`: createScale9Sprite: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/scene2d/src/sprite.ts`: createSprite: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}

### `@flighthq/scene2d-canvas`

- **package** `upstream/packages/scene2d-canvas/src`: Generated crate is missing 25 of 146 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d-canvas/src/canvasBitmapText.ts`: canvasBitmapTextRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"__flight_argument_0"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasBitmapTextureResolver.ts`: resolveCanvasBitmapTexture: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"element","optional":false,"type":{"kind":"dynamic"}},{"name":"version","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene2d-canvas/src/canvasParticleEmitter2D.ts`: canvasParticleEmitter2DRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"__flight_argument_0"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasQuadBatch.ts`: canvasQuadBatchRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"__flight_argument_0"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasRenderState.ts`: createCanvasRenderState: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasRichText.ts`: canvasRichTextRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"__flight_argument_0"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasScale9Shape.ts`: canvasScale9ShapeRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"__flight_argument_0"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasScale9Sprite.ts`: canvasScale9SpriteRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"__flight_argument_0"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasShape.ts`: renderCanvasShapeCommands: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"arguments":[{"kind":"identifier","name":"state"}],"callee":{"kind":"identifier","name":"getRenderStateRuntime"},"kind":"call","optional":false,"typeArguments":[]},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasSprite.ts`: canvasSpriteRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"__flight_argument_0"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasTextLabel.ts`: canvasTextLabelRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"__flight_argument_0"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasTextureResolver.ts`: connectCanvasTextureResolverMisses: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/scene2d-canvas/src/canvasTextureWindowSource.ts`: resolveCanvasTextureWindowSource: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"element","optional":false,"type":{"kind":"dynamic"}},{"name":"surface","optional":false,"type":{"arguments":[],"kind":"named","name":"CanvasSurface"}},{"name":"flipX","optional":false,"type":{"kind":"primitive","name":"Bool"}},{"name":"flipY","optional":false,"type":{"kind":"primitive","name":"Bool"}},{"name":"imageVersion","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"source","optional":false,"type":{"kind":"dynamic"}},{"name":"textureVersion","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"uvOffsetX","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"uvOffsetY","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"uvRotation","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"uvScaleX","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"uvScaleY","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene2d-canvas/src/canvasTilemap.ts`: canvasTilemapRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"applyBlendMode","object":{"kind":"identifier","name":"__flight_argument_0"},"optional":false}

### `@flighthq/scene2d-formats`

- **package** `upstream/packages/scene2d-formats/src`: Generated crate is missing 63 of 215 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d-formats/src/lottieCounts.ts`: tallyMasks: typeof operand has no inferred Rust type: {"kind":"identifier","name":"entry"}
- **emission** `upstream/packages/scene2d-formats/src/lottieDocument.ts`: createLottieLayerNode: portable field name cannot distinguish an omitted property from explicit null
- **emission** `upstream/packages/scene2d-formats/src/lottieGradientPaint.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/lottieGradientShapeItems.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/lottieImageLayer.ts`: isImageAsset: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/scene2d-formats/src/lottiePrecompositionLayer.ts`: isPrecompositionAsset: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/scene2d-formats/src/lottieRegistry.ts`: registerLottieLayerHandler: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"handle","optional":false,"type":{"arguments":[],"kind":"named","name":"LottieLayerHandler"}},{"name":"kind","optional":false,"type":{"arguments":[],"kind":"named","name":"LottieLayerKind"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene2d-formats/src/lottieStrokeShapeItem.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/lottieTestFixtures.ts`: Maximum call stack size exceeded
- **emission** `upstream/packages/scene2d-formats/src/lottieTextLayer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/riveAnimation.ts`: createRiveAnimationClip: portable field name cannot distinguish an omitted property from explicit null
- **emission** `upstream/packages/scene2d-formats/src/riveAssetBinding.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/riveDocument.ts`: _floatView: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene2d-formats/src/riveDrawOrder.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/riveImportRegistry.ts`: createRiveImportRegistry: portable record conversion requires string map keys
- **emission** `upstream/packages/scene2d-formats/src/riveLayout.ts`: createRiveGridContainerStyle: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"discriminantValue":"fixed","name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"size","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene2d-formats/src/riveRegistrars.ts`: createRiveImportRegistryFromOptions: portable record conversion requires string map keys
- **emission** `upstream/packages/scene2d-formats/src/riveRequirements.ts`: riveFamilyRootKeys: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/scene2d-formats/src/riveScene2D.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/riveShapePaint.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/riveShapePath.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/riveSolo.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/svgColor.ts`: parseSvgColor: Array.filter requires an inline callback
- **emission** `upstream/packages/scene2d-formats/src/svgGeometryElement.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (11 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene2d-formats/src/svgRegistry.ts`: registerSvgClipHandler: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"handle","optional":false,"type":{"arguments":[],"kind":"named","name":"SvgClipHandler"}},{"name":"kind","optional":false,"type":{"arguments":[],"kind":"named","name":"SvgClipKind"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene2d-formats/src/svgStyle.ts`: resolveSvgStyle: delete Rust lowering is not implemented
- **emission** `upstream/packages/scene2d-formats/src/svgTextElement.ts`: createSvgTextNode: object field enabled is not initialized by its structural spreads
- **emission** `upstream/packages/scene2d-formats/src/svgXml.ts`: parseSvgNumberList: Array.map requires an inline or inferred named callback

### `@flighthq/scene2d-gl`

- **package** `upstream/packages/scene2d-gl/src`: Generated crate is missing 14 of 93 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d-gl/src/glColorAdjustmentMaterialFeature.ts`: isTintMaterialData: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/scene2d-gl/src/glMeshShapeRenderer.ts`: glMeshShapeRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"arguments":[{"kind":"identifier","name":"state"}],"callee":{"kind":"identifier","name":"getGlRenderStateRuntime"},"kind":"call","optional":false,"typeArguments":[]},"optional":false}
- **emission** `upstream/packages/scene2d-gl/src/glRasterShapeRenderer.ts`: drawGlRasterShape: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/scene2d-gl/src/glScale9Shape.ts`: drawGlScale9Shape: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"arguments":[{"kind":"identifier","name":"state"}],"callee":{"kind":"identifier","name":"getGlRenderStateRuntime"},"kind":"call","optional":false,"typeArguments":[]},"optional":false}
- **emission** `upstream/packages/scene2d-gl/src/glScale9Sprite.ts`: drawGlScale9Sprite: Array.map requires an inline or inferred named callback
- **emission** `upstream/packages/scene2d-gl/src/glVelocity.ts`: glNode2DVelocityWriter: upstream/packages/scene2d-gl/src/glVelocity.ts: cannot infer return type for glNode2DVelocityWriter

### `@flighthq/scene2d-resources`

- **package** `upstream/packages/scene2d-resources/src`: Generated crate is missing 6 of 22 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d-resources/src/builtInScene2DDocumentImporters.ts`: _decoder: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts`: loadScene2DAudioResources: upstream/packages/scene2d-resources/src/loadScene2DAudioResources.ts:30:27: taskAll output type is not recovered
- **emission** `upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts`: loadScene2DImageResources: upstream/packages/scene2d-resources/src/loadScene2DImageResources.ts:31:25: taskAll output type is not recovered
- **emission** `upstream/packages/scene2d-resources/src/scene2DDocumentImporterRegistry.ts`: registerScene2DDocumentImporter: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"importDocument","optional":false,"type":{"arguments":[],"kind":"named","name":"Scene2DDocumentImporter"}},{"name":"kind","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"matches","optional":false,"type":{"arguments":[],"kind":"named","name":"Scene2DDocumentImporterMatcher"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene2d-resources/src/scene2DDocumentSource.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.

### `@flighthq/scene2d-wgpu`

- **package** `upstream/packages/scene2d-wgpu/src`: Generated crate is missing 17 of 91 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuColorAdjustmentMaterialFeature.ts`: isTintMaterialData: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuMeshShapeRenderer.ts`: wgpuMeshShapeRenderer: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"arguments":[{"kind":"identifier","name":"state"}],"callee":{"kind":"identifier","name":"getWgpuRenderStateRuntime"},"kind":"call","optional":false,"typeArguments":[]},"optional":false}
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuRasterShapeRenderer.ts`: drawWgpuRasterShape: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"kind":"identifier","name":"runtime"},"optional":false}
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuRendererData.ts`: createWgpuRendererData: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuScale9Shape.ts`: drawWgpuScale9Shape: optional call requires an inferred nullable function: {"kind":"property","name":"registryMiss","object":{"arguments":[{"kind":"identifier","name":"state"}],"callee":{"kind":"identifier","name":"getWgpuRenderStateRuntime"},"kind":"call","optional":false,"typeArguments":[]},"optional":false}
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuScale9Sprite.ts`: drawWgpuScale9Sprite: Array.map requires an inline or inferred named callback
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuSprite.ts`: drawWgpuSprite: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"alpha","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene2d-wgpu/src/wgpuVelocity.ts`: wgpuNode2DVelocityWriter: upstream/packages/scene2d-wgpu/src/wgpuVelocity.ts: cannot infer return type for wgpuNode2DVelocityWriter

### `@flighthq/scene3d`

- **package** `upstream/packages/scene3d/src`: Generated crate is missing 2 of 70 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene3d/src/billboardCamera.ts`: orientBillboardSubtree: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"data","optional":true,"type":{"inner":{"arguments":[],"kind":"named","name":"NodeData"},"kind":"nullable"}},{"name":"enabled","optional":true,"type":{"kind":"primitive","name":"Bool"}},{"name":"kind","optional":true,"type":{"arguments":[],"kind":"named","name":"Kind"}},{"name":"name","optional":true,"type":{"inner":{"kind":"primitive","name":"String"},"kind":"nullable"}},{"name":"alpha","optional":true,"type":{"kind":"primitive","name":"Float"}},{"name":"visible","optional":true,"type":{"kind":"primitive","name":"Bool"}},{"name":"position","optional":true,"type":{"arguments":[],"kind":"named","name":"Vector3"}},{"name":"rotation","optional":true,"type":{"arguments":[],"kind":"named","name":"Quaternion"}},{"name":"scale","optional":true,"type":{"arguments":[],"kind":"named","name":"Vector3"}},{"name":"geometry","optional":true,"type":{"arguments":[],"kind":"named","name":"MeshGeometry"}},{"name":"materials","optional":true,"type":{"element":{"inner":{"arguments":[],"kind":"named","name":"Material3D"},"kind":"nullable"},"kind":"array"}},{"name":"mode","optional":true,"type":{"arguments":[],"kind":"named","name":"BillboardMode"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene3d/src/enableScene3DGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d/src/prepareScene3DMorph.ts`: Maximum call stack size exceeded

### `@flighthq/scene3d-formats`

- **package** `upstream/packages/scene3d-formats/src`: Generated crate is missing 74 of 344 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene3d-formats/src/awd2BlockDispatch.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/awd2CameraHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/awd2GeometryHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/awd2Header.ts`: parseAwd2Header: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/awd2LightingHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/awd2MaterialHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/awd2Parse.ts`: rehydrateAwd2Body: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/awd2Reader.ts`: readAwdString: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/awd2SceneStructureHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/awd2SkeletonHandler.ts`: buildAwdDocumentAnimation: spread Rust lowering is not implemented
- **emission** `upstream/packages/scene3d-formats/src/colladaAnimationDecoder.ts`: decodeColladaAnimationsFromRoot: Array.filter requires an inline callback
- **emission** `upstream/packages/scene3d-formats/src/colladaCameraDecoder.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/colladaControllerDecoder.ts`: decodeColladaMorphsFromRoot: Array.filter requires an inline callback
- **emission** `upstream/packages/scene3d-formats/src/colladaGeometryDecoder.ts`: colladaGeometryDecoder: spread Rust lowering is not implemented
- **emission** `upstream/packages/scene3d-formats/src/colladaLightDecoder.ts`: parseFiniteNumberList: Array.filter requires an inline callback
- **emission** `upstream/packages/scene3d-formats/src/colladaMaterial.ts`: parseNumbers: Array.map requires an inline or inferred named callback
- **emission** `upstream/packages/scene3d-formats/src/colladaParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/colladaSceneShared.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/colladaXml.ts`: colladaNumbers: Array.filter requires an inline callback
- **emission** `upstream/packages/scene3d-formats/src/gltfCoreFeatureRegistry.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/gltfExtensionHandlerRegistry.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/gltfFeatures.ts`: extractGlbJsonChunk: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/gltfParse.ts`: readAccessor: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/md2Features.ts`: collectMd2Features: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/md2Parse.ts`: parseMd2WithSectionHandlers: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/md5AnimParse.ts`: buildAnimationClip: portable value conversion requires a statically recoverable source type
- **emission** `upstream/packages/scene3d-formats/src/mergeAwd2ParseOptions.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/objRequirements.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/stlFeatures.ts`: readBinaryStlTriangleCount: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/stlParse.ts`: readBinaryStlFacets: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/stlTestHelper.ts`: buildBinaryStl: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/threeDsCameraHandler.ts`: appendThreeDsCameraDocument: Math.atan Rust lowering is not implemented
- **emission** `upstream/packages/scene3d-formats/src/threeDsChunkCensus.ts`: collectThreeDsChunkCounts: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-formats/src/threeDsKeyframeHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/threeDsLightHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/threeDsMaterialHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/threeDsMeshHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (17 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-formats/src/threeDsParse.ts`: parseThreeDsDocumentWithDispatch: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object

### `@flighthq/scene3d-gl`

- **package** `upstream/packages/scene3d-gl/src`: Generated crate is missing 20 of 205 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene3d-gl/src/glEnvironmentCube.ts`: ensureGlEnvironmentSourceCube: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"TextureCommon"}],"fields":[{"discriminantValue":"2d-array","name":"dimension","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"sources","optional":false,"type":{"element":{"inner":{"arguments":[],"kind":"named","name":"TextureSource"},"kind":"nullable"},"kind":"array"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts`: explainGlPbrExtensions: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts`: buildGlPbrStandardDefineKey: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"TextureCommon"}],"fields":[{"discriminantValue":"2d-array","name":"dimension","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"sources","optional":false,"type":{"element":{"inner":{"arguments":[],"kind":"named","name":"TextureSource"},"kind":"nullable"},"kind":"array"}}],"kind":"anonymous"}
- **emission** `upstream/packages/scene3d-gl/src/glShadedPrelude.ts`: isGlModifierSnippetTable: instanceof Rust lowering requires a portable typed-array constructor

### `@flighthq/scene3d-resources`

- **package** `upstream/packages/scene3d-resources/src`: Generated crate is missing 9 of 54 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene3d-resources/src/enableScene3DResourceFailureGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-resources/src/explainScene3DResourceCoverage.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/scene3d-resources/src/gltfLoad.ts`: loadGltfExternalBuffers: upstream/packages/scene3d-resources/src/gltfLoad.ts:72:23: taskAll output type is not recovered
- **emission** `upstream/packages/scene3d-resources/src/imageResourceFetch.ts`: createWebImageResourceFetch: upstream/packages/scene3d-resources/src/imageResourceFetch.ts:5:10: portableTask createWebImageResourceFetch.anonymous:a9c2a94a69cb: Portable task Rust lowering is not implemented.
- **emission** `upstream/packages/scene3d-resources/src/loadScene3DResources.ts`: loadScene3DResources: upstream/packages/scene3d-resources/src/loadScene3DResources.ts:45:3: await value type is not recovered
- **emission** `upstream/packages/scene3d-resources/src/resolveScene3DResources.ts`: requestWorkingResolutions: taskThen Rust lowering is reserved for Pass 27 Stage 4
- **emission** `upstream/packages/scene3d-resources/src/sceneDocumentSource.ts`: Portable task source still requires OpaqueHostValue; recover every value crossing the task boundary before execution.
- **emission** `upstream/packages/scene3d-resources/src/sceneMaterialTextureRegistry.ts`: createScene3DMaterialTextureRegistry: portable record conversion requires string map keys

### `@flighthq/scene3d-wgpu`

- **package** `upstream/packages/scene3d-wgpu/src`: Generated crate is missing 41 of 170 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/scene3d-wgpu/src/enableWgpuScene3DCustomShaderGuards.ts`: hasBinding: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/scene3d-wgpu/src/wgpuCustomShaderMeshMaterialRenderer.ts`: ensureCustomTextureBindGroup: dynamic for-in Rust enumeration is not implemented
- **emission** `upstream/packages/scene3d-wgpu/src/wgpuMeshPipeline.ts`: stashWgpuUvTransform: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/scene3d-wgpu/src/wgpuScene3DTestHelper.ts`: validateWriteBufferData: instanceof Rust lowering requires a portable typed-array constructor
- **emission** `upstream/packages/scene3d-wgpu/src/wgpuShadedPrelude.ts`: createShadedBinding: portable value conversion requires a statically recoverable source type

### `@flighthq/sdk`

- **package** `upstream/packages/sdk/src`: Generated crate is missing 9357 of 9360 upstream exports across 155 manifest lanes; re-export or declaration synthesis is required.

### `@flighthq/selection`

- **package** `upstream/packages/selection/src`: Generated crate is missing 1 of 28 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/selection/src/lassoSelection.ts`: createLassoSelection: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state
- **emission** `upstream/packages/selection/src/marqueeSelection.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/selection/src/selectionState.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/shading`

- **emission** `upstream/packages/shading/src/modifierRegistry.ts`: createModifierRegistry: portable record conversion requires string map keys

### `@flighthq/shape`

- **package** `upstream/packages/shape/src`: Generated crate is missing 10 of 100 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shape/src/morphShape.ts`: createMorphShape: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/shape/src/scale9Shape.ts`: createScale9Shape: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/shape/src/shape.ts`: createShape: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/shape/src/shapeBounds.ts`: defaultShapeBoundsCubicCurveTo: upstream/packages/shape/src/shapeBounds.ts: cannot infer return type for defaultShapeBoundsCubicCurveTo

### `@flighthq/shape-formats`

- **emission** `upstream/packages/shape-formats/src/shapeCommandSchemas.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/shape-formats/src/shapeJson.ts`: formatShapeJson: portable value conversion requires a statically recoverable source type

### `@flighthq/shortcut`

- **package** `upstream/packages/shortcut/src`: Generated crate is missing 1 of 21 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/shortcut/src/shortcut.ts`: parseAcceleratorDetailed: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/shortcut/src/shortcutExplicitDependency.ts`: attachGlobalShortcut: upstream/packages/shortcut/src/shortcutExplicitDependency.ts:54:3: portable task try/catch/finally lowering is not implemented

### `@flighthq/skeleton2d`

- **package** `upstream/packages/skeleton2d/src`: Generated crate is missing 1 of 64 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/skeleton2d/src/skeleton2d.ts`: cloneSkeleton2D: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"length","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"name","optional":true,"type":{"inner":{"kind":"primitive","name":"String"},"kind":"nullable"}},{"name":"parentIndex","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"rotation","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"scaleX","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"scaleY","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"shearX","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"shearY","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"transformMode","optional":false,"type":{"arguments":[],"kind":"named","name":"TransformInherit2D"}},{"name":"x","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"y","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}

### `@flighthq/skeleton2d-formats`

- **package** `upstream/packages/skeleton2d-formats/src`: Generated crate is missing 33 of 209 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesAnimationsHandler.ts`: dragonBonesAnimationsSectionHandler: typeof operand has no inferred Rust type: {"kind":"identifier","name":"entry"}
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesBonesHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (8 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesBoneTimelineHandler.ts`: dragonBonesBoneTimelineHandler: typeof operand has no inferred Rust type: {"kind":"identifier","name":"rawTimeline"}
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesParse.ts`: parseDragonBonesSkeletonWithRegistry: typeof operand has no inferred Rust type: {"kind":"identifier","name":"first"}
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesParseHelpers.ts`: dragonBonesFrames: typeof operand has no inferred Rust type: {"kind":"identifier","name":"entry"}
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesRegistry.ts`: registerDragonBonesSectionHandler: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"handle","optional":false,"type":{"arguments":[],"kind":"named","name":"DragonBonesSectionHandler"}},{"name":"kind","optional":false,"type":{"arguments":[],"kind":"named","name":"DragonBonesSectionKind"}}],"kind":"anonymous"}
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesSectionCounts.ts`: countAnimationTimelines: typeof operand has no inferred Rust type: {"kind":"identifier","name":"animEntry"}
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesSkinsHandler.ts`: parseDragonBonesSkins: typeof operand has no inferred Rust type: {"kind":"identifier","name":"rawSkin"}
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesSlotsHandler.ts`: parseDragonBonesSlots: typeof operand has no inferred Rust type: {"kind":"identifier","name":"entry"}
- **emission** `upstream/packages/skeleton2d-formats/src/dragonBonesSlotTimelineHandler.ts`: parseDragonBonesSlotTimelines: typeof operand has no inferred Rust type: {"kind":"identifier","name":"entry"}
- **emission** `upstream/packages/skeleton2d-formats/src/skeleton2dJsonAnalyzer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (8 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d-formats/src/spineBinaryBonesHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (8 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d-formats/src/spineBinaryBoneTimelineHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (8 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d-formats/src/spineBinaryParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (8 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d-formats/src/spineBinaryReader.ts`: createSpineBinaryReader: portable value conversion requires a statically recoverable source type
- **emission** `upstream/packages/skeleton2d-formats/src/spineBinaryRegistry.ts`: registerSpineBinarySectionHandler: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"handle","optional":false,"type":{"arguments":[],"kind":"named","name":"SpineBinarySectionHandler"}},{"name":"kind","optional":false,"type":{"arguments":[],"kind":"named","name":"SpineBinarySectionKind"}}],"kind":"anonymous"}
- **emission** `upstream/packages/skeleton2d-formats/src/spineBinarySectionCounts.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (8 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d-formats/src/spineBinarySkinsHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (8 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d-formats/src/spineJsonBonesHandler.ts`: parseSpineBones: typeof operand has no inferred Rust type: {"kind":"identifier","name":"entry"}
- **emission** `upstream/packages/skeleton2d-formats/src/spineJsonBoneTimelineHandler.ts`: addSpineBoneChannel: typeof operand has no inferred Rust type: {"kind":"identifier","name":"key"}
- **emission** `upstream/packages/skeleton2d-formats/src/spineJsonRegistry.ts`: registerSpineJsonSectionHandler: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"handle","optional":false,"type":{"arguments":[],"kind":"named","name":"SpineJsonSectionHandler"}},{"name":"kind","optional":false,"type":{"arguments":[],"kind":"named","name":"SpineJsonSectionKind"}}],"kind":"anonymous"}
- **emission** `upstream/packages/skeleton2d-formats/src/spineJsonSectionCounts.ts`: countAnimationTimelines: typeof operand has no inferred Rust type: {"kind":"identifier","name":"animEntry"}
- **emission** `upstream/packages/skeleton2d-formats/src/spineJsonSkinsHandler.ts`: parseSpineSkins: typeof operand has no inferred Rust type: {"kind":"identifier","name":"skin"}
- **emission** `upstream/packages/skeleton2d-formats/src/spineJsonSlotsHandler.ts`: parseSpineSlots: typeof operand has no inferred Rust type: {"kind":"identifier","name":"entry"}
- **emission** `upstream/packages/skeleton2d-formats/src/spineJsonSlotTimelineHandler.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (8 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/skeleton2d-formats/src/spineParse.ts`: parseSpineDrawOrderTimeline: typeof operand has no inferred Rust type: {"kind":"identifier","name":"entry"}

### `@flighthq/skeleton3d`

- **package** `upstream/packages/skeleton3d/src`: Generated crate is missing 2 of 27 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/skeleton3d/src/prepareScene3DSkinning.ts`: Maximum call stack size exceeded

### `@flighthq/snapshot`

- **emission** `upstream/packages/snapshot/src/enableSnapshotGuards.ts`: nonPlainSnapshotKind: instanceof Rust lowering requires a portable typed-array constructor

### `@flighthq/socket`

- **emission** `upstream/packages/socket/src/explainSocketSendFailure.ts`: explainSocketSendFailure: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"discriminantValue":"not-open","name":"reason","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"readyState","optional":false,"type":{"arguments":[],"kind":"named","name":"SocketReadyState"}},{"name":"url","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}

### `@flighthq/spatial`

- **package** `upstream/packages/spatial/src`: Generated crate is missing 2 of 34 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/spatial/src/bvh3D.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/spatial/src/spatialIndex.ts`: initializeSpatialIndex2D: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"SpatialIndexBackend2D"},{"arguments":[],"kind":"named","name":"Entity"}],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/spatial/src/spatialIndex3D.ts`: initializeSpatialIndex3D: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"SpatialIndexBackend3D"},{"arguments":[],"kind":"named","name":"Entity"}],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/spatial/src/uniformGrid.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/spatial/src/uniformGrid3D.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/spring`

- **emission** `upstream/packages/spring/src/springConfig.ts`: createSpringConfigFromPhysical: portable value conversion requires a statically recoverable source type

### `@flighthq/spritesheet-formats`

- **emission** `upstream/packages/spritesheet-formats/src/asepriteSerialize.ts`: dataToHashDocument: object field scale is not initialized and has no Rust default
- **emission** `upstream/packages/spritesheet-formats/src/texturePackerSerialize.ts`: dataToHashDocument: object field scale is not initialized and has no Rust default

### `@flighthq/statechart`

- **emission** `upstream/packages/statechart/src/enableStatechartGuards.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/statechart/src/statechart.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/statusbar`

- **emission** `upstream/packages/statusbar/src/statusbar.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/surface`

- **package** `upstream/packages/surface/src`: Generated crate is missing 3 of 14 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/surface/src/canvasSurface.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/surface/src/surface.ts`: allocateSurface: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state

### `@flighthq/swf`

- **package** `upstream/packages/swf/src`: Generated crate is missing 34 of 114 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/swf/src/mergeSwfParseOptions.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/swf/src/swfControlHandler.ts`: readSwfButtonDefinition: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfDefineMorphShapeHandler.ts`: readSwfMorphShapeBody: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfDefineShapeHandler.ts`: readSwfShapeBody: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfDocument.ts`: createSwfTagParseState: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"bytes","optional":false,"type":{"arguments":[],"kind":"named","name":"Uint8Array"}},{"name":"mimeType","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/swf/src/swfEditTextHandler.ts`: swfEditTextHandler: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfFilter.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/swf/src/swfFontHandler.ts`: readSwfFontDefinition: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfFrameActionTestHelper.ts`: buildFrameScriptAbc: new-expression Rust lowering is not implemented: crate::OpaqueHostValue::Object
- **emission** `upstream/packages/swf/src/swfImageTexture.ts`: acquireSwfImageTexture: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"TextureCommon"}],"fields":[{"discriminantValue":"2d-array","name":"dimension","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"sources","optional":false,"type":{"element":{"inner":{"arguments":[],"kind":"named","name":"TextureSource"},"kind":"nullable"},"kind":"array"}}],"kind":"anonymous"}
- **emission** `upstream/packages/swf/src/swfMorphShape.ts`: createSwfMorphShape: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfNode.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/swf/src/swfReader.ts`: SwfReader: upstream/packages/swf/src/swfReader.ts:5: class methods and static fields are not implemented for SwfReader
- **emission** `upstream/packages/swf/src/swfScriptHandler.ts`: readSwfDoAction: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfShapeTestHelper.ts`: ShapeWriter: upstream/packages/swf/src/swfShapeTestHelper.ts:5: class methods and static fields are not implemented for ShapeWriter
- **emission** `upstream/packages/swf/src/swfSpriteHandler.ts`: swfSpriteHandler: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfStaticTextHandler.ts`: swfStaticTextHandler: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfTagStreamTestHelper.ts`: createSwfFileBytes: spread Rust lowering is not implemented
- **emission** `upstream/packages/swf/src/swfText.ts`: readSwfFontGlyphOutlineSource: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfTimelineParse.ts`: readSwfTimeline: new-expression Rust lowering is not implemented: swf_reader
- **emission** `upstream/packages/swf/src/swfVideoHandler.ts`: swfVideoHandler: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"codecId","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"deblocking","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"frameCount","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"height","optional":false,"type":{"kind":"primitive","name":"Float"}},{"name":"smoothing","optional":false,"type":{"kind":"primitive","name":"Bool"}},{"name":"width","optional":false,"type":{"kind":"primitive","name":"Float"}}],"kind":"anonymous"}

### `@flighthq/text`

- **package** `upstream/packages/text/src`: Generated crate is missing 10 of 93 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/text/src/nativeText.ts`: createNativeText: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/text/src/richText.ts`: createRichText: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}
- **emission** `upstream/packages/text/src/textLabel.ts`: createTextLabel: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}

### `@flighthq/text-markup`

- **emission** `upstream/packages/text-markup/src/markupTagRegistry.ts`: createMarkupTagRegistry: portable record conversion requires string map keys
- **emission** `upstream/packages/text-markup/src/textMarkup.ts`: handleMarkupToken: delete Rust lowering is not implemented

### `@flighthq/textsegment`

- **emission** `upstream/packages/textsegment/src/textSegmenterBackend.ts`: createWebTextSegmenterBackend: object field segment is not initialized and has no Rust default

### `@flighthq/textshaper`

- **package** `upstream/packages/textshaper/src`: Generated crate is missing 5 of 36 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/textshaper/src/textShaper.ts`: explainTextShaperOperation: typeof operand has no inferred Rust type: {"index":{"kind":"identifier","name":"operation"},"kind":"element","object":{"kind":"identifier","name":"hostTextShaper"},"optional":false}
- **emission** `upstream/packages/textshaper/src/textShaperCache.ts`: initializeTextShaperCache: EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state
- **emission** `upstream/packages/textshaper/src/textShaperPool.ts`: acquireShapedRun: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"ShapedRun"},{"arguments":[],"kind":"named","name":"Entity"}],"fields":[],"kind":"anonymous"}

### `@flighthq/texture`

- **package** `upstream/packages/texture/src`: Generated crate is missing 3 of 59 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/texture/src/cubeTexture.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/texture/src/renderTexture.ts`: createRenderTexture: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"TextureCommon"}],"fields":[{"discriminantValue":"2d-array","name":"dimension","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"sources","optional":false,"type":{"element":{"inner":{"arguments":[],"kind":"named","name":"TextureSource"},"kind":"nullable"},"kind":"array"}}],"kind":"anonymous"}
- **emission** `upstream/packages/texture/src/texture.ts`: createTexture: optional property dimension has no inferred receiver field
- **emission** `upstream/packages/texture/src/videoTexture.ts`: createVideoTexture: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"TextureCommon"}],"fields":[{"discriminantValue":"2d-array","name":"dimension","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"sources","optional":false,"type":{"element":{"inner":{"arguments":[],"kind":"named","name":"TextureSource"},"kind":"nullable"},"kind":"array"}}],"kind":"anonymous"}

### `@flighthq/texture-formats`

- **package** `upstream/packages/texture-formats/src`: Generated crate is missing 10 of 23 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/texture-formats/src/byteReader.ts`: createByteReader: portable value conversion requires a statically recoverable source type
- **emission** `upstream/packages/texture-formats/src/parseAtf.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/texture-formats/src/parseBasis.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/texture-formats/src/parseDds.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/texture-formats/src/parseKtx2.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (4 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/textureatlas`

- **emission** `upstream/packages/textureatlas/src/textureAtlasFrom.ts`: createTextureAtlasFromImageResource: anonymous structural type has no synthesized Rust identity: {"extends":[{"arguments":[],"kind":"named","name":"TextureCommon"}],"fields":[{"discriminantValue":"2d-array","name":"dimension","optional":false,"type":{"kind":"primitive","name":"String"}},{"name":"sources","optional":false,"type":{"element":{"inner":{"arguments":[],"kind":"named","name":"TextureSource"},"kind":"nullable"},"kind":"array"}}],"kind":"anonymous"}

### `@flighthq/textureatlas-formats`

- **emission** `upstream/packages/textureatlas-formats/src/textureAtlasAsepriteParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/textureatlas-formats/src/texturePackerAtlasParse.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (2 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/tilemap`

- **package** `upstream/packages/tilemap/src`: Generated crate is missing 6 of 24 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tilemap/src/tilemap.ts`: createTilemap: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[],"kind":"anonymous"}

### `@flighthq/tilemap-formats`

- **emission** `upstream/packages/tilemap-formats/src/tiledJsonParse.ts`: boolField: typeof operand has no inferred Rust type: {"kind":"identifier","name":"value"}
- **emission** `upstream/packages/tilemap-formats/src/tiledTmjFormat.ts`: formatTiledTilesetJson: JSON.stringify replacer and spacing arguments are not implemented

### `@flighthq/tokens`

- **package** `upstream/packages/tokens/src`: Generated crate is missing 1 of 10 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tokens/src/flightDocumentSceneTokens.ts`: Maximum call stack size exceeded
- **emission** `upstream/packages/tokens/src/flightDocumentTokenReference.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (1 opaque sources exceeds the approved baseline of 0); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tokens/src/substituteFlightDocumentSceneTokens.ts`: Maximum call stack size exceeded

### `@flighthq/tool-manifest`

- **package** `upstream/packages/tool-manifest/src`: Generated crate is missing 7 of 7 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **lowering** `upstream/packages/tool-manifest/src/manifestTool.ts`: TypeScript lowering produced diagnostics.
- **emission** `upstream/packages/tool-manifest/src/requirementSetFile.ts`: readRequirementCatalogFile: typeof operand has no inferred Rust type: {"index":{"kind":"identifier","name":"field"},"kind":"element","object":{"kind":"identifier","name":"raw"},"optional":false}

### `@flighthq/tool-pipeline`

- **package** `upstream/packages/tool-pipeline/src`: Generated crate is missing 12 of 12 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tool-pipeline/src/pipelineBuild.ts`: buildToolPipeline: upstream/packages/tool-pipeline/src/pipelineBuild.ts:49:19: await value type is not recovered
- **emission** `upstream/packages/tool-pipeline/src/pipelineConfig.ts`: parseToolPipelineConfig: typeof operand has no inferred Rust type: {"kind":"identifier","name":"id"}
- **emission** `upstream/packages/tool-pipeline/src/pipelineTool.ts`: runToolPipeline: upstream/packages/tool-pipeline/src/pipelineTool.ts:20:3: portable task catch bindings are not implemented

### `@flighthq/tool-registry`

- **package** `upstream/packages/tool-registry/src`: Generated crate is missing 2 of 2 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tool-registry/src/registryTool.ts`: runRegistryTool: JSON.stringify replacer and spacing arguments are not implemented

### `@flighthq/tray`

- **package** `upstream/packages/tray/src`: Generated crate is missing 4 of 30 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tray/src/tray.ts`: createTrayIcon: upstream/packages/tray/src/tray.ts:76:3: portable task catch bindings are not implemented

### `@flighthq/tween`

- **package** `upstream/packages/tween/src`: Generated crate is missing 4 of 33 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/tween/src/internal.ts`: hasTweenProperty: in-operator requires a static property name or an opaque host receiver
- **emission** `upstream/packages/tween/src/timer.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tween/src/tween.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.
- **emission** `upstream/packages/tween/src/tweenProgress.ts`: Substrate-neutral Rust emission requires OpaqueHostValue after static type recovery (3 opaque sources exceeds the approved baseline of 1); add typed IR/lowering or declare an explicit host-backend package policy instead of erasing the value type.

### `@flighthq/updater`

- **emission** `upstream/packages/updater/src/updater.ts`: OPERATION_FAILED: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"discriminantValue":"operation-failed","name":"reason","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}

### `@flighthq/video`

- **emission** `upstream/packages/video/src/videoResourceFrom.ts`: loadVideoResourceFromBlob: upstream/packages/video/src/videoResourceFrom.ts:22:3: portable task catch bindings are not implemented

### `@flighthq/vite-plugin-manifest`

- **package** `upstream/packages/vite-plugin-manifest/src`: Generated crate is missing 14 of 19 upstream exports across 2 manifest lanes; re-export or declaration synthesis is required.
- **emission** `upstream/packages/vite-plugin-manifest/src/composeContentAnalyzers.ts`: composeContentAnalyzers: optional call requires an inferred nullable function: {"kind":"property","name":"collectReferences","object":{"kind":"identifier","name":"analyzer"},"optional":false}
- **emission** `upstream/packages/vite-plugin-manifest/src/contentAnalyzers.ts`: bedrockRequirementSet: anonymous structural type has no synthesized Rust identity: {"extends":[],"fields":[{"name":"facet","optional":false,"type":{"kind":"dynamic"}},{"name":"key","optional":false,"type":{"kind":"primitive","name":"String"}}],"kind":"anonymous"}
- **emission** `upstream/packages/vite-plugin-manifest/src/manifestModuleSource.ts`: generateManifestModuleSource: optional element access requires an inferred nullable collection
- **emission** `upstream/packages/vite-plugin-manifest/src/manifestPlugin.ts`: createManifestPlugin: upstream/packages/vite-plugin-manifest/src/manifestPlugin.ts:82:3: portableTask createManifestPlugin.build: Portable task Rust lowering is not implemented.

## Candidate compile blockers

### `@flighthq/image-codec`

- **E0057** `generated/candidates/flighthq-image-codec/src/decode_image.rs`: this function takes 2 arguments but 1 argument was supplied
- **E0308** `generated/candidates/flighthq-image-codec/src/decode_image.rs`: mismatched types
- **E0308** `generated/candidates/flighthq-image-codec/src/decode_image.rs`: mismatched types
- **E0609** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: no field `byte_length` on type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0609** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: no field `byte_length` on type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0308** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: mismatched types
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0608** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: cannot index into a value of type `FlightUnion2<Vec<u8>, Vec<u8>>`
- **E0606** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: casting `&FlightUnion2<Vec<u8>, Vec<u8>>` as `usize` is invalid
- **E0609** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: no field `byte_length` on type `&Vec<u8>`
- **E0609** `generated/candidates/flighthq-image-codec/src/detect_image_mime_type.rs`: no field `byte_length` on type `&Vec<u8>`
- **E0308** `generated/candidates/flighthq-image-codec/src/encode_image.rs`: mismatched types

### `@flighthq/ipc`

- **E0425** `generated/candidates/flighthq-ipc/src/ipc.rs`: cannot find type `NoInfer` in this scope

### `@flighthq/shell`

- **E0609** `generated/candidates/flighthq-shell/src/shell.rs`: no field `to_lower_case` on type `()`

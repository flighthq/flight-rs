// Compile-time release gate for the Flight package installed from npm.
//
// `index.ts` re-exports the complete upstream module and shadows the Rust-backed functions. A new
// upstream export therefore arrives automatically, but a changed signature on one of those shadows
// can make the facade cease to be a drop-in even while a small runtime sample still passes. Assigning
// the facade module to the exact installed upstream module type checks the complete public surface,
// including every shadowed signature. The release workflow installs the dispatched Flight version
// before compiling this file, so drift blocks that version rather than being inferred from the pin.

import type * as publishedFlight from '@flighthq/bitmap';

import type * as facade from './src/index';

type PublishedFlightApi = typeof publishedFlight;
type FacadeApi = typeof facade;

const compatibleWithPublishedFlight: FacadeApi extends PublishedFlightApi ? true : never = true;

void compatibleWithPublishedFlight;

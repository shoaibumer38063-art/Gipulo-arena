import { Gallery } from './design-system/gallery/Gallery';

// The shell currently hosts the Design System gallery — the foundation preview surface.
// Real editor UI (canvas, panels wired to the engine) arrives only after the foundation is
// accepted; nothing here calls the engine beyond the protocol handshake contract.
export function App() {
  return <Gallery />;
}

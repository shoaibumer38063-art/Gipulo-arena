/**
 * Design System Gallery — the story page (dev preview surface).
 *
 * Renders every implemented component in its states, plus an editor-frame demo showing the
 * DS in situ (40px toolbar · 240px panels · canvas). This is FOUNDATION tooling, not editor
 * UI: the real editor arrives only after this foundation is accepted.
 *
 * Reference: docs/design-system/ ("the answer to 'what should this look like?'").
 */
import { useState } from 'react';

import {
  Button,
  Checkbox,
  Chip,
  Field,
  Icon,
  type IconName,
  LayerRow,
  Menu,
  type MenuEntry,
  Panel,
  PropertyRow,
  Radio,
  SearchField,
  Section,
  Segmented,
  Slider,
  Tabs,
  Toggle,
  Tooltip,
  applyTheme,
  type ThemePreference,
} from '../index';
import { PROTOCOL_VERSION } from '../../engine/client';
import './Gallery.css';

const ICON_NAMES: IconName[] = [
  'move',
  'frame',
  'rect',
  'pen',
  'text',
  'hand',
  'eye',
  'lock',
  'chev',
  'check',
  'plus',
  'comp',
  'inst',
  'minus',
  'cross',
  'search',
  'more',
];

function Example({ label, docRef, children }: { label: string; docRef: string; children: React.ReactNode }) {
  return (
    <div className="gallery-example">
      <div className="gallery-example__caption">
        {label} <span className="gallery-example__ref">{docRef}</span>
      </div>
      <div className="gallery-example__content">{children}</div>
    </div>
  );
}

function SectionTitle({ children }: { children: React.ReactNode }) {
  return <h2 className="gallery-section-title">{children}</h2>;
}

export function Gallery() {
  const [theme, setTheme] = useState<ThemePreference>('light');
  const [view, setView] = useState<'components' | 'editor'>('components');

  // interactive demo state
  const [toggleOn, setToggleOn] = useState(true);
  const [check, setCheck] = useState(true);
  const [indeterminate, setIndeterminate] = useState(true);
  const [radio, setRadio] = useState('a');
  const [segment, setSegment] = useState<'design' | 'prototype'>('design');
  const [sliderValue, setSliderValue] = useState(64);
  const [fieldX, setFieldX] = useState<number | null>(128);
  const [tabValue, setTabValue] = useState<'layers' | 'assets'>('layers');
  const [search, setSearch] = useState('');
  const [lastAction, setLastAction] = useState<string | null>(null);

  function changeTheme(next: ThemePreference) {
    setTheme(next);
    applyTheme(next);
  }

  const menuItems: MenuEntry[] = [
    { id: 'new', label: 'New frame', icon: 'frame', shortcut: 'F' },
    { id: 'copy', label: 'Copy', shortcut: '⌘C' },
    { id: 'paste', label: 'Paste', shortcut: '⌘⇧V', disabled: true },
    { id: 'sep-1', separator: true },
    { id: 'duplicate', label: 'Duplicate', shortcut: '⌘D' },
    { id: 'bring', label: 'Bring forward', shortcut: '⌘]' },
    { id: 'sep-2', separator: true },
    {
      id: 'component',
      label: 'Component',
      items: [
        { id: 'create-comp', label: 'Create component', icon: 'comp', shortcut: '⌘⌥K' },
        { id: 'detach', label: 'Detach instance', icon: 'inst' },
      ],
    },
    { id: 'checked-demo', label: 'Show rulers', shortcut: '⇧R', checked: true },
    { id: 'sep-3', separator: true },
    { id: 'delete', label: 'Delete', shortcut: '⌫', destructive: true },
  ];

  return (
    <div className="gallery">
      <header className="gallery-header">
        <div className="gallery-header__brand">
          <span className="gallery-logo" aria-hidden="true" />
          <span className="gallery-header__title">Gipulo Design System</span>
          <span className="gallery-header__version">foundation v1</span>
        </div>
        <div className="gallery-header__controls">
          <Tabs
            tabs={[
              { id: 'components', label: 'Components' },
              { id: 'editor', label: 'Editor preview' },
            ]}
            value={view}
            onChange={setView}
            ariaLabel="Gallery view"
          />
          <Segmented
            options={[
              { value: 'light', label: 'Light' },
              { value: 'dark', label: 'Dark' },
              { value: 'system', label: 'System' },
            ]}
            value={theme}
            onChange={changeTheme}
            ariaLabel="Theme"
          />
        </div>
      </header>

      {view === 'components' ? (
        <main className="gallery-main">
          <SectionTitle>Buttons — one gradient action per view</SectionTitle>
          <div className="gallery-row">
            <Button variant="primary">Share</Button>
            <Button variant="secondary">Secondary</Button>
            <Button variant="ghost">Ghost</Button>
            <Tooltip label="Rectangle" shortcut="R">
              <Button variant="icon" icon="rect" ariaLabel="Rectangle tool" />
            </Tooltip>
            <Button variant="destructive">Delete</Button>
            <Button variant="primary" loading>
              Publishing
            </Button>
            <Button variant="secondary" disabled>
              Disabled
            </Button>
          </div>

          <SectionTitle>Fields & search</SectionTitle>
          <div className="gallery-row gallery-row--wide">
            <div className="gallery-field">
              <Field label="X" value={fieldX} scrub onCommit={(value) => setFieldX(Number(value))} ariaLabel="X position" />
            </div>
            <div className="gallery-field">
              <Field label="W" value={null} ariaLabel="Width (mixed)" onCommit={() => undefined} />
            </div>
            <div className="gallery-field">
              <Field label="R" value="12.5" invalid ariaLabel="Corner radius (invalid)" onCommit={() => undefined} />
            </div>
            <div className="gallery-field">
              <Field label="H" value="240" disabled ariaLabel="Height (disabled)" />
            </div>
            <div className="gallery-field gallery-field--search">
              <SearchField value={search} onChange={setSearch} ariaLabel="Search layers" />
            </div>
          </div>
          <div className="gallery-note">
            X field is scrubbable — drag the label (⌥ for fine steps). Second field shows the
            readable “Mixed” placeholder. Third field is the invalid state.
          </div>

          <SectionTitle>Selection controls</SectionTitle>
          <div className="gallery-row gallery-row--wide">
            <Toggle checked={toggleOn} onChange={setToggleOn} ariaLabel="Toggle demo" />
            <Toggle checked={false} onChange={() => undefined} ariaLabel="Off toggle" />
            <Toggle checked={true} onChange={() => undefined} disabled ariaLabel="Disabled toggle" />
            <span className="gallery-divider" />
            <Checkbox checked={check} onChange={setCheck} label="Published" />
            <Checkbox checked={false} indeterminate={indeterminate} onChange={(value) => { setIndeterminate(false); setCheck(value); }} label="Mixed" />
            <Checkbox checked={false} onChange={() => undefined} disabled label="Disabled" />
            <span className="gallery-divider" />
            <Radio name="demo" value="a" checked={radio === 'a'} onChange={() => setRadio('a')} label="Auto" />
            <Radio name="demo" value="b" checked={radio === 'b'} onChange={() => setRadio('b')} label="Manual" />
          </div>

          <div className="gallery-row gallery-row--wide">
            <Segmented
              options={[
                { value: 'design', label: 'Design' },
                { value: 'prototype', label: 'Prototype' },
              ]}
              value={segment}
              onChange={setSegment}
              ariaLabel="Mode"
            />
            <div className="gallery-slider">
              <Slider value={sliderValue} onChange={setSliderValue} ariaLabel="Opacity" />
            </div>
            <div className="gallery-field">
              <Field label="%" value={sliderValue} scrub onCommit={(value) => setSliderValue(Number(value))} ariaLabel="Opacity percent" />
            </div>
          </div>

          <SectionTitle>Menu — one system for every pop menu</SectionTitle>
          <Example label="Selection context menu (interactive: arrows, Enter, Escape, hover submenu)" docRef="06 §1">
            <div className="gallery-menuStage">
              <Menu items={menuItems} ariaLabel="Demo menu" onSelect={setLastAction} onClose={() => undefined} />
            </div>
            <div className="gallery-note">
              {lastAction ? `Last action: ${lastAction}` : 'Select an item — actions land here.'}
            </div>
          </Example>

          <SectionTitle>Tabs & chips</SectionTitle>
          <div className="gallery-row gallery-row--wide">
            <div className="gallery-tabsBox">
              <Tabs
                tabs={[
                  { id: 'layers', label: 'Layers' },
                  { id: 'assets', label: 'Assets' },
                ]}
                value={tabValue}
                onChange={setTabValue}
                ariaLabel="Panel tabs"
              />
            </div>
            <Chip>Default</Chip>
            <Chip active>Active</Chip>
            <Chip error>Missing font</Chip>
          </div>

          <SectionTitle>Icons — one language, 16 grid, stroke 1.25</SectionTitle>
          <div className="gallery-icons">
            {ICON_NAMES.map((name) => (
              <div key={name} className="gallery-icons__cell">
                <Icon name={name} />
                <span>{name}</span>
              </div>
            ))}
          </div>

          <SectionTitle>Layer rows — every state (the hottest component)</SectionTitle>
          <Example label="States matrix" docRef="06 §9">
            <div className="gallery-layerStage">
              <div role="tree" aria-label="Layer states demo">
                <LayerRow kind="frame" name="Frame — default" />
                <LayerRow kind="rect" name="Selected" selected />
                <LayerRow kind="group" name="Descendant of selection" descendantSelected depth={1} />
                <LayerRow kind="image" name="Hidden layer" hidden />
                <LayerRow kind="vector" name="Locked layer" locked />
                <LayerRow kind="comp" name="Component" component hasChildren expanded />
                <LayerRow kind="inst" name="Instance" component />
                <LayerRow kind="text" name="Missing font — error" error />
                <LayerRow kind="rect" name="Dragging" dragState="dragging" />
                <LayerRow kind="rect" name="Drop inside frame" dragState="dropInside" />
                <LayerRow kind="rect" name="Insert above me" insertIndicator="before" />
                <LayerRow kind="rect" name="Insert below me" insertIndicator="after" />
                <LayerRow kind="text" name="Renaming" renaming onRenameCommit={() => undefined} />
              </div>
            </div>
          </Example>
          <Example label="Deep nesting — 16px per level, 8px beyond level 8, guide lines" docRef="06 §9">
            <div className="gallery-layerStage" role="tree" aria-label="Deep nesting demo">
              {Array.from({ length: 12 }, (_, depth) => (
                <LayerRow key={depth} kind={depth % 3 === 0 ? 'group' : 'rect'} name={`Level ${depth}`} depth={depth} />
              ))}
            </div>
          </Example>

          <SectionTitle>Property rows</SectionTitle>
          <div className="gallery-props">
            <PropertyRow label="Position">
              <Field label="X" value={128} scrub onCommit={() => undefined} ariaLabel="X" />
              <Field label="Y" value={64} scrub onCommit={() => undefined} ariaLabel="Y" />
            </PropertyRow>
            <PropertyRow label="Size">
              <Field label="W" value={320} scrub onCommit={() => undefined} ariaLabel="W" />
              <Field label="H" value={180} scrub onCommit={() => undefined} ariaLabel="H" />
            </PropertyRow>
            <PropertyRow label="Opacity">
              <Slider value={sliderValue} onChange={setSliderValue} ariaLabel="Row opacity" />
              <Field label="%" value={sliderValue} scrub onCommit={(value) => setSliderValue(Number(value))} ariaLabel="Opacity percent" />
            </PropertyRow>
            <PropertyRow label="Visible">
              <Toggle checked={toggleOn} onChange={setToggleOn} ariaLabel="Row visible" />
            </PropertyRow>
            <PropertyRow label="Radius" disabled>
              <Field label="R" value={12} ariaLabel="Radius (disabled row)" />
            </PropertyRow>
          </div>
        </main>
      ) : (
        <main className="gallery-editor" aria-label="Editor preview (illustrative)">
          <div className="gallery-toolbar">
            <span className="gallery-logo" aria-hidden="true" />
            <span className="gallery-toolbar__sep" />
            <button type="button" className="gallery-tool is-active" aria-label="Move tool">
              <Icon name="move" />
            </button>
            <button type="button" className="gallery-tool" aria-label="Frame tool">
              <Icon name="frame" />
            </button>
            <button type="button" className="gallery-tool" aria-label="Rectangle tool">
              <Icon name="rect" />
            </button>
            <button type="button" className="gallery-tool" aria-label="Pen tool">
              <Icon name="pen" />
            </button>
            <button type="button" className="gallery-tool" aria-label="Text tool">
              <Icon name="text" />
            </button>
            <button type="button" className="gallery-tool" aria-label="Hand tool">
              <Icon name="hand" />
            </button>
            <span className="gallery-toolbar__sep" />
            <span className="gallery-toolbar__file">
              <span className="gallery-toolbar__path">Gipulo /</span> Untitled — Explore
            </span>
            <span className="gallery-toolbar__spacer" />
            <Button variant="primary">Share</Button>
          </div>

          <div className="gallery-editor__body">
            <Panel className="gallery-editor__left">
              <Tabs
                tabs={[
                  { id: 'layers', label: 'Layers' },
                  { id: 'assets', label: 'Assets' },
                ]}
                value={tabValue}
                onChange={setTabValue}
                ariaLabel="Left panel tabs"
              />
              <div className="gallery-editor__search">
                <SearchField value={search} onChange={setSearch} ariaLabel="Search layers" />
              </div>
              <div className="gallery-editor__tree" role="tree" aria-label="Layers">
                <LayerRow kind="frame" name="Frame — Explore" hasChildren expanded />
                <LayerRow kind="group" name="Hero" depth={1} hasChildren expanded selected />
                <LayerRow kind="text" name="Title — Gipulo" depth={2} descendantSelected />
                <LayerRow kind="text" name="Subtitle" depth={2} descendantSelected hidden />
                <LayerRow kind="rect" name="CTA" depth={2} descendantSelected />
                <LayerRow kind="group" name="Feature grid" depth={1} hasChildren />
                <LayerRow kind="comp" name="Card / Feature" depth={1} component insertIndicator="before" />
                <LayerRow kind="inst" name="Card / Pricing" depth={1} component />
                <LayerRow kind="image" name="Backdrop" depth={1} locked />
              </div>
            </Panel>

            <div className="gallery-canvas">
              <div className="gallery-canvas__frameLabel">Frame — 1440</div>
              <div className="gallery-canvas__frame">
                <div className="gallery-canvas__selection">
                  <span className="gallery-canvas__pill">320 × 180</span>
                  <span className="gallery-canvas__handle" style={{ left: -4, top: -4 }} />
                  <span className="gallery-canvas__handle" style={{ right: -4, top: -4 }} />
                  <span className="gallery-canvas__handle" style={{ left: -4, bottom: -4 }} />
                  <span className="gallery-canvas__handle" style={{ right: -4, bottom: -4 }} />
                </div>
              </div>
              <div className="gallery-canvas__note">
                illustrative only — real canvas chrome renders from the Rust core (ADR-001)
              </div>
            </div>

            <Panel className="gallery-editor__right">
              <div className="gallery-editor__panelTitle">Selection</div>
              <Section title="Position & size">
                <PropertyRow label="Position">
                  <Field label="X" value={128} scrub onCommit={() => undefined} ariaLabel="X" />
                  <Field label="Y" value={64} scrub onCommit={() => undefined} ariaLabel="Y" />
                </PropertyRow>
                <PropertyRow label="Size">
                  <Field label="W" value={320} scrub onCommit={() => undefined} ariaLabel="W" />
                  <Field label="H" value={180} scrub onCommit={() => undefined} ariaLabel="H" />
                </PropertyRow>
              </Section>
              <Section title="Appearance">
                <PropertyRow label="Opacity">
                  <Slider value={sliderValue} onChange={setSliderValue} ariaLabel="Opacity" />
                  <Field label="%" value={sliderValue} scrub onCommit={(value) => setSliderValue(Number(value))} ariaLabel="Opacity percent" />
                </PropertyRow>
                <PropertyRow label="Blend">
                  <Segmented
                    options={[
                      { value: 'normal', label: 'Normal' },
                      { value: 'multiply', label: 'Multiply' },
                    ]}
                    value="normal"
                    onChange={() => undefined}
                    ariaLabel="Blend mode"
                  />
                </PropertyRow>
                <PropertyRow label="Visible">
                  <Toggle checked={toggleOn} onChange={setToggleOn} ariaLabel="Visible" />
                </PropertyRow>
              </Section>
              <Section title="Auto layout" defaultOpen={false}>
                <PropertyRow label="Direction">
                  <Segmented
                    options={[
                      { value: 'h', label: 'H' },
                      { value: 'v', label: 'V' },
                    ]}
                    value="h"
                    onChange={() => undefined}
                    ariaLabel="Direction"
                  />
                </PropertyRow>
              </Section>
            </Panel>
          </div>
        </main>
      )}

      <footer className="gallery-footer">
        Foundation preview — no editor features exist yet. Engine protocol v{PROTOCOL_VERSION} ·
        tokens from <code>docs/design-system/02_tokens.md</code> ·{' '}
        {view === 'components' ? 'components view' : 'editor frame (illustrative)'}
      </footer>
    </div>
  );
}

/**
 * Gipulo Design System — public surface.
 *
 * Features import from here ONLY (never from component files directly), so the module can
 * evolve without breaking consumers. See ./README.md for the conventions that bind this
 * folder, and docs/design-system/ for the authored system.
 */

export { applyTheme, cssVar, TOKEN_NAMES } from './tokens';
export type { ThemePreference, TokenName } from './tokens';

export { Icon } from './components/Icon';
export type { IconName, IconSize } from './components/Icon';

export { Button } from './components/Button';
export type { ButtonVariant } from './components/Button';

export { Segmented } from './components/Segmented';
export type { SegmentedOption } from './components/Segmented';

export { Field } from './components/Field';

export { SearchField } from './components/SearchField';

export { Toggle } from './components/Toggle';

export { Checkbox, Radio } from './components/Choice';

export { Slider } from './components/Slider';

export { Tooltip } from './components/Tooltip';

export { Menu } from './components/Menu';
export type { MenuEntry, MenuItemDef, MenuSeparatorDef } from './components/Menu';

export { Tabs } from './components/Tabs';
export type { TabDef } from './components/Tabs';

export { Chip } from './components/Chip';

export { Panel, Section } from './components/Panel';

export { PropertyRow } from './components/PropertyRow';

export { LayerRow } from './components/LayerRow';
export type { LayerDragState, LayerInsertIndicator } from './components/LayerRow';

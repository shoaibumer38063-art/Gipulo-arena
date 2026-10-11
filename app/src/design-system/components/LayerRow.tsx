/**
 * LayerRow — docs/design-system/06_components_surfaces.md §9.
 * The hottest component in the editor: 28px rows, indent law (16px/level, 8px beyond level 8),
 * per-level indent guides, hover-revealed actions (that row only), and every drag/rename/
 * selection state from the reference. HOT tier: flat paint, color/opacity transitions only.
 */
import { useEffect, useRef, useState } from 'react';

import { Icon, type IconName } from './Icon';
import './LayerRow.css';

export type LayerDragState = 'none' | 'dragging' | 'dropInside';
export type LayerInsertIndicator = 'none' | 'before' | 'after';

interface LayerRowProps {
  kind: IconName;
  name: string;
  /** 0-based nesting depth. */
  depth?: number;
  selected?: boolean;
  /** Tinted treatment for a descendant of the selected container. */
  descendantSelected?: boolean;
  hidden?: boolean;
  locked?: boolean;
  /** Component/instance rows use the purple identity treatment. */
  component?: boolean;
  /** Error rows (e.g. missing font) use danger colors. */
  error?: boolean;
  hasChildren?: boolean;
  expanded?: boolean;
  renaming?: boolean;
  dragState?: LayerDragState;
  insertIndicator?: LayerInsertIndicator;
  onSelect?: () => void;
  onToggleExpand?: () => void;
  onToggleHidden?: () => void;
  onToggleLocked?: () => void;
  onRenameCommit?: (name: string) => void;
}

/** Indent-guide x-positions for each ancestor level (16px steps, 8px beyond level 8). */
function guidePositions(depth: number): number[] {
  const positions: number[] = [];
  let pad = 8;
  for (let level = 0; level < depth; level += 1) {
    positions.push(pad + 8 - 0.5);
    pad += level + 1 <= 8 ? 16 : 8;
  }
  return positions;
}

function indentPadding(depth: number): number {
  const wideLevels = Math.min(depth, 8);
  const deepLevels = Math.max(0, depth - 8);
  return 8 + wideLevels * 16 + deepLevels * 8;
}

export function LayerRow({
  kind,
  name,
  depth = 0,
  selected = false,
  descendantSelected = false,
  hidden = false,
  locked = false,
  component = false,
  error = false,
  hasChildren = false,
  expanded = false,
  renaming = false,
  dragState = 'none',
  insertIndicator = 'none',
  onSelect,
  onToggleExpand,
  onToggleHidden,
  onToggleLocked,
  onRenameCommit,
}: LayerRowProps) {
  const [draftName, setDraftName] = useState<string | null>(null);
  const renameRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (renaming) {
      setDraftName(name);
      renameRef.current?.focus();
      renameRef.current?.select();
    }
  }, [renaming, name]);

  const classes = [
    'gip-layer',
    selected ? 'is-selected' : '',
    descendantSelected && !selected ? 'is-descendant-selected' : '',
    hidden ? 'is-hidden' : '',
    locked ? 'is-locked' : '',
    component ? 'is-component' : '',
    error ? 'is-error' : '',
    dragState === 'dragging' ? 'is-dragging' : '',
    dragState === 'dropInside' ? 'is-drop-inside' : '',
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <div className="gip-layer__wrap">
      {insertIndicator === 'before' && <InsertIndicator />}
      <div
        className={classes}
        style={{ paddingLeft: indentPadding(depth) }}
        role="treeitem"
        aria-selected={selected}
        aria-level={depth + 1}
        aria-expanded={hasChildren ? expanded : undefined}
        tabIndex={selected ? 0 : -1}
        onClick={onSelect}
        onKeyDown={(event) => {
          if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            onSelect?.();
          }
        }}
      >
        {guidePositions(depth).map((left) => (
          <span key={left} className="gip-layer__guide" style={{ left }} aria-hidden="true" />
        ))}

        {hasChildren ? (
          <button
            type="button"
            className={`gip-layer__chev${expanded ? ' is-open' : ''}`}
            aria-label={expanded ? 'Collapse' : 'Expand'}
            onClick={(event) => {
              event.stopPropagation();
              onToggleExpand?.();
            }}
          >
            <Icon name="chev" size={12} />
          </button>
        ) : (
          <span className="gip-layer__chevSpacer" />
        )}

        <span className="gip-layer__icon">
          <Icon name={kind} size={16} />
        </span>

        {renaming && draftName !== null ? (
          <input
            ref={renameRef}
            className="gip-layer__rename"
            value={draftName}
            aria-label="Layer name"
            onChange={(event) => setDraftName(event.target.value)}
            onBlur={() => {
              onRenameCommit?.(draftName);
              setDraftName(null);
            }}
            onKeyDown={(event) => {
              event.stopPropagation();
              if (event.key === 'Enter') {
                onRenameCommit?.(draftName);
                setDraftName(null);
              } else if (event.key === 'Escape') {
                setDraftName(null);
              }
            }}
          />
        ) : (
          <span className="gip-layer__name">{name}</span>
        )}

        <span className="gip-layer__actions">
          <button
            type="button"
            className="gip-layer__action gip-layer__action--lock"
            aria-label={locked ? 'Unlock layer' : 'Lock layer'}
            aria-pressed={locked}
            onClick={(event) => {
              event.stopPropagation();
              onToggleLocked?.();
            }}
          >
            <Icon name="lock" size={16} />
          </button>
          <button
            type="button"
            className="gip-layer__action gip-layer__action--eye"
            aria-label={hidden ? 'Show layer' : 'Hide layer'}
            aria-pressed={hidden}
            onClick={(event) => {
              event.stopPropagation();
              onToggleHidden?.();
            }}
          >
            <Icon name="eye" size={16} />
          </button>
        </span>
      </div>
      {insertIndicator === 'after' && <InsertIndicator />}
    </div>
  );
}

/** 2px accent line + 6px circle marker at the insertion point (06 §9). */
function InsertIndicator() {
  return (
    <div className="gip-layer__insert" aria-hidden="true">
      <span className="gip-layer__insertCircle" />
    </div>
  );
}

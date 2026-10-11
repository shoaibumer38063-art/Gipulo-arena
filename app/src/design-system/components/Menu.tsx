/**
 * Menu — docs/design-system/06_components_surfaces.md §1.
 * One system for dropdowns, context menus, toolbar menus, property menus: 224px, radius 13,
 * accent-hover items, check gutter, shortcuts, destructive items, separators, submenus
 * (hover intent 150ms per canvas §27, ArrowRight/Left, Escape closes innermost first).
 * Keyboard-first: it is fully operable without a pointer (11 §3).
 *
 * v1 scope: one submenu level (deeper nesting arrives with whichever feature needs it).
 */
import { useEffect, useRef, useState } from 'react';
import type { CSSProperties, KeyboardEvent as ReactKeyboardEvent } from 'react';

import { Icon, type IconName } from './Icon';
import './Menu.css';

export interface MenuItemDef {
  id: string;
  label: string;
  icon?: IconName;
  shortcut?: string;
  checked?: boolean;
  disabled?: boolean;
  destructive?: boolean;
  /** Presence turns the item into a submenu opener. */
  items?: MenuEntry[];
}

export interface MenuSeparatorDef {
  id: string;
  separator: true;
}

export type MenuEntry = MenuItemDef | MenuSeparatorDef;

function isSeparator(entry: MenuEntry): entry is MenuSeparatorDef {
  return 'separator' in entry && entry.separator === true;
}

const HOVER_INTENT_MS = 150;

interface MenuProps {
  items: ReadonlyArray<MenuEntry>;
  onSelect?: ((id: string, path: ReadonlyArray<string>) => void) | undefined;
  /** Called on Escape at the top level (close the menu + return focus to invoker). */
  onClose?: (() => void) | undefined;
  ariaLabel: string;
  autoFocus?: boolean;
  className?: string;
  style?: CSSProperties;
}

export function Menu({
  items,
  onSelect,
  onClose,
  ariaLabel,
  autoFocus = true,
  className,
  style,
}: MenuProps) {
  const rootRef = useRef<HTMLDivElement>(null);
  const [activeId, setActiveId] = useState<string | null>(null);
  const [openSubmenuId, setOpenSubmenuId] = useState<string | null>(null);
  const hoverTimer = useRef<number | null>(null);

  const enabledIds = items
    .filter((entry): entry is MenuItemDef => !isSeparator(entry) && !entry.disabled)
    .map((entry) => entry.id);

  useEffect(() => {
    if (autoFocus) rootRef.current?.focus();
  }, [autoFocus]);

  useEffect(() => {
    function onPointerDown(event: PointerEvent) {
      if (rootRef.current && !rootRef.current.contains(event.target as Node)) {
        onClose?.();
      }
    }
    document.addEventListener('pointerdown', onPointerDown);
    return () => document.removeEventListener('pointerdown', onPointerDown);
  }, [onClose]);

  useEffect(() => {
    return () => {
      if (hoverTimer.current !== null) window.clearTimeout(hoverTimer.current);
    };
  }, []);

  function moveActive(direction: 1 | -1) {
    if (enabledIds.length === 0) return;
    const current = activeId ? enabledIds.indexOf(activeId) : -1;
    const next = (current + direction + enabledIds.length) % enabledIds.length;
    const id = enabledIds[next];
    if (id) setActiveId(id);
  }

  function activate(entry: MenuItemDef, path: ReadonlyArray<string>) {
    if (entry.disabled) return;
    if (entry.items) {
      setOpenSubmenuId((current) => (current === entry.id ? null : entry.id));
      return;
    }
    onSelect?.(entry.id, path);
    onClose?.();
  }

  function onKeyDown(event: ReactKeyboardEvent<HTMLDivElement>) {
    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        moveActive(1);
        break;
      case 'ArrowUp':
        event.preventDefault();
        moveActive(-1);
        break;
      case 'Home':
        event.preventDefault();
        if (enabledIds[0]) setActiveId(enabledIds[0]);
        break;
      case 'End':
        event.preventDefault();
        if (enabledIds.length > 0) setActiveId(enabledIds[enabledIds.length - 1] ?? null);
        break;
      case 'ArrowRight': {
        const entry = items.find((item) => !isSeparator(item) && item.id === activeId);
        if (entry && !isSeparator(entry) && entry.items) {
          event.preventDefault();
          setOpenSubmenuId(entry.id);
        }
        break;
      }
      case 'ArrowLeft':
        if (openSubmenuId) {
          event.preventDefault();
          setOpenSubmenuId(null);
        }
        break;
      case 'Enter':
      case ' ': {
        const entry = items.find((item) => !isSeparator(item) && item.id === activeId);
        if (entry && !isSeparator(entry)) {
          event.preventDefault();
          activate(entry, []);
        }
        break;
      }
      case 'Escape':
        event.preventDefault();
        if (openSubmenuId) setOpenSubmenuId(null);
        else onClose?.();
        break;
      default:
        break;
    }
  }

  return (
    <div
      ref={rootRef}
      role="menu"
      aria-label={ariaLabel}
      aria-activedescendant={activeId ? `${ariaLabel}-${activeId}` : undefined}
      tabIndex={-1}
      className={className ? `gip-menu ${className}` : 'gip-menu'}
      style={style}
      onKeyDown={onKeyDown}
    >
      {items.map((entry) => {
        if (isSeparator(entry)) {
          return <div key={entry.id} className="gip-menu__separator" role="separator" />;
        }

        const item = entry;
        const isActive = activeId === item.id;
        const submenuOpen = openSubmenuId === item.id;

        function scheduleSubmenu(open: boolean) {
          if (hoverTimer.current !== null) window.clearTimeout(hoverTimer.current);
          if (open && item.items) {
            hoverTimer.current = window.setTimeout(() => setOpenSubmenuId(item.id), HOVER_INTENT_MS);
          } else if (!open) {
            setOpenSubmenuId((current) => (current === item.id ? null : current));
          }
        }

        return (
          <div
            key={item.id}
            className="gip-menu__itemWrap"
            onMouseEnter={() => {
              setActiveId(item.id);
              scheduleSubmenu(true);
            }}
            onMouseLeave={() => scheduleSubmenu(false)}
          >
            <button
              type="button"
              role="menuitem"
              id={`${ariaLabel}-${item.id}`}
              tabIndex={-1}
              disabled={item.disabled}
              aria-haspopup={item.items ? 'menu' : undefined}
              aria-expanded={item.items ? submenuOpen : undefined}
              className={[
                'gip-menu__item',
                isActive ? 'is-active' : '',
                item.destructive ? 'is-destructive' : '',
                item.checked ? 'is-checked' : '',
              ]
                .filter(Boolean)
                .join(' ')}
              onClick={() => activate(item, [])}
            >
              <span className="gip-menu__gutter">
                {item.checked ? (
                  <Icon name="check" size={12} />
                ) : item.icon ? (
                  <Icon name={item.icon} size={16} />
                ) : null}
              </span>
              <span className="gip-menu__label">{item.label}</span>
              {item.shortcut && <span className="gip-menu__shortcut">{item.shortcut}</span>}
              {item.items && (
                <span className="gip-menu__subArrow">
                  <Icon name="chev" size={12} />
                </span>
              )}
            </button>

            {submenuOpen && item.items && (
              <Menu
                items={item.items}
                onSelect={onSelect}
                onClose={() => setOpenSubmenuId(null)}
                ariaLabel={`${ariaLabel}-${item.id}-submenu`}
                autoFocus={false}
                className="gip-menu--sub"
              />
            )}
          </div>
        );
      })}
    </div>
  );
}

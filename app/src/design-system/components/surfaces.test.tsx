import { act, fireEvent, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, it, vi } from 'vitest';

import { Menu, type MenuEntry } from './Menu';
import { Tabs } from './Tabs';
import { Tooltip } from './Tooltip';

const menuItems: MenuEntry[] = [
  { id: 'copy', label: 'Copy', shortcut: '⌘C' },
  { id: 'paste', label: 'Paste', disabled: true },
  { id: 'sep', separator: true },
  {
    id: 'component',
    label: 'Component',
    items: [{ id: 'create-comp', label: 'Create component' }],
  },
  { id: 'delete', label: 'Delete', destructive: true },
];

afterEach(() => {
  vi.useRealTimers();
});

describe('Menu (06 §1)', () => {
  it('renders items, separators and shortcuts', () => {
    render(<Menu items={menuItems} ariaLabel="Test menu" autoFocus={false} />);
    expect(screen.getByRole('menu', { name: 'Test menu' })).toBeInTheDocument();
    expect(screen.getByRole('menuitem', { name: /Copy/ })).toBeInTheDocument();
    expect(screen.getByText('⌘C')).toBeInTheDocument();
    expect(screen.getAllByRole('separator')).toHaveLength(1);
  });

  it('keyboard: ArrowDown skips disabled items and Enter selects (then closes)', () => {
    const onSelect = vi.fn();
    const onClose = vi.fn();
    render(<Menu items={menuItems} onSelect={onSelect} onClose={onClose} ariaLabel="Test menu" />);
    const menu = screen.getByRole('menu', { name: 'Test menu' });
    fireEvent.keyDown(menu, { key: 'ArrowDown' }); // Copy (first enabled)
    fireEvent.keyDown(menu, { key: 'ArrowDown' }); // skips disabled Paste -> Component (submenu opener)
    fireEvent.keyDown(menu, { key: 'Enter' }); // opens submenu, does not select
    expect(onSelect).not.toHaveBeenCalled();
    // Move back up to Copy and select it.
    fireEvent.keyDown(menu, { key: 'Escape' }); // close submenu first
    fireEvent.keyDown(menu, { key: 'Home' });
    fireEvent.keyDown(menu, { key: 'Enter' });
    expect(onSelect).toHaveBeenCalledWith('copy', []);
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('Escape at the top level closes the menu', () => {
    const onClose = vi.fn();
    render(<Menu items={menuItems} onClose={onClose} ariaLabel="Test menu" />);
    fireEvent.keyDown(screen.getByRole('menu', { name: 'Test menu' }), { key: 'Escape' });
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('hover opens a submenu after the 150ms intent delay', () => {
    vi.useFakeTimers();
    render(<Menu items={menuItems} ariaLabel="Test menu" autoFocus={false} />);
    const componentItem = screen.getByRole('menuitem', { name: /Component/ });
    fireEvent.mouseEnter(componentItem.parentElement ?? componentItem);
    expect(screen.getAllByRole('menu')).toHaveLength(1); // not yet — intent delay
    act(() => {
      vi.advanceTimersByTime(160);
    });
    expect(screen.getAllByRole('menu')).toHaveLength(2);
    expect(screen.getByRole('menuitem', { name: 'Create component' })).toBeInTheDocument();
  });

  it('clicking a leaf item selects it', async () => {
    const onSelect = vi.fn();
    render(<Menu items={menuItems} onSelect={onSelect} ariaLabel="Test menu" autoFocus={false} />);
    await userEvent.click(screen.getByRole('menuitem', { name: /Delete/ }));
    expect(onSelect).toHaveBeenCalledWith('delete', []);
  });
});

describe('Tooltip (06 §4)', () => {
  it('appears after the hover delay and disappears on leave', () => {
    vi.useFakeTimers();
    render(
      <Tooltip label="Rectangle" shortcut="R">
        <button type="button">tool</button>
      </Tooltip>,
    );
    const anchor = screen.getByText('tool').parentElement;
    expect(anchor).not.toBeNull();
    fireEvent.mouseEnter(anchor as HTMLElement);
    expect(screen.queryByRole('tooltip')).not.toBeInTheDocument();
    act(() => {
      vi.advanceTimersByTime(420);
    });
    const tooltip = screen.getByRole('tooltip');
    expect(tooltip).toHaveTextContent('Rectangle');
    expect(tooltip).toHaveTextContent('R');
    fireEvent.mouseLeave(anchor as HTMLElement);
    expect(screen.queryByRole('tooltip')).not.toBeInTheDocument();
  });

  it('shows immediately on keyboard focus (keyboard users skip the delay)', () => {
    vi.useFakeTimers();
    render(
      <Tooltip label="Vertical padding">
        <button type="button">padding</button>
      </Tooltip>,
    );
    const anchor = screen.getByText('padding').parentElement as HTMLElement;
    fireEvent.focus(anchor);
    act(() => {
      vi.advanceTimersByTime(1);
    });
    expect(screen.getByRole('tooltip')).toHaveTextContent('Vertical padding');
  });
});

describe('Tabs (06 §6)', () => {
  const tabs = [
    { id: 'layers', label: 'Layers' },
    { id: 'assets', label: 'Assets' },
  ] as const;

  it('exposes tablist semantics and switches on click', async () => {
    const onChange = vi.fn();
    render(<Tabs tabs={tabs} value="layers" onChange={onChange} ariaLabel="Panel tabs" />);
    expect(screen.getByRole('tablist', { name: 'Panel tabs' })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: 'Layers' })).toHaveAttribute('aria-selected', 'true');
    await userEvent.click(screen.getByRole('tab', { name: 'Assets' }));
    expect(onChange).toHaveBeenCalledWith('assets');
  });

  it('arrow keys move between tabs (roving tabindex)', () => {
    const onChange = vi.fn();
    render(<Tabs tabs={tabs} value="layers" onChange={onChange} ariaLabel="Panel tabs" />);
    fireEvent.keyDown(screen.getByRole('tablist', { name: 'Panel tabs' }), { key: 'ArrowRight' });
    expect(onChange).toHaveBeenCalledWith('assets');
    expect(screen.getByRole('tab', { name: 'Layers' })).toHaveAttribute('tabindex', '0');
  });
});

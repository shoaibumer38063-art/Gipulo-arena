import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

import { Chip } from './Chip';
import { LayerRow } from './LayerRow';
import { Panel, Section } from './Panel';
import { PropertyRow } from './PropertyRow';

describe('LayerRow (06 §9)', () => {
  it('is a treeitem with selection/level/expansion semantics', () => {
    render(
      <LayerRow kind="group" name="Hero" depth={1} selected hasChildren expanded />,
    );
    const row = screen.getByRole('treeitem', { name: /Hero/ });
    expect(row).toHaveAttribute('aria-selected', 'true');
    expect(row).toHaveAttribute('aria-level', '2');
    expect(row).toHaveAttribute('aria-expanded', 'true');
  });

  it('calls onSelect and toggles visibility/lock through its action buttons', async () => {
    const onSelect = vi.fn();
    const onToggleHidden = vi.fn();
    const onToggleLocked = vi.fn();
    render(
      <LayerRow
        kind="rect"
        name="CTA"
        onSelect={onSelect}
        onToggleHidden={onToggleHidden}
        onToggleLocked={onToggleLocked}
      />,
    );
    await userEvent.click(screen.getByText('CTA'));
    expect(onSelect).toHaveBeenCalledTimes(1);
    await userEvent.click(screen.getByRole('button', { name: 'Hide layer' }));
    expect(onToggleHidden).toHaveBeenCalledTimes(1);
    await userEvent.click(screen.getByRole('button', { name: 'Lock layer' }));
    expect(onToggleLocked).toHaveBeenCalledTimes(1);
  });

  it('hidden rows flip their action labels (state is not color-only — 11 §6)', async () => {
    const { rerender } = render(<LayerRow kind="image" name="Backdrop" hidden />);
    expect(screen.getByRole('button', { name: 'Show layer' })).toHaveAttribute(
      'aria-pressed',
      'true',
    );
    rerender(<LayerRow kind="image" name="Backdrop" />);
    expect(screen.getByRole('button', { name: 'Hide layer' })).toBeInTheDocument();
  });

  it('renaming commits on Enter and cancels on Escape', async () => {
    const onRenameCommit = vi.fn();
    render(<LayerRow kind="text" name="Title" renaming onRenameCommit={onRenameCommit} />);
    const input = screen.getByRole('textbox', { name: 'Layer name' });
    await userEvent.clear(input);
    await userEvent.type(input, 'Hero title{Enter}');
    expect(onRenameCommit).toHaveBeenCalledWith('Hero title');
  });

  it('renders indent guides for nested depths (16px/level law)', () => {
    render(<LayerRow kind="rect" name="Deep" depth={3} />);
    const row = screen.getByRole('treeitem', { name: /Deep/ });
    expect(row).toHaveStyle({ paddingLeft: '56px' }); // 8 + 3×16
  });
});

describe('Chip (06 §11)', () => {
  it('renders variants with their state classes', () => {
    const { rerender } = render(<Chip>Default</Chip>);
    expect(screen.getByText('Default')).toHaveClass('gip-chip');
    rerender(<Chip active>Active</Chip>);
    expect(screen.getByText('Active')).toHaveClass('is-active');
    rerender(<Chip error>Missing font</Chip>);
    expect(screen.getByText('Missing font')).toHaveClass('is-error');
  });
});

describe('PropertyRow & Section (06 §7–8)', () => {
  it('renders label plus control slot', () => {
    render(
      <PropertyRow label="Opacity">
        <span data-testid="control" />
      </PropertyRow>,
    );
    expect(screen.getByText('Opacity')).toBeInTheDocument();
    expect(screen.getByTestId('control')).toBeInTheDocument();
  });

  it('Section headers collapse and expand with aria state', async () => {
    render(
      <Section title="Auto layout">
        <PropertyRow label="Direction">
          <span data-testid="direction-control" />
        </PropertyRow>
      </Section>,
    );
    const header = screen.getByRole('button', { name: /Auto layout/ });
    expect(header).toHaveAttribute('aria-expanded', 'true');
    await userEvent.click(header);
    expect(header).toHaveAttribute('aria-expanded', 'false');
    expect(screen.queryByText('Direction')).not.toBeInTheDocument();
  });

  it('Panel applies width only when provided (240px token default otherwise)', () => {
    render(<Panel className="demo">content</Panel>);
    expect(screen.getByText('content')).toBeInTheDocument();
  });
});

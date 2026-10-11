import { fireEvent, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

import { Button } from './Button';
import { Checkbox, Radio } from './Choice';
import { Field } from './Field';
import { Segmented } from './Segmented';
import { Slider } from './Slider';
import { Toggle } from './Toggle';

describe('Button (05 §1)', () => {
  it('renders label and fires onClick', async () => {
    const onClick = vi.fn();
    render(<Button onClick={onClick}>Share</Button>);
    await userEvent.click(screen.getByRole('button', { name: 'Share' }));
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it('disabled buttons do not fire and expose disabled state', async () => {
    const onClick = vi.fn();
    render(
      <Button disabled onClick={onClick}>
        Disabled
      </Button>,
    );
    const button = screen.getByRole('button', { name: 'Disabled' });
    expect(button).toBeDisabled();
    await userEvent.click(button);
    expect(onClick).not.toHaveBeenCalled();
  });

  it('loading blocks interaction, sets aria-busy, and keeps the label (no layout shift)', () => {
    const onClick = vi.fn();
    render(
      <Button loading onClick={onClick}>
        Publishing
      </Button>,
    );
    const button = screen.getByRole('button', { name: 'Publishing' });
    expect(button).toHaveAttribute('aria-busy', 'true');
    expect(button).toBeDisabled();
  });

  it('icon variant carries its accessible name', () => {
    render(<Button variant="icon" icon="rect" ariaLabel="Rectangle tool" />);
    expect(screen.getByRole('button', { name: 'Rectangle tool' })).toBeInTheDocument();
  });
});

describe('Toggle (05 §5)', () => {
  it('is a switch with aria-checked and toggles on click', async () => {
    const onChange = vi.fn();
    render(<Toggle checked={false} onChange={onChange} ariaLabel="Visible" />);
    const toggle = screen.getByRole('switch', { name: 'Visible' });
    expect(toggle).toHaveAttribute('aria-checked', 'false');
    await userEvent.click(toggle);
    expect(onChange).toHaveBeenCalledWith(true);
  });

  it('activates from the keyboard (Space) — buttons keep native activation', async () => {
    const onChange = vi.fn();
    render(<Toggle checked={false} onChange={onChange} ariaLabel="Keyboard" />);
    screen.getByRole('switch', { name: 'Keyboard' }).focus();
    await userEvent.keyboard(' ');
    expect(onChange).toHaveBeenCalledWith(true);
  });
});

describe('Checkbox & Radio (05 §6)', () => {
  it('checkbox reports changes and renders the label', async () => {
    const onChange = vi.fn();
    render(<Checkbox checked={false} onChange={onChange} label="Published" />);
    await userEvent.click(screen.getByRole('checkbox', { name: 'Published' }));
    expect(onChange).toHaveBeenCalledWith(true);
  });

  it('indeterminate state sets the input property (mixed values)', () => {
    render(<Checkbox checked={false} indeterminate onChange={() => undefined} label="Mixed" />);
    const input = screen.getByRole('checkbox', { name: 'Mixed' }) as HTMLInputElement;
    expect(input.indeterminate).toBe(true);
  });

  it('radios share a group name and report selection', async () => {
    const onChange = vi.fn();
    render(
      <>
        <Radio name="demo" value="a" checked={false} onChange={onChange} label="Auto" />
        <Radio name="demo" value="b" checked={true} onChange={onChange} label="Manual" />
      </>,
    );
    expect(screen.getByRole('radio', { name: 'Manual' })).toBeChecked();
    await userEvent.click(screen.getByRole('radio', { name: 'Auto' }));
    expect(onChange).toHaveBeenCalledTimes(1);
  });
});

describe('Segmented (05 §2)', () => {
  const options = [
    { value: 'h', label: 'H' },
    { value: 'v', label: 'V' },
  ] as const;

  it('is a radiogroup and switches on click', async () => {
    const onChange = vi.fn();
    render(<Segmented options={options} value="h" onChange={onChange} ariaLabel="Direction" />);
    expect(screen.getByRole('radiogroup', { name: 'Direction' })).toBeInTheDocument();
    await userEvent.click(screen.getByRole('radio', { name: 'V' }));
    expect(onChange).toHaveBeenCalledWith('v');
  });

  it('arrow keys move the selection', () => {
    const onChange = vi.fn();
    render(<Segmented options={options} value="h" onChange={onChange} ariaLabel="Direction" />);
    fireEvent.keyDown(screen.getByRole('radiogroup', { name: 'Direction' }), { key: 'ArrowRight' });
    expect(onChange).toHaveBeenCalledWith('v');
  });
});

describe('Field (05 §3)', () => {
  it('commits on Enter and reverts on Escape (interactions I6)', async () => {
    const onCommit = vi.fn();
    render(<Field label="X" value={128} onCommit={onCommit} ariaLabel="X position" />);
    const input = screen.getByLabelText('X position');
    await userEvent.clear(input);
    await userEvent.type(input, '256{Enter}');
    expect(onCommit).toHaveBeenCalledWith('256');

    onCommit.mockClear();
    await userEvent.clear(input);
    await userEvent.type(input, '999{Escape}');
    expect(onCommit).not.toHaveBeenCalled();
    expect((screen.getByLabelText('X position') as HTMLInputElement).value).toBe('128');
  });

  it('renders the readable Mixed placeholder for null values', () => {
    render(<Field label="W" value={null} ariaLabel="Width" />);
    expect(screen.getByPlaceholderText('Mixed')).toBeInTheDocument();
  });

  it('marks invalid fields for assistive tech', () => {
    render(<Field label="R" value="12" invalid ariaLabel="Radius" />);
    expect(screen.getByLabelText('Radius')).toHaveAttribute('aria-invalid', 'true');
  });

  it('scrubbing the label emits previews and commits once on release', () => {
    const onCommit = vi.fn();
    const onPreview = vi.fn();
    render(
      <Field label="X" value={128} scrub onCommit={onCommit} onPreview={onPreview} ariaLabel="X scrub" />,
    );
    const label = screen.getByText('X');
    fireEvent.pointerDown(label, { clientX: 100, pointerId: 1 });
    fireEvent.pointerMove(label, { clientX: 140, pointerId: 1 });
    expect(onPreview).toHaveBeenCalledWith('148'); // 40px / 2 = +20 units
    expect(onCommit).not.toHaveBeenCalled(); // previews are transient
    fireEvent.pointerUp(label, { clientX: 140, pointerId: 1 });
    expect(onCommit).toHaveBeenCalledWith('148'); // one commit for the whole gesture
  });
});

describe('Slider (05 §7)', () => {
  it('exposes the slider role with bounds and value', () => {
    render(<Slider value={64} onChange={() => undefined} ariaLabel="Opacity" />);
    const slider = screen.getByRole('slider', { name: 'Opacity' });
    expect(slider).toHaveAttribute('aria-valuemin', '0');
    expect(slider).toHaveAttribute('aria-valuemax', '100');
    expect(slider).toHaveAttribute('aria-valuenow', '64');
  });

  it('arrow keys step by 1, Shift by 10, Home/End jump', () => {
    const onChange = vi.fn();
    render(<Slider value={64} onChange={onChange} ariaLabel="Opacity" />);
    const slider = screen.getByRole('slider', { name: 'Opacity' });
    fireEvent.keyDown(slider, { key: 'ArrowRight' });
    expect(onChange).toHaveBeenLastCalledWith(65);
    fireEvent.keyDown(slider, { key: 'ArrowRight', shiftKey: true });
    expect(onChange).toHaveBeenLastCalledWith(74);
    fireEvent.keyDown(slider, { key: 'Home' });
    expect(onChange).toHaveBeenLastCalledWith(0);
    fireEvent.keyDown(slider, { key: 'End' });
    expect(onChange).toHaveBeenLastCalledWith(100);
  });
});

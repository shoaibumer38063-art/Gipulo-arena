import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it } from 'vitest';

import { Gallery } from './Gallery';

/** Smoke suite: the gallery must render every section and switch views/themes (dev surface). */
describe('Design System gallery', () => {
  it('renders the component sections', () => {
    render(<Gallery />);
    expect(screen.getByText('Gipulo Design System')).toBeInTheDocument();
    expect(screen.getByText(/Buttons — one gradient action per view/)).toBeInTheDocument();
    expect(screen.getByText(/Layer rows — every state/)).toBeInTheDocument();
    expect(screen.getByRole('menu', { name: 'Demo menu' })).toBeInTheDocument();
  });

  it('switches to the editor preview and shows the toolbar', async () => {
    render(<Gallery />);
    await userEvent.click(screen.getByRole('tab', { name: 'Editor preview' }));
    expect(screen.getByLabelText('Editor preview (illustrative)')).toBeInTheDocument();
    expect(screen.getByLabelText('Move tool')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Share' })).toBeInTheDocument();
  });

  it('applies the dark theme to the document root', async () => {
    render(<Gallery />);
    await userEvent.click(screen.getByRole('radio', { name: 'Dark' }));
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark');
    await userEvent.click(screen.getByRole('radio', { name: 'System' }));
    expect(document.documentElement.hasAttribute('data-theme')).toBe(false);
  });
});

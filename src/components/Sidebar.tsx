import { useState } from 'react';
import type { VaultInfo, NoteSummary, NoteRecord } from '../types/ipc';

interface SidebarProps {
  vault: VaultInfo;
  notes: NoteSummary[];
  selectedNote: NoteRecord | null;
  onSelectNote: (path: string) => void;
  onCreateNote: () => void;
  onToggleTheme: () => void;
  isDark: boolean;
}

function vaultDisplayName(path: string): string {
  return path.replace(/\\/g, '/').split('/').filter(Boolean).pop() ?? path;
}

function formatDate(iso: string): string {
  const date = new Date(iso);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const days = Math.floor(diff / 86_400_000);
  if (days === 0) return 'today';
  if (days === 1) return 'yesterday';
  if (days < 7) return `${days}d ago`;
  if (days < 365)
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  return date.getFullYear().toString();
}

export function Sidebar({
  vault,
  notes,
  selectedNote,
  onSelectNote,
  onCreateNote,
  onToggleTheme,
  isDark,
}: SidebarProps) {
  const [query, setQuery] = useState('');

  const filtered = query.trim()
    ? notes.filter((n) =>
        n.title.toLowerCase().includes(query.toLowerCase()),
      )
    : notes;

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <span className="vault-name" title={vault.path}>
          {vaultDisplayName(vault.path)}
        </span>
        <button
          className="icon-btn"
          onClick={onCreateNote}
          title="New note"
          aria-label="New note"
        >
          +
        </button>
      </div>

      <div className="sidebar-search">
        <input
          className="search-input"
          type="text"
          placeholder="filter notes"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          spellCheck={false}
        />
      </div>

      <div className="sidebar-list">
        {filtered.length === 0 ? (
          <p className="sidebar-empty">
            {query ? 'no matches' : 'no notes yet'}
          </p>
        ) : (
          filtered.map((note) => (
            <div
              key={note.id}
              className={`sidebar-item${selectedNote?.id === note.id ? ' active' : ''}`}
              onClick={() => onSelectNote(note.path)}
            >
              <span className="sidebar-item-title">{note.title}</span>
              <span className="sidebar-item-date">
                {formatDate(note.updated_at)}
              </span>
            </div>
          ))
        )}
      </div>

      <div className="sidebar-footer">
        <button
          className="icon-btn"
          onClick={onToggleTheme}
          title={isDark ? 'Switch to light mode' : 'Switch to dark mode'}
          aria-label="Toggle theme"
        >
          {isDark ? '○' : '●'}
        </button>
      </div>
    </aside>
  );
}

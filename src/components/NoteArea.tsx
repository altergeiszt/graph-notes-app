import { useState } from 'react';
import { useDebouncedCallback } from 'use-debounce';
import { invoke } from '@tauri-apps/api/core';
import type { NoteRecord } from '../types/ipc';
import { Editor } from './Editor';
import { Preview } from './Preview';

type PaneMode = 'editor' | 'preview' | 'split';

interface NoteAreaProps {
  note: NoteRecord;
  mode: PaneMode;
  isDark: boolean;
  onModeChange: (mode: PaneMode) => void;
}

const MODES: { key: PaneMode; label: string }[] = [
  { key: 'editor',  label: 'edit' },
  { key: 'split',   label: 'split' },
  { key: 'preview', label: 'preview' },
];

export function NoteArea({ note, mode, isDark, onModeChange }: NoteAreaProps) {
  const [content, setContent] = useState(note.content);
  const debouncedSave = useDebouncedCallback(
    (val: string) => invoke('note_save', { path: note.path, content: val }),
    500,
  );

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
      <div className="note-toolbar">
        <span className="note-title">{note.title}</span>
        <div className="mode-toggle" role="group" aria-label="View mode">
          {MODES.map(({ key, label }) => (
            <button
              key={key}
              className={`mode-btn${mode === key ? ' active' : ''}`}
              onClick={() => onModeChange(key)}
            >
              {label}
            </button>
          ))}
        </div>
      </div>

      <div
        className="note-content"
        style={{
          gridTemplateColumns: mode === 'split' ? '1fr 1fr' : '1fr',
        }}
      >
        {(mode === 'editor' || mode === 'split') && (
          <Editor
            content={content}
            onChange={(v: string) => {
              setContent(v);
              debouncedSave(v);
            }}
            noteId={note.id}
            isDark={isDark}
          />
        )}
        {(mode === 'preview' || mode === 'split') && (
          <Preview content={content} />
        )}
      </div>
    </div>
  );
}

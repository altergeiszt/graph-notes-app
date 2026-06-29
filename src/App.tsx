import './App.css';
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import type {
  VaultInfo,
  IndexProgressPayload,
  IndexDonePayload,
  NoteSummary,
  NoteRecord,
} from './types/ipc';
import { useTheme } from './hooks/useTheme';
import { Sidebar } from './components/Sidebar';
import { NoteArea } from './components/NoteArea';
import { BacklinksPanel } from './components/BacklinksPanel';

type AppStatus = 'idle' | 'indexing' | 'ready';
type PaneMode = 'editor' | 'preview' | 'split';

function App() {
  const { isDark, toggleTheme } = useTheme();
  const [status, setStatus] = useState<AppStatus>('idle');
  const [vault, setVault] = useState<VaultInfo | null>(null);
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [notes, setNotes] = useState<NoteSummary[]>([]);
  const [selectedNote, setSelectedNote] = useState<NoteRecord | null>(null);
  const [paneMode, setPaneMode] = useState<PaneMode>('editor');
  const [isCreating, setIsCreating] = useState(false);

  useEffect(() => {
    const unlistenProgress = listen<IndexProgressPayload>(
      'vault_index_progress',
      (event) => setProgress(event.payload.pct),
    );

    const unlistenDone = listen<IndexDonePayload>(
      'vault_index_done',
      (event) => {
        setVault((prev) =>
          prev ? { ...prev, note_count: event.payload.note_count } : null,
        );
        setStatus('ready');
      },
    );

    const unlistenError = listen<string>(
      'vault_index_error',
      (event) => {
        setError(event.payload);
        setStatus('idle');
      },
    );

    return () => {
      unlistenProgress.then((f) => f());
      unlistenDone.then((f) => f());
      unlistenError.then((f) => f());
    };
  }, []);

  useEffect(() => {
    if (status === 'ready') {
      invoke<NoteSummary[]>('note_list')
        .then(setNotes)
        .catch(console.error);
    }
  }, [status]);

  async function handleOpenVault() {
    const selected = await open({ directory: true, multiple: false });
    if (!selected || Array.isArray(selected)) return;

    setError(null);
    setStatus('indexing');
    setProgress(0);

    try {
      const info = await invoke<VaultInfo>('vault_open', { path: selected });
      setVault(info);
    } catch (err) {
      setError(String(err));
      setStatus('idle');
    }
  }

  async function handleSelectNote(path: string) {
    try {
      const note = await invoke<NoteRecord>('note_read', { path });
      setSelectedNote(note);
    } catch (err) {
      console.error('Failed to read note:', err);
    }
  }

  async function handleCreateNote() {
    if (isCreating) return;
    setIsCreating(true);
    try {
      const summary = await invoke<NoteSummary>('note_create', {
        title: 'Untitled',
      });
      const refreshed = await invoke<NoteSummary[]>('note_list');
      setNotes(refreshed);
      await handleSelectNote(summary.path);
    } catch (err) {
      setError(String(err));
      const refreshed = await invoke<NoteSummary[]>('note_list').catch(() => notes);
      setNotes(refreshed);
    } finally {
      setIsCreating(false);
    }
  }

  if (status === 'idle') {
    return (
      <main className="idle-screen">
        <div className="idle-content">
          <h1 className="app-name">GraphNotes</h1>
          <p className="app-tagline">local-first knowledge graph</p>
          <button className="open-vault-btn" onClick={handleOpenVault}>
            open vault
          </button>
          {error && <p className="error-msg">{error}</p>}
        </div>
      </main>
    );
  }

  if (status === 'indexing') {
    return (
      <main className="idle-screen">
        <div className="idle-content">
          <p className="indexing-label">indexing vault</p>
          <div className="progress-track">
            <div
              className="progress-fill"
              style={{ width: `${progress}%` }}
            />
          </div>
          <p className="progress-pct">{progress}%</p>
        </div>
      </main>
    );
  }

  return (
    <div className="workspace">
      <Sidebar
        vault={vault!}
        notes={notes}
        selectedNote={selectedNote}
        onSelectNote={handleSelectNote}
        onCreateNote={handleCreateNote}
        onToggleTheme={toggleTheme}
        isDark={isDark}
        isCreating={isCreating}
      />

      <div className="editor-area">
        {selectedNote ? (
          <NoteArea
            note={selectedNote}
            mode={paneMode}
            isDark={isDark}
            onModeChange={setPaneMode}
          />
        ) : (
          <div className="empty-state">
            select or create a note
          </div>
        )}
      </div>

      {selectedNote && (
        <BacklinksPanel
          note={selectedNote}
          onNavigate={handleSelectNote}
        />
      )}
    </div>
  );
}

export default App;

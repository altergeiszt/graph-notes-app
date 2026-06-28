import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { BacklinkEntry, NoteRecord } from '../types/ipc';

interface Props {
  note: NoteRecord;
  onNavigate: (path: string) => void;
}

export function BacklinksPanel({ note, onNavigate }: Props) {
  const [backlinks, setBacklinks] = useState<BacklinkEntry[]>([]);
  const [unlinked, setUnlinked] = useState<BacklinkEntry[]>([]);

  useEffect(() => {
    invoke<BacklinkEntry[]>('graph_query_backlinks', { path: note.path })
      .then(setBacklinks)
      .catch(() => setBacklinks([]));

    invoke<BacklinkEntry[]>('graph_query_unlinked_mentions', {
      path: note.path,
      title: note.title,
    })
      .then(setUnlinked)
      .catch(() => setUnlinked([]));
  }, [note.id]);

  return (
    <div className="backlinks-panel">
      <section>
        <h3>Linked Mentions ({backlinks.length})</h3>
        {backlinks.map((bl) => (
          <div
            key={bl.source_path}
            onClick={() => onNavigate(bl.source_path)}
            className="backlink-entry"
            style={{ cursor: 'pointer' }}
          >
            <strong>{bl.source_title}</strong>
            <p className="snippet">{bl.snippet}</p>
          </div>
        ))}
      </section>

      <section>
        <h3>Unlinked Mentions ({unlinked.length})</h3>
        {unlinked.map((ul) => (
          <div
            key={ul.source_path}
            onClick={() => onNavigate(ul.source_path)}
            className="backlink-entry unlinked"
            style={{ cursor: 'pointer' }}
          >
            <strong>{ul.source_title}</strong>
            <p className="snippet">{ul.snippet}</p>
          </div>
        ))}
      </section>
    </div>
  );
}

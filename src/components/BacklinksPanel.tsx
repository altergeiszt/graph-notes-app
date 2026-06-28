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
    <aside className="backlinks-panel">
      <div className="backlinks-panel-header">backlinks</div>

      <div className="backlinks-scroll">
        <div className="backlinks-section">
          <p className="backlinks-heading">
            linked &mdash; {backlinks.length}
          </p>
          {backlinks.length === 0 ? (
            <p className="backlinks-empty">none</p>
          ) : (
            backlinks.map((bl) => (
              <div
                key={bl.source_path}
                className="backlinks-entry"
                onClick={() => onNavigate(bl.source_path)}
              >
                <span className="backlinks-entry-title">
                  {bl.source_title}
                </span>
                {bl.snippet && (
                  <p className="backlinks-snippet">{bl.snippet}</p>
                )}
              </div>
            ))
          )}
        </div>

        <div className="backlinks-section">
          <p className="backlinks-heading">
            unlinked &mdash; {unlinked.length}
          </p>
          {unlinked.length === 0 ? (
            <p className="backlinks-empty">none</p>
          ) : (
            unlinked.map((ul) => (
              <div
                key={ul.source_path}
                className="backlinks-entry"
                onClick={() => onNavigate(ul.source_path)}
              >
                <span className="backlinks-entry-title">
                  {ul.source_title}
                </span>
                {ul.snippet && (
                  <p className="backlinks-snippet">{ul.snippet}</p>
                )}
              </div>
            ))
          )}
        </div>
      </div>
    </aside>
  );
}

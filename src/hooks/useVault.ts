import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import type { VaultInfo, IndexProgressPayload, IndexDonePayload } from '../types/ipc';

export function useVault(
  onProgress?: (pct: number) => void,
  onDone?: (noteCount: number) => void,
) {
  const openVault = async () => {
    const selected = await open({ directory: true, multiple: false });
    if (!selected || Array.isArray(selected)) return;

    const unlistenProgress = await listen<IndexProgressPayload>(
      'vault_index_progress',
      (event) => onProgress?.(event.payload.pct),
    );
    const unlistenDone = await listen<IndexDonePayload>(
      'vault_index_done',
      (event) => {
        onDone?.(event.payload.note_count);
        unlistenProgress();
        unlistenDone();
      },
    );

    await invoke<VaultInfo>('vault_open', { path: selected });
  };

  return { openVault };
}

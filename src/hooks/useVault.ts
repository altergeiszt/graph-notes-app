import { invoke } from '@tauri-apps/api/core'; 
import { listen } from '@tauri-apps/api/event'; 
import { open } from '@tauri-apps/plugin-dialog'; 
import type { VaultInfo, ProgressPayload } from '../types/ipc'; 
 export function useVault() { 
  const openVault = async () => { 
    const selected = await open({ directory: true, multiple: false }); 
    if (!selected || Array.isArray(selected)) return; 
     // Listen for progress before invoking 
    const unlistenProgress = await listen<ProgressPayload>( 
      'vault_index_progress', 
      (event) => setIndexProgress(event.payload.pct) 
    ); 
    const unlistenDone = await listen<number>( 
      'vault_index_done', 
      (event) => { setNoteCount(event.payload); unlistenProgress(); unlistenDone(); 

}     ); 
     await invoke<VaultInfo>('vault_open', { path: selected }); 
  }; 
   return { openVault }; 
}
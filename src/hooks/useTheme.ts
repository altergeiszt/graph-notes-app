import { useEffect, useState } from 'react'; 
import { load as loadStore } from '@tauri-apps/plugin-store'; 
 export function useTheme() { 
  const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches; 
  const [isDark, setIsDark] = useState(systemDark); 
   useEffect(() => { 
    // Load persisted preference 
    loadStore('settings.json').then(store => 
      store.get<boolean>('dark_mode').then(v => {
        if (v !== undefined) setIsDark(v);
      })
    ); 
  }, []); 
   useEffect(() => { 
    document.documentElement.classList.toggle('dark', isDark); 
  }, [isDark]); 
   const toggleTheme = async () => { 
    const next = !isDark; 
    setIsDark(next); 
    const store = await loadStore('settings.json'); 
    await store.set('dark_mode', next); 
    await store.save(); 
  }; 
   return { isDark, toggleTheme }; 
} 
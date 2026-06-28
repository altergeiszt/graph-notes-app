import { useEffect, useRef } from 'react';
import { EditorView, keymap, lineNumbers, type ViewUpdate } from '@codemirror/view';
import { EditorState } from '@codemirror/state'; 
import { defaultKeymap, historyKeymap, history } from '@codemirror/commands'; 
import { markdown } from '@codemirror/lang-markdown'; 
import { languages } from '@codemirror/language-data'; 
import { oneDark } from '@codemirror/theme-one-dark'; 
 interface EditorProps { 
  content: string;             // Controlled from parent (React state) 
  onChange: (val: string) => void; 
  noteId: string;              // Used to detect note switches 
  isDark: boolean; 
} 
 export function Editor({ content, onChange, noteId, isDark }: EditorProps) { 
  const containerRef = useRef<HTMLDivElement>(null); 
  const viewRef = useRef<EditorView | null>(null); 
   // Re-create EditorView when note switches 
  useEffect(() => { 
    if (!containerRef.current) return; 
    viewRef.current?.destroy(); 
     const state = EditorState.create({ 
      doc: content, 
      extensions: [ 
        history(), 
        lineNumbers(), 
        markdown({ codeLanguages: languages }), 
        keymap.of([...defaultKeymap, ...historyKeymap]), 
        isDark ? oneDark : [], 
        EditorView.updateListener.of((update: ViewUpdate) => {
          if (update.docChanged) { 
            onChange(update.state.doc.toString()); 
          } 
        }), 
        EditorView.theme({ 
          '&': { height: '100%', fontSize: '14px' }, 
          '.cm-scroller': { overflow: 'auto', fontFamily: 'var(--font-mono)' }, 
        }), 
      ], 
    });
    viewRef.current = new EditorView({ state, parent: containerRef.current }); 
     return () => viewRef.current?.destroy(); 
  // eslint-disable-next-line react-hooks/exhaustive-deps 
  }, [noteId]); // Only re-create when note changes, NOT on content or isDark 
   // Sync isDark changes without re-creating 
  useEffect(() => { 
    // Dispatch a config transaction — CM6 supports dynamic theme swapping 
    // Implementation: use a StateField for theme; dispatch reconfigure() 
  }, [isDark]); 
   return <div ref={containerRef} style={{ height: '100%', width: '100%' }} />; 
}
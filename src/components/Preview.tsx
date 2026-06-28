import { useMemo } from 'react'; 
import { unified } from 'unified'; 
import remarkParse from 'remark-parse'; 
import remarkGfm from 'remark-gfm'; 
import remarkMath from 'remark-math'; 
import remarkRehype from 'remark-rehype'; 
import rehypeKatex from 'rehype-katex'; 
import rehypeHighlight from 'rehype-highlight'; 
import rehypeStringify from 'rehype-stringify'; 
import 'katex/dist/katex.min.css'; 
 const processor = unified() 
  .use(remarkParse)        // parse Markdown 
  .use(remarkGfm)          // GitHub-flavoured Markdown (tables, task lists) 
  .use(remarkMath)         // $...$ and $$...$$ math nodes 
  .use(remarkRehype, { allowDangerousHtml: false }) 
  .use(rehypeKatex)        // render math nodes with KaTeX 
  .use(rehypeHighlight)    // syntax-highlight fenced code blocks
  .use(rehypeStringify);    // serialize HTML
  
interface PreviewProps { content: string;}

export function Preview({content} : PreviewProps) {
    const html = useMemo(() => {
        try {
            return String(processor.processSync(content));
        } catch {
            return '<p style="color:red">Render error</p>';
        }
    }, [content]);
    return <div className="prose dark:prose-invert max-w-none p-4 overflow-auto h-full" dangerouslySetInnerHTML={{ __html: html }}/>;
}


// src/lib/remarkTransclusion.ts 
// A custom remark plugin that transforms ![[Target]] into a placeholder node, 
// then a rehype plugin replaces the placeholder with fetched note HTML. 
 import { invoke } from '@tauri-apps/api/core'; 
import type { NoteRecord } from '../types/ipc'; 
 // Because fetching is async and remark/rehype run sync, 
// we pre-fetch all transclusion targets before running the pipeline. 
 export async function prefetchTransclusions( 
  content: string 
): Promise<Map<string, string>> { 
  const transclusionRe = /!\[\[([^\]#|]+?)(?:#([^\]|]+?))?\]\]/g; 
  const fetches = new Map<string, Promise<string>>(); 
   for (const match of content.matchAll(transclusionRe)) { 
    const target = match[1].trim(); 
    if (!fetches.has(target)) { 
      fetches.set(target, invoke<NoteRecord>('note_read', { path: target }) 
        .then(r => r.content) 
        .catch(() => `> Note not found: ${target}`)); 
    } 
  } 
   const resolved = new Map<string, string>(); 
  for (const [key, promise] of fetches) { 
    resolved.set(key, await promise); 
  } 
  return resolved; 
} 
 // Then in Preview.tsx, before running the processor: 
  const transclusions = await prefetchTransclusions(content); 
  const expandedContent = content.replace( 
  /!\[\[([^\]]+?)\]\]/g, 
  (_, target) => transclusions.get(target.trim()) ?? `> Note not found: 
${target}` 
// );
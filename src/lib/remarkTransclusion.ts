import { invoke } from '@tauri-apps/api/core';
import type { NoteRecord } from '../types/ipc';

const TRANSCLUSION_RE = /!\[\[([^\]#|]+?)(?:#([^\]|]+?))?\]\]/g;

/**
 * Pre-fetch all transclusion targets found in content before the sync
 * remark/rehype pipeline runs. Returns a map of target name → raw content.
 *
 * Circular transclusions are broken by tracking in-progress paths.
 */
export async function prefetchTransclusions(
  content: string,
  inProgress: Set<string> = new Set()
): Promise<Map<string, string>> {
  const fetches = new Map<string, Promise<string>>();

  for (const match of content.matchAll(TRANSCLUSION_RE)) {
    const target = match[1].trim();
    if (fetches.has(target) || inProgress.has(target)) continue;

    inProgress.add(target);
    fetches.set(
      target,
      invoke<NoteRecord>('note_read', { path: target })
        .then(async (record) => {
          // Recursively expand nested transclusions
          const nested = await prefetchTransclusions(record.content, inProgress);
          let expanded = record.content;
          for (const [nestedTarget, nestedContent] of nested) {
            expanded = expanded.replace(
              new RegExp(`!\\[\\[${escapeRegex(nestedTarget)}(?:#[^\\]]*)?\\]\\]`, 'g'),
              nestedContent
            );
          }
          return expanded;
        })
        .catch(() => `> *Note not found: ${target}*`)
    );
  }

  const resolved = new Map<string, string>();
  for (const [key, promise] of fetches) {
    resolved.set(key, await promise);
  }
  return resolved;
}

/**
 * Expand all ![[Target]] tokens in content using the pre-fetched map.
 */
export function expandTransclusions(
  content: string,
  resolved: Map<string, string>
): string {
  return content.replace(TRANSCLUSION_RE, (_, target) => {
    const key = target.trim();
    return resolved.get(key) ?? `> *Note not found: ${key}*`;
  });
}

function escapeRegex(str: string): string {
  return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

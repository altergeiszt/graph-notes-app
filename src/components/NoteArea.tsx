import { useState } from "react";
import { useDebouncedCallback } from "use-debounce";
import { invoke } from "@tauri-apps/api/core";
import type { NoteRecord } from "../types/ipc";
import { Editor } from "./Editor";
import { Preview } from "./Preview";

type PaneMode = "editor" | "preview" | "split";

interface NoteAreaProps {
  note: NoteRecord;
  mode: PaneMode;
  isDark: boolean;
}

export function NoteArea({ note, mode, isDark }: NoteAreaProps) {
  const [content, setContent] = useState(note.content);
  const debouncedSave = useDebouncedCallback(
    (val: string) => invoke("note_save", { path: note.path, content: val }),
    500,
  );

  return (
    <div
      style={{
        display: "grid",
        gridTemplateColumns: mode === "split" ? "1fr 1fr" : "1fr",
        height: "100%",
      }}
    >
      {(mode === "editor" || mode === "split") && (
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
      {(mode === "preview" || mode === "split") && (
        <Preview content={content} />
      )}
    </div>
  );
}

import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import type {
  VaultInfo,
  IndexProgressPayload,
  IndexDonePayload,
} from "./types/ipc";

type AppStatus = "idle" | "indexing" | "ready";

function App() {
  const [status, setStatus] = useState<AppStatus>("idle");
  const [vault, setVault] = useState<VaultInfo | null>(null);
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const unlistenProgress = listen<IndexProgressPayload>(
      "vault_index_progress",
      (event) => setProgress(event.payload.pct),
    );

    const unlistenDone = listen<IndexDonePayload>(
      "vault_index_done",
      (event) => {
        setVault((prev) =>
          prev ? { ...prev, noteCount: event.payload.noteCount } : null,
        );
        setStatus("ready");
      },
    );

    return () => {
      unlistenProgress.then((f) => f());
      unlistenDone.then((f) => f());
    };
  }, []);

  async function handleOpenVault() {
    const selected = await open({ directory: true, multiple: false });
    if (!selected || Array.isArray(selected)) return;

    setError(null);
    setStatus("indexing");
    setProgress(0);

    try {
      const info = await invoke<VaultInfo>("vault_open", { path: selected });
      setVault(info);
    } catch (err) {
      setError(String(err));
      setStatus("idle");
    }
  }

  if (status === "idle") {
    return (
      <main className="flex flex-col items-center justify-center min-h-screen gap-6">
        <h1 className="text-4xl font-bold tracking-tight">GraphNotes</h1>
        <p className="text-sm opacity-60">
          A local-first, graph-centric knowledge base
        </p>
        <button
          onClick={handleOpenVault}
          className="px-6 py-3 rounded-lg font-medium text-sm"
        >
          Open Vault
        </button>
        {error && (
          <p className="text-sm text-red-500 max-w-sm text-center">{error}</p>
        )}
      </main>
    );
  }

  if (status === "indexing") {
    return (
      <main className="flex flex-col items-center justify-center min-h-screen gap-4">
        <p className="text-sm font-medium">Indexing vault…</p>
        <div className="w-64 h-1.5 rounded-full bg-black/10 overflow-hidden">
          <div
            className="h-full bg-blue-500 transition-all duration-200"
            style={{ width: `${progress}%` }}
          />
        </div>
        <p className="text-xs opacity-50">{progress}%</p>
      </main>
    );
  }

  // ready — placeholder until Phase 1.4 adds the note list sidebar
  return (
    <main className="flex flex-col items-center justify-center min-h-screen gap-3">
      <p className="text-sm font-medium">Vault ready</p>
      <p className="text-xs opacity-50 max-w-sm truncate">{vault?.path}</p>
      <p className="text-xs opacity-50">{vault?.noteCount} notes indexed</p>
    </main>
  );
}

export default App;

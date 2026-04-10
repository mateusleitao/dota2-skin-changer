import type { BackupInfo } from "../services/tauri";

interface BackupManagerProps {
  backups: BackupInfo[];
  isLoading: boolean;
  onRestore: (id: string) => void;
}

export function BackupManager({
  backups,
  isLoading,
  onRestore,
}: BackupManagerProps) {
  if (backups.length === 0) {
    return (
      <div className="rounded-xl bg-[#1a2332] p-5">
        <h2 className="text-lg font-bold mb-2 text-gray-200">Backups</h2>
        <p className="text-sm text-gray-500">No backups available.</p>
      </div>
    );
  }

  return (
    <div className="rounded-xl bg-[#1a2332] p-5">
      <h2 className="text-lg font-bold mb-3 text-gray-200">Backups</h2>
      <div className="space-y-2 max-h-48 overflow-y-auto">
        {backups.map((backup) => (
          <div
            key={backup.id}
            className="flex items-center justify-between px-3 py-2 rounded-lg bg-[#0f1923] text-sm"
          >
            <div className="flex-1 min-w-0">
              <p className="text-gray-300 text-xs">
                {new Date(backup.timestamp).toLocaleString()}
              </p>
              <p className="text-gray-500 text-xs font-mono truncate">
                SHA256: {backup.sha256.substring(0, 16)}...
              </p>
            </div>
            <button
              onClick={() => onRestore(backup.id)}
              disabled={isLoading}
              className="ml-3 px-3 py-1 rounded bg-amber-800 hover:bg-amber-700 transition text-xs font-medium disabled:opacity-50"
            >
              Restore
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}

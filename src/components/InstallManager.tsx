import type { HookStatus } from "../services/tauri";

interface InstallManagerProps {
  status: HookStatus;
  gameDetected: boolean;
  isLoading: boolean;
  onInstall: () => void;
  onUninstall: () => void;
}

export function InstallManager({
  status,
  gameDetected,
  isLoading,
  onInstall,
  onUninstall,
}: InstallManagerProps) {
  const isInstalled =
    typeof status === "object" && "Installed" in status;

  return (
    <div className="rounded-xl bg-[#1a2332] p-5">
      <h2 className="text-lg font-bold mb-3 text-gray-200">Hook Manager</h2>

      <div className="flex gap-3">
        <button
          onClick={onInstall}
          disabled={isLoading || !gameDetected || isInstalled}
          className="flex-1 px-4 py-3 rounded-lg bg-emerald-700 hover:bg-emerald-600 transition text-sm font-bold uppercase tracking-wide disabled:opacity-30 disabled:cursor-not-allowed"
        >
          {isLoading ? "Installing..." : "Install Hook"}
        </button>

        <button
          onClick={onUninstall}
          disabled={isLoading || !isInstalled}
          className="flex-1 px-4 py-3 rounded-lg bg-red-800 hover:bg-red-700 transition text-sm font-bold uppercase tracking-wide disabled:opacity-30 disabled:cursor-not-allowed"
        >
          {isLoading ? "Removing..." : "Uninstall"}
        </button>
      </div>

      {!gameDetected && (
        <p className="text-xs text-gray-500 mt-2">
          Detect Dota 2 installation first.
        </p>
      )}
    </div>
  );
}

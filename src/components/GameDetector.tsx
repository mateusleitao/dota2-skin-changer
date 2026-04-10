import type { GameInfo } from "../services/tauri";

interface GameDetectorProps {
  gameInfo: GameInfo | null;
  isLoading: boolean;
  error: string | null;
  onDetect: () => void;
}

export function GameDetector({
  gameInfo,
  isLoading,
  error,
  onDetect,
}: GameDetectorProps) {
  return (
    <div className="rounded-xl bg-[#1a2332] p-5">
      <h2 className="text-lg font-bold mb-3 text-gray-200">
        Dota 2 Installation
      </h2>

      {gameInfo ? (
        <div className="space-y-2">
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 rounded-full bg-emerald-500" />
            <span className="text-sm text-emerald-400">Detected</span>
          </div>
          <p className="text-xs text-gray-400 font-mono break-all">
            {gameInfo.path}
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          {error ? (
            <p className="text-sm text-red-400">{error}</p>
          ) : (
            <p className="text-sm text-gray-400">
              Click below to auto-detect your Dota 2 installation.
            </p>
          )}
          <button
            onClick={onDetect}
            disabled={isLoading}
            className="px-4 py-2 rounded-lg bg-[#2a3a4d] hover:bg-[#354a61] transition text-sm font-medium disabled:opacity-50"
          >
            {isLoading ? "Detecting..." : "Detect Dota 2"}
          </button>
        </div>
      )}
    </div>
  );
}

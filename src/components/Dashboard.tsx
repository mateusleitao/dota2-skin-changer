import { useCallback, useEffect, useRef } from "react";
import { useAppStore } from "../stores/appStore";
import * as api from "../services/tauri";
import { StatusBar } from "./StatusBar";
import { GameDetector } from "./GameDetector";
import { InstallManager } from "./InstallManager";
import { BackupManager } from "./BackupManager";
import { ItemBrowser } from "./ItemBrowser";

export function Dashboard() {
  const {
    gameInfo,
    hookStatus,
    backups,
    itemCatalog,
    isLoading,
    error,
    setGameInfo,
    setHookStatus,
    setBackups,
    setItemCatalog,
    setLoading,
    setError,
  } = useAppStore();

  const initialized = useRef(false);

  const refreshAll = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const game = await api.detectGame();
      setGameInfo(game);

      const [status, bk, catalog] = await Promise.all([
        api.getHookStatus(),
        api.getBackups(),
        api.getItemCatalog(),
      ]);

      setHookStatus(status);
      setBackups(bk);
      setItemCatalog(catalog);
    } catch {
      // Tauri APIs unavailable in browser/test
    } finally {
      setLoading(false);
    }
  }, [setLoading, setError, setGameInfo, setHookStatus, setBackups, setItemCatalog]);

  useEffect(() => {
    if (!initialized.current) {
      initialized.current = true;
      refreshAll();
    }
  }, [refreshAll]);

  const handleDetect = async () => {
    setLoading(true);
    setError(null);
    try {
      const game = await api.detectGame();
      setGameInfo(game);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const handleInstall = async () => {
    setLoading(true);
    setError(null);
    try {
      await api.installHook();
      await refreshAll();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const handleUninstall = async () => {
    setLoading(true);
    setError(null);
    try {
      await api.uninstallHook();
      await refreshAll();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const handleRestore = async (id: string) => {
    setLoading(true);
    setError(null);
    try {
      await api.restoreBackup(id);
      await refreshAll();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const handleRefreshCatalog = async () => {
    setLoading(true);
    try {
      const catalog = await api.getItemCatalog();
      setItemCatalog(catalog);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col h-screen">
      <header className="flex items-center justify-between px-6 py-4 border-b border-gray-800">
        <h1 className="text-xl font-bold tracking-tight bg-gradient-to-r from-amber-400 to-emerald-400 bg-clip-text text-transparent">
          Dota 2 Skin Changer
        </h1>
        <StatusBar status={hookStatus} />
      </header>

      <main className="flex-1 overflow-y-auto p-6 space-y-4">
        {error && (
          <div className="px-4 py-3 rounded-lg bg-red-900/30 border border-red-800 text-sm text-red-300">
            {error}
          </div>
        )}

        <GameDetector
          gameInfo={gameInfo}
          isLoading={isLoading}
          error={error}
          onDetect={handleDetect}
        />

        <InstallManager
          status={hookStatus}
          gameDetected={gameInfo !== null}
          isLoading={isLoading}
          onInstall={handleInstall}
          onUninstall={handleUninstall}
        />

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <BackupManager
            backups={backups}
            isLoading={isLoading}
            onRestore={handleRestore}
          />
          <ItemBrowser
            catalog={itemCatalog}
            onRefresh={handleRefreshCatalog}
            isLoading={isLoading}
          />
        </div>
      </main>

      <footer className="px-6 py-3 border-t border-gray-800 text-xs text-gray-600 text-center">
        Client-side only. Use at your own risk. Not affiliated with Valve.
      </footer>
    </div>
  );
}

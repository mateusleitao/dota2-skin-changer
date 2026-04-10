import { create } from "zustand";
import type {
  GameInfo,
  HookStatus,
  BackupInfo,
  ItemCatalog,
} from "../services/tauri";

interface AppState {
  gameInfo: GameInfo | null;
  hookStatus: HookStatus;
  backups: BackupInfo[];
  itemCatalog: ItemCatalog | null;
  isLoading: boolean;
  error: string | null;

  setGameInfo: (info: GameInfo | null) => void;
  setHookStatus: (status: HookStatus) => void;
  setBackups: (backups: BackupInfo[]) => void;
  setItemCatalog: (catalog: ItemCatalog | null) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useAppStore = create<AppState>((set) => ({
  gameInfo: null,
  hookStatus: "NotInstalled",
  backups: [],
  itemCatalog: null,
  isLoading: false,
  error: null,

  setGameInfo: (info) => set({ gameInfo: info }),
  setHookStatus: (status) => set({ hookStatus: status }),
  setBackups: (backups) => set({ backups }),
  setItemCatalog: (catalog) => set({ itemCatalog: catalog }),
  setLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),
}));

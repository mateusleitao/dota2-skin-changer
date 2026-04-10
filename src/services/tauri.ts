import { invoke } from "@tauri-apps/api/core";

export interface GameInfo {
  path: string;
  vpk_path: string;
  steam_api_path: string;
  version: string | null;
}

export type HookStatus =
  | "NotInstalled"
  | { Installed: { version: string; item_count: number } }
  | { Error: string };

export interface InstallResult {
  success: boolean;
  item_count: number;
  backup_id: string;
}

export interface BackupInfo {
  id: string;
  timestamp: string;
  sha256: string;
  app_version: string;
}

export interface HeroCosmeticSummary {
  hero_name: string;
  item_count: number;
  slots: string[];
}

export interface ItemCatalog {
  total_items: number;
  heroes: HeroCosmeticSummary[];
}

export async function detectGame(): Promise<GameInfo> {
  return invoke<GameInfo>("detect_game");
}

export async function getHookStatus(): Promise<HookStatus> {
  return invoke<HookStatus>("get_hook_status");
}

export async function installHook(): Promise<InstallResult> {
  return invoke<InstallResult>("install_hook");
}

export async function uninstallHook(): Promise<void> {
  return invoke<void>("uninstall_hook");
}

export async function getBackups(): Promise<BackupInfo[]> {
  return invoke<BackupInfo[]>("get_backups");
}

export async function restoreBackup(id: string): Promise<void> {
  return invoke<void>("restore_backup", { id });
}

export async function getItemCatalog(): Promise<ItemCatalog> {
  return invoke<ItemCatalog>("get_item_catalog");
}

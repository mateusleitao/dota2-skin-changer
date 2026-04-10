import type { HookStatus } from "../services/tauri";

interface StatusBarProps {
  status: HookStatus;
}

function getStatusInfo(status: HookStatus): {
  label: string;
  color: string;
  detail?: string;
} {
  if (status === "NotInstalled") {
    return { label: "Not Installed", color: "bg-gray-600" };
  }
  if (typeof status === "object" && "Installed" in status) {
    return {
      label: "Hook Active",
      color: "bg-emerald-600",
      detail: `v${status.Installed.version} — ${status.Installed.item_count.toLocaleString()} items`,
    };
  }
  if (typeof status === "object" && "Error" in status) {
    return { label: "Error", color: "bg-red-600", detail: status.Error };
  }
  return { label: "Unknown", color: "bg-gray-600" };
}

export function StatusBar({ status }: StatusBarProps) {
  const info = getStatusInfo(status);

  return (
    <div className="flex items-center gap-3 px-4 py-2 rounded-lg bg-[#1a2332]">
      <div className={`w-3 h-3 rounded-full ${info.color}`} />
      <span className="font-semibold text-sm">{info.label}</span>
      {info.detail && (
        <span className="text-xs text-gray-400">{info.detail}</span>
      )}
    </div>
  );
}

import { describe, it, expect } from "vitest";
import { useAppStore } from "../stores/appStore";

describe("appStore", () => {
  it("has correct initial state", () => {
    const state = useAppStore.getState();
    expect(state.gameInfo).toBeNull();
    expect(state.hookStatus).toBe("NotInstalled");
    expect(state.backups).toEqual([]);
    expect(state.itemCatalog).toBeNull();
    expect(state.isLoading).toBe(false);
    expect(state.error).toBeNull();
  });

  it("updates gameInfo", () => {
    const info = {
      path: "C:\\test",
      vpk_path: "C:\\test\\vpk",
      steam_api_path: "C:\\test\\api",
      version: null,
    };
    useAppStore.getState().setGameInfo(info);
    expect(useAppStore.getState().gameInfo).toEqual(info);
  });

  it("updates hookStatus", () => {
    useAppStore
      .getState()
      .setHookStatus({ Installed: { version: "0.1.0", item_count: 100 } });
    const status = useAppStore.getState().hookStatus;
    expect(typeof status).toBe("object");
    if (typeof status === "object" && "Installed" in status) {
      expect(status.Installed.version).toBe("0.1.0");
      expect(status.Installed.item_count).toBe(100);
    }
  });

  it("updates error", () => {
    useAppStore.getState().setError("something went wrong");
    expect(useAppStore.getState().error).toBe("something went wrong");

    useAppStore.getState().setError(null);
    expect(useAppStore.getState().error).toBeNull();
  });

  it("updates backups", () => {
    const backups = [
      {
        id: "abc",
        timestamp: "2026-04-09T00:00:00Z",
        sha256: "deadbeef",
        app_version: "0.1.0",
      },
    ];
    useAppStore.getState().setBackups(backups);
    expect(useAppStore.getState().backups).toHaveLength(1);
    expect(useAppStore.getState().backups[0].id).toBe("abc");
  });
});

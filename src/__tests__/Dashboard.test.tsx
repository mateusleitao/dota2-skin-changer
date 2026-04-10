import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { Dashboard } from "../components/Dashboard";

vi.mock("../services/tauri", () => ({
  detectGame: vi.fn().mockRejectedValue(new Error("Not in Tauri")),
  getHookStatus: vi.fn().mockRejectedValue(new Error("Not in Tauri")),
  getBackups: vi.fn().mockRejectedValue(new Error("Not in Tauri")),
  getItemCatalog: vi.fn().mockRejectedValue(new Error("Not in Tauri")),
  installHook: vi.fn(),
  uninstallHook: vi.fn(),
  restoreBackup: vi.fn(),
}));

describe("Dashboard", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders the title", () => {
    render(<Dashboard />);
    expect(screen.getByText("Dota 2 Skin Changer")).toBeDefined();
  });

  it("renders the install manager section", () => {
    render(<Dashboard />);
    expect(screen.getByText("Hook Manager")).toBeDefined();
  });

  it("renders footer disclaimer", () => {
    render(<Dashboard />);
    expect(
      screen.getByText(/Client-side only.*Not affiliated with Valve/),
    ).toBeDefined();
  });
});

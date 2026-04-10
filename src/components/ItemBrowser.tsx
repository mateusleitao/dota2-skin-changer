import type { ItemCatalog } from "../services/tauri";

interface ItemBrowserProps {
  catalog: ItemCatalog | null;
  onRefresh: () => void;
  isLoading: boolean;
}

export function ItemBrowser({
  catalog,
  onRefresh,
  isLoading,
}: ItemBrowserProps) {
  return (
    <div className="rounded-xl bg-[#1a2332] p-5">
      <div className="flex items-center justify-between mb-3">
        <h2 className="text-lg font-bold text-gray-200">Item Database</h2>
        <button
          onClick={onRefresh}
          disabled={isLoading}
          className="px-3 py-1 rounded bg-[#2a3a4d] hover:bg-[#354a61] transition text-xs font-medium disabled:opacity-50"
        >
          {isLoading ? "Loading..." : "Refresh"}
        </button>
      </div>

      {catalog ? (
        <div>
          <p className="text-sm text-emerald-400 mb-3">
            {catalog.total_items.toLocaleString()} cosmetics loaded
          </p>
          <div className="grid grid-cols-2 gap-2 max-h-64 overflow-y-auto">
            {catalog.heroes
              .sort((a, b) => b.item_count - a.item_count)
              .slice(0, 20)
              .map((hero) => (
                <div
                  key={hero.hero_name}
                  className="px-3 py-2 rounded bg-[#0f1923] text-xs"
                >
                  <p className="text-gray-300 font-medium truncate">
                    {hero.hero_name.replace("npc_dota_hero_", "")}
                  </p>
                  <p className="text-gray-500">
                    {hero.item_count} items &middot; {hero.slots.length} slots
                  </p>
                </div>
              ))}
          </div>
        </div>
      ) : (
        <p className="text-sm text-gray-500">
          Install the hook to generate the item database.
        </p>
      )}
    </div>
  );
}

// Shared module for fetching and parsing game data
import init, { get_tikkers_url, get_getikt_url, get_config_url, parse_game_data } from './punten_telling_tik_spel/pkg/punten_telling_tik_spel.js';

let gameData = null;
let isInitialized = false;

/**
 * Initialize WASM and fetch/parse game data
 * @returns {Promise<Object>} Parsed game data with teams and tikkers
 */
export async function getGameData() {
  // Return cached data if already loaded
  if (gameData) {
    return gameData;
  }

  // Initialize WASM if not done yet
  if (!isInitialized) {
    await init();
    isInitialized = true;
  }

  // Get the Google Sheet URLs from WASM
  const tikkersUrl = get_tikkers_url();
  const getiktUrl = get_getikt_url();
  const configUrl = get_config_url();

  // Fetch all CSV files in parallel
  const [tikkersResponse, getiktResponse, configResponse] = await Promise.all([
    fetch(tikkersUrl),
    fetch(getiktUrl),
    fetch(configUrl)
  ]);

  // Check responses
  if (!tikkersResponse.ok || !getiktResponse.ok || !configResponse.ok) {
    throw new Error(
      `Failed to fetch CSV files: ` +
      `Tikkers ${tikkersResponse.status}, ` +
      `Getikt ${getiktResponse.status}, ` +
      `Config ${configResponse.status}`
    );
  }

  // Get CSV text
  const tikkersCsv = await tikkersResponse.text();
  const getiktCsv = await getiktResponse.text();
  const configCsv = await configResponse.text();

  // Parse using WASM
  gameData = parse_game_data(tikkersCsv, getiktCsv, configCsv);

  return gameData;
}

/**
 * Clear cached data (useful for refresh)
 */
export function clearCache() {
  gameData = null;
}

/**
 * Refresh data by clearing cache and fetching again
 * @returns {Promise<Object>} Fresh game data
 */
export async function refreshGameData() {
  clearCache();
  return getGameData();
}

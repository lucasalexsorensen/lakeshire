import { getAllWindows } from "@tauri-apps/api/window";

export async function initialize() {
  const allWindows = await getAllWindows();
  for (const window of allWindows) {
    // await window.setIgnoreCursorEvents(true);
    await window.setVisibleOnAllWorkspaces(true);
    await window.setAlwaysOnTop(true);
  }
}

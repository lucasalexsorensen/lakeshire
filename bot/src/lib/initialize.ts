import { getAllWindows } from "@tauri-apps/api/window";
import { unregisterAll } from "@tauri-apps/plugin-global-shortcut";
import { attachConsole } from "@tauri-apps/plugin-log";
import { registerShortcuts } from "./shortcuts";

export function initialize(): () => void {
  const detachPromise = attachConsole();
  initializeWindows();
  registerShortcuts();

  const uninitialize = () => {
    detachPromise.then((detach) => {
      detach();
    });
    unregisterAll();
  };

  return uninitialize;
}

async function initializeWindows() {
  const allWindows = await getAllWindows();
  for (const window of allWindows) {
    // await window.setIgnoreCursorEvents(true);
    // await window.setVisibleOnAllWorkspaces(true);
    await window.setAlwaysOnTop(true);
  }
}

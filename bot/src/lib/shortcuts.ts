import { register } from "@tauri-apps/plugin-global-shortcut";
import { getAllWindows } from "@tauri-apps/api/window";

export async function registerShortcuts() {
  await register("CommandOrControl+Shift+L", async (event) => {
    if (event.state !== "Pressed") return;

    const allWindows = await getAllWindows();
    const mainWindow = allWindows[0];

    const isFocused = await mainWindow.isFocused();
    if (isFocused) {
      await mainWindow.hide();
    } else {
      await mainWindow.show();
      await mainWindow.setFocus();
    }
  });
}

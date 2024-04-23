import { invoke } from "@tauri-apps/api";

const handleOpenUrl = (url) => {
    invoke("openurl", { url });
    // Browser (Pure Vue Preview) Support
    if (window.__TAURI_IPC__ === ignoreTauriIPC) {
        window.open(url, "_blank");
    }
};

const ignoreTauriIPC = (kwargs) => {
    console.warn("Tauri IPC called with args:", kwargs ? kwargs : "(No args)");
};

export { handleOpenUrl, ignoreTauriIPC };

import { invoke } from "@tauri-apps/api";

/**
 * 打开链接
 * @param {String} url - 要打开的链接
 */
const handleOpenUrl = (url) => {
    invoke("openurl", { url });
    // Browser (Pure Vue Preview) Support
    if (window.__TAURI_IPC__ === ignoreTauriIPC) {
        window.open(url, "_blank");
    }
};

/**
 * 当仅以纯 Vue 预览模式运行时，忽略 Tauri IPC 调用，并打印调试信息
 * @param {*} kwargs - IPC 调用参数
 */
const ignoreTauriIPC = (kwargs) => {
    window.__TAURI_IPC__ = console.log("Tauri IPC called with args:", kwargs ? kwargs : "(No args)");
};

export { handleOpenUrl, ignoreTauriIPC };

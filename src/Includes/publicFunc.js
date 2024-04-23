import { invoke } from "@tauri-apps/api";

const handleOpenUrl = (url) => {
    invoke("openurl", { url });
    // Browser (Pure Vue Preview) Support
    window.open(url, "_blank");
};

export { handleOpenUrl };

import { invoke } from "@tauri-apps/api";

const handleOpenUrl = (url) => {
    invoke('openurl', { url })
}

export { handleOpenUrl }
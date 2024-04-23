import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
// Plugins
import { createPinia } from "pinia";
import router from "./router.js";

const app = createApp(App);
// Load plugins
const pinia = createPinia();
app.use(pinia);
app.use(router);
// Mount
app.mount("#app");

window.onload = () => {
    if (window.__TAURI_IPC__ == void 0) {
        console.warn(
            "Could not find window.__TAURI_IPC__, are you running in a Tauri app?\n",
            "You may find some error if running this frontend on a browser instead of a Tauri window. Or else your app has some problem.\n",
            "Now set window.__TAURI_IPC__ to an empty function to avoid this warning.\n",
        );
        window.__TAURI_IPC__ = (kwargs) => {
            console.warn("Tauri IPC called with args:", kwargs);
        }
    }
};

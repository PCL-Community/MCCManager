import { createApp } from "vue";
import "./styles.css"
import App from "./App.vue";
// Plugins
import { createPinia } from "pinia"
import router from "./router.js";

const app = createApp(App)
// Load plugins
const pinia = createPinia()
app.use(pinia)
app.use(router)
// Mount
app.mount("#app");

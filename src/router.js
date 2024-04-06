import { createRouter, createWebHistory } from "vue-router";

// Components
import Home from "./pages/Home.vue";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            // For first-time loading
            path: "/index.html",
            name: "Home-FirstTime",
            component: Home,
        },
        {
            path: "/Home",
            name: "Home",
            component: Home,
        },
    ],
});

export default router
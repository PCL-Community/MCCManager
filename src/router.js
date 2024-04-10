import { createRouter, createWebHistory } from "vue-router";
import { invoke } from "@tauri-apps/api";

// Components
import Home from "./pages/Home.vue";
import Settings from "./pages/Settings.vue";

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
            // For Pure Vue-Viewing
            path: "/",
            name: "Home-Root",
            component: Home,
        },
        {
            path: "/Home",
            name: "Home",
            component: Home,
        },
        {
            path: "/Settings",
            name: "Settings",
            component: Settings,
        },
    ],
});

const homeList = ["/", "/index.html", "/Home"];
const enlargeList = ["/Settings"];

router.beforeEach((to, from, next) => {
    const ctn = document.querySelector("main#container");
    const lb = document.querySelector("div#buttomBtn");
    ctn.classList.add("fadeOut");
    if (!homeList.includes(to.fullPath)) {
        lb.classList.add("fadeOut");
        setTimeout(() => {
            lb.style.transform = "translateY(260px)";
        }, 500);
    }
    if (enlargeList.includes(to.fullPath)) {
        invoke("enlargewindow");
    }
    if (enlargeList.includes(from.fullPath)) {
        invoke("shrinkwindow");
    }
    setTimeout(() => {
        next();
    }, 700);
});

router.afterEach((to, _from) => {
    const ctn = document.querySelector("main#container");
    ctn.classList.remove("fadeOut");
    const lb = document.querySelector("div#buttomBtn");
    lb.style.display = "none";
    if (homeList.includes(to.fullPath)) {
        lb.style.display = "flex";
        lb.classList.remove("fadeOut");
        setTimeout(() => {
            lb.style.transform = "none";
        }, 500);
    }
});

export default router;

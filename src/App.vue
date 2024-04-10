<script setup>
import { nextTick, ref, watch } from "vue";
import { RouterView } from "vue-router";
// Comps
import CustomTitleBar from "./components/CustomTitleBar.vue";
import LaunchButton from "./components/LaunchButton.vue";
// Stores

const components = {
    CustomTitleBar,
    LaunchButton,
};

const main = ref(null);

const hrefMap = {
    0: "/",
    1: "Settings",
};
</script>

<template>
    <div id="container">
        <CustomTitleBar />
        <main id="container">
            <RouterView />
        </main>
        <div id="buttomBtn">
            <LaunchButton />
        </div>
    </div>
</template>

<style scoped lang="less">
div#container {
    inset: 0;
    overflow: hidden;
    display: grid;
    grid-template-columns: 1fr;
    grid-template-rows: 50px 1fr;
    grid-template-areas: "TitleBar" "Main";
    height: 100%;
    div#TitleBar {
        grid-area: TitleBar;
    }
    main#container {
        width: 451.2px;
        grid-area: Main;
        opacity: 0;
        transition: opacity 0.5s ease-in-out;
        animation: PageFadein 0.5s forwards;
        &.fadeOut {
            animation: PageFadeout 0.5s forwards;
        }
    }
    div#buttomBtn {
        width: 451.2px;
        display: flex;
        justify-content: center;
        transform: translateY(260px);
        animation: LaunchButtonFadein 0.5s forwards;
        &.fadeOut {
            transition: transform 0.5s ease-in-out;
            animation: LaunchButtonFadeout 0.5s forwards;
        }
    }
}
</style>

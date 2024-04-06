<script setup>
import { RouterView } from "vue-router";
import CustomTitleBar from "./components/CustomTitleBar.vue";

const components = {
    CustomTitleBar,
};
</script>

<template>
    <div id="container">
        <CustomTitleBar />
        <main id="container">
            <RouterView />
        </main>
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
    div#TitleBar{
        grid-area: TitleBar;
    }
    main#container{
        grid-area: Main;
    }
}
</style>

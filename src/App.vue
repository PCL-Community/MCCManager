<script setup>
import CustomTitleBar from './components/CustomTitleBar.vue';

const components = {
    CustomTitleBar
}
</script>

<template>
    <div id="container">
        <CustomTitleBar />
    </div>
</template>

<style scoped lang="less">
div#container {
    display: block;
    inset: 0;
    overflow: hidden;
}
</style>

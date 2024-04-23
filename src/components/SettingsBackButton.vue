<script setup>
import IconArrowLeft from "./icons/IconArrowLeft.vue";
</script>

<template>
    <div class="SettingsBackButton">
        <IconArrowLeft :width="25" :height="25" color="var(--title-c)" />
    </div>
</template>

<style scoped lang="less">
div.SettingsBackButton {
    width: 50px;
    height: 50px;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    transition: background-color .3s ease-in-out;
    border-radius: 6px;
    &:hover {
        background-color: var(--slb-bgc-hover);
    }
}
</style>
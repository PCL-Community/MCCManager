<script setup>
import IconNextArrow from "./icons/IconNextArrow.vue";

const name = "SettingsItemButton";
const components = {
    IconNextArrow,
};
</script>

<template>
    <div class="SettingsItemButton">
        <slot name="icon"></slot>
        <span class="SettingsItemButtonText">
            <slot></slot>
        </span>
        <div class="SettingsItemButtonRight">
            <IconNextArrow :width="13" :height="13" color="var(--title-c)" />
        </div>
    </div>
</template>

<style scoped lang="less">
div.SettingsItemButton {
    width: calc(100% - 20px);
    height: 40px;
    display: flex;
    justify-content: right;
    align-items: center;
    gap: 15px;
    padding: 2px 15px;
    cursor: pointer;
    border-radius: 6px;
    border: 1px solid var(--border-c);
    background-color: var(--si-btn-bgc);
    transition: background-color 0.2s ease-in-out;
    &:hover {
        background-color: var(--sib-bg);
    }
    &:active {
        background-color: var(--sib-bg2);
    }
    span.SettingsItemButtonText {
        color: var(--title-c);
        font-size: 0.9em;
        transform: translateY(-1px);
    }
    div.SettingsItemButtonRight {
        margin-left: auto;
        opacity: 0.4;
    }
}
</style>

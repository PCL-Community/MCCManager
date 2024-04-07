<script setup>
import { invoke } from "@tauri-apps/api";
import TitleBarButton from "./TitleBarButton.vue";
import IconClose from "./icons/IconClose.vue";
import IconMinimize from "./icons/IconMinimize.vue";
import { reactive } from "vue";

const name = "CustomTitleBar";
const components = {
    TitleBarButton,
    IconClose,
    IconMinimize,
};

const handleMinimize = () => {
    invoke("minimize");
};
const handleClose = () => {
    invoke("close");
};
</script>

<template>
    <div id="TitleBar" data-tauri-drag-region>
        <div class="leftInfo">
            <span
                style="
                    width: 30px;
                    height: 30px;
                    background-image: url('app-icon.png');
                    background-size: 30px 30px;
                "></span>
            <span class="title">MCCManager</span>
        </div>
        <div class="controlBtns">
            <title-bar-button @click="handleMinimize()">
                <template #icon>
                    <IconMinimize
                        :width="20"
                        :height="20"
                        :stroke-width="2.5"
                        color="var(--title-c)" />
                </template>
            </title-bar-button>
            <title-bar-button @click="handleClose()">
                <template #icon>
                    <IconClose
                        :width="20"
                        :height="20"
                        :stroke-width="3"
                        color="#D03050" />
                </template>
            </title-bar-button>
        </div>
    </div>
</template>

<style scoped lang="less">
div#TitleBar {
    width: calc(100% - calc(12px * 2));
    height: 50px;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    background-color: var(--tb-bgc);
    transition: background-color 0.2s ease-in-out;
    border-radius: var(--window-radius) var(--window-radius) 0 0;
    padding: 0 12px;
    user-select: none;
    &:hover {
        background-color: var(--tb-bgc-hover);
    }
    div.leftInfo {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 8px;
        color: var(--title-c);
    }
    div.controlBtns {
        margin-left: auto;
        display: flex;
        gap: 6px;
    }
}
</style>

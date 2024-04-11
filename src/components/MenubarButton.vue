<script setup>
const name = "MenubarButton";
</script>

<template>
    <div id="MenubarButton">
        <div id="inside">
            <slot name="icon"></slot>
            <div style="transform: translateY(-2px)">
                <slot></slot>
            </div>
        </div>
    </div>
</template>

<style scoped lang="less">
div#MenubarButton {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    border: 1px solid var(--border-c);
    border-radius: 6px;
    transition: background-color 0.2s ease-in-out;
    div#inside {
        display: flex;
        justify-content: center;
        align-items: center;
        border-radius: 6px;
        cursor: pointer;
        gap: 0 25px;
        padding: 0 17px 0 13px;
    }
    &:hover {
        background-color: var(--mbb-bg);
    }
    &:active {
        transition: background-color 0.1s ease-in-out;
        background-color: var(--mbb-bg2);
    }
}
</style>

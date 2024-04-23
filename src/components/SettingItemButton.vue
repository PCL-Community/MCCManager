<script setup>
</script>

<template>
    <div class="SettingItemButton">
        <slot name="iconbefore"></slot>
        <span class="SettingItemButtonText">
            <slot></slot>
        </span>
        <span class="SettingItemButtonRight">
            <slot name="iconafter"></slot>
        </span>
    </div>
</template>

<style scoped lang="less">
div.SettingItemButton {
    width: calc(100% - 30px);
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
    &[disabled] {
        background-color: var(--sib-bg-disable);
        cursor: not-allowed;
    }
    span.SettingItemButtonText {
        color: var(--title-c);
        font-size: 0.9em;
        transform: translateY(-1px);
    }
    span.SettingItemButtonRight {
        margin-left: auto;
        opacity: 0.4;
        transform: translateY(2px);
    }
}
</style>

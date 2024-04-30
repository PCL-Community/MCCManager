<script setup>
import Account from "./2sAccount.vue"
import MCCConfig from "./2sMCC_Config.vue"
</script>

<template>
    <div id="PageProfile">
        <Account />
        <br />
        <MCCConfig />
    </div>
</template>

<style scoped lang="less">
div#PageProfile {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: 7px 20px;
    transform: translateY(100px);
    transition: transform 0.2s ease-in-out;
    animation: SettingsSubPageFedein 0.2s ease-in-out forwards;
    &.fadeOut {
        transform: none;
        animation: SettingsSubPageFadeout 0.2s ease-in-out forwards;
    }
}
</style>

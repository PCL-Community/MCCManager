<script setup>
import TitleText from "../../components/TitleText.vue";
import NoteText from "../../components/NoteText.vue";
import SettingItemButton from "../../components/SettingItemButton.vue";
</script>

<template>
    <div id="PageProfile">
        <TitleText>App 账户</TitleText>
        <NoteText>咕？还没有这个东东呢！</NoteText>
        <br />
        <TitleText>MCC 实例档案</TitleText>
        <SettingItemButton disabled>
            新增档案
        </SettingItemButton>
        <NoteText>当前版本的 MCCManager 尚不支持创建与管理 MCC 实例档案</NoteText>
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

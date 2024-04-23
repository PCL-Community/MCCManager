<script setup>
import TitleText from "../../components/TitleText.vue";
import NoteText from "../../components/NoteText.vue";
import SettingItemButton from "../../components/SettingItemButton.vue";
// Icons
import IconDownload from "../../components/icons/IconDownload.vue";
</script>

<template>
    <div id="PageCore">
        <NoteText>当前版本的 MCCManager 尚不支持此设置</NoteText>
        <TitleText>核心管理</TitleText>
        <SettingItemButton disabled>
            下载核心
            <template #iconafter>
                <IconDownload :width="26" :height="26" style="transform: translateY(-2px);" />
            </template>
        </SettingItemButton>
    </div>
</template>

<style scoped lang="less">
div#PageCore {
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

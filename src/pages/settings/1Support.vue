<script setup>
// Comps
import { ref } from "vue";
import NoteText from "../../components/NoteText.vue";
// Icons
import IconInfo from "../../components/icons/IconInfo.vue";
</script>

<template>
    <div id="PageSupport">
        <div id="SupportTitleCard">
            <IconInfo :width="40" :height="40" color="var(--title-c)" />
            <br />
            <span id="SupportTitle1">MCCManager 帮助与支持网站</span>
            <span id="SupportTitle2">暂无</span>
        </div>
        <br />
        <NoteText>
            如要了解如果更好的使用 MCCManager
            及其功能，或取得帮助，请通过您的电脑或智能设备访问 MCCManager
            帮助与支持网站
        </NoteText>
        <NoteText>
            您当前使用的是早期预览体验版本的
            MCCManager，大部分功能尚未完工且无法使用
        </NoteText>
    </div>
</template>

<style scoped lang="less">
div#PageSupport {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: 7px 20px;
    transform: translateY(100px);
    transition: transform 0.2s ease-in-out;
    animation: SettingsSubPageFedein 0.2s ease-in-out forwards;
    margin-top: 10px;
    &.fadeOut {
        transform: none;
        animation: SettingsSubPageFadeout 0.2s ease-in-out forwards;
    }
    div#SupportTitleCard {
        width: 100%;
        height: 160px;
        border-radius: 10px;
        background-color: var(--sp1-card-bgc);
        border: 1px solid var(--main-bg);
        box-shadow: 0 0 10px 2px var(--sp1-card-bdc);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        span#SupportTitle1 {
            font-size: 12px;
            margin-bottom: 6px;
        }
        span#SupportTitle2 {
            font-size: 16px;
        }
    }
}
</style>

<script setup>
import TitleText from "../../components/TitleText.vue";
import LittleText from "../../components/LittleText.vue";
import NoteText from "../../components/NoteText.vue";
import SettingItemButton from "../../components/SettingItemButton.vue";
// Icon
import IconShare from "../../components/icons/IconShare.vue";
// Modules
import { handleOpenUrl } from "../../Includes/publicFunc";

const name = "PageDonate";
const components = {
    TitleText,
    LittleText,
    NoteText,
    SettingItemButton,
    // Icon
    IconShare,
};
</script>

<template>
    <div id="PageDonate">
        <TitleText style="margin-top: 0">赞助者计划</TitleText>
        <SettingItemButton @click="handleOpenUrl('https://afdian.net/@lingyunawa')">
            <span id="beforeafd">立即通过</span>
            <span id="afdian"></span>
            <span id="afterafd">爱发电 支持开发者</span>
            <template #iconafter>
                <IconShare :width="20" :height="20"></IconShare>
            </template>
        </SettingItemButton>
        <TitleText>赞助列表</TitleText>
        <LittleText>无论您赞助与否，我们都感谢您使用 MCCManager</LittleText>
        <LittleText
            >如果您通过 爱发电 赞助了 MCCManager，您的 爱发电
            用户名将展示于此</LittleText
        >
        <br />
        <LittleText>此列表按照发电时间自动排序，每六小时自动更新</LittleText>
        <br />
        <NoteText>此版本的 MCCManager 尚不支持加载赞助列表</NoteText>
    </div>
</template>

<style scoped lang="less">
div#PageDonate {
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
    span#beforeafd {
        margin-right: 0.5em;
    }
    span#afdian {
        display: inline-block;
        width: 20px;
        height: 20px;
        background-image: url("../../assets/afdian.ico");
        background-size: 20px 20px;
        position: absolute;
    }
    span#afterafd {
        margin-left: calc(0.25em + 20px);
    }
}
</style>

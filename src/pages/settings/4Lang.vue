<script setup>
import TitleText from "../../components/TitleText.vue";
import NoteText from "../../components/NoteText.vue";
</script>

<template>
    <div id="PageLang">
        <TitleText>切换语言 - Change Language</TitleText>
        <NoteText>当前版本的 MCCManager 尚不支持此设置</NoteText>
        <NoteText>计划于 0.4 版本开启测试</NoteText>
        <br />
        <TitleText>异域计划团队 - Localization Project Teams</TitleText>
        <NoteText>计划于 0.4 版本开启测试，届时将公布加入方式</NoteText>
    </div>
</template>

<style scoped lang="less">
div#PageLang {
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

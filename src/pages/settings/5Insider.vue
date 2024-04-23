<script setup>
import TitleText from "../../components/TitleText.vue";
import LittleText from "../../components/LittleText.vue";
import NoteText from "../../components/NoteText.vue";
</script>

<template>
    <div id="PageInsider">
        <NoteText>当前版本的 MCCManager 尚不支持此设置</NoteText>
        <TitleText>简介</TitleText>
        <LittleText>MCCManager 全新出品！在开发的前期埋下了一些奇奇怪怪的 Bug 是很常见的事情</LittleText>
        <LittleText>现在，你也可以加入我们让 MCCManager 变得更好！</LittleText>
        <LittleText>加入 MCCManager 预览体验计划，抢鲜体验，给予反馈。</LittleText>
        <br>
        <TitleText>配置更新通道</TitleText>
        <LittleText>我们为 MCCManager 引入了分通道更新机制，这意味着您可以更加频繁的获取更新！</LittleText>
    </div>
</template>

<style scoped lang="less">
div#PageInsider {
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

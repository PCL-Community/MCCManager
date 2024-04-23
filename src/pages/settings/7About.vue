<script setup>
import TitleText from "../../components/TitleText.vue";
import LittleText from "../../components/LittleText.vue";
import SettingItemButton from "../../components/SettingItemButton.vue";
import IconShare from "../../components/icons/IconShare.vue";
// Version file
import versionJson from "../../../version.json";

const yearNow = new Date().getFullYear();
let cpYear;
if (yearNow == 2024) {
    cpYear = "2024";
} else {
    cpYear = `2024 - ${yearNow}`;
}
</script>

<template>
    <div id="PageAbout">
        <TitleText>关于</TitleText>
        <LittleText>
            MCCManager 开发团队的全体成员在这里诚挚的感谢您使用 MCCManager！
        </LittleText>
        <br />
        <TitleText>版本信息</TitleText>
        <LittleText>版本号：{{ versionJson.version }}</LittleText>
        <LittleText>
            构建通道：{{ versionJson.channel[0] }} -
            {{ versionJson.channel[1] }}
        </LittleText>
        <LittleText>
            构建日期和序号：{{ versionJson.buildTime }} #{{
                versionJson.buildOrder
            }}
        </LittleText>
        <br />
        <TitleText>开发者</TitleText>
        <LittleText>MCCManager 本体：LingyunAwA</LittleText>
        <br />
        <TitleText>灵感</TitleText>
        <LittleText>
            MCCManager 内使用的部分用户交互界面风格源自 任天堂 (Nintendo)。
        </LittleText>
        <br />
        <TitleText>使用的开源组件</TitleText>
        <LittleText>
            了解 MCCManager 所使用到的开放源代码组件与其许可证协议的副本
        </LittleText>
        <SettingItemButton style="margin-top: 8px">
            知识产权的表述
            <template #iconafter>
                <IconShare :width="20" :height="20"></IconShare>
            </template>
        </SettingItemButton>
        <TitleText>版权</TitleText>
        <LittleText>
            &copy; {{ cpYear }} MCCManager 保留所有权利。All Rights Reserved.
        </LittleText>
    </div>
</template>

<style scoped lang="less">
div#PageAbout {
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

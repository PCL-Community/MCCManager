<script setup>
// Modules
import router from "../router";
import packageJson from "../../package.json";
// Comps
import SettingsBackButton from "../components/SettingsBackButton.vue";
import SettingsItemGroup from "../components/SettingsItemGroup.vue";
import SettingsItemButton from "../components/SettingsItemButton.vue";
// Icons
import IconInfo from "../components/icons/IconInfo.vue";
import IconCore from "../components/icons/IconCore.vue";
import IconProfile from "../components/icons/IconProfile.vue";
import IconLang from "../components/icons/IconLang.vue";
import IconInsider from "../components/icons/IconInsider.vue";
import IconDonate from "../components/icons/IconDonate.vue";
import IconAbout from "../components/icons/IconAbout.vue";

const name = "Settings";
const components = {
    SettingsBackButton,
    SettingsItemGroup,
    SettingsItemButton,
    // Icons
    IconInfo,
    IconCore,
    IconProfile,
    IconLang,
    IconInsider,
    IconDonate,
    IconAbout,
};

const handleBackToHome = () => {
    router.push("/");
};
</script>

<template>
    <div id="Settings">
        <div id="SettingsLeft">
            <div id="sLTop">
                <SettingsBackButton @click="handleBackToHome()" />
                <div id="sLTitle">
                    <span id="sLTitleMain">MCCManager 本体设置</span>
                    <p id="sLTitleSub">
                        当前 App 版本为 v{{ packageJson.version }}
                    </p>
                </div>
            </div>
            <br />
            <div id="sLMain">
                <SettingsItemGroup title="基本">
                    <SettingsItemButton>
                        <template #icon>
                            <IconInfo
                                :width="22"
                                :height="22"
                                color="var(--title-c)" />
                        </template>
                        支持
                    </SettingsItemButton>
                </SettingsItemGroup>
                <SettingsItemGroup title="游戏">
                    <SettingsItemButton>
                        <template #icon>
                            <IconProfile
                                :width="22"
                                :height="22"
                                color="var(--title-c)" />
                        </template>
                        账户与档案
                    </SettingsItemButton>
                    <SettingsItemButton>
                        <template #icon>
                            <IconCore
                                :width="22"
                                :height="22"
                                color="var(--title-c)" />
                        </template>
                        MCC 核心
                    </SettingsItemButton>
                </SettingsItemGroup>
                <SettingsItemGroup title="个性化">
                    <SettingsItemButton>
                        <template #icon>
                            <IconLang
                                :width="22"
                                :height="22"
                                color="var(--title-c)" />
                        </template>
                        语言 / Language
                    </SettingsItemButton>
                </SettingsItemGroup>
                <SettingsItemGroup title="更多">
                    <SettingsItemButton>
                        <template #icon>
                            <IconInsider
                                :width="22"
                                :height="22"
                                color="var(--title-c)" />
                        </template>
                        预览体验计划
                    </SettingsItemButton>
                    <SettingsItemButton>
                        <template #icon>
                            <IconDonate
                                :width="22"
                                :height="22"
                                color="var(--title-c)" />
                        </template>
                        赞助者计划
                    </SettingsItemButton>
                    <SettingsItemButton>
                        <template #icon>
                            <IconAbout
                                :width="21"
                                :height="21"
                                color="var(--title-c)" />
                        </template>
                        关于
                    </SettingsItemButton>
                </SettingsItemGroup>
            </div>
        </div>
        <div id="SettingsMain"></div>
    </div>
</template>

<style scoped lang="less">
div#Settings {
    display: grid;
    grid-template-columns: calc(451.2px - 10px) 1fr;
    grid-template-rows: 1fr;
    grid-template-areas: "left main";
    padding: 10px;
    width: calc(100% - 20px);
    height: calc(100% - 20px);
    position: absolute;
    background-color: var(--p-s-bg);
    div#SettingsLeft {
        grid-area: left;
        div#sLTop {
            display: flex;
            align-items: center;
            gap: 10px;
        }
        div#sLTitle {
            display: block;
            span#sLTitleMain {
                font-size: 20px;
                font-weight: bold;
            }
            p#sLTitleSub {
                font-size: 12px;
                margin: 4px 0 0;
            }
        }
        div#sLMain {
            overflow-y: scroll;
            padding: 10px 20px;
            display: flex;
            flex-direction: column;
            gap: 15px;
        }
    }
    div#SettingsMain {
        grid-area: main;
    }
}
</style>

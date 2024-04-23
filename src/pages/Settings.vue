<script setup>
// Modules
import { reactive } from "vue";
import router from "../router";
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
// SubPages
import PageSupport from "./settings/1Support.vue";
import PageProfile from "./settings/2Profile.vue";
import PageCore from "./settings/3Core.vue";
import PageLang from "./settings/4Lang.vue";
import PageInsider from "./settings/5Insider.vue";
import PageDonate from "./settings/6Donate.vue";
import PageAbout from "./settings/7About.vue";
// Version file
import versionJson from "../../version.json";

const settings = reactive({
    spId: 0,
    sclass: "",
});

const handleBackToHome = () => {
    router.push("/");
};

const handleChangeSettingPage = (newSpId) => {
    if (settings.spId == newSpId) return;
    settings.sclass = "fadeOut";
    setTimeout(() => {
        settings.spId = newSpId;
        settings.sclass = "";
    }, 200);
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
                        当前版本为 {{ versionJson.channel[0] }} v{{ versionJson.version }}
                    </p>
                </div>
            </div>
            <br />
            <div id="sLMain">
                <SettingsItemGroup title="基本">
                    <SettingsItemButton @click="handleChangeSettingPage(1)">
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
                    <SettingsItemButton @click="handleChangeSettingPage(2)">
                        <template #icon>
                            <IconProfile
                                :width="22"
                                :height="22"
                                color="var(--title-c)" />
                        </template>
                        账户与档案
                    </SettingsItemButton>
                    <SettingsItemButton @click="handleChangeSettingPage(3)">
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
                    <SettingsItemButton @click="handleChangeSettingPage(4)">
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
                    <SettingsItemButton @click="handleChangeSettingPage(5)">
                        <template #icon>
                            <IconInsider
                                :width="22"
                                :height="22"
                                color="var(--title-c)" />
                        </template>
                        预览体验计划
                    </SettingsItemButton>
                    <SettingsItemButton @click="handleChangeSettingPage(6)">
                        <template #icon>
                            <IconDonate
                                :width="22"
                                :height="22"
                                color="var(--title-c)" />
                        </template>
                        赞助者计划
                    </SettingsItemButton>
                    <SettingsItemButton @click="handleChangeSettingPage(7)">
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
        <div id="SettingsMain">
            <PageSupport v-if="settings.spId === 1" :class="settings.sclass" />
            <PageProfile v-if="settings.spId === 2" :class="settings.sclass" />
            <PageCore v-if="settings.spId === 3" :class="settings.sclass" />
            <PageLang v-if="settings.spId === 4" :class="settings.sclass" />
            <PageInsider v-if="settings.spId === 5" :class="settings.sclass" />
            <PageDonate v-if="settings.spId === 6" :class="settings.sclass" />
            <PageAbout v-if="settings.spId === 7" :class="settings.sclass" />
        </div>
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
        overflow: hidden;
        padding-top: 60px;
    }
}
</style>

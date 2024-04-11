<script setup>
import { reactive } from "vue";
// Comps
import MenubarButton from "../components/MenubarButton.vue";
import MenubarSquareButton from "../components/MenubarSquareButton.vue";
// Icons
import IconSetting from "../components/icons/IconSetting.vue";
import IconSun from "../components/icons/IconSun.vue";
import IconMoon from "../components/icons/IconMoon.vue";
import IconTask from "../components/icons/IconTask.vue";
import IconDonate from "../components/icons/IconDonate.vue";
// Router
import router from "../router";

const name = "Home";
const components = {
    MenubarButton,
    MenubarSquareButton,
    // Icons
    IconSetting,
    IconSun,
    IconMoon,
    IconTask,
    IconDonate,
};

const theme = reactive({
    value: 0,
    text: "亮色主题",
});

const handleChangeTheme = () => {
    theme.value = theme.value === 0 ? 1 : 0;
    switch (theme.value) {
        case 0:
            document.body.removeAttribute("app-theme");
            theme.text = "亮色主题";
            break;
        case 1:
            document.body.setAttribute("app-theme", "dark");
            theme.text = "暗色主题";
            break;
    }
};

const handleChangePage = (page) => {
    switch (page) {
        case 0:
            router.push("/Settings");
            break;
        case 1:
            break;
        default:
            throw new Error(`Unknown page: ${page}`);
    }
};
</script>

<template>
    <div id="Home">
        <div id="buttom">
            <div id="Menubar">
                <div id="MenubarBorder"></div>
                <div id="MenubarContent">
                    <MenubarSquareButton @click="handleChangePage(0)">
                        <IconSetting
                            :width="32"
                            :height="32"
                            color="var(--title-c)" />
                    </MenubarSquareButton>
                    <MenubarButton @click="handleChangeTheme()">
                        <template #icon>
                            <IconSun
                                :width="28"
                                :height="28"
                                color="var(--title-c)" />
                        </template>
                        <span>{{ theme.text }}</span>
                    </MenubarButton>
                    <MenubarButton>
                        <template #icon>
                            <IconTask
                                :width="28"
                                :height="28"
                                color="var(--title-c)" />
                        </template>
                        <span>实例列表</span>
                    </MenubarButton>
                    <MenubarSquareButton>
                        <IconDonate
                            :width="32"
                            :height="32"
                            color="var(--icon-failed)"
                            :strokeWidth="1" />
                    </MenubarSquareButton>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped lang="less">
div#Home {
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    align-items: center;
    width: calc(451.2px - 16px);
    height: calc(100% - 16px);
    padding: 8px;
    div#buttom {
        width: 100%;
        margin-top: auto;
        display: flex;
        justify-content: center;
        align-items: center;
    }
}

div#Menubar {
    width: 420px;
    height: 55px;
    position: relative;
    div#MenubarBorder {
        width: 100%;
        height: 100%;
        background-color: var(--main-bg);
        border-radius: 4px;
        box-shadow: 5px 5px 10px 5px rgba(0, 0, 0, 0.25);
    }
    div#MenubarContent {
        display: flex;
        align-items: center;
        position: absolute;
        top: 0;
        left: 0;
        gap: 0 6.5px;
        width: calc(100% - 10px);
        height: calc(100% - 10px);
        padding: 5px;
    }
}
</style>

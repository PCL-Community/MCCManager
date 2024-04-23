<script setup>
import { reactive, nextTick, watchEffect } from "vue";
// Comps
import AnimationSpan26 from "./AnimationSpan26.vue";
import LaunchIcon from "./LaunchIcon.vue";
import IconLaunchFinish from "./icons/IconLaunchFinish.vue";
import IconLaunchFailed from "./icons/IconLaunchFailed.vue";

const vars = reactive({
    main: "启动 MCC 实例",
    sub: "右键管理 >",
    state: 0,
});

const bools = reactive({
    showmain: true,
    lock: false,
    inloading: false,
});

const startLaunch = () => {
    if (bools.lock == false) {
        bools.lock = true;
        bools.inloading = true;
        bools.showmain = false;
        nextTick(() => {
            vars.state = 1;
            vars.main = "启动中...";
            bools.showmain = true;
            vars.sub = "不急";
        });
        setTimeout(() => {
            bools.showmain = false;
            nextTick(() => {
                vars.state = 3;
                vars.main = "啊哦 还不支持启动哦";
                bools.showmain = true;
                bools.inloading = false;
                vars.sub = "稍后再试吧";
            });
            setTimeout(() => {
                bools.showmain = false;
                nextTick(() => {
                    vars.state = 0;
                    vars.main = "启动 MCC 实例";
                    bools.showmain = true;
                    vars.sub = "右键管理 >";
                    bools.lock = false;
                });
            }, 3000);
        }, 3000);
    }
};

watchEffect(() => {
    localStorage.setItem("data_335bff7c_launchBtnLock", bools.lock);
});
</script>

<template>
    <div id="LaunchButton">
        <div id="ButtonBorder"></div>
        <span
            id="ButtonMain"
            @click="startLaunch()"
            :class="{ running: bools.inloading }">
            <LaunchIcon v-if="bools.showmain">
                <span
                    v-if="vars.state === 0 || vars.state === 1"
                    id="MCCIcon"></span>
                <IconLaunchFinish
                    v-if="vars.state === 2"
                    :width="60"
                    :height="60"
                    :color="'var(--icon-finish)'"></IconLaunchFinish>
                <IconLaunchFailed
                    v-if="vars.state === 3"
                    :width="60"
                    :height="60"
                    :color="'var(--icon-failed)'"></IconLaunchFailed>
            </LaunchIcon>
            <div id="launchvars">
                <AnimationSpan26 v-if="bools.showmain" :text="vars.main" />
                <span id="launchSubTitle">{{ vars.sub }}</span>
            </div>
        </span>
    </div>
</template>

<style scoped lang="less">
div#LaunchButton {
    width: 400px;
    height: 100px;
    position: relative;
    padding: 10px;
    margin-bottom: 10px;
    div#ButtonBorder {
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
        background-color: var(--main-bg);
        border-radius: 4px;
        box-shadow: 5px 5px 10px 5px rgba(0, 0, 0, 0.25);
    }
    span#ButtonMain {
        width: calc(100% - 10px);
        height: calc(100% - 10px);
        position: absolute;
        top: 5px;
        left: 5px;
        cursor: pointer;
        border-radius: 6px;
        border: none;
        transition: scale 0.05s ease-in-out, backgroud 0.5s ease-in-out;
        display: flex;
        justify-content: center;
        align-items: center;
        background-image: linear-gradient(
            45deg,
            var(--lgc-1) 0%,
            var(--lgc-1) 31.1%,
            var(--lgc-2) 72.1%,
            var(--lgc-2) 100%
        );
        &:active {
            scale: 0.97;
        }
        div#launchvars {
            z-index: 2;
            display: block;
            text-align: left;
            position: relative;
            margin-left: 20px;
            transform: translateY(-4px);
            span#launchTitle {
                font: normal 26px Arial;
                color: var(--title-c);
            }
            span#launchSubTitle {
                font: normal 13px Arial;
                color: var(--title-c);
            }
        }
        &.running {
            animation: MoveingBackground 3s linear infinite;
            background-image: linear-gradient(
                45deg,
                var(--lgc-1) 0%,
                var(--lgc-1) 15.55%,
                var(--lgc-2) 36.05%,
                var(--lgc-2) 50%,
                var(--lgc-2) 65.55%,
                var(--lgc-1) 86.05%,
                var(--lgc-1) 100%,
                var(--lgc-1) 0%
            );
        }
    }
}

span#MCCIcon {
    width: 70px;
    height: 70px;
    display: inline-block;
    background-image: url("../assets/MCC_logo_with_edge.png");
    background-size: 70px 70px;
}
</style>

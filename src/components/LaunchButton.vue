<script setup>
import { reactive, nextTick } from "vue";
import AnimationSpan26 from "./AnimationSpan26.vue";
const name = "LaunchButton";
const components = {
    AnimationSpan26,
};

const text = reactive({
    main: "启动 MCC 实例",
    sub: "右键管理 >",
});

const bools = reactive({
    showmain: true,
    lock: false,
});

const startLaunch = () => {
    if (bools.lock == false) {
        bools.lock = true;
        bools.showmain = false;
        nextTick(() => {
            text.main = "启动中...";
            bools.showmain = true;
            text.sub = "不急";
        });
        setTimeout(() => {
            bools.showmain = false;
            nextTick(() => {
                text.main = "啊哦 还不支持启动哦";
                bools.showmain = true;
                text.sub = "稍后再试吧";
            });
            setTimeout(() => {
                bools.showmain = false;
                nextTick(() => {
                    text.main = "启动 MCC 实例";
                    bools.showmain = true;
                    text.sub = "右键管理 >";
                    bools.lock = false;
                });
            }, 3000);
        }, 10000);
    }
};
</script>

<template>
    <div id="LaunchButton">
        <div id="ButtonBorder"></div>
        <button id="ButtonMain" @click="startLaunch()">
            <div id="content">
                <img
                    src="../assets/MCC_logo_with_edge.png"
                    width="70"
                    height="70" />
                <div id="launchText">
                    <AnimationSpan26 v-if="bools.showmain" :text="text.main" />
                    <span id="launchSubTitle">{{ text.sub }}</span>
                </div>
            </div>
        </button>
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
    button#ButtonMain {
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
            to right,
            rgb(228, 178, 223) 0%,
            rgb(217, 167, 231) 50%,
            rgb(187, 141, 229) 100%
        );
        background-size: 200% 100%;
        body[app-theme="dark"] & {
            background-image: linear-gradient(
                to right,
                rgb(16, 25, 71) 0%,
                rgb(16, 23, 89) 50%,
                rgb(7, 8, 82) 100%
            );
        }
        &:active {
            scale: 0.97;
        }
        div#content {
            backdrop-filter: blur(1.5px);
            display: flex;
            img {
                margin-right: 20px;
            }
            div#launchText {
                display: block;
                text-align: left;
                span#launchTitle {
                    font: normal 26px Arial;
                    color: var(--title-c);
                }
                span#launchSubTitle {
                    font: normal 13px Arial;
                    color: var(--title-c);
                }
            }
        }
        &.running {
            animation: moveingBackground 3s linear infinite;
        }
    }
}
</style>

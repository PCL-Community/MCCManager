<script setup>
import LaunchButton from '../components/LaunchButton.vue';

const name = "Home";
const components = {
    LaunchButton
}
</script>

<template>
    <div id="Home">
        <div id="buttom">
            <LaunchButton />
        </div>
    </div>
</template>

<style scoped lang="less">
div#Home {
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    align-items: center;
    width: calc(100% - 16px);
    height: calc(100% - 16px);
    padding: 8px;
    div#buttom {
        margin-top: auto;
        display: flex;
        justify-content: center;
        align-items: center;
    }
}
</style>

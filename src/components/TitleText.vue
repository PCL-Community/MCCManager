<script setup>
const name = "TitleText";
</script>

<template>
    <span class="TitleText">
        <slot></slot>
    </span>
</template>

<style scoped lang="less">
span.TitleText {
    font-size: 20px;
    font-weight: bold;
    margin: 10px 0;
    width: 100%;
}
</style>

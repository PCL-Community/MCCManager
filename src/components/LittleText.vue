<script setup>
const name = "LittleText";
</script>

<template>
    <span class="NoteText">
        <slot></slot>
    </span>
</template>

<style scoped lang="less">
span.NoteText {
    opacity: 0.7;
    font-size: 12px;
    margin: 2px 0;
    width: 100%;
}
</style>

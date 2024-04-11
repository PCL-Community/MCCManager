<script setup>
const name = "NoteText";
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
    margin: 10px 0;
}
</style>

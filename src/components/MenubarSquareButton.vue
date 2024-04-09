<script setup>
import { ref, onMounted } from "vue";

const name = "MenubarSquareButton";

const msb = ref(null);

onMounted(() => {
    msb.value.style.width = `${msb.value.clientHeight}px`;
});
</script>

<template>
    <span ref="msb" id="MenubarSquareButton">
            <slot></slot>
    </span>
</template>

<style scoped lang="less">
span#MenubarSquareButton {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    border: 1px solid var(--border-c);
    border-radius: 6px;
    transition: background-color 0.2s ease-in-out;
    cursor: pointer;
    &:hover {
        background-color: var(--mbb-bg);
    }
    &:active {
        background-color: var(--mbb-bg2);
    }
}
</style>

<script setup>
const emit = defineEmits(["click"])
</script>

<template>
    <span class="button" @click="$emit('click')">
        <slot name="icon">
            <div class="icon"></div>
        </slot>
    </span>
</template>

<style scoped lang="less">
span.button {
    cursor: pointer;
    width: 50px;
    height: 35px;
    background-color: transparent;
    border: none;
    transition: background-color 0.2s ease-in-out;
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 3px;
    opacity: 1;
    &:hover {
        background-color: var(--tb-btn-bgc-hover);
    }
}
</style>

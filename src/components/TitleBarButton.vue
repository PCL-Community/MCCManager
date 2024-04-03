<script setup>
const name = "TitleBarButton";
const emit = defineEmits(["click"])
</script>

<template>
    <button class="button" @click="$emit('click')">
        <slot name="icon">
            <div class="icon"></div>
        </slot>
    </button>
</template>

<style scoped lang="less">
.button {
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

<script setup>
const name = "LaunchIcon"
const props = defineProps({
})
</script>

<template>
    <span class="LaunchIcon">
        <slot></slot>
    </span>
</template>

<style>
span.LaunchIcon{
    animation: IconWhipIn 0.5s forwards;
    width: 70px;
    height: 70px;
    display: flex;
    justify-content: center;
    align-items: center;
}

@keyframes IconWhipIn {
    0% {
        scale: 1;
    }
    10% {
        scale: 0.9;
    }
    70% {
        scale: 1.1;
    }
    100% {
        scale: 1;
    }
}
</style>
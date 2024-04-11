<script setup>
const props = defineProps({
    width: Number,
    height: Number,
    color: String,
    strokeWidth: Number,
});
</script>

<template>
    <svg
        class="icon"
        viewBox="0 0 1024 1024"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        :width="props.width"
        :height="props.height">
        <path
            d="M671.968 912c-12.288 0-24.576-4.672-33.952-14.048L286.048 545.984a48 48 0 0 1 0-67.872l351.968-352a48 48 0 1 1 67.872 67.872L387.872 512.032l318.016 318.016A48 48 0 0 1 671.968 912z"
            :fill="props.color"></path>
    </svg>
</template>

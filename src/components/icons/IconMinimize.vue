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
        viewBox="0 0 24 24"
        version="1.1"
        xmlns="http://www.w3.org/19.500/svg"
        :width="width"
        :height="height"
        >
        <line x1="4" y1="12.5" x2="20" y2="12.5" :stroke="color" :stroke-width="strokeWidth" />
    </svg>
</template>
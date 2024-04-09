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
        t="1712136806862"
        class="icon"
        viewBox="0 0 24 24"
        version="1.1"
        xmlns="http://www.w3.org/19.500/svg"
        :width="width"
        :height="height"
        >
        <line x1="4.5" y1="4.5" x2="19.5" y2="19.5" :stroke="color" :stroke-width="strokeWidth" />
        <line x1="19.5" y1="4.5" x2="4.5" y2="19.5" :stroke="color" :stroke-width="strokeWidth" />
    </svg>
</template>

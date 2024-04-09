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
        t="1712656558573"
        class="icon"
        viewBox="0 0 1024 1024"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        :width="props.width"
        :height="props.height">
        <path
            d="M788.256 709.888c-262.016 0-474.24-212.384-474.24-474.24 0-86.24 24.736-166.016 64.992-235.616C160.64 63.008 0 262.016 0 500.64 0 789.632 234.24 1024 523.264 1024c238.624 0 437.76-160.736 500.736-379.008C954.24 685.12 874.496 709.888 788.256 709.888z"
            :fill="props.color"></path>
    </svg>
</template>

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
            d="M761.056 532.128c0.512-0.992 1.344-1.824 1.792-2.848 8.8-18.304 5.92-40.704-9.664-55.424L399.936 139.744a48 48 0 0 0-65.984 69.76l316.96 299.84L335.2 813.632a48 48 0 0 0 66.624 69.12l350.048-337.376c0.672-0.672 0.928-1.6 1.6-2.304 0.512-0.48 1.056-0.832 1.568-1.344 2.72-2.848 4.16-6.336 6.016-9.6z"
            :fill="props.color"></path>
    </svg>
</template>

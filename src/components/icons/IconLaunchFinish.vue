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
        t="1712499009794"
        class="icon"
        viewBox="0 0 1024 1024"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        :width="width"
        :height="height">
        <path
            d="M512 0a512 512 0 1 0 512 512A512 512 0 0 0 512 0z m0 955.733A443.733 443.733 0 1 1 955.733 512 443.733 443.733 0 0 1 512 955.733z"
            :fill="color"></path>
        <path
            d="M753.323 318.805L439.637 632.491l-168.96-168.619A34.133 34.133 0 1 0 222.208 512l193.195 193.195a34.133 34.133 0 0 0 48.128 0l337.92-337.92a34.133 34.133 0 1 0-48.128-48.128z"
            :fill="color"></path>
    </svg>
</template>

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
            d="M481.89428 662.579034V421.631895h-60.236785v-60.236785h120.473569v301.183924h60.313585v60.185585h-180.710354V662.527834h60.236785zM511.999872 1023.999744a511.999872 511.999872 0 1 1 511.999872-511.999872 511.564672 511.564672 0 0 1-511.999872 511.999872z m0-60.236785A451.763087 451.763087 0 1 0 60.236785 511.999872 455.065486 455.065486 0 0 0 511.999872 963.762959z m30.105592-722.815819v60.236785h-60.236784V240.94714z"
            :fill="props.color"></path>
    </svg>
</template>

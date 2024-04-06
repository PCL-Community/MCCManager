<template>
    <div class="AnimationSpan">
        <span class="animated-text" ref="textRef">
            <div
                style="
                    height: 32px;
                    display: inline-flex;
                    align-items: flex-end;
                "
                v-for="(char, index) in text">
                <span
                    :key="index"
                    :class="{ whip: animate }"
                    :style="{
                        'animation-delay': index * 0.05 + 's',
                        'min-height': char != 'm' && char != 'c' ? 34.4 : 29.6,
                    }"
                    >{{ char != " " ? char : "&nbsp;" }}</span
                >
            </div>
        </span>
    </div>
</template>

<script setup>
import { onMounted, ref } from "vue";

const name = "AnimationSpan26";
const props = defineProps({
    text: String,
});

const animate = ref(false);

onMounted(() => {
    animate.value = true;
    setTimeout(() => {
        animate.value = false;
    }, 2000);
});
</script>

<style scoped lang="less">
.AnimationSpan {
    min-width: 195px;
    min-height: 40.8px;
    margin-top: 7px;
    margin-bottom: -4px;
}

.animated-text {
    font-size: 26px;
    height: 32px;
}

.whip {
    animation: whipIn 0.5s forwards;
}

@keyframes whipIn {
    0% {
        font-size: 26px;
    }
    50% {
        font-size: 24px;
    }
    100% {
        font-size: 26px;
    }
}
</style>

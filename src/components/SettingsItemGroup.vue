<script setup>
const props = defineProps({
    title: String
});
</script>

<template>
    <div class="SettingsItemGroup">
        <span class="sigTitle">{{ props.title }}</span>
        <div class="SettingsItemGroupContent">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped lang="less">
div.SettingsItemGroup{
    display: flex;
    flex-direction: column;
    gap: 15px;
    span.sigTitle{
        font-weight: bold;
        font-size: 17px;
    }
    div.SettingsItemGroupContent{
        display: flex;
        flex-direction: column;
        gap: 3px;
    }
}
</style>

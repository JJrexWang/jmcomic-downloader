<script setup lang="ts">
import { ref, watch } from 'vue'
import { useStore } from '../../../store.ts'
import { NInput, NInputGroup, NInputGroupLabel, NInputNumber, NRadioButton, NRadioGroup, useMessage } from 'naive-ui'

const store = useStore()

const message = useMessage()

const proxyHost = ref<string>(store.config?.proxyHost ?? '')
const customApiDomain = ref<string>(store.config?.customApiDomain ?? '')

watch([() => store.config?.apiDomainMode, () => store.config?.customApiDomain], () => {
  message.warning('切换线路后可能需要重新登录')
})
</script>

<template>
  <div v-if="store.config !== undefined" class="flex flex-col">
    <span class="font-bold">下载速度</span>
    <div class="flex flex-col gap-1">
      <div class="flex gap-1">
        <n-input-group class="w-35%">
          <n-input-group-label size="small">章节并发数</n-input-group-label>
          <n-input-number
            class="w-full"
            v-model:value="store.config.chapterConcurrency"
            size="small"
            @update:value="message.warning('对章节并发数的修改需要重启才能生效')"
            :min="1"
            :parse="(x: string) => Number(x)" />
        </n-input-group>
        <n-input-group class="w-65%">
          <n-input-group-label size="small">每个章节下载完成后休息</n-input-group-label>
          <n-input-number
            class="w-full"
            v-model:value="store.config.chapterDownloadIntervalSec"
            size="small"
            :min="0"
            :parse="(x: string) => Number(x)" />
          <n-input-group-label size="small">秒</n-input-group-label>
        </n-input-group>
      </div>
      <div class="flex gap-1">
        <n-input-group class="w-35%">
          <n-input-group-label size="small">图片并发数</n-input-group-label>
          <n-input-number
            class="w-full"
            v-model:value="store.config.imgConcurrency"
            size="small"
            @update-value="message.warning('对图片并发数的修改需要重启才能生效')"
            :min="1"
            :parse="(x: string) => Number(x)" />
        </n-input-group>
        <n-input-group class="w-65%">
          <n-input-group-label size="small">每张图片下载完成后休息</n-input-group-label>
          <n-input-number
            class="w-full"
            v-model:value="store.config.imgDownloadIntervalSec"
            size="small"
            :min="0"
            :parse="(x: string) => Number(x)" />
          <n-input-group-label size="small">秒</n-input-group-label>
        </n-input-group>
      </div>
      <n-input-group>
        <n-input-group-label size="small">下载整个收藏夹时，每处理完一个收藏夹中的漫画后休息</n-input-group-label>
        <n-input-number
          class="w-full"
          v-model:value="store.config.downloadAllFavoritesIntervalSec"
          size="small"
          :min="0"
          :parse="(x: string) => Number(x)" />
        <n-input-group-label size="small">秒</n-input-group-label>
      </n-input-group>
      <n-input-group>
        <n-input-group-label size="small">更新库存时，每处理完一个已下载的漫画后休息</n-input-group-label>
        <n-input-number
          class="w-full"
          v-model:value="store.config.updateDownloadedComicsIntervalSec"
          size="small"
          :min="0"
          :parse="(x: string) => Number(x)" />
        <n-input-group-label size="small">秒</n-input-group-label>
      </n-input-group>
    </div>

    <span class="font-bold mt-2">API域名</span>
    <n-radio-group v-model:value="store.config.apiDomainMode" size="small">
      <n-radio-button value="Domain1">线路1</n-radio-button>
      <n-radio-button value="Domain2">线路2</n-radio-button>
      <n-radio-button value="Domain3">线路3</n-radio-button>
      <n-radio-button value="Domain4">线路4</n-radio-button>
      <n-radio-button value="Domain5">线路5</n-radio-button>
      <n-radio-button value="Custom">自定义</n-radio-button>
    </n-radio-group>
    <n-input-group v-if="store.config.apiDomainMode === 'Custom'" class="mt-1">
      <n-input-group-label size="small">自定义API域名</n-input-group-label>
      <n-input
        v-model:value="customApiDomain"
        size="small"
        placeholder=""
        @blur="store.config.customApiDomain = customApiDomain"
        @keydown.enter="store.config.customApiDomain = customApiDomain" />
    </n-input-group>

    <span class="font-bold mt-2">代理类型</span>
    <n-radio-group v-model:value="store.config.proxyMode" size="small">
      <n-radio-button value="System">系统代理</n-radio-button>
      <n-radio-button value="NoProxy">直连</n-radio-button>
      <n-radio-button value="Custom">自定义</n-radio-button>
    </n-radio-group>
    <n-input-group v-if="store.config.proxyMode === 'Custom'" class="mt-1">
      <n-input-group-label size="small">http://</n-input-group-label>
      <n-input
        v-model:value="proxyHost"
        size="small"
        placeholder=""
        @blur="store.config.proxyHost = proxyHost"
        @keydown.enter="store.config.proxyHost = proxyHost" />
      <n-input-group-label size="small">:</n-input-group-label>
      <n-input-number
        v-model:value="store.config.proxyPort"
        size="small"
        placeholder=""
        :parse="(x: string) => parseInt(x)" />
    </n-input-group>
  </div>
</template>

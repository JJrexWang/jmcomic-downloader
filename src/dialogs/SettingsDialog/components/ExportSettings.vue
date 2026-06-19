<script setup lang="ts">
import { useStore } from '../../../store.ts'
import { NCheckbox, NInputGroup, NInputGroupLabel, NInputNumber, NSelect } from 'naive-ui'

const store = useStore()

const exportSkipModeOptions = [
  { label: '不跳过，每次都重新导出', value: 'None' },
  { label: '跳过已存在的文件', value: 'SkipExisting' },
  { label: '跳过曾导出过的章节', value: 'SkipExported' },
]
</script>

<template>
  <div v-if="store.config !== undefined" class="flex flex-col">
    <div class="flex gap-1 items-center">
      <n-input-group class="w-70">
        <n-input-group-label size="small">创建pdf并发数</n-input-group-label>
        <n-input-number
          class="w-full"
          v-model:value="store.config.createPdfConcurrency"
          size="small"
          :min="1"
          :parse="(x: string) => Number(x)" />
      </n-input-group>
      <n-checkbox class="ml-4 w-fit" v-model:checked="store.config.enableMergePdf">创建完成后自动合并</n-checkbox>
    </div>

    <n-input-group class="mt-2">
      <n-input-group-label size="small">导出策略</n-input-group-label>
      <n-select
        v-model:value="store.config.exportSkipMode"
        :options="exportSkipModeOptions"
        size="small"
        class="w-50" />
    </n-input-group>
  </div>
</template>

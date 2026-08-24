<script setup lang="ts">
import { useStore } from '../../../store.ts'
import { NCheckbox, NInputGroup, NInputGroupLabel, NInputNumber, NSelect, NTooltip } from 'naive-ui'

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
      <n-tooltip placement="top" trigger="hover">
        <div>
          <span>在</span>
          <span class="rounded bg-gray-500 px-1">章节详情</span>
          <span>里手动勾选导出的PDF一律不会自动合并</span>
        </div>
        <div>
          <span>只有在</span>
          <span class="rounded bg-gray-500 px-1">本地库存</span>
          <span>里直接导出整部作品为PDF</span>
        </div>
        <div>
          <span>且导出策略不为</span>
          <span class="rounded bg-gray-500 px-1">跳过曾导出过的章节</span>
          <span>时才会触发自动合并</span>
        </div>
        <template #trigger>
          <n-checkbox class="ml-4 w-fit" v-model:checked="store.config.enableMergePdf">创建完成后自动合并</n-checkbox>
        </template>
      </n-tooltip>
    </div>

    <n-tooltip placement="top" trigger="hover">
      <div>
        <span>只影响</span>
        <span class="rounded bg-gray-500 px-1">本地库存</span>
        <span>里直接导出整部作品时的行为</span>
      </div>
      <div>
        <span>在</span>
        <span class="rounded bg-gray-500 px-1">章节详情</span>
        <span>里手动勾选导出时一律以</span>
        <span class="rounded bg-gray-500 px-1">不跳过，每次都重新导出</span>
        <span>处理</span>
      </div>
      <template #trigger>
        <n-input-group class="mt-2 w-fit">
          <n-input-group-label size="small">导出策略</n-input-group-label>
          <n-select
            v-model:value="store.config.exportSkipMode"
            :options="exportSkipModeOptions"
            size="small"
            class="w-50" />
        </n-input-group>
      </template>
    </n-tooltip>
  </div>
</template>

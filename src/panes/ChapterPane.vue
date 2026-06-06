<script setup lang="tsx">
import { PartialSelectionOptions, SelectionArea, SelectionEvent } from '@viselect/vue'
import { computed, defineComponent, nextTick, PropType, ref, watch, watchEffect, useTemplateRef } from 'vue'
import { ChapterInfo, commands, DownloadTaskState } from '../bindings.ts'
import { useStore } from '../store.ts'
import { PhFolderOpen } from '@phosphor-icons/vue'
import { DropdownOption, NButton, NCheckbox, NDropdown, NEmpty } from 'naive-ui'
import IconButton from '../components/IconButton.vue'

const store = useStore()

type State = DownloadTaskState | 'Idle'

const selectionOptions: PartialSelectionOptions = {
  selectables: '.selectable',
  features: { deselectOnBlur: true },
  boundaries: '.chapter-pane-selection-container',
}

const chapterInfos = computed<ChapterInfo[]>(() => store.pickedComic?.chapterInfos ?? [])
const checkedIds = ref<Set<number>>(new Set())
const selectedIds = ref<Set<number>>(new Set())
const selectionAreaRef = useTemplateRef('selectionAreaRef')

watch(
  () => store.pickedComic,
  () => {
    checkedIds.value.clear()
    selectedIds.value.clear()
    selectionAreaRef.value?.selection?.clearSelection()
  },
)

watchEffect(() => {
  if (store.pickedComic === undefined) {
    return
  }

  const selectableChapterIds = new Set(
    chapterInfos.value.filter((chapter) => isChapterSelectable(chapter)).map((chapter) => chapter.chapterId),
  )

  for (const id of checkedIds.value) {
    if (!selectableChapterIds.has(id)) {
      checkedIds.value.delete(id)
    }
  }

  for (const id of selectedIds.value) {
    if (!selectableChapterIds.has(id)) {
      selectedIds.value.delete(id)
    }
  }
})

function extractIds(elements: Element[]): number[] {
  return elements
    .map((element) => element.getAttribute('data-key'))
    .filter(Boolean)
    .map(Number)
    .filter((id) => chapterInfos.value.find((chapter) => chapter.chapterId === id) !== undefined)
}

function unselectAll({ event, selection }: SelectionEvent) {
  if (!event?.ctrlKey && !event?.metaKey) {
    selection.clearSelection()
    selectedIds.value.clear()
  }
}

function updateSelectedIds({
  store: {
    changed: { added, removed },
  },
}: SelectionEvent) {
  extractIds(added).forEach((id) => selectedIds.value.add(id))
  extractIds(removed).forEach((id) => selectedIds.value.delete(id))
}

const dropdownX = ref<number>(0)
const dropdownY = ref<number>(0)
const dropdownShowing = ref<boolean>(false)
const dropdownOptions: DropdownOption[] = [
  {
    label: '勾选',
    key: 'check',
    props: {
      onClick: () => {
        // 只有未勾选的才会被勾选
        selectedIds.value.forEach((id) => checkedIds.value.add(id))
        dropdownShowing.value = false
      },
    },
  },
  {
    label: '取消勾选',
    key: 'uncheck',
    props: {
      onClick: () => {
        selectedIds.value.forEach((id) => checkedIds.value.delete(id))
        dropdownShowing.value = false
      },
    },
  },
  {
    label: '全选',
    key: 'check all',
    props: {
      onClick: () => {
        chapterInfos.value
          .filter((chapter) => isChapterSelectable(chapter))
          .forEach((chapter) => checkedIds.value.add(chapter.chapterId))
        dropdownShowing.value = false
      },
    },
  },
  {
    label: '取消全选',
    key: 'uncheck all',
    props: {
      onClick: () => {
        checkedIds.value.clear()
        dropdownShowing.value = false
      },
    },
  },
]
async function showDropdown(e: MouseEvent) {
  dropdownShowing.value = false
  await nextTick()
  dropdownShowing.value = true
  dropdownX.value = e.clientX
  dropdownY.value = e.clientY
}

async function downloadChapters() {
  if (store.pickedComic === undefined) {
    return
  }
  // 下载勾选的章节
  const chapterIdsToDownload = chapterInfos.value
    .filter((chapter) => isChapterSelectable(chapter) && checkedIds.value.has(chapter.chapterId))
    .map((c) => c.chapterId)
  for (const chapterId of chapterIdsToDownload) {
    // 创建下载任务
    const result = await commands.createDownloadTask(store.pickedComic, chapterId)
    if (result.status === 'error') {
      console.error(result.error)
    }
  }
}

async function refreshChapters() {
  if (store.pickedComic === undefined) {
    return
  }
  const result = await commands.getComic(store.pickedComic.id)
  if (result.status === 'error') {
    console.error(result.error)
    return
  }
  store.pickedComic = result.data
}

async function showComicDownloadDirInFileManager() {
  if (store.pickedComic === undefined) {
    return
  }

  const comicDownloadDir = store.pickedComic.comicDownloadDir
  if (comicDownloadDir === undefined || comicDownloadDir === null) {
    console.error('comicDownloadDir的值为undefined或null')
    return
  }

  const result = await commands.showPathInFileManager(comicDownloadDir)
  if (result.status === 'error') {
    console.error(result.error)
  }
}

function getChapterState(chapterInfo: ChapterInfo): State {
  return store.progresses.get(chapterInfo.chapterId)?.state ?? 'Idle'
}

function isDownloadingChapter(chapter: ChapterInfo) {
  const state = getChapterState(chapter)
  return state === 'Pending' || state === 'Downloading' || state === 'Paused'
}

function isDownloadedChapter(chapterInfo: ChapterInfo) {
  return chapterInfo.isDownloaded === true
}

function isChapterSelectable(chapterInfo: ChapterInfo) {
  return !isDownloadedChapter(chapterInfo) && !isDownloadingChapter(chapterInfo)
}

const ChapterCheckbox = defineComponent({
  name: 'ChapterCheckbox',
  props: {
    chapter: {
      type: Object as PropType<ChapterInfo>,
      required: true,
    },
  },
  setup: (props) => {
    return () => (
      <NCheckbox
        data-key={props.chapter.chapterId}
        class={[
          'hover:bg-gray-200!',
          {
            selectable: isChapterSelectable(props.chapter),
            selected: selectedIds.value.has(props.chapter.chapterId),
            downloaded: isDownloadedChapter(props.chapter),
            downloading: !isDownloadedChapter(props.chapter) && isDownloadingChapter(props.chapter),
          },
        ]}
        checked={checkedIds.value.has(props.chapter.chapterId)}
        onUpdate:checked={(checked: boolean) => {
          if (checked) {
            checkedIds.value.add(props.chapter.chapterId)
          } else {
            checkedIds.value.delete(props.chapter.chapterId)
          }
        }}
        label={props.chapter.chapterTitle}
        disabled={!isChapterSelectable(props.chapter)}
      />
    )
  },
})
</script>

<template>
  <div class="h-full flex flex-col gap-2 box-border">
    <n-empty v-if="store.pickedComic === undefined" description="请先进行漫画搜索" />
    <template v-else>
      <div class="flex items-center select-none pt-2 gap-1 px-2">
        左键拖动进行框选，右键打开菜单
        <n-button class="ml-auto" size="small" @click="refreshChapters">刷新</n-button>
        <n-button size="small" type="primary" @click="downloadChapters">下载勾选章节</n-button>
      </div>

      <SelectionArea
        ref="selectionAreaRef"
        :options="selectionOptions"
        @move="updateSelectedIds"
        @start="unselectAll" />

      <div class="chapter-pane-selection-container flex-1 px-2 pt-0 overflow-auto" @contextmenu="showDropdown">
        <div class="grid grid-cols-3 gap-1.5">
          <ChapterCheckbox v-for="chapter in chapterInfos" :key="chapter.chapterId" :chapter="chapter" />
        </div>
      </div>

      <div class="flex p-2 pt-0">
        <img
          class="w-24 mr-4 object-cover"
          :src="`https://cdn-msp3.18comic.vip/media/albums/${store.pickedComic.id}_3x4.jpg`"
          alt=""
          referrerpolicy="no-referrer" />
        <div class="flex flex-col w-full justify-between">
          <div class="flex flex-col">
            <span class="font-bold text-lg line-clamp-2">{{ store.pickedComic.name }}</span>
            <span class="text-red">作者：{{ store.pickedComic.author }}</span>
            <span class="text-gray">标签：{{ store.pickedComic.tags }}</span>
            <IconButton
              v-if="store.pickedComic.isDownloaded"
              class="w-fit"
              title="打开下载目录"
              @click="showComicDownloadDirInFileManager">
              <PhFolderOpen :size="24" />
            </IconButton>
          </div>
        </div>
      </div>
    </template>

    <n-dropdown
      placement="bottom-start"
      trigger="manual"
      :x="dropdownX"
      :y="dropdownY"
      :options="dropdownOptions"
      :show="dropdownShowing"
      :on-clickoutside="() => (dropdownShowing = false)" />
  </div>
</template>

<style scoped>
.chapter-pane-selection-container {
  @apply select-none overflow-auto;
}

.chapter-pane-selection-container .selected {
  @apply bg-[rgb(204,232,255)] !important;
}

.chapter-pane-selection-container .downloaded {
  @apply bg-[rgba(24,160,88,0.16)];
}

.chapter-pane-selection-container .downloading {
  @apply bg-[rgba(114,46,209,0.16)];
}

:deep(.n-checkbox__label) {
  @apply overflow-hidden whitespace-nowrap text-ellipsis;
}

:global(.selection-area) {
  @apply bg-[rgba(46,115,252,0.5)];
}
</style>

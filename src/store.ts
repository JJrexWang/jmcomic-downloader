import { defineStore } from 'pinia'
import { CurrentTabName, ProgressData, ProgressesPaneTabName } from './types.ts'
import { Comic, Config, GetFavoriteResult, GetUserProfileRespData, GetWeeklyResult, SearchResult } from './bindings.ts'
import { ref } from 'vue'

export const useStore = defineStore('store', () => {
  const config = ref<Config>()
  const userProfile = ref<GetUserProfileRespData>()
  const pickedComic = ref<Comic>()
  const currentTabName = ref<CurrentTabName>('search')
  const progresses = ref<Map<number, ProgressData>>(new Map())
  const getFavoriteResult = ref<GetFavoriteResult>()
  const searchResult = ref<SearchResult>()
  const progressesPaneTabName = ref<ProgressesPaneTabName>('uncompleted')
  const getWeeklyResult = ref<GetWeeklyResult>()
  const downloadedComics = ref<Comic[]>([])

  return {
    config,
    userProfile,
    pickedComic,
    currentTabName,
    progresses,
    getFavoriteResult,
    searchResult,
    progressesPaneTabName,
    getWeeklyResult,
    downloadedComics,
  }
})

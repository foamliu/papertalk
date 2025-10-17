import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useAppStore = defineStore('app', () => {
  // State
  const currentPdf = ref(null)
  const currentPage = ref(1)
  const totalPages = ref(0)
  const zoomLevel = ref(100)
  const isDarkMode = ref(false)
  const selectedText = ref('')
  const translatedText = ref('')
  const translating = ref(false)
  const notes = ref([])
  const ollamaStatus = ref('checking') // 'checking', 'running', 'not-found'

  // Getters
  const hasPdf = computed(() => currentPdf.value !== null)
  const canGoPrev = computed(() => currentPage.value > 1)
  const canGoNext = computed(() => currentPage.value < totalPages.value)
  const hasSelectedText = computed(() => selectedText.value.trim().length > 0)

  // Actions
  const setCurrentPdf = (pdf) => {
    currentPdf.value = pdf
    currentPage.value = 1
  }

  const setTotalPages = (pages) => {
    totalPages.value = pages
  }

  const goToPage = (page) => {
    if (page >= 1 && page <= totalPages.value) {
      currentPage.value = page
    }
  }

  const nextPage = () => {
    if (canGoNext.value) {
      currentPage.value++
    }
  }

  const prevPage = () => {
    if (canGoPrev.value) {
      currentPage.value--
    }
  }

  const setZoomLevel = (zoom) => {
    zoomLevel.value = Math.max(50, Math.min(200, zoom))
  }

  const toggleDarkMode = () => {
    isDarkMode.value = !isDarkMode.value
  }

  const setSelectedText = (text) => {
    selectedText.value = text.trim()
  }

  const setTranslatedText = (text) => {
    translatedText.value = text
  }

  const setTranslating = (status) => {
    translating.value = status
  }

  const addNote = (note) => {
    if (note.trim()) {
      notes.value.push({
        id: Date.now(),
        text: note.trim(),
        page: currentPage.value,
        createdAt: new Date().toISOString()
      })
    }
  }

  const setOllamaStatus = (status) => {
    ollamaStatus.value = status
  }

  return {
    // State
    currentPdf,
    currentPage,
    totalPages,
    zoomLevel,
    isDarkMode,
    selectedText,
    translatedText,
    translating,
    notes,
    ollamaStatus,

    // Getters
    hasPdf,
    canGoPrev,
    canGoNext,
    hasSelectedText,

    // Actions
    setCurrentPdf,
    setTotalPages,
    goToPage,
    nextPage,
    prevPage,
    setZoomLevel,
    toggleDarkMode,
    setSelectedText,
    setTranslatedText,
    setTranslating,
    addNote,
    setOllamaStatus
  }
})

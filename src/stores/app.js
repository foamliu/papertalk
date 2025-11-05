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
  const isStreaming = ref(false)
  const streamingText = ref('')
  
  // 大模型配置
  const modelConfig = ref({
    selectedModel: 'ollama', // ollama, deepseek, kimi
    ollama: {
      baseUrl: 'http://127.0.0.1:11434',
      model: 'qwen3:8b'
    },
    deepseek: {
      apiKey: '',
      baseUrl: 'https://api.deepseek.com',
      model: 'deepseek-chat'
    },
    kimi: {
      apiKey: '',
      baseUrl: 'https://api.moonshot.cn',
      model: 'moonshot-v1-8k'
    }
  })

  // Chat 相关状态
  const activePanel = ref('translation') // translation, chat
  const chatMessages = ref([])
  const currentChatMessage = ref('')
  const isChatting = ref(false)
  const chatStreamingText = ref('')

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

  const setStreaming = (status) => {
    isStreaming.value = status
  }

  const setStreamingText = (text) => {
    streamingText.value = text
  }

  const appendStreamingText = (chunk) => {
    streamingText.value += chunk
  }

  const clearStreamingText = () => {
    streamingText.value = ''
  }

  // 大模型配置相关操作
  const setSelectedModel = (model) => {
    modelConfig.value.selectedModel = model
  }

  const updateModelConfig = (modelType, config) => {
    modelConfig.value[modelType] = { ...modelConfig.value[modelType], ...config }
  }

  const getCurrentModelConfig = () => {
    return modelConfig.value[modelConfig.value.selectedModel]
  }

  // Chat 相关操作
  const setActivePanel = (panel) => {
    activePanel.value = panel
  }

  const addChatMessage = (message) => {
    chatMessages.value.push({
      id: Date.now(),
      role: message.role,
      content: message.content,
      timestamp: new Date().toISOString()
    })
  }

  const setCurrentChatMessage = (message) => {
    currentChatMessage.value = message
  }

  const setIsChatting = (status) => {
    isChatting.value = status
  }

  const setChatStreamingText = (text) => {
    chatStreamingText.value = text
  }

  const appendChatStreamingText = (chunk) => {
    chatStreamingText.value += chunk
  }

  const clearChatStreamingText = () => {
    chatStreamingText.value = ''
  }

  const clearChatMessages = () => {
    chatMessages.value = []
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
    isStreaming,
    streamingText,
    modelConfig,
    activePanel,
    chatMessages,
    currentChatMessage,
    isChatting,
    chatStreamingText,

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
    setOllamaStatus,
    setStreaming,
    setStreamingText,
    appendStreamingText,
    clearStreamingText,
    
    // 大模型配置操作
    setSelectedModel,
    updateModelConfig,
    getCurrentModelConfig,

    // Chat 操作
    setActivePanel,
    addChatMessage,
    setCurrentChatMessage,
    setIsChatting,
    setChatStreamingText,
    appendChatStreamingText,
    clearChatStreamingText,
    clearChatMessages
  }
})

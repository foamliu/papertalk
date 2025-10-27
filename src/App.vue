<template>
  <div id="app" :class="{ 'dark-mode': appStore.isDarkMode }">
    <el-container class="app-container">
      <!-- Header -->
      <el-header class="app-header">
        <div class="header-content">
          <h1 class="app-title">PaperTalk</h1>
          <div class="header-actions">
            <el-button @click="toggleDarkMode" :icon="appStore.isDarkMode ? 'Sunny' : 'Moon'" circle />
            <el-button @click="openFile" type="primary" :icon="FolderOpened">打开PDF</el-button>
          </div>
        </div>
      </el-header>

      <el-container class="main-container">
        <!-- PDF Viewer -->
        <el-main class="pdf-container">
          <div v-if="!appStore.currentPdf" class="welcome-screen">
            <el-empty description="请打开PDF文件开始阅读">
              <el-button type="primary" @click="openFile">打开PDF</el-button>
            </el-empty>
          </div>
          
          <div v-else class="pdf-viewer-wrapper">
            <PdfViewer
              :pdf-url="appStore.currentPdf"
              :current-page="appStore.currentPage"
              :zoom-level="appStore.zoomLevel"
              @update:current-page="appStore.goToPage"
              @update:zoom-level="appStore.setZoomLevel"
              @text-selected="appStore.setSelectedText"
              @page-count-changed="appStore.setTotalPages"
            />
          </div>
        </el-main>

        <!-- Translation Panel -->
        <el-aside class="translation-panel" width="350px">
          <div class="panel-header">
            <h3>翻译</h3>
          </div>
          
          <div class="translation-section">
            <h4>原文</h4>
            <el-input
              v-model="appStore.selectedText"
              type="textarea"
              :rows="8"
              placeholder="选择文本后点击翻译按钮"
              readonly
              class="original-textarea"
            />
            
            <el-button 
              type="primary" 
              @click="translateSelectedText" 
              :loading="appStore.translating"
              :disabled="!appStore.hasSelectedText"
              style="margin-top: 10px;"
            >
              {{ appStore.translating ? '翻译中...' : '翻译' }}
            </el-button>
          </div>

          <div class="translation-result">
            <h4>译文</h4>
            <div v-if="appStore.isStreaming" class="streaming-translation">
              <el-input
                v-model="appStore.streamingText"
                type="textarea"
                :rows="15"
                placeholder="翻译结果将实时显示在这里..."
                readonly
                class="translation-textarea streaming-textarea"
              />
              <div class="streaming-indicator">
                <span class="streaming-dot"></span>
                <span>实时翻译中...</span>
              </div>
            </div>
            <el-input
              v-else
              v-model="appStore.translatedText"
              type="textarea"
              :rows="15"
              placeholder="翻译结果将显示在这里"
              readonly
              class="translation-textarea"
            />
          </div>
        </el-aside>
      </el-container>
    </el-container>

    <!-- Ollama Detection Dialog -->
    <el-dialog
      v-model="showOllamaDialog"
      title="Ollama 未检测到"
      width="500px"
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      :show-close="false"
    >
      <p>PaperTalk 需要 Ollama 来提供本地翻译功能。</p>
      <p>请安装 Ollama 并确保它在后台运行。</p>
      
      <template #footer>
        <el-button @click="openOllamaWebsite" type="primary">
          前往 Ollama 官网下载
        </el-button>
        <el-button @click="retryOllamaCheck">
          重试检测
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { listen } from '@tauri-apps/api/event'
import { FolderOpened, Sunny, Moon } from '@element-plus/icons-vue'
import { useAppStore } from './stores/app'
import PdfViewer from './components/PdfViewer.vue'

// Store
const appStore = useAppStore()

// Local state
const showOllamaDialog = ref(false)

// Methods
const toggleDarkMode = () => {
  appStore.toggleDarkMode()
}

const openFile = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'PDF',
        extensions: ['pdf']
      }]
    })
    
    if (selected) {
      // Use the selected file path directly
      appStore.setCurrentPdf(selected)
    }
  } catch (error) {
    console.error('Failed to open file:', error)
  }
}

const translateSelectedText = async () => {
  if (!appStore.selectedText) return
  
  console.log('开始流式翻译，文本:', appStore.selectedText)
  console.log('当前状态 - translating:', appStore.translating, 'isStreaming:', appStore.isStreaming)
  
  // 使用流式翻译
  appStore.setTranslating(true)
  appStore.setStreaming(true)
  appStore.clearStreamingText()
  
  console.log('设置状态后 - translating:', appStore.translating, 'isStreaming:', appStore.isStreaming)
  
  try {
    console.log('调用后端流式翻译命令...')
    const result = await invoke('translate_text_stream', { text: appStore.selectedText })
    console.log('后端流式翻译命令调用完成，结果:', result)
    console.log('当前 streamingText:', appStore.streamingText)
    
    // 无论如何都显示翻译结果
    if (result) {
      console.log('设置翻译结果:', result)
      appStore.setTranslatedText(result)
      console.log('设置后 translatedText:', appStore.translatedText)
    }
    appStore.setStreaming(false)
    appStore.setTranslating(false)
    console.log('最终状态 - translating:', appStore.translating, 'isStreaming:', appStore.isStreaming)
  } catch (error) {
    console.error('Streaming translation failed:', error)
    appStore.setTranslatedText('翻译失败，请检查 Ollama 是否运行')
    appStore.setStreaming(false)
    appStore.setTranslating(false)
  }
}

// 监听流式翻译事件
const setupEventListeners = async () => {
  console.log('设置流式翻译事件监听器...')
  
  try {
    // 监听翻译片段
    const unlistenChunk = await listen('translation_chunk', (event) => {
      const chunk = event.payload
      console.log('收到翻译片段:', chunk)
      appStore.appendStreamingText(chunk)
    })
    
    // 监听翻译完成
    const unlistenComplete = await listen('translation_complete', (event) => {
      const fullText = event.payload
      console.log('翻译完成，完整文本:', fullText)
      appStore.setTranslatedText(fullText)
      appStore.setStreaming(false)
      appStore.setTranslating(false)
    })
    
    console.log('事件监听器设置成功')
    
    return () => {
      unlistenChunk()
      unlistenComplete()
    }
  } catch (error) {
    console.error('设置事件监听器失败:', error)
    return () => {}
  }
}


const openOllamaWebsite = async () => {
  try {
    await invoke('open_ollama_website')
  } catch (error) {
    console.error('Failed to open website:', error)
  }
}

const retryOllamaCheck = async () => {
  try {
    const isOllamaRunning = await invoke('check_ollama')
    if (isOllamaRunning) {
      showOllamaDialog.value = false
      appStore.setOllamaStatus('running')
    }
  } catch (error) {
    console.error('Ollama check failed:', error)
  }
}

// Lifecycle
let cleanupEventListeners = null

onMounted(async () => {
  console.log('App mounted, setting up event listeners...')
  
  // Check if Ollama is running on app start
  try {
    const isOllamaRunning = await invoke('check_ollama')
    if (!isOllamaRunning) {
      showOllamaDialog.value = true
      appStore.setOllamaStatus('not-found')
    } else {
      appStore.setOllamaStatus('running')
    }
  } catch (error) {
    console.error('Failed to check Ollama:', error)
    showOllamaDialog.value = true
    appStore.setOllamaStatus('not-found')
  }
  
  // Setup event listeners for streaming translation
  cleanupEventListeners = await setupEventListeners()
  console.log('Event listeners setup completed')
})

onUnmounted(() => {
  if (cleanupEventListeners) {
    cleanupEventListeners()
  }
})
</script>

<style scoped>
#app {
  height: 100vh;
  background-color: #f5f5f5;
}

#app.dark-mode {
  background-color: #1a1a1a;
  color: #ffffff;
}

.app-container {
  height: 100vh;
}

.app-header {
  background-color: #ffffff;
  border-bottom: 1px solid #e4e7ed;
  padding: 0 20px;
  height: 60px;
  display: flex;
  align-items: center;
}

.dark-mode .app-header {
  background-color: #2d2d2d;
  border-bottom-color: #434343;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  height: 100%;
}

.app-title {
  margin: 0;
  color: #409eff;
  font-size: 24px;
  font-weight: bold;
}

.header-actions {
  display: flex;
  gap: 10px;
}

.main-container {
  height: calc(100vh - 60px);
  overflow: hidden;
}

.pdf-container {
  padding: 0;
  background-color: #ffffff;
  overflow: auto;
  display: flex;
  flex-direction: column;
}

.dark-mode .pdf-container {
  background-color: #2d2d2d;
}

.welcome-screen {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}

.pdf-viewer-wrapper {
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: 20px;
  overflow: auto;
}

/* 关键修复：确保PDF容器正确居中和对齐 */
.pdf-viewer-wrapper :deep(.pdf-viewer) {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
}

.pdf-viewer-wrapper :deep(.pdf-content) {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  width: 100%;
}

.pdf-viewer-wrapper :deep(.pdf-page-container) {
  display: inline-block;
  position: relative;
  margin: 0 auto;
}

.pdf-viewer-wrapper :deep(.canvas-container) {
  position: relative;
  display: inline-block;
  margin: 0 auto;
}

.pdf-viewer-wrapper :deep(.text-layer) {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: auto;
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
  z-index: 2;
  line-height: 1;
  font-family: sans-serif;
  overflow: visible;
}

.translation-panel {
  background-color: #f8f9fa;
  border-left: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.dark-mode .translation-panel {
  background-color: #2d2d2d;
  border-left-color: #434343;
}

.panel-header {
  padding: 15px 20px;
  border-bottom: 1px solid #e4e7ed;
  background-color: #ffffff;
  flex-shrink: 0;
}

.dark-mode .panel-header {
  background-color: #363636;
  border-bottom-color: #434343;
}

.panel-header h3 {
  margin: 0;
  color: #303133;
}

.dark-mode .panel-header h3 {
  color: #ffffff;
}

.translation-section,
.translation-result {
  padding: 20px;
  border-bottom: 1px solid #e4e7ed;
  flex-shrink: 0;
}

.translation-section {
  flex: 0 0 35%;
  display: flex;
  flex-direction: column;
}

.translation-result {
  flex: 0 0 65%;
  display: flex;
  flex-direction: column;
}

.translation-section h4,
.translation-result h4 {
  margin: 0 0 10px 0;
  color: #606266;
}

.dark-mode .translation-section h4,
.dark-mode .translation-result h4 {
  color: #cccccc;
}

.original-textarea {
  flex: 1;
}

.original-textarea :deep(.el-textarea__inner) {
  height: 100%;
  resize: none;
}

.translation-textarea {
  flex: 1;
}

.translation-textarea :deep(.el-textarea__inner) {
  height: 100%;
  resize: none;
}

/* 流式翻译样式 */
.streaming-translation {
  position: relative;
}

.streaming-textarea {
  margin-bottom: 8px;
}

.streaming-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: #409eff;
  padding: 4px 8px;
  background-color: #f0f9ff;
  border-radius: 4px;
  border: 1px solid #e1f5fe;
}

.dark-mode .streaming-indicator {
  background-color: #1e3a5f;
  border-color: #2d4a6e;
  color: #87ceeb;
}

.streaming-dot {
  width: 8px;
  height: 8px;
  background-color: #409eff;
  border-radius: 50%;
  animation: pulse 1.5s infinite;
}

.dark-mode .streaming-dot {
  background-color: #87ceeb;
}

@keyframes pulse {
  0% {
    transform: scale(0.8);
    opacity: 0.5;
  }
  50% {
    transform: scale(1.2);
    opacity: 1;
  }
  100% {
    transform: scale(0.8);
    opacity: 0.5;
  }
}
</style>

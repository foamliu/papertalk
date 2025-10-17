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
            <h3>翻译 & 笔记</h3>
          </div>
          
          <div class="translation-section">
            <h4>原文</h4>
            <el-input
              v-model="appStore.selectedText"
              type="textarea"
              :rows="3"
              placeholder="选择文本后点击翻译按钮"
              readonly
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
            <el-input
              v-model="appStore.translatedText"
              type="textarea"
              :rows="4"
              placeholder="翻译结果将显示在这里"
              readonly
            />
          </div>

          <div class="notes-section">
            <h4>笔记</h4>
            <el-input
              v-model="currentNote"
              type="textarea"
              :rows="3"
              placeholder="添加笔记..."
              @blur="saveNote"
            />
            
            <div class="notes-list" v-if="appStore.notes.length > 0">
              <h5>历史笔记</h5>
              <div 
                v-for="note in appStore.notes" 
                :key="note.id"
                class="note-item"
              >
                <div class="note-text">{{ note.text }}</div>
                <div class="note-meta">第 {{ note.page }} 页</div>
              </div>
            </div>
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
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { FolderOpened, Sunny, Moon } from '@element-plus/icons-vue'
import { useAppStore } from './stores/app'
import PdfViewer from './components/PdfViewer.vue'

// Store
const appStore = useAppStore()

// Local state
const currentNote = ref('')
const showOllamaDialog = ref(false)

// Methods
const toggleDarkMode = () => {
  appStore.toggleDarkMode()
}

const openFile = async () => {
  try {
    const filePath = await invoke('open_file_dialog')
    if (filePath) {
      // For Tauri apps, we need to use the file path directly
      // PDF.js should be able to handle local file paths
      appStore.setCurrentPdf(filePath)
    }
  } catch (error) {
    console.error('Failed to open file:', error)
  }
}

const translateSelectedText = async () => {
  if (!appStore.selectedText) return
  
  appStore.setTranslating(true)
  try {
    const result = await invoke('translate_text', { text: appStore.selectedText })
    appStore.setTranslatedText(result)
  } catch (error) {
    console.error('Translation failed:', error)
    appStore.setTranslatedText('翻译失败，请检查 Ollama 是否运行')
  } finally {
    appStore.setTranslating(false)
  }
}

const saveNote = () => {
  if (currentNote.value.trim()) {
    appStore.addNote(currentNote.value)
    currentNote.value = ''
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
onMounted(async () => {
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
}

.dark-mode .app-header {
  background-color: #2d2d2d;
  border-bottom-color: #434343;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
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
}

.pdf-container {
  padding: 0;
  background-color: #ffffff;
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
}

.translation-panel {
  background-color: #f8f9fa;
  border-left: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
}

.dark-mode .translation-panel {
  background-color: #2d2d2d;
  border-left-color: #434343;
}

.panel-header {
  padding: 15px 20px;
  border-bottom: 1px solid #e4e7ed;
  background-color: #ffffff;
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
.translation-result,
.notes-section {
  padding: 20px;
  border-bottom: 1px solid #e4e7ed;
}

.translation-section h4,
.translation-result h4,
.notes-section h4 {
  margin: 0 0 10px 0;
  color: #606266;
}

.dark-mode .translation-section h4,
.dark-mode .translation-result h4,
.dark-mode .notes-section h4 {
  color: #cccccc;
}

.notes-section {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.notes-section :deep(.el-textarea) {
  flex: 1;
}

.notes-section :deep(.el-textarea__inner) {
  height: 100%;
}

.notes-list {
  margin-top: 20px;
}

.notes-list h5 {
  margin: 0 0 10px 0;
  color: #909399;
  font-size: 14px;
}

.dark-mode .notes-list h5 {
  color: #999;
}

.note-item {
  padding: 8px 12px;
  margin-bottom: 8px;
  background-color: #f0f2f5;
  border-radius: 4px;
  border-left: 3px solid #409eff;
}

.dark-mode .note-item {
  background-color: #363636;
}

.note-text {
  font-size: 14px;
  line-height: 1.4;
  margin-bottom: 4px;
}

.note-meta {
  font-size: 12px;
  color: #909399;
}

.dark-mode .note-meta {
  color: #999;
}
</style>

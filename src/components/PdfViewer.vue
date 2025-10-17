<template>
  <div class="pdf-viewer">
    <div class="pdf-controls">
      <el-button-group>
        <el-button @click="prevPage" :disabled="!canGoPrev || loading" :icon="ArrowLeft" />
        <el-button>{{ currentPage }} / {{ totalPages }}</el-button>
        <el-button @click="nextPage" :disabled="!canGoNext || loading" :icon="ArrowRight" />
      </el-button-group>
      
      <el-slider
        :model-value="zoomLevel"
        :min="50"
        :max="200"
        :step="10"
        style="width: 200px; margin: 0 20px;"
        show-stops
        @update:model-value="(value) => $emit('update:zoomLevel', value)"
      />
      
      <span>{{ zoomLevel }}%</span>
    </div>
    
    <div class="pdf-content" @mouseup="handleTextSelection">
      <div v-if="loading" class="loading-state">
        <el-icon class="is-loading" :size="32"><Loading /></el-icon>
        <p>正在加载PDF...</p>
      </div>
      
      <div v-else-if="error" class="error-state">
        <el-icon :size="32"><Warning /></el-icon>
        <p>PDF加载失败: {{ error }}</p>
        <el-button @click="retryLoad" type="primary">重试</el-button>
      </div>
      
      <div v-else-if="!pdfDoc" class="empty-state">
        <p>请选择PDF文件</p>
      </div>
      
      <div v-else class="pdf-page-container" :style="{ transform: `scale(${zoomLevel / 100})`, transformOrigin: 'top left' }">
        <canvas 
          ref="canvasRef" 
          class="pdf-canvas"
          @click="handleCanvasClick"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch, onUnmounted } from 'vue'
import * as pdfjsLib from 'pdfjs-dist'
import { ArrowLeft, ArrowRight, Loading, Warning } from '@element-plus/icons-vue'

// PDF.js worker setup
pdfjsLib.GlobalWorkerOptions.workerSrc = new URL(
  'pdfjs-dist/build/pdf.worker.min.js',
  import.meta.url
).toString()

// Props
const props = defineProps({
  pdfUrl: {
    type: String,
    default: null
  },
  currentPage: {
    type: Number,
    default: 1
  },
  zoomLevel: {
    type: Number,
    default: 100
  }
})

// Emits
const emit = defineEmits(['update:currentPage', 'update:zoomLevel', 'textSelected', 'pageCountChanged'])

// Refs
const canvasRef = ref(null)
const pdfDoc = ref(null)
const totalPages = ref(0)
const loading = ref(false)
const error = ref('')

// Computed
const canGoPrev = computed(() => props.currentPage > 1)
const canGoNext = computed(() => props.currentPage < totalPages.value)

// Methods
const loadPdf = async () => {
  if (!props.pdfUrl) return

  loading.value = true
  error.value = ''
  
  try {
    console.log('Loading PDF from:', props.pdfUrl)
    const loadingTask = pdfjsLib.getDocument(props.pdfUrl)
    pdfDoc.value = await loadingTask.promise
    totalPages.value = pdfDoc.value.numPages
    emit('pageCountChanged', totalPages.value)
    
    await renderPage(props.currentPage)
  } catch (err) {
    console.error('Error loading PDF:', err)
    error.value = err.message || '无法加载PDF文件'
  } finally {
    loading.value = false
  }
}

const retryLoad = () => {
  loadPdf()
}

const renderPage = async (pageNum) => {
  if (!pdfDoc.value) return

  try {
    const page = await pdfDoc.value.getPage(pageNum)
    const canvas = canvasRef.value
    const ctx = canvas.getContext('2d')
    
    const viewport = page.getViewport({ scale: 1 })
    const scale = Math.min(1, 800 / viewport.width) // Limit max width to 800px
    
    const scaledViewport = page.getViewport({ scale })
    
    canvas.width = scaledViewport.width
    canvas.height = scaledViewport.height
    
    const renderContext = {
      canvasContext: ctx,
      viewport: scaledViewport
    }
    
    await page.render(renderContext).promise
  } catch (error) {
    console.error('Error rendering page:', error)
  }
}

const prevPage = () => {
  if (canGoPrev.value) {
    emit('update:currentPage', props.currentPage - 1)
  }
}

const nextPage = () => {
  if (canGoNext.value) {
    emit('update:currentPage', props.currentPage + 1)
  }
}

const handleTextSelection = () => {
  const selection = window.getSelection()
  const text = selection.toString().trim()
  
  if (text) {
    emit('textSelected', text)
  }
}

const handleCanvasClick = (event) => {
  // Clear any existing text selection when clicking on canvas
  window.getSelection().removeAllRanges()
}

// Watchers
watch(() => props.pdfUrl, loadPdf)
watch(() => props.currentPage, (newPage) => {
  if (pdfDoc.value) {
    renderPage(newPage)
  }
})

// Lifecycle
onMounted(() => {
  if (props.pdfUrl) {
    loadPdf()
  }
})

onUnmounted(() => {
  if (pdfDoc.value) {
    pdfDoc.value.destroy()
  }
})
</script>

<style scoped>
.pdf-viewer {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.pdf-controls {
  padding: 15px 20px;
  border-bottom: 1px solid #e4e7ed;
  background-color: #fafafa;
  display: flex;
  align-items: center;
}

.dark-mode .pdf-controls {
  background-color: #363636;
  border-bottom-color: #434343;
}

.pdf-content {
  flex: 1;
  padding: 20px;
  overflow: auto;
  display: flex;
  justify-content: center;
}

.pdf-page-container {
  background: white;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  display: inline-block;
}

.dark-mode .pdf-page-container {
  background: #1a1a1a;
}

.pdf-canvas {
  display: block;
  max-width: 100%;
  height: auto;
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 400px;
  color: #909399;
  text-align: center;
}

.loading-state p,
.error-state p,
.empty-state p {
  margin-top: 16px;
  font-size: 16px;
}

.error-state {
  color: #f56c6c;
}

.empty-state {
  color: #909399;
}
</style>

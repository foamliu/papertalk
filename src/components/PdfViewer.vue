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
import { ref, shallowRef, computed, onMounted, watch, onUnmounted } from 'vue'
import * as pdfjsLib from 'pdfjs-dist'
import { ArrowLeft, ArrowRight, Loading, Warning } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/tauri'
import { nextTick } from 'vue'

// PDF.js worker setup
console.log('🔧 Setting up PDF.js worker...')
// 使用本地worker文件
pdfjsLib.GlobalWorkerOptions.workerSrc = new URL('pdfjs-dist/build/pdf.worker.min.mjs', import.meta.url).toString()
console.log('✅ PDF.js worker setup completed with local worker')

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
const pdfDoc = shallowRef(null)
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
    console.log('🚀 Starting PDF load process...')
    console.log('📁 PDF URL:', props.pdfUrl)
    
    // For Tauri apps, we need to load the PDF data through the backend
    console.log('📤 Calling Tauri backend to get PDF data...')
    const pdfData = await invoke('get_pdf_data', { path: props.pdfUrl })
    console.log('✅ PDF data received from backend, size:', pdfData.length, 'bytes')
    
    // Convert the Uint8Array to a typed array for PDF.js
    const typedArray = new Uint8Array(pdfData)
    console.log('📊 Typed array created, length:', typedArray.length)
    
    // Load PDF from the binary data
    console.log('📖 Loading PDF document with PDF.js...')
    const loadingTask = pdfjsLib.getDocument({ data: typedArray })
    pdfDoc.value = await loadingTask.promise
    console.log('✅ PDF document loaded successfully')
    
    totalPages.value = pdfDoc.value.numPages
    console.log('📄 Total pages:', totalPages.value)
    emit('pageCountChanged', totalPages.value)
    
    console.log('🎨 Rendering page', props.currentPage)
    // 关掉 loading → 等 DOM → 渲染
    loading.value = false
    await nextTick()
    await renderPage(props.currentPage)
    console.log('✅ PDF loading completed successfully')
  } catch (err) {
    console.error('❌ Error loading PDF:', err)
    error.value = err.message || '无法加载PDF文件'
  } finally {
    // 已经提前关了，这里不用再写 loading.value = false
  }
}

const retryLoad = () => {
  loadPdf()
}

const renderPage = async (pageNum) => {
  await nextTick()
  const canvas = canvasRef.value
  if (!pdfDoc.value || !canvas) return

  const { zoomLevel } = props          // ⭐ 补上这行
  const page  = await pdfDoc.value.getPage(pageNum)
  const dpr   = window.devicePixelRatio || 1
  const scale = (800 / page.getViewport({ scale: 1 }).width) * (zoomLevel / 100)
  const viewport = page.getViewport({ scale })

  canvas.width  = viewport.width * dpr
  canvas.height = viewport.height * dpr
  const ctx = canvas.getContext('2d')
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0)

  canvas.style.width  = viewport.width + 'px'
  canvas.style.height = viewport.height + 'px'

  await page.render({ canvasContext: ctx, viewport }).promise
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
  console.log('🏗️ Vue component mounted')
  console.log('📁 Current PDF URL:', props.pdfUrl)
  console.log('🖼️ Canvas ref:', canvasRef.value)
  
  if (props.pdfUrl) {
    console.log('🚀 Starting PDF load from onMounted')
    loadPdf()
  } else {
    console.log('ℹ️ No PDF URL provided, waiting for props update')
  }
})

// 添加组件创建时的日志
console.log('📦 PdfViewer component created')
console.log('🔧 PDF.js worker URL:', pdfjsLib.GlobalWorkerOptions.workerSrc)

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

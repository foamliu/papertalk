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
import { invoke } from '@tauri-apps/api/tauri'

// PDF.js worker setup
console.log('🔧 Setting up PDF.js worker...')
try {
  const workerUrl = new URL(
    'pdfjs-dist/build/pdf.worker.min.js',
    import.meta.url
  ).toString()
  console.log('📄 PDF.js worker URL:', workerUrl)
  pdfjsLib.GlobalWorkerOptions.workerSrc = workerUrl
  console.log('✅ PDF.js worker setup completed')
} catch (error) {
  console.error('❌ PDF.js worker setup failed:', error)
  // 使用CDN作为备选方案
  pdfjsLib.GlobalWorkerOptions.workerSrc = 'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/3.11.174/pdf.worker.min.js'
  console.log('🔄 Using CDN worker as fallback')
}

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
    await renderPage(props.currentPage)
    console.log('✅ PDF loading completed successfully')
  } catch (err) {
    console.error('❌ Error loading PDF:', err)
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
    console.log('🎨 Starting to render page', pageNum)
    const page = await pdfDoc.value.getPage(pageNum)
    console.log('📄 Page object obtained')
    
    const canvas = canvasRef.value
    console.log('🖼️ Canvas element:', canvas)
    
    if (!canvas) {
      console.error('❌ Canvas element is null!')
      return
    }
    
    // 检查canvas在DOM中的状态
    console.log('🔍 Checking canvas DOM state...')
    console.log('📍 Canvas parent element:', canvas.parentElement)
    console.log('📐 Canvas computed style - display:', getComputedStyle(canvas).display)
    console.log('📐 Canvas computed style - visibility:', getComputedStyle(canvas).visibility)
    console.log('📐 Canvas computed style - width:', getComputedStyle(canvas).width)
    console.log('📐 Canvas computed style - height:', getComputedStyle(canvas).height)
    console.log('📐 Canvas computed style - opacity:', getComputedStyle(canvas).opacity)
    
    const ctx = canvas.getContext('2d')
    console.log('🎨 Canvas context obtained')
    
    const viewport = page.getViewport({ scale: 1 })
    console.log('📏 Original viewport - width:', viewport.width, 'height:', viewport.height)
    
    const scale = Math.min(1, 800 / viewport.width) // Limit max width to 800px
    console.log('📐 Calculated scale:', scale)
    
    const scaledViewport = page.getViewport({ scale })
    console.log('📏 Scaled viewport - width:', scaledViewport.width, 'height:', scaledViewport.height)
    
    canvas.width = scaledViewport.width
    canvas.height = scaledViewport.height
    console.log('🖼️ Canvas dimensions set - width:', canvas.width, 'height:', canvas.height)
    
    const renderContext = {
      canvasContext: ctx,
      viewport: scaledViewport
    }
    
    console.log('🖌️ Starting page render...')
    await page.render(renderContext).promise
    console.log('✅ Page render completed successfully')
    
    // 渲染完成后再次检查canvas状态
    console.log('🔍 Post-render canvas check:')
    console.log('📊 Canvas has content:', canvas.width > 0 && canvas.height > 0)
    console.log('🎨 Canvas context is valid:', !!ctx)
  } catch (error) {
    console.error('❌ Error rendering page:', error)
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

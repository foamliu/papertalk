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
      
      <div v-else class="pdf-page-container">
        <div class="canvas-container">
          <canvas 
            ref="canvasRef" 
            class="pdf-canvas"
            @click="handleCanvasClick"
          />
          <!-- 文本层容器 -->
          <div 
            ref="textLayerRef" 
            class="text-layer"
          />
        </div>
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
const textLayerRef = ref(null)
const pdfDoc = shallowRef(null)
const totalPages = ref(0)
const loading = ref(false)
const error = ref('')
const currentViewport = ref(null)

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
    
    const pdfData = await invoke('get_pdf_data', { path: props.pdfUrl })
    console.log('✅ PDF data received from backend, size:', pdfData.length, 'bytes')
    
    const typedArray = new Uint8Array(pdfData)
    const loadingTask = pdfjsLib.getDocument({ data: typedArray })
    pdfDoc.value = await loadingTask.promise
    
    totalPages.value = pdfDoc.value.numPages
    emit('pageCountChanged', totalPages.value)
    
    loading.value = false
    await nextTick()
    await renderPage(props.currentPage)
    console.log('✅ PDF loading completed successfully')
  } catch (err) {
    console.error('❌ Error loading PDF:', err)
    error.value = err.message || '无法加载PDF文件'
  }
}

const retryLoad = () => {
  loadPdf()
}

const renderPage = async (pageNum) => {
  await nextTick()
  const canvas = canvasRef.value
  const textLayer = textLayerRef.value
  if (!pdfDoc.value || !canvas || !textLayer) return

  const page = await pdfDoc.value.getPage(pageNum)
  const dpr = window.devicePixelRatio || 1
  
  // 固定canvas宽度，像样例一样
  const canvasWidth = 800
  const scale = canvasWidth / page.getViewport({ scale: 1 }).width * (props.zoomLevel / 100)
  const viewport = page.getViewport({ scale })
  
  currentViewport.value = viewport

  // 渲染画布
  canvas.width = viewport.width * dpr
  canvas.height = viewport.height * dpr
  const ctx = canvas.getContext('2d')
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0)

  canvas.style.width = viewport.width + 'px'
  canvas.style.height = viewport.height + 'px'

  await page.render({ canvasContext: ctx, viewport }).promise

  // 渲染文本层
  await renderTextLayer(page, viewport, textLayer)
}

// 使用PDF.js内置的TextLayer类
const renderTextLayer = async (page, viewport, textLayerDiv) => {
  // 清空文本层
  textLayerDiv.innerHTML = ''
  
  try {
    // 获取文本内容
    const textContent = await page.getTextContent()
    
    // 设置文本层样式 - 精确匹配canvas位置
    textLayerDiv.style.width = viewport.width + 'px'
    textLayerDiv.style.height = viewport.height + 'px'
    textLayerDiv.style.position = 'absolute'
    textLayerDiv.style.top = '0'
    textLayerDiv.style.left = '0'
    textLayerDiv.style.pointerEvents = 'auto'
    textLayerDiv.style.userSelect = 'text'
    textLayerDiv.style.zIndex = '2'
    
    // 使用PDF.js的TextLayer类
    const textLayer = new pdfjsLib.TextLayer({
      textContent: textContent,
      container: textLayerDiv,
      viewport: viewport,
      // 关键：禁用enhancedTextSelection以保持选择功能
      enhancedTextSelection: false
    })
    
    await textLayer.render()
    
    console.log('✅ PDF.js TextLayer渲染完成')
    
  } catch (error) {
    console.error('❌ 文本层渲染失败:', error)
    // 降级到手动渲染
    await renderTextLayerFallback(page, viewport, textLayerDiv)
  }
}

// 降级方案：手动渲染文本层 - 修复位置偏右下问题
const renderTextLayerFallback = async (page, viewport, textLayerDiv) => {
  try {
    const textContent = await page.getTextContent()
    const textFragments = textContent.items
    
    textFragments.forEach((textItem) => {
      const { str, transform, dir } = textItem
      
      if (!str.trim()) return
      
      const [a, b, c, d, e, f] = transform
      
      const span = document.createElement('span')
      span.textContent = str
      
      // 修复位置计算 - 参考index.html中的正确方法
      const scaleFactor = viewport.scale
      const fontSize = Math.sqrt(a * a + b * b) * scaleFactor
      
      // 关键修复：使用正确的坐标转换
      const tx = pdfjsLib.Util.transform(viewport.transform, transform)
      const left = tx[4]
      const top = tx[5]
      
      span.style.cssText = `
        position: absolute;
        left: ${left}px;
        top: ${top}px;
        font-size: ${fontSize}px;
        line-height: 1;
        white-space: pre;
        color: transparent;
        pointer-events: auto;
        user-select: text;
        -webkit-user-select: text;
        -moz-user-select: text;
        -ms-user-select: text;
        z-index: 1;
        mix-blend-mode: multiply;
        font-family: sans-serif;
        cursor: text;
        transform: scale(${1 / scaleFactor});
        transform-origin: 0 0;
      `
      
      if (dir === 'rtl') {
        span.style.direction = 'rtl'
      }
      
      textLayerDiv.appendChild(span)
    })
    
    console.log('✅ 降级文本层渲染完成')
  } catch (error) {
    console.error('❌ 降级文本层渲染失败:', error)
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
  window.getSelection().removeAllRanges()
}

// Watchers
watch(() => props.pdfUrl, loadPdf)
watch(() => props.currentPage, (newPage) => {
  if (pdfDoc.value) {
    renderPage(newPage)
  }
})
watch(() => props.zoomLevel, () => {
  if (pdfDoc.value) {
    renderPage(props.currentPage)
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
  position: relative;
}

.canvas-container {
  position: relative;
  display: inline-block;
}

.dark-mode .pdf-page-container {
  background: #1a1a1a;
}

.pdf-canvas {
  display: block;
  max-width: 100%;
  height: auto;
  position: relative;
  z-index: 1;
}

/* 文本层样式 - 确保可以选择 */
.text-layer {
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

.text-layer span {
  position: absolute;
  white-space: pre;
  cursor: text;
  color: transparent;
  pointer-events: auto;
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
}

.text-layer ::selection {
  background: rgba(0, 0, 255, 0.3);
}

.text-layer ::-moz-selection {
  background: rgba(0, 0, 255, 0.3);
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
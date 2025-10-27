<!-- PdfViewer.vue -->
<template>
  <div class="pdf-viewer">
    <!-- 工具栏 -->
    <div class="pdf-controls">
      <el-button-group>
        <el-button
          :icon="ArrowLeft"
          :disabled="!canGoPrev || loading"
          @click="prevPage"
        />
        <el-button>{{ currentPage }} / {{ totalPages }}</el-button>
        <el-button
          :icon="ArrowRight"
          :disabled="!canGoNext || loading"
          @click="nextPage"
        />
      </el-button-group>

      <el-slider
        :model-value="zoomLevel"
        :min="50"
        :max="200"
        :step="10"
        style="width: 200px; margin: 0 20px"
        show-stops
        @update:model-value="(v) => $emit('update:zoomLevel', v)"
      />
      <span>{{ zoomLevel }}%</span>
    </div>

    <!-- 内容区 -->
    <div 
      class="pdf-content" 
      @mouseup="handleTextSelection"
      @wheel="handleWheel"
      ref="pdfContentRef"
    >
      <!-- 加载中 -->
      <div v-if="loading" class="loading-state">
        <el-icon class="is-loading" :size="32"><Loading /></el-icon>
        <p>正在加载 PDF…</p>
      </div>

      <!-- 加载失败 -->
      <div v-else-if="error" class="error-state">
        <el-icon :size="32"><Warning /></el-icon>
        <p>PDF 加载失败：{{ error }}</p>
        <el-button type="primary" @click="retryLoad">重试</el-button>
      </div>

      <!-- 空状态 -->
      <div v-else-if="!pdfDoc" class="empty-state">
        <p>请选择 PDF 文件</p>
      </div>

      <!-- 渲染区 -->
      <div v-else class="pdf-page-container">
        <div class="canvas-container">
          <canvas ref="canvasRef" class="pdf-canvas" @click="handleCanvasClick" />
          <!-- 文本层 -->
          <div ref="textLayerRef" class="textLayer" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
/* ---------------------------------
 * 依赖
 * --------------------------------- */
import { ref, shallowRef, computed, watch, onMounted, onUnmounted, nextTick } from 'vue'
import * as pdfjsLib from 'pdfjs-dist'
import { ArrowLeft, ArrowRight, Loading, Warning } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'

/* ---------------------------------
 * PDF.js 配置
 * --------------------------------- */
pdfjsLib.GlobalWorkerOptions.workerSrc =
  new URL('pdfjs-dist/build/pdf.worker.min.mjs', import.meta.url).toString()

/* ---------------------------------
 * Props / Emits
 * --------------------------------- */
const props = defineProps({
  pdfUrl: { type: String, default: null },
  currentPage: { type: Number, default: 1 },
  zoomLevel: { type: Number, default: 100 }
})
const emit = defineEmits([
  'update:currentPage',
  'update:zoomLevel',
  'textSelected',
  'pageCountChanged'
])

/* ---------------------------------
 * 响应式数据
 * --------------------------------- */
const canvasRef = ref(null)
const textLayerRef = ref(null)
const pdfDoc = shallowRef(null)
const totalPages = ref(0)
const loading = ref(false)
const error = ref('')
const pdfContentRef = ref(null)
const isAtBottom = ref(false)
const isAtTop = ref(true)

/* ---------------------------------
 * 计算属性
 * --------------------------------- */
const canGoPrev = computed(() => props.currentPage > 1)
const canGoNext = computed(() => props.currentPage < totalPages.value)

/* ---------------------------------
 * 核心方法
 * --------------------------------- */
async function loadPdf() {
  if (!props.pdfUrl) return
  loading.value = true
  error.value = ''
  try {
    const pdfData = await invoke('get_pdf_data', { path: props.pdfUrl })
    const doc = await pdfjsLib.getDocument({ data: new Uint8Array(pdfData) })
      .promise
    pdfDoc.value = doc
    totalPages.value = doc.numPages
    emit('pageCountChanged', totalPages.value)
    loading.value = false
    await nextTick()
    await renderPage(props.currentPage)
  } catch (e) {
    console.error('PDF 加载失败', e)
    error.value = e.message || '无法加载 PDF'
    loading.value = false
  }
}

async function renderPage(pageNum) {
  await nextTick()
  const canvas = canvasRef.value
  const textDiv = textLayerRef.value
  if (!pdfDoc.value || !canvas || !textDiv) return

  const page = await pdfDoc.value.getPage(pageNum)
  const viewport = page.getViewport({ scale: props.zoomLevel / 100 })
  const dpr = window.devicePixelRatio || 1

  // 渲染 Canvas
  canvas.width = viewport.width * dpr
  canvas.height = viewport.height * dpr
  const ctx = canvas.getContext('2d')
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0)
  canvas.style.width = viewport.width + 'px'
  canvas.style.height = viewport.height + 'px'
  await page.render({ canvasContext: ctx, viewport }).promise

  // 渲染文本层
  const textContent = await page.getTextContent()
  // 清空旧内容
  textDiv.innerHTML = ''
  // 使用官方 TextLayer
  const textLayer = new pdfjsLib.TextLayer({
    textContentSource: textContent,
    container: textDiv,
    viewport,
    enhancedTextSelection: false
  })
  await textLayer.render()
}

function retryLoad() {
  loadPdf()
}
function prevPage() {
  if (canGoPrev.value) emit('update:currentPage', props.currentPage - 1)
}
function nextPage() {
  if (canGoNext.value) emit('update:currentPage', props.currentPage + 1)
}
function handleTextSelection() {
  const text = window.getSelection().toString().trim()
  if (text) emit('textSelected', text)
}
function handleCanvasClick() {
  window.getSelection().removeAllRanges()
}

/* ---------------------------------
 * 鼠标滚轮处理
 * --------------------------------- */
function handleWheel(event) {
  if (!pdfContentRef.value) return
  
  const container = pdfContentRef.value
  const scrollTop = container.scrollTop
  const scrollHeight = container.scrollHeight
  const clientHeight = container.clientHeight
  
  // 更新滚动位置状态
  isAtTop.value = scrollTop === 0
  isAtBottom.value = scrollTop + clientHeight >= scrollHeight - 5 // 容差5px
  
  // 检查是否需要翻页
  if (event.deltaY > 0) {
    // 向下滚动
    if (isAtBottom.value && canGoNext.value) {
      event.preventDefault()
      nextPage()
      // 重置滚动位置到顶部
      nextTick(() => {
        container.scrollTop = 0
      })
    }
  } else if (event.deltaY < 0) {
    // 向上滚动
    if (isAtTop.value && canGoPrev.value) {
      event.preventDefault()
      prevPage()
      // 重置滚动位置到底部
      nextTick(() => {
        container.scrollTop = container.scrollHeight - container.clientHeight
      })
    }
  }
}

/* ---------------------------------
 * 监听
 * --------------------------------- */
watch(() => props.pdfUrl, loadPdf)
watch(() => props.currentPage, (n) => {
  if (pdfDoc.value) {
    renderPage(n)
    // 每次翻页后重置滚动位置到顶部
    nextTick(() => {
      if (pdfContentRef.value) {
        pdfContentRef.value.scrollTop = 0
      }
    })
  }
})
watch(() => props.zoomLevel, () => pdfDoc.value && renderPage(props.currentPage))

onMounted(() => props.pdfUrl && loadPdf())
onUnmounted(() => pdfDoc.value && pdfDoc.value.destroy())
</script>

<style scoped>
/* 引入官方文本层样式（核心） */
@import 'pdfjs-dist/web/pdf_viewer.css';

.pdf-viewer {
  height: 100%;
  display: flex;
  flex-direction: column;
}
.pdf-controls {
  padding: 15px 20px;
  border-bottom: 1px solid #e4e7ed;
  background: #fafafa;
  display: flex;
  align-items: center;
  gap: 12px;
}
.pdf-content {
  flex: 1;
  padding: 20px;
  overflow: auto;
  display: flex;
  justify-content: center;
}
.pdf-page-container {
  background: #fff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  display: inline-block;
  position: relative;
}
.canvas-container {
  position: relative;
  display: inline-block;
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
}
.error-state {
  color: #f56c6c;
}
</style>

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
const containerWidth = ref(0)

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
    
    // 先渲染页面，然后计算适合容器宽度的默认缩放级别
    await renderPage(props.currentPage)
    await calculateOptimalZoom()
  } catch (e) {
    console.error('PDF 加载失败', e)
    error.value = e.message || '无法加载 PDF'
    loading.value = false
  }
}

// 计算适合容器宽度的最佳缩放级别
async function calculateOptimalZoom() {
  if (!pdfDoc.value || !pdfContentRef.value) return
  
  const container = pdfContentRef.value
  
  // 使用默认的PDF页面尺寸（A4尺寸：595x842 points）
  // 大多数PDF使用A4尺寸，这是一个合理的默认值
  const defaultPageWidth = 595 // A4 width in points
  
  // 计算容器可用宽度（减去内边距）
  const availableWidth = container.clientWidth - 40 // 减去左右内边距
  
  // 计算适合宽度的缩放比例
  const optimalScale = (availableWidth / defaultPageWidth) * 100
  
  // 限制缩放范围在50%-200%之间
  const clampedZoom = Math.max(50, Math.min(200, Math.round(optimalScale)))
  
  // 更新缩放级别
  emit('update:zoomLevel', clampedZoom)
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

  // 渲染文本层 - 使用更精确的文本选择方法
  const textContent = await page.getTextContent()
  // 清空旧内容
  textDiv.innerHTML = ''
  
  // 创建自定义文本层，确保精确对齐
  await renderCustomTextLayer(textDiv, textContent, viewport)
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
function handleCanvasClick() {
  window.getSelection().removeAllRanges()
}

// 修复文本层对齐问题
function fixTextLayerAlignment(textDiv, viewport) {
  if (!textDiv) return
  
  // 确保文本层与canvas精确对齐
  textDiv.style.width = viewport.width + 'px'
  textDiv.style.height = viewport.height + 'px'
  textDiv.style.left = '0px'
  textDiv.style.top = '0px'
  
  // 优化文本选择：修复跨列选择问题
  const textSpans = textDiv.querySelectorAll('span')
  textSpans.forEach(span => {
    // 移除可能影响选择精度的样式
    span.style.lineHeight = '1'
    span.style.letterSpacing = '0px'
    
    // 确保文本span有正确的定位
    if (!span.style.left || !span.style.top) {
      const rect = span.getBoundingClientRect()
      if (rect.width > 0 && rect.height > 0) {
        span.style.position = 'absolute'
      }
    }
  })
}


// 自定义文本层渲染 - 解决文本选择错位问题
async function renderCustomTextLayer(container, textContent, viewport) {
  // 确保容器样式正确
  container.style.width = viewport.width + 'px'
  container.style.height = viewport.height + 'px'
  container.style.position = 'absolute'
  container.style.left = '0px'
  container.style.top = '0px'
  container.style.overflow = 'hidden'
  container.style.lineHeight = '1'
  container.style.fontSize = '1px' // 最小化字体影响
  container.style.color = 'transparent'
  container.style.userSelect = 'text'
  container.style.webkitUserSelect = 'text'
  container.style.mozUserSelect = 'text'
  container.style.msUserSelect = 'text'

  // 渲染文本项
  for (const item of textContent.items) {
    const textSpan = document.createElement('span')
    
    // 设置文本内容
    textSpan.textContent = item.str
    
    // 精确计算位置和尺寸
    const transform = viewport.transform
    const scale = viewport.scale
    
    // 计算精确位置（考虑PDF坐标系统）
    const left = (item.transform[4] * scale) + 'px'
    const top = (viewport.height - (item.transform[5] * scale) - (item.height * scale)) + 'px'
    const width = (item.width * scale) + 'px'
    const height = (item.height * scale) + 'px'
    
    // 设置样式
    textSpan.style.position = 'absolute'
    textSpan.style.left = left
    textSpan.style.top = top
    textSpan.style.width = width
    textSpan.style.height = height
    textSpan.style.whiteSpace = 'pre'
    textSpan.style.fontSize = (item.height * scale) + 'px'
    textSpan.style.lineHeight = '1'
    textSpan.style.letterSpacing = '0px'
    textSpan.style.fontFamily = item.fontName || 'sans-serif'
    textSpan.style.color = 'transparent'
    textSpan.style.cursor = 'text'
    
    // 添加数据属性用于调试
    textSpan.dataset.text = item.str
    textSpan.dataset.originalLeft = item.transform[4]
    textSpan.dataset.originalTop = item.transform[5]
    
    container.appendChild(textSpan)
  }
  
  await nextTick()
  
  // 验证文本层对齐
  verifyTextLayerAlignment(container, viewport)
}

// 验证文本层对齐
function verifyTextLayerAlignment(container, viewport) {
  const textSpans = container.querySelectorAll('span')
  let misalignedCount = 0
  
  textSpans.forEach(span => {
    const rect = span.getBoundingClientRect()
    const expectedLeft = parseFloat(span.style.left)
    const expectedTop = parseFloat(span.style.top)
    
    // 检查位置偏差
    const leftDiff = Math.abs(rect.left - expectedLeft)
    const topDiff = Math.abs(rect.top - expectedTop)
    
    if (leftDiff > 2 || topDiff > 2) {
      misalignedCount++
      console.warn('文本span位置偏差:', {
        expected: { left: expectedLeft, top: expectedTop },
        actual: { left: rect.left, top: rect.top },
        diff: { left: leftDiff, top: topDiff }
      })
    }
  })
  
  if (misalignedCount > 0) {
    console.warn(`发现 ${misalignedCount} 个文本span位置偏差`)
  }
}

// 完全重写的文本选择处理 - 解决错位问题
function handleTextSelection() {
  const selection = window.getSelection()
  if (!selection.rangeCount) return
  
  const range = selection.getRangeAt(0)
  let selectedText = range.toString().trim()
  
  if (!selectedText) return
  
  // 获取文本层
  const textLayer = textLayerRef.value
  if (!textLayer) {
    emit('textSelected', selectedText)
    return
  }
  
  // 检查选择是否错位
  const rangeRect = range.getBoundingClientRect()
  const textSpans = textLayer.querySelectorAll('span')
  
  // 如果选择范围明显错位，使用精确选择
  if (isSelectionMisaligned(rangeRect, textSpans)) {
    const preciseText = getPreciseTextFromSelection(rangeRect, textSpans)
    if (preciseText && preciseText !== selectedText) {
      selectedText = preciseText
    }
  }
  
  // 清理文本（移除多余空格和换行）
  selectedText = cleanSelectedText(selectedText)
  
  emit('textSelected', selectedText)
}

// 检查选择是否错位
function isSelectionMisaligned(rangeRect, textSpans) {
  if (!textSpans.length) return false
  
  // 计算选择范围内的文本span数量
  let spansInRange = 0
  for (const span of textSpans) {
    const spanRect = span.getBoundingClientRect()
    if (isRectOverlap(spanRect, rangeRect)) {
      spansInRange++
    }
  }
  
  // 如果选择范围内没有文本span，说明选择错位
  return spansInRange === 0
}

// 检查矩形是否重叠
function isRectOverlap(rect1, rect2) {
  return !(
    rect1.right < rect2.left ||
    rect1.left > rect2.right ||
    rect1.bottom < rect2.top ||
    rect1.top > rect2.bottom
  )
}

// 从选择范围获取精确文本
function getPreciseTextFromSelection(rangeRect, textSpans) {
  const selectedSpans = []
  
  for (const span of textSpans) {
    const spanRect = span.getBoundingClientRect()
    
    // 检查span是否在选择范围内
    if (isRectOverlap(spanRect, rangeRect)) {
      // 计算重叠面积
      const overlapArea = calculateOverlapArea(spanRect, rangeRect)
      const spanArea = spanRect.width * spanRect.height
      
      // 如果重叠面积超过span面积的50%，认为被选中
      if (overlapArea > spanArea * 0.5) {
        selectedSpans.push({
          span,
          text: span.textContent || span.innerText,
          left: spanRect.left
        })
      }
    }
  }
  
  // 按从左到右排序
  selectedSpans.sort((a, b) => a.left - b.left)
  
  // 组合文本
  return selectedSpans.map(item => item.text).join(' ').trim()
}

// 计算矩形重叠面积
function calculateOverlapArea(rect1, rect2) {
  const xOverlap = Math.max(0, Math.min(rect1.right, rect2.right) - Math.max(rect1.left, rect2.left))
  const yOverlap = Math.max(0, Math.min(rect1.bottom, rect2.bottom) - Math.max(rect1.top, rect2.top))
  return xOverlap * yOverlap
}

// 清理选中文本
function cleanSelectedText(text) {
  return text
    .replace(/\s+/g, ' ') // 合并多个空格
    .replace(/\n/g, ' ')  // 替换换行符为空格
    .trim()
}

/* ---------------------------------
 * 鼠标滚轮处理
 * --------------------------------- */
function handleWheel(event) {
  if (!pdfContentRef.value) return
  
  // 检查是否按下Ctrl键进行缩放
  if (event.ctrlKey) {
    event.preventDefault()
    
    // 计算缩放增量（基于滚轮方向）
    const zoomDelta = event.deltaY > 0 ? -10 : 10
    
    // 计算新的缩放级别
    const newZoom = Math.max(50, Math.min(200, props.zoomLevel + zoomDelta))
    
    // 更新缩放级别
    emit('update:zoomLevel', newZoom)
    return
  }
  
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

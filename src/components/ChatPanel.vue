<template>
  <div class="chat-panel">
    <div class="chat-header">
      <h3>Chat</h3>
      <el-button 
        v-if="appStore.chatMessages.length > 0"
        @click="clearChat"
        size="small"
        type="text"
        :icon="Delete"
      >
        清空对话
      </el-button>
    </div>
    
    <div class="chat-container">
      <!-- 消息列表 -->
      <div class="messages-container" ref="messagesContainer">
        <div 
          v-for="message in appStore.chatMessages" 
          :key="message.id"
          :class="['message', message.role]"
        >
          <div class="message-avatar">
            <el-avatar 
              v-if="message.role === 'user'" 
              :size="32" 
              :icon="User"
            />
            <el-avatar 
              v-else 
              :size="32" 
              :icon="ChatDotRound"
              style="background-color: #409eff;"
            />
          </div>
          <div class="message-content">
            <div class="message-text" v-html="formatMessage(message.content)"></div>
            <div class="message-time">
              {{ formatTime(message.timestamp) }}
            </div>
          </div>
        </div>

        <!-- 流式响应消息 -->
        <div 
          v-if="appStore.isChatting && appStore.chatStreamingText"
          class="message assistant streaming"
        >
          <div class="message-avatar">
            <el-avatar 
              :size="32" 
              :icon="ChatDotRound"
              style="background-color: #409eff;"
            />
          </div>
          <div class="message-content">
            <div class="message-text streaming-text">
              <span v-html="formatMessage(appStore.chatStreamingText)"></span>
              <span class="streaming-cursor">|</span>
            </div>
            <div class="streaming-indicator">
              <span class="streaming-dot"></span>
              <span>AI 正在思考...</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 输入区域 -->
      <div class="input-container">
        <div class="input-wrapper">
          <el-input
            v-model="appStore.currentChatMessage"
            type="textarea"
            :rows="3"
            placeholder="请先在论文中选中相关文本，然后输入您的问题..."
            :disabled="appStore.isChatting"
            @keydown.enter.exact.prevent="sendMessage"
            class="chat-input"
            resize="none"
          />
          <div class="input-actions">
            <el-button 
              @click="sendMessage" 
              type="primary" 
              :loading="appStore.isChatting"
              :disabled="!canSendMessage"
              class="send-button"
            >
              {{ appStore.isChatting ? '发送中...' : '发送' }}
            </el-button>
          </div>
        </div>
        <div class="input-tips">
          <span>按 Enter 发送，Shift + Enter 换行</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, nextTick, watch, onMounted, onUnmounted } from 'vue'
import { useAppStore } from '../stores/app'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { User, ChatDotRound, Delete } from '@element-plus/icons-vue'

const appStore = useAppStore()
const messagesContainer = ref(null)

// 计算属性
const canSendMessage = computed(() => {
  return appStore.currentChatMessage.trim().length > 0 && !appStore.isChatting
})

// 方法
const formatMessage = (text) => {
  // 简单的格式化，将换行转换为 <br>
  return text.replace(/\n/g, '<br>')
}

const formatTime = (timestamp) => {
  const date = new Date(timestamp)
  return date.toLocaleTimeString('zh-CN', { 
    hour: '2-digit', 
    minute: '2-digit' 
  })
}

const scrollToBottom = async () => {
  await nextTick()
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
}

const sendMessage = async () => {
  if (!canSendMessage.value) return

  const message = appStore.currentChatMessage.trim()
  if (!message) return

  // 添加用户消息
  appStore.addChatMessage({
    role: 'user',
    content: message
  })

  // 清空输入框
  appStore.setCurrentChatMessage('')
  
  // 滚动到底部
  await scrollToBottom()

  // 开始聊天
  appStore.setIsChatting(true)
  appStore.clearChatStreamingText()

  try {
    console.log('发送聊天消息:', message)
    
    // 准备配置数据
    const config = {
      selected_model: appStore.modelConfig.selectedModel,
      ollama: {
        base_url: appStore.modelConfig.ollama.baseUrl,
        model: appStore.modelConfig.ollama.model
      },
      deepseek: {
        api_key: appStore.modelConfig.deepseek.apiKey,
        base_url: appStore.modelConfig.deepseek.baseUrl,
        model: appStore.modelConfig.deepseek.model
      },
      kimi: {
        api_key: appStore.modelConfig.kimi.apiKey,
        base_url: appStore.modelConfig.kimi.baseUrl,
        model: appStore.modelConfig.kimi.model
      }
    }
    
    console.log('使用模型配置:', config)
    
    // 构建包含论文上下文的消息
    let enhancedMessage = message
    
    // 如果有选中的文本，添加到上下文中
    if (appStore.selectedText && appStore.selectedText.trim().length > 0) {
      enhancedMessage = `用户正在阅读论文的以下内容：\n\n${appStore.selectedText}\n\n用户的问题：${message}`
      console.log('添加论文上下文（选中文本）:', appStore.selectedText)
    } else {
      // 如果没有选中文本，提示用户先选中论文内容
      enhancedMessage = `用户想要了解论文的核心思想，但还没有提供具体的论文内容。请提醒用户先选中论文中的相关段落，然后重新提问。\n\n用户的问题：${message}`
      console.log('没有选中文本，提示用户先选中内容')
    }
    
    // 调用后端聊天命令
    await invoke('chat_with_config', { 
      message: enhancedMessage,
      config: config
    })
    
    // 注意：这里不再手动添加AI回复消息
    // 因为流式响应会通过事件监听器自动添加
    
  } catch (error) {
    console.error('聊天失败:', error)
    appStore.addChatMessage({
      role: 'assistant',
      content: `抱歉，聊天过程中出现了错误：${error.message || error}`
    })
    appStore.setIsChatting(false)
    await scrollToBottom()
  }
}

const clearChat = () => {
  appStore.clearChatMessages()
  appStore.clearChatStreamingText()
}

// 监听消息变化，自动滚动
watch(() => appStore.chatMessages, scrollToBottom, { deep: true })

// 监听流式文本变化
watch(() => appStore.chatStreamingText, scrollToBottom)

// 设置事件监听器
const setupEventListeners = async () => {
  console.log('设置聊天事件监听器...')
  
  try {
    // 监听聊天片段
    const unlistenChunk = await listen('chat_chunk', (event) => {
      const chunk = event.payload
      console.log('收到聊天片段:', chunk)
      appStore.appendChatStreamingText(chunk)
    })
    
    // 监听聊天完成
    const unlistenComplete = await listen('chat_complete', (event) => {
      const fullText = event.payload
      console.log('聊天完成，完整文本:', fullText)
      
      // 添加AI回复消息
      appStore.addChatMessage({
        role: 'assistant',
        content: fullText
      })
      
      appStore.setIsChatting(false)
      appStore.clearChatStreamingText()
    })
    
    console.log('聊天事件监听器设置成功')
    
    return () => {
      unlistenChunk()
      unlistenComplete()
    }
  } catch (error) {
    console.error('设置聊天事件监听器失败:', error)
    return () => {}
  }
}

// 组件挂载时设置事件监听器
let cleanupEventListeners = null

onMounted(async () => {
  cleanupEventListeners = await setupEventListeners()
})

onUnmounted(() => {
  if (cleanupEventListeners) {
    cleanupEventListeners()
  }
})
</script>

<style scoped>
.chat-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: #ffffff;
}

.dark-mode .chat-panel {
  background-color: #2d2d2d;
}

.chat-header {
  padding: 15px 20px;
  border-bottom: 1px solid #e4e7ed;
  background-color: #ffffff;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-shrink: 0;
}

.dark-mode .chat-header {
  background-color: #363636;
  border-bottom-color: #434343;
}

.chat-header h3 {
  margin: 0;
  color: #303133;
}

.dark-mode .chat-header h3 {
  color: #ffffff;
}

.chat-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.message {
  display: flex;
  gap: 12px;
  max-width: 100%;
}

.message.user {
  flex-direction: row-reverse;
}

.message-avatar {
  flex-shrink: 0;
}

.message-content {
  max-width: calc(100% - 44px);
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.message.user .message-content {
  align-items: flex-end;
}

.message-text {
  padding: 12px 16px;
  border-radius: 12px;
  line-height: 1.5;
  word-wrap: break-word;
  max-width: 100%;
}

.message.user .message-text {
  background-color: #409eff;
  color: white;
  border-bottom-right-radius: 4px;
}

.message.assistant .message-text {
  background-color: #f5f7fa;
  color: #303133;
  border-bottom-left-radius: 4px;
}

.dark-mode .message.assistant .message-text {
  background-color: #363636;
  color: #ffffff;
}

.message-time {
  font-size: 12px;
  color: #909399;
  padding: 0 4px;
}

.dark-mode .message-time {
  color: #a0a0a0;
}

/* 流式消息样式 */
.message.streaming .message-text {
  background-color: #f0f9ff;
  border: 1px solid #e1f5fe;
}

.dark-mode .message.streaming .message-text {
  background-color: #1e3a5f;
  border-color: #2d4a6e;
}

.streaming-text {
  position: relative;
}

.streaming-cursor {
  animation: blink 1s infinite;
  color: #409eff;
}

@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
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
  margin-top: 8px;
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

/* 输入区域样式 */
.input-container {
  padding: 20px;
  border-top: 1px solid #e4e7ed;
  background-color: #ffffff;
  flex-shrink: 0;
}

.dark-mode .input-container {
  background-color: #363636;
  border-top-color: #434343;
}

.input-wrapper {
  display: flex;
  gap: 12px;
  align-items: flex-end;
}

.chat-input {
  flex: 1;
}

.chat-input :deep(.el-textarea__inner) {
  resize: none;
  border-radius: 8px;
}

.input-actions {
  display: flex;
  gap: 8px;
}

.send-button {
  height: 40px;
  min-width: 80px;
}

.input-tips {
  margin-top: 8px;
  text-align: center;
  font-size: 12px;
  color: #909399;
}

.dark-mode .input-tips {
  color: #a0a0a0;
}

/* 滚动条样式 */
.messages-container::-webkit-scrollbar {
  width: 6px;
}

.messages-container::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.dark-mode .messages-container::-webkit-scrollbar-track {
  background: #363636;
}

.messages-container::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

.dark-mode .messages-container::-webkit-scrollbar-thumb {
  background: #606060;
}

.messages-container::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

.dark-mode .messages-container::-webkit-scrollbar-thumb:hover {
  background: #808080;
}
</style>

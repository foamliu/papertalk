<template>
  <el-dialog
    v-model="visible"
    title="大模型配置"
    width="600px"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
  >
    <div class="model-config">
      <!-- 模型选择 -->
      <div class="model-selection">
        <h4>选择模型</h4>
        <el-radio-group v-model="selectedModel" @change="handleModelChange">
          <el-radio label="ollama">Ollama (本地)</el-radio>
          <el-radio label="deepseek">DeepSeek v3 (云端)</el-radio>
          <el-radio label="kimi">Kimi (云端)</el-radio>
        </el-radio-group>
      </div>

      <!-- 配置表单 -->
      <div class="config-form">
        <!-- Ollama 配置 -->
        <div v-if="selectedModel === 'ollama'" class="model-config-section">
          <h4>Ollama 配置</h4>
          <el-form :model="config.ollama" label-width="120px">
            <el-form-item label="API地址">
              <el-input v-model="config.ollama.baseUrl" placeholder="http://127.0.0.1:11434" />
            </el-form-item>
            <el-form-item label="模型名称">
              <el-input v-model="config.ollama.model" placeholder="qwen3:8b" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="testOllamaConnection" :loading="testingOllama">
                测试连接
              </el-button>
            </el-form-item>
          </el-form>
        </div>

        <!-- DeepSeek 配置 -->
        <div v-if="selectedModel === 'deepseek'" class="model-config-section">
          <h4>DeepSeek v3 配置</h4>
          <el-form :model="config.deepseek" label-width="120px">
            <el-form-item label="API密钥">
              <el-input 
                v-model="config.deepseek.apiKey" 
                type="password" 
                placeholder="请输入DeepSeek API密钥"
                show-password
              />
            </el-form-item>
            <el-form-item label="API地址">
              <el-input v-model="config.deepseek.baseUrl" placeholder="https://api.deepseek.com" />
            </el-form-item>
            <el-form-item label="模型名称">
              <el-input v-model="config.deepseek.model" placeholder="deepseek-chat" />
            </el-form-item>
          </el-form>
        </div>

        <!-- Kimi 配置 -->
        <div v-if="selectedModel === 'kimi'" class="model-config-section">
          <h4>Kimi 配置</h4>
          <el-form :model="config.kimi" label-width="120px">
            <el-form-item label="API密钥">
              <el-input 
                v-model="config.kimi.apiKey" 
                type="password" 
                placeholder="请输入Kimi API密钥"
                show-password
              />
            </el-form-item>
            <el-form-item label="API地址">
              <el-input v-model="config.kimi.baseUrl" placeholder="https://api.moonshot.cn" />
            </el-form-item>
            <el-form-item label="模型名称">
              <el-input v-model="config.kimi.model" placeholder="moonshot-v1-8k" />
            </el-form-item>
          </el-form>
        </div>
      </div>

      <!-- 当前模型状态 -->
      <div class="model-status">
        <h4>当前状态</h4>
        <div class="status-info">
          <el-tag :type="getStatusType()" size="large">
            {{ getStatusText() }}
          </el-tag>
          <span class="current-model">当前使用: {{ getCurrentModelName() }}</span>
        </div>
      </div>
    </div>

    <template #footer>
      <el-button @click="handleCancel">取消</el-button>
      <el-button type="primary" @click="handleSave">保存配置</el-button>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { useAppStore } from '../stores/app'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['update:modelValue'])

const appStore = useAppStore()
const visible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

// 本地状态
const selectedModel = ref(appStore.modelConfig.selectedModel)
const config = ref(JSON.parse(JSON.stringify(appStore.modelConfig)))
const testingOllama = ref(false)

// 模型名称映射
const modelNames = {
  ollama: 'Ollama 本地模型',
  deepseek: 'DeepSeek v3',
  kimi: 'Kimi'
}

// 方法
const handleModelChange = (model) => {
  selectedModel.value = model
}

const testOllamaConnection = async () => {
  testingOllama.value = true
  try {
    const isRunning = await invoke('check_ollama')
    if (isRunning) {
      ElMessage.success('Ollama 连接成功！')
    } else {
      ElMessage.error('Ollama 未运行，请检查服务状态')
    }
  } catch (error) {
    ElMessage.error('连接测试失败：' + error.message)
  } finally {
    testingOllama.value = false
  }
}

const getStatusType = () => {
  const model = appStore.modelConfig.selectedModel
  if (model === 'ollama') {
    return appStore.ollamaStatus === 'running' ? 'success' : 'danger'
  }
  return 'info'
}

const getStatusText = () => {
  const model = appStore.modelConfig.selectedModel
  if (model === 'ollama') {
    switch (appStore.ollamaStatus) {
      case 'running': return 'Ollama 运行中'
      case 'not-found': return 'Ollama 未找到'
      default: return '检查中...'
    }
  }
  return '云端模型'
}

const getCurrentModelName = () => {
  const model = appStore.modelConfig.selectedModel
  return modelNames[model] || '未知模型'
}

const handleSave = () => {
  // 更新存储中的配置
  appStore.setSelectedModel(selectedModel.value)
  appStore.updateModelConfig('ollama', config.value.ollama)
  appStore.updateModelConfig('deepseek', config.value.deepseek)
  appStore.updateModelConfig('kimi', config.value.kimi)
  
  ElMessage.success('配置已保存')
  visible.value = false
}

const handleCancel = () => {
  // 重置为原始配置
  selectedModel.value = appStore.modelConfig.selectedModel
  config.value = JSON.parse(JSON.stringify(appStore.modelConfig))
  visible.value = false
}

// 监听可见性变化
watch(visible, (newVal) => {
  if (newVal) {
    // 对话框打开时刷新配置
    selectedModel.value = appStore.modelConfig.selectedModel
    config.value = JSON.parse(JSON.stringify(appStore.modelConfig))
  }
})
</script>

<style scoped>
.model-config {
  max-height: 60vh;
  overflow-y: auto;
}

.model-selection {
  margin-bottom: 20px;
  padding-bottom: 20px;
  border-bottom: 1px solid #e4e7ed;
}

.model-selection h4 {
  margin: 0 0 12px 0;
  color: #303133;
}

.config-form {
  margin-bottom: 20px;
}

.model-config-section {
  margin-bottom: 20px;
}

.model-config-section h4 {
  margin: 0 0 16px 0;
  color: #303133;
  font-size: 14px;
}

.model-status {
  padding-top: 20px;
  border-top: 1px solid #e4e7ed;
}

.model-status h4 {
  margin: 0 0 12px 0;
  color: #303133;
}

.status-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.current-model {
  color: #606266;
  font-size: 14px;
}

/* 暗色模式支持 */
:deep(.el-dialog) {
  .model-selection,
  .model-config-section,
  .model-status {
    border-color: #434343;
  }
  
  .model-selection h4,
  .model-config-section h4,
  .model-status h4 {
    color: #ffffff;
  }
  
  .current-model {
    color: #cccccc;
  }
}
</style>

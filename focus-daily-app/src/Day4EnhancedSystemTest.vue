<template>
  <div class="day4-enhanced-test p-6 bg-gray-50 min-h-screen">
    <div class="max-w-6xl mx-auto">
      <!-- 标题 -->
      <div class="text-center mb-8">
        <h1 class="text-3xl font-bold text-gray-800 mb-2">
          🚀 Week 3 Day 4 增强系统测试
        </h1>
        <p class="text-gray-600">
          测试增强事件系统、性能监控和音频事件功能
        </p>
      </div>

      <!-- 系统状态概览 -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div class="bg-white rounded-lg shadow-md p-6">
          <h3 class="text-lg font-semibold text-gray-800 mb-4">
            🎯 系统健康状态
          </h3>
          <div v-if="systemHealth" class="space-y-2">
            <div class="flex justify-between">
              <span>整体状态:</span>
              <span :class="getHealthStatusColor(systemHealth.overall_status)">
                {{ systemHealth.overall_status }}
              </span>
            </div>
            <div class="flex justify-between">
              <span>性能评分:</span>
              <span class="font-semibold">{{ systemHealth.performance_score }}/100</span>
            </div>
            <div class="flex justify-between">
              <span>事件系统:</span>
              <span :class="getHealthStatusColor(systemHealth.event_system_health)">
                {{ systemHealth.event_system_health }}
              </span>
            </div>
          </div>
          <div v-else class="text-gray-500">
            未获取到系统健康数据
          </div>
        </div>

        <div class="bg-white rounded-lg shadow-md p-6">
          <h3 class="text-lg font-semibold text-gray-800 mb-4">
            📊 事件统计
          </h3>
          <div v-if="eventStats" class="space-y-2">
            <div class="flex justify-between">
              <span>总事件数:</span>
              <span class="font-semibold">{{ eventStats.total_events }}</span>
            </div>
            <div class="flex justify-between">
              <span>成功处理:</span>
              <span class="text-green-600">{{ eventStats.successful_events }}</span>
            </div>
            <div class="flex justify-between">
              <span>处理失败:</span>
              <span class="text-red-600">{{ eventStats.failed_events }}</span>
            </div>
            <div class="flex justify-between">
              <span>平均处理时间:</span>
              <span>{{ eventStats.average_processing_time.toFixed(2) }}ms</span>
            </div>
          </div>
          <div v-else class="text-gray-500">
            未获取到事件统计数据
          </div>
        </div>

        <div class="bg-white rounded-lg shadow-md p-6">
          <h3 class="text-lg font-semibold text-gray-800 mb-4">
            🔄 事件队列状态
          </h3>
          <div v-if="queueStatus" class="space-y-2">
            <div class="flex justify-between">
              <span>当前队列大小:</span>
              <span class="font-semibold">{{ queueStatus.current_size }}</span>
            </div>
            <div class="flex justify-between">
              <span>最大容量:</span>
              <span>{{ queueStatus.max_size }}</span>
            </div>
            <div class="flex justify-between">
              <span>使用率:</span>
              <span :class="getUtilizationColor(queueStatus.utilization)">
                {{ queueStatus.utilization.toFixed(1) }}%
              </span>
            </div>
          </div>
          <div v-else class="text-gray-500">
            未获取到队列状态数据
          </div>
        </div>
      </div>

      <!-- 功能测试区域 -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <!-- 增强事件系统测试 -->
        <div class="bg-white rounded-lg shadow-md p-6">
          <h3 class="text-xl font-semibold text-gray-800 mb-4">
            🎪 增强事件系统测试
          </h3>
          
          <div class="space-y-4">
            <button 
              @click="initializeSystem"
              class="w-full bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              初始化增强系统
            </button>

            <button 
              @click="getEventStats"
              class="w-full bg-green-500 hover:bg-green-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              获取事件统计
            </button>

            <button 
              @click="getEventHistory"
              class="w-full bg-purple-500 hover:bg-purple-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              获取事件历史
            </button>

            <button 
              @click="cleanupExpiredEvents"
              class="w-full bg-orange-500 hover:bg-orange-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              清理过期事件
            </button>
          </div>

          <!-- 事件历史显示 -->
          <div v-if="eventHistory.length > 0" class="mt-6">
            <h4 class="font-semibold text-gray-700 mb-2">最近事件历史:</h4>
            <div class="max-h-40 overflow-y-auto bg-gray-50 rounded p-3">
              <div 
                v-for="event in eventHistory.slice(0, 5)" 
                :key="event.id"
                class="text-sm mb-2 p-2 bg-white rounded border-l-4"
                :class="getEventCategoryColor(event.category)"
              >
                <div class="font-medium">{{ event.category }} - {{ getPriorityText(event.priority) }}</div>
                <div class="text-gray-600 text-xs">{{ formatTimestamp(event.timestamp) }}</div>
              </div>
            </div>
          </div>
        </div>

        <!-- 性能监控测试 -->
        <div class="bg-white rounded-lg shadow-md p-6">
          <h3 class="text-xl font-semibold text-gray-800 mb-4">
            📈 性能监控测试
          </h3>
          
          <div class="space-y-4">
            <button 
              @click="getPerformanceReport"
              class="w-full bg-indigo-500 hover:bg-indigo-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              获取性能报告
            </button>

            <button 
              @click="getSystemHealth"
              class="w-full bg-teal-500 hover:bg-teal-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              检查系统健康
            </button>

            <button 
              @click="getActiveAlerts"
              class="w-full bg-red-500 hover:bg-red-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              获取活跃警报
            </button>

            <button 
              @click="triggerSystemDiagnostics"
              class="w-full bg-yellow-500 hover:bg-yellow-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              触发系统诊断
            </button>

            <button 
              @click="resetPerformanceData"
              class="w-full bg-gray-500 hover:bg-gray-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              重置性能数据
            </button>
          </div>

          <!-- 活跃警报显示 -->
          <div v-if="activeAlerts.length > 0" class="mt-6">
            <h4 class="font-semibold text-gray-700 mb-2">活跃警报:</h4>
            <div class="space-y-2">
              <div 
                v-for="alert in activeAlerts" 
                :key="alert.id"
                class="p-3 rounded border-l-4"
                :class="getAlertSeverityColor(alert.severity)"
              >
                <div class="font-medium">{{ alert.alert_type }}</div>
                <div class="text-sm text-gray-600">{{ alert.message }}</div>
                <button 
                  @click="resolveAlert(alert.id)"
                  class="mt-2 text-xs bg-gray-200 hover:bg-gray-300 px-2 py-1 rounded"
                >
                  解决
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- 音频事件测试 -->
        <div class="bg-white rounded-lg shadow-md p-6">
          <h3 class="text-xl font-semibold text-gray-800 mb-4">
            🎵 音频事件测试
          </h3>
          
          <div class="space-y-4">
            <button 
              @click="playAudioEvent('PlayFocusStart')"
              class="w-full bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              播放专注开始音效
            </button>

            <button 
              @click="playAudioEvent('PlayFocusEnd')"
              class="w-full bg-blue-400 hover:bg-blue-500 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              播放专注结束音效
            </button>

            <button 
              @click="playAudioEvent('PlayMicroBreakStart')"
              class="w-full bg-green-500 hover:bg-green-600 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              播放微休息开始音效
            </button>

            <button 
              @click="playAudioEvent('PlayMicroBreakEnd')"
              class="w-full bg-green-400 hover:bg-green-500 text-white font-semibold py-2 px-4 rounded transition-colors"
            >
              播放微休息结束音效
            </button>

            <div class="border-t pt-4">
              <h4 class="font-semibold text-gray-700 mb-2">音量控制:</h4>
              <div class="flex items-center space-x-2">
                <input 
                  v-model="audioVolume" 
                  type="range" 
                  min="0" 
                  max="100" 
                  class="flex-1"
                  @change="setAudioVolume"
                >
                <span class="text-sm text-gray-600">{{ audioVolume }}%</span>
              </div>
            </div>

            <div class="flex space-x-2">
              <button 
                @click="muteAudio"
                class="flex-1 bg-red-500 hover:bg-red-600 text-white font-semibold py-2 px-4 rounded transition-colors"
              >
                静音
              </button>
              <button 
                @click="unmuteAudio"
                class="flex-1 bg-green-500 hover:bg-green-600 text-white font-semibold py-2 px-4 rounded transition-colors"
              >
                取消静音
              </button>
            </div>
          </div>
        </div>

        <!-- 系统诊断结果 -->
        <div class="bg-white rounded-lg shadow-md p-6">
          <h3 class="text-xl font-semibold text-gray-800 mb-4">
            🔍 系统诊断结果
          </h3>
          
          <div v-if="diagnosticsResult" class="space-y-4">
            <div class="bg-gray-50 rounded p-4">
              <h4 class="font-semibold text-gray-700 mb-2">诊断摘要:</h4>
              <p class="text-sm text-gray-600">{{ diagnosticsResult.summary || '系统运行正常' }}</p>
            </div>

            <div v-if="diagnosticsResult.recommendations && diagnosticsResult.recommendations.length > 0">
              <h4 class="font-semibold text-gray-700 mb-2">优化建议:</h4>
              <ul class="list-disc list-inside space-y-1">
                <li 
                  v-for="recommendation in diagnosticsResult.recommendations" 
                  :key="recommendation"
                  class="text-sm text-gray-600"
                >
                  {{ recommendation }}
                </li>
              </ul>
            </div>

            <div v-if="diagnosticsResult.event_queue_status">
              <h4 class="font-semibold text-gray-700 mb-2">队列状态:</h4>
              <div class="text-sm text-gray-600">
                使用率: {{ diagnosticsResult.event_queue_status.utilization.toFixed(1) }}%
                ({{ diagnosticsResult.event_queue_status.current_size }}/{{ diagnosticsResult.event_queue_status.max_size }})
              </div>
            </div>
          </div>
          <div v-else class="text-gray-500">
            点击"触发系统诊断"查看详细信息
          </div>
        </div>
      </div>

      <!-- 操作日志 -->
      <div class="mt-8 bg-white rounded-lg shadow-md p-6">
        <h3 class="text-xl font-semibold text-gray-800 mb-4">
          📝 操作日志
        </h3>
        <div class="max-h-60 overflow-y-auto bg-gray-50 rounded p-4">
          <div 
            v-for="(log, index) in operationLogs" 
            :key="index"
            class="text-sm mb-2 p-2 bg-white rounded border-l-4"
            :class="getLogTypeColor(log.type)"
          >
            <div class="flex justify-between items-start">
              <span class="font-medium">{{ log.message }}</span>
              <span class="text-xs text-gray-500">{{ formatTimestamp(log.timestamp) }}</span>
            </div>
            <div v-if="log.details" class="text-xs text-gray-600 mt-1">
              {{ log.details }}
            </div>
          </div>
        </div>
        <button 
          @click="clearLogs"
          class="mt-4 bg-gray-500 hover:bg-gray-600 text-white font-semibold py-2 px-4 rounded transition-colors"
        >
          清空日志
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 响应式数据
const systemHealth = ref(null)
const eventStats = ref(null)
const eventHistory = ref([])
const activeAlerts = ref([])
const queueStatus = ref(null)
const diagnosticsResult = ref(null)
const operationLogs = ref([])
const audioVolume = ref(50)

// 添加日志
const addLog = (message, type = 'info', details = null) => {
  operationLogs.value.unshift({
    message,
    type,
    details,
    timestamp: new Date().toISOString()
  })
  
  // 限制日志数量
  if (operationLogs.value.length > 50) {
    operationLogs.value = operationLogs.value.slice(0, 50)
  }
}

// 初始化系统
const initializeSystem = async () => {
  try {
    await invoke('init_timer_manager')
    addLog('✅ 增强系统初始化成功', 'success')
    
    // 初始化后获取基础数据
    await Promise.all([
      getSystemHealth(),
      getEventStats(),
      getQueueStatus()
    ])
  } catch (error) {
    addLog('❌ 系统初始化失败', 'error', error.toString())
  }
}

// 获取系统健康状态
const getSystemHealth = async () => {
  try {
    const health = await invoke('get_system_health')
    systemHealth.value = health
    addLog('📊 获取系统健康状态成功', 'success')
  } catch (error) {
    addLog('❌ 获取系统健康状态失败', 'error', error.toString())
  }
}

// 获取事件统计
const getEventStats = async () => {
  try {
    const stats = await invoke('get_event_stats')
    eventStats.value = stats
    addLog('📈 获取事件统计成功', 'success')
  } catch (error) {
    addLog('❌ 获取事件统计失败', 'error', error.toString())
  }
}

// 获取事件历史
const getEventHistory = async () => {
  try {
    const history = await invoke('get_event_history', { limit: 10 })
    eventHistory.value = history
    addLog(`📚 获取事件历史成功 (${history.length} 条记录)`, 'success')
  } catch (error) {
    addLog('❌ 获取事件历史失败', 'error', error.toString())
  }
}

// 获取队列状态
const getQueueStatus = async () => {
  try {
    const [currentSize, maxSize] = await invoke('get_event_queue_status')
    queueStatus.value = {
      current_size: currentSize,
      max_size: maxSize,
      utilization: (currentSize / maxSize) * 100
    }
    addLog('🔄 获取队列状态成功', 'success')
  } catch (error) {
    addLog('❌ 获取队列状态失败', 'error', error.toString())
  }
}

// 清理过期事件
const cleanupExpiredEvents = async () => {
  try {
    await invoke('cleanup_expired_events')
    addLog('🧹 清理过期事件成功', 'success')
    await getQueueStatus() // 刷新队列状态
  } catch (error) {
    addLog('❌ 清理过期事件失败', 'error', error.toString())
  }
}

// 获取性能报告
const getPerformanceReport = async () => {
  try {
    const report = await invoke('get_performance_report')
    addLog('📊 获取性能报告成功', 'success', `性能评分: ${report.health.performance_score}/100`)
  } catch (error) {
    addLog('❌ 获取性能报告失败', 'error', error.toString())
  }
}

// 获取活跃警报
const getActiveAlerts = async () => {
  try {
    const alerts = await invoke('get_active_alerts')
    activeAlerts.value = alerts
    addLog(`⚠️ 获取活跃警报成功 (${alerts.length} 个警报)`, alerts.length > 0 ? 'warning' : 'success')
  } catch (error) {
    addLog('❌ 获取活跃警报失败', 'error', error.toString())
  }
}

// 解决警报
const resolveAlert = async (alertId) => {
  try {
    await invoke('resolve_performance_alert', { alertId })
    addLog('✅ 警报已解决', 'success')
    await getActiveAlerts() // 刷新警报列表
  } catch (error) {
    addLog('❌ 解决警报失败', 'error', error.toString())
  }
}

// 触发系统诊断
const triggerSystemDiagnostics = async () => {
  try {
    const result = await invoke('trigger_system_diagnostics')
    diagnosticsResult.value = result
    addLog('🔍 系统诊断完成', 'success')
  } catch (error) {
    addLog('❌ 系统诊断失败', 'error', error.toString())
  }
}

// 重置性能数据
const resetPerformanceData = async () => {
  try {
    await invoke('reset_performance_data')
    addLog('🔄 性能数据已重置', 'success')
    await getSystemHealth() // 刷新健康状态
  } catch (error) {
    addLog('❌ 重置性能数据失败', 'error', error.toString())
  }
}

// 播放音频事件
const playAudioEvent = async (eventType) => {
  try {
    const audioEvent = { [eventType]: null }
    await invoke('play_audio_event', { 
      audioEvent, 
      priority: 'normal' 
    })
    addLog(`🎵 播放音频事件: ${eventType}`, 'success')
  } catch (error) {
    addLog(`❌ 播放音频事件失败: ${eventType}`, 'error', error.toString())
  }
}

// 设置音量
const setAudioVolume = async () => {
  try {
    const audioEvent = { 
      SetVolume: { 
        level: audioVolume.value / 100 
      } 
    }
    await invoke('play_audio_event', { 
      audioEvent, 
      priority: 'normal' 
    })
    addLog(`🔊 设置音量: ${audioVolume.value}%`, 'success')
  } catch (error) {
    addLog('❌ 设置音量失败', 'error', error.toString())
  }
}

// 静音
const muteAudio = async () => {
  try {
    const audioEvent = { Mute: null }
    await invoke('play_audio_event', { 
      audioEvent, 
      priority: 'normal' 
    })
    addLog('🔇 音频已静音', 'success')
  } catch (error) {
    addLog('❌ 静音失败', 'error', error.toString())
  }
}

// 取消静音
const unmuteAudio = async () => {
  try {
    const audioEvent = { Unmute: null }
    await invoke('play_audio_event', { 
      audioEvent, 
      priority: 'normal' 
    })
    addLog('🔊 音频已取消静音', 'success')
  } catch (error) {
    addLog('❌ 取消静音失败', 'error', error.toString())
  }
}

// 清空日志
const clearLogs = () => {
  operationLogs.value = []
  addLog('🧹 日志已清空', 'info')
}

// 样式辅助函数
const getHealthStatusColor = (status) => {
  const colors = {
    'Excellent': 'text-green-600 font-semibold',
    'Good': 'text-blue-600 font-semibold',
    'Warning': 'text-yellow-600 font-semibold',
    'Critical': 'text-red-600 font-semibold'
  }
  return colors[status] || 'text-gray-600'
}

const getUtilizationColor = (utilization) => {
  if (utilization > 80) return 'text-red-600 font-semibold'
  if (utilization > 60) return 'text-yellow-600 font-semibold'
  return 'text-green-600 font-semibold'
}

const getEventCategoryColor = (category) => {
  const colors = {
    'Timer': 'border-blue-500',
    'MicroBreak': 'border-green-500',
    'Audio': 'border-purple-500',
    'System': 'border-gray-500',
    'Notification': 'border-yellow-500',
    'Analytics': 'border-indigo-500'
  }
  return colors[category] || 'border-gray-300'
}

const getAlertSeverityColor = (severity) => {
  const colors = {
    'Info': 'border-blue-500 bg-blue-50',
    'Warning': 'border-yellow-500 bg-yellow-50',
    'Error': 'border-red-500 bg-red-50',
    'Critical': 'border-red-600 bg-red-100'
  }
  return colors[severity] || 'border-gray-300 bg-gray-50'
}

const getLogTypeColor = (type) => {
  const colors = {
    'success': 'border-green-500',
    'error': 'border-red-500',
    'warning': 'border-yellow-500',
    'info': 'border-blue-500'
  }
  return colors[type] || 'border-gray-300'
}

const getPriorityText = (priority) => {
  const priorities = {
    0: 'Low',
    1: 'Normal', 
    2: 'High',
    3: 'Critical'
  }
  return priorities[priority] || 'Unknown'
}

const formatTimestamp = (timestamp) => {
  return new Date(timestamp).toLocaleString('zh-CN')
}

// 组件挂载时初始化
onMounted(() => {
  addLog('🚀 Day 4 增强系统测试组件已加载', 'info')
})
</script>

<style scoped>
/* 自定义滚动条样式 */
.overflow-y-auto::-webkit-scrollbar {
  width: 6px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}
</style> 
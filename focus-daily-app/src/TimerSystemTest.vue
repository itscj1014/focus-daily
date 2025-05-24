<template>
  <div class="max-w-6xl mx-auto p-6">
    <h1 class="text-3xl font-bold text-gray-900 mb-8">定时器系统验证测试</h1>
    
    <!-- 系统状态显示 -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <h2 class="text-xl font-semibold mb-4">系统状态</h2>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <h3 class="font-medium text-gray-700">定时器状态</h3>
          <div v-if="timerState" class="mt-2 p-4 bg-gray-50 rounded">
            <div class="grid grid-cols-2 gap-2 text-sm">
              <div><strong>状态:</strong> {{ timerState.status }}</div>
              <div><strong>阶段:</strong> {{ timerState.phase }}</div>
              <div><strong>剩余时间:</strong> {{ formatTime(timerState.remaining_duration) }}</div>
              <div><strong>已用时间:</strong> {{ formatTime(timerState.elapsed_duration) }}</div>
              <div><strong>总时长:</strong> {{ formatTime(timerState.total_duration) }}</div>
              <div><strong>进度:</strong> {{ (timerState.elapsed_duration / timerState.total_duration * 100).toFixed(1) }}%</div>
              <div><strong>微休息次数:</strong> {{ timerState.micro_break_count }}</div>
              <div><strong>下次微休息:</strong> {{ timerState.next_micro_break_at ? formatTime(timerState.next_micro_break_at) : '未设置' }}</div>
            </div>
          </div>
          <div v-else class="mt-2 text-gray-500">未获取到状态</div>
        </div>
        
        <div>
          <h3 class="font-medium text-gray-700">循环状态</h3>
          <div class="mt-2 p-4 bg-gray-50 rounded">
            <div class="text-lg font-semibold" :class="getCycleStateColor(cycleState)">
              {{ cycleState || '未知' }}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 基本控制测试 -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <h2 class="text-xl font-semibold mb-4">基本控制测试</h2>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <h3 class="font-medium text-gray-700 mb-3">系统初始化</h3>
          <button
            @click="initTimerManager"
            class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded mr-2 mb-2"
            :disabled="loading"
          >
            初始化定时器管理器
          </button>
          <button
            @click="getTimerState"
            class="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded mr-2 mb-2"
            :disabled="loading"
          >
            获取定时器状态
          </button>
          <button
            @click="getCycleState"
            class="bg-purple-500 hover:bg-purple-600 text-white px-4 py-2 rounded mb-2"
            :disabled="loading"
          >
            获取循环状态
          </button>
        </div>

        <div>
          <h3 class="font-medium text-gray-700 mb-3">定时器控制</h3>
          <button
            @click="pauseTimer"
            class="bg-yellow-500 hover:bg-yellow-600 text-white px-4 py-2 rounded mr-2 mb-2"
            :disabled="loading"
          >
            暂停定时器
          </button>
          <button
            @click="resumeTimer"
            class="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded mr-2 mb-2"
            :disabled="loading"
          >
            恢复定时器
          </button>
          <button
            @click="resetTimer"
            class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded mb-2"
            :disabled="loading"
          >
            重置定时器
          </button>
        </div>
      </div>
    </div>

    <!-- 会话启动测试 -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <h2 class="text-xl font-semibold mb-4">会话启动测试</h2>
      <div class="grid grid-cols-3 gap-4">
        <div class="text-center">
          <h3 class="font-medium text-gray-700 mb-3">专注会话</h3>
          <button
            @click="startFocusSession"
            class="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg w-full mb-2"
            :disabled="loading"
          >
            开始90分钟专注
          </button>
          <div class="text-sm text-gray-600">
            标准的90分钟专注时段<br>
            包含随机微休息提醒
          </div>
        </div>

        <div class="text-center">
          <h3 class="font-medium text-gray-700 mb-3">长休息</h3>
          <button
            @click="startLongBreakSession"
            class="bg-green-600 hover:bg-green-700 text-white px-6 py-3 rounded-lg w-full mb-2"
            :disabled="loading"
          >
            开始20分钟长休息
          </button>
          <div class="text-sm text-gray-600">
            专注会话后的长休息<br>
            恢复注意力和精神
          </div>
        </div>

        <div class="text-center">
          <h3 class="font-medium text-gray-700 mb-3">微休息</h3>
          <button
            @click="startMicroBreakSession"
            class="bg-orange-600 hover:bg-orange-700 text-white px-6 py-3 rounded-lg w-full mb-2"
            :disabled="loading"
          >
            开始15秒微休息
          </button>
          <button
            @click="skipMicroBreak"
            class="bg-orange-400 hover:bg-orange-500 text-white px-6 py-2 rounded w-full"
            :disabled="loading"
          >
            跳过微休息
          </button>
        </div>
      </div>
    </div>

    <!-- 事件监听测试 -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <h2 class="text-xl font-semibold mb-4">事件监听 & 自动刷新</h2>
      <div class="flex items-center space-x-4 mb-4">
        <button
          @click="startAutoRefresh"
          v-if="!autoRefreshInterval"
          class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded"
        >
          开始自动刷新 (1秒)
        </button>
        <button
          @click="stopAutoRefresh"
          v-else
          class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded"
        >
          停止自动刷新
        </button>
        <span class="text-sm text-gray-600">自动获取状态更新</span>
      </div>
      
      <div class="grid grid-cols-2 gap-4">
        <div>
          <h3 class="font-medium text-gray-700 mb-2">事件日志</h3>
          <div class="bg-gray-50 p-3 rounded max-h-40 overflow-y-auto">
            <div v-if="eventLog.length === 0" class="text-gray-500 text-sm">
              暂无事件日志
            </div>
            <div
              v-for="(event, index) in eventLog"
              :key="index"
              class="text-sm mb-1 font-mono"
            >
              <span class="text-gray-500">{{ event.time }}</span>
              <span class="ml-2" :class="getEventColor(event.type)">{{ event.message }}</span>
            </div>
          </div>
        </div>

        <div>
          <h3 class="font-medium text-gray-700 mb-2">快速测试</h3>
          <button
            @click="quickTest"
            class="bg-purple-500 hover:bg-purple-600 text-white px-4 py-2 rounded mb-2 w-full"
            :disabled="loading"
          >
            快速功能测试 (15秒)
          </button>
          <div class="text-sm text-gray-600">
            启动15秒专注→暂停→恢复→完成的完整流程测试
          </div>
        </div>
      </div>
    </div>

    <!-- 错误和状态显示 -->
    <div v-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      <strong>错误：</strong> {{ error }}
      <button @click="clearError" class="ml-2 text-red-500 hover:text-red-700">✕</button>
    </div>

    <div v-if="success" class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
      <strong>成功：</strong> {{ success }}
      <button @click="clearSuccess" class="ml-2 text-green-500 hover:text-green-700">✕</button>
    </div>

    <div v-if="loading" class="text-center">
      <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
      <p class="mt-2 text-gray-600">正在处理...</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 响应式数据
const loading = ref(false)
const error = ref<string | null>(null)
const success = ref<string | null>(null)
const timerState = ref<any>(null)
const cycleState = ref<string>('')
const eventLog = ref<Array<{time: string, type: string, message: string}>>([])
const autoRefreshInterval = ref<number | null>(null)

// 工具函数
function formatTime(seconds: number): string {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const secs = seconds % 60
  
  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
  } else {
    return `${minutes}:${secs.toString().padStart(2, '0')}`
  }
}

function getCycleStateColor(state: string): string {
  switch (state) {
    case 'WaitingToStart': return 'text-gray-600'
    case 'InFocusSession': return 'text-blue-600'
    case 'InLongBreak': return 'text-green-600'
    case 'InMicroBreak': return 'text-orange-600'
    case 'Completed': return 'text-purple-600'
    default: return 'text-gray-500'
  }
}

function getEventColor(type: string): string {
  switch (type) {
    case 'success': return 'text-green-600'
    case 'error': return 'text-red-600'
    case 'info': return 'text-blue-600'
    case 'warning': return 'text-yellow-600'
    default: return 'text-gray-600'
  }
}

function addEventLog(type: string, message: string) {
  const now = new Date()
  eventLog.value.unshift({
    time: now.toLocaleTimeString(),
    type,
    message
  })
  // 只保留最近50条
  if (eventLog.value.length > 50) {
    eventLog.value = eventLog.value.slice(0, 50)
  }
}

function clearError() {
  error.value = null
}

function clearSuccess() {
  success.value = null
}

// API调用函数
async function callTimerCommand(commandName: string, params: any = {}) {
  loading.value = true
  error.value = null
  success.value = null
  
  try {
    const result = await invoke(commandName, params)
    addEventLog('success', `${commandName} 执行成功`)
    success.value = `${commandName} 执行成功`
    return result
  } catch (err) {
    const errorMsg = err instanceof Error ? err.message : `${commandName} 执行失败`
    error.value = errorMsg
    addEventLog('error', errorMsg)
    throw err
  } finally {
    loading.value = false
  }
}

// 定时器管理器初始化
async function initTimerManager() {
  await callTimerCommand('init_timer_manager')
  await getTimerState()
  await getCycleState()
}

// 获取状态
async function getTimerState() {
  const state = await callTimerCommand('get_timer_state')
  timerState.value = state
  return state
}

async function getCycleState() {
  const state = await callTimerCommand('get_cycle_state')
  cycleState.value = String(state)
  return state
}

// 会话控制
async function startFocusSession() {
  const sessionId = await callTimerCommand('start_focus_session')
  addEventLog('info', `专注会话已启动，会话ID: ${sessionId}`)
  await getTimerState()
  await getCycleState()
}

async function startLongBreakSession() {
  const sessionId = await callTimerCommand('start_long_break_session')
  addEventLog('info', `长休息已启动，会话ID: ${sessionId}`)
  await getTimerState()
  await getCycleState()
}

async function startMicroBreakSession() {
  const sessionId = await callTimerCommand('start_micro_break_session')
  addEventLog('info', `微休息已启动，会话ID: ${sessionId}`)
  await getTimerState()
  await getCycleState()
}

// 定时器控制
async function pauseTimer() {
  await callTimerCommand('pause_timer')
  await getTimerState()
}

async function resumeTimer() {
  await callTimerCommand('resume_timer')
  await getTimerState()
}

async function resetTimer() {
  await callTimerCommand('reset_timer')
  await getTimerState()
  await getCycleState()
}

async function skipMicroBreak() {
  await callTimerCommand('skip_micro_break')
  await getTimerState()
  await getCycleState()
}

// 自动刷新
function startAutoRefresh() {
  autoRefreshInterval.value = setInterval(async () => {
    try {
      await getTimerState()
      await getCycleState()
    } catch (err) {
      // 静默处理自动刷新错误
    }
  }, 1000)
}

function stopAutoRefresh() {
  if (autoRefreshInterval.value) {
    clearInterval(autoRefreshInterval.value)
    autoRefreshInterval.value = null
  }
}

// 快速测试
async function quickTest() {
  addEventLog('info', '开始快速功能测试...')
  
  try {
    // 1. 初始化
    await initTimerManager()
    await new Promise(resolve => setTimeout(resolve, 500))
    
    // 2. 启动15秒专注会话（便于测试）
    await startFocusSession()
    addEventLog('info', '已启动专注会话，等待3秒...')
    await new Promise(resolve => setTimeout(resolve, 3000))
    
    // 3. 暂停
    await pauseTimer()
    addEventLog('info', '已暂停，等待2秒...')
    await new Promise(resolve => setTimeout(resolve, 2000))
    
    // 4. 恢复
    await resumeTimer()
    addEventLog('info', '已恢复，等待2秒...')
    await new Promise(resolve => setTimeout(resolve, 2000))
    
    // 5. 重置
    await resetTimer()
    addEventLog('success', '快速测试完成！所有基本功能正常')
    
  } catch (err) {
    addEventLog('error', `快速测试失败: ${err}`)
  }
}

// 清理资源
onUnmounted(() => {
  stopAutoRefresh()
})
</script> 
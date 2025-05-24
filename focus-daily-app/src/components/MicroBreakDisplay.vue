<template>
  <div class="micro-break-container">
    <!-- 微休息提醒弹窗 -->
    <div 
      v-if="showMicroBreak" 
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      @click.self="handleBackgroundClick"
    >
      <div class="bg-white rounded-lg shadow-xl p-8 max-w-md w-full mx-4 transform transition-all duration-300">
        <!-- 微休息标题 -->
        <div class="text-center mb-6">
          <div class="w-16 h-16 bg-orange-100 rounded-full flex items-center justify-center mx-auto mb-4">
            <svg class="w-8 h-8 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707"/>
            </svg>
          </div>
          <h2 class="text-2xl font-bold text-gray-800 mb-2">微休息时间</h2>
          <p class="text-gray-600">
            第 {{ microBreakCount }} 次微休息 • {{ formatTime(remainingTime) }}
          </p>
        </div>

        <!-- 圆形进度条 -->
        <div class="relative w-32 h-32 mx-auto mb-6">
          <svg class="w-32 h-32 transform -rotate-90" viewBox="0 0 120 120">
            <circle 
              cx="60" 
              cy="60" 
              r="50" 
              stroke="#f3f4f6" 
              stroke-width="8" 
              fill="none"
            />
            <circle 
              cx="60" 
              cy="60" 
              r="50" 
              stroke="#f97316" 
              stroke-width="8" 
              fill="none"
              stroke-linecap="round"
              :stroke-dasharray="circumference"
              :stroke-dashoffset="circumference - (progress * circumference)"
              class="transition-all duration-1000 ease-linear"
            />
          </svg>
          <div class="absolute inset-0 flex items-center justify-center">
            <span class="text-2xl font-bold text-gray-800">{{ remainingTime }}</span>
          </div>
        </div>

        <!-- 休息建议 -->
        <div class="text-center mb-6">
          <p class="text-gray-700 mb-2">{{ currentRestSuggestion }}</p>
          <div class="text-sm text-gray-500">
            <span v-if="fatigueLevel > 0.5" class="text-amber-600">
              ⚠️ 检测到疲劳度较高 ({{ Math.round(fatigueLevel * 100) }}%)
            </span>
          </div>
        </div>

        <!-- 控制按钮 -->
        <div class="flex space-x-3">
          <button
            @click="completeMicroBreak"
            class="flex-1 bg-orange-600 hover:bg-orange-700 text-white font-semibold py-3 px-4 rounded-lg transition-colors"
          >
            开始休息
          </button>
          <button
            @click="skipMicroBreak"
            :disabled="!canSkip"
            :class="canSkip 
              ? 'bg-gray-200 hover:bg-gray-300 text-gray-700' 
              : 'bg-gray-100 text-gray-400 cursor-not-allowed'"
            class="flex-1 font-semibold py-3 px-4 rounded-lg transition-colors"
          >
            跳过 ({{ remainingSkips }}/{{ skipLimit }})
          </button>
        </div>

        <!-- 跳过限制提示 -->
        <div v-if="!canSkip" class="mt-4 p-3 bg-red-50 border border-red-200 rounded-lg">
          <p class="text-red-700 text-sm text-center">
            <span class="font-medium">已达到跳过限制</span>
            <br>为了您的健康，请完成这次微休息
          </p>
        </div>
      </div>
    </div>

    <!-- 微休息进行中界面 -->
    <div 
      v-if="showMicroBreakActive" 
      class="fixed inset-0 bg-gradient-to-br from-orange-50 to-orange-100 flex items-center justify-center z-50"
    >
      <div class="text-center">
        <!-- 放松动画 -->
        <div class="mb-8">
          <div class="w-24 h-24 bg-orange-200 rounded-full flex items-center justify-center mx-auto mb-4 animate-pulse">
            <svg class="w-12 h-12 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1.01M15 10h1.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
          </div>
          <h2 class="text-3xl font-bold text-orange-800 mb-2">正在休息中</h2>
          <p class="text-orange-700 text-lg">{{ formatTime(remainingTime) }} 后继续专注</p>
        </div>

        <!-- 呼吸引导 -->
        <div class="mb-8">
          <div class="w-16 h-16 bg-orange-300 rounded-full mx-auto animate-breathing"></div>
          <p class="text-orange-700 mt-4">{{ breathingText }}</p>
        </div>

        <!-- 跳过按钮 -->
        <button
          @click="skipActiveMicroBreak"
          class="bg-white bg-opacity-80 hover:bg-opacity-100 text-orange-800 font-semibold py-2 px-6 rounded-lg transition-all"
        >
          提前结束休息
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Props
interface Props {
  show?: boolean
  showActive?: boolean
  duration?: number
  remainingTime?: number
  microBreakCount?: number
  skipLimit?: number
  remainingSkips?: number
  fatigueLevel?: number
}

const props = withDefaults(defineProps<Props>(), {
  show: false,
  showActive: false,
  duration: 15,
  remainingTime: 15,
  microBreakCount: 1,
  skipLimit: 3,
  remainingSkips: 3,
  fatigueLevel: 0.0
})

// Emits
const emit = defineEmits<{
  skip: []
  complete: []
  skipActive: []
}>()

// 响应式数据
const showMicroBreak = ref(false)
const showMicroBreakActive = ref(false)
const breathingPhase = ref<'inhale' | 'exhale'>('inhale')
const breathingTimer = ref<number | null>(null)

// 计算属性
const progress = computed(() => {
  if (props.duration === 0) return 0
  return 1 - (props.remainingTime / props.duration)
})

const circumference = computed(() => 2 * Math.PI * 50)

const canSkip = computed(() => props.remainingSkips > 0)

const currentRestSuggestion = computed(() => {
  const suggestions = [
    "眨眨眼睛，放松眼部肌肉",
    "深呼吸，让肩膀放松下来",
    "轻轻转动头部，缓解颈部压力",
    "站起来伸展一下身体",
    "看看远处的景物，放松视野",
    "喝一口水，保持身体水分",
    "做几个简单的拉伸动作"
  ]
  return suggestions[props.microBreakCount % suggestions.length]
})

const breathingText = computed(() => {
  return breathingPhase.value === 'inhale' ? '缓慢吸气...' : '缓慢呼气...'
})

// 方法
function formatTime(seconds: number): string {
  if (seconds < 60) {
    return `${seconds}秒`
  }
  const minutes = Math.floor(seconds / 60)
  const remainingSeconds = seconds % 60
  return remainingSeconds > 0 ? `${minutes}分${remainingSeconds}秒` : `${minutes}分钟`
}

function handleBackgroundClick() {
  // 防止意外点击关闭，需要明确选择操作
}

async function skipMicroBreak() {
  if (!canSkip.value) return
  
  try {
    await invoke('skip_micro_break')
    emit('skip')
    showMicroBreak.value = false
  } catch (error) {
    console.error('跳过微休息失败:', error)
  }
}

async function completeMicroBreak() {
  try {
    emit('complete')
    showMicroBreak.value = false
    showMicroBreakActive.value = true
    startBreathingGuide()
  } catch (error) {
    console.error('开始微休息失败:', error)
  }
}

async function skipActiveMicroBreak() {
  try {
    await invoke('skip_micro_break')
    emit('skipActive')
    showMicroBreakActive.value = false
    stopBreathingGuide()
  } catch (error) {
    console.error('跳过活动微休息失败:', error)
  }
}

function startBreathingGuide() {
  breathingTimer.value = setInterval(() => {
    breathingPhase.value = breathingPhase.value === 'inhale' ? 'exhale' : 'inhale'
  }, 4000) // 4秒切换一次，吸气4秒，呼气4秒
}

function stopBreathingGuide() {
  if (breathingTimer.value) {
    clearInterval(breathingTimer.value)
    breathingTimer.value = null
  }
}

// 监听 props 变化
function updateDisplayState() {
  showMicroBreak.value = props.show
  showMicroBreakActive.value = props.showActive
  
  if (props.showActive) {
    startBreathingGuide()
  } else {
    stopBreathingGuide()
  }
}

// 生命周期
onMounted(() => {
  updateDisplayState()
})

onUnmounted(() => {
  stopBreathingGuide()
})

// 暴露方法给父组件
defineExpose({
  showPrompt: () => showMicroBreak.value = true,
  hidePrompt: () => showMicroBreak.value = false,
  showActive: () => {
    showMicroBreakActive.value = true
    startBreathingGuide()
  },
  hideActive: () => {
    showMicroBreakActive.value = false
    stopBreathingGuide()
  }
})
</script>

<style scoped>
@keyframes breathing {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.2);
  }
}

.animate-breathing {
  animation: breathing 4s ease-in-out infinite;
}

/* 平滑的进度条动画 */
.transition-all {
  transition-property: stroke-dashoffset;
}
</style> 
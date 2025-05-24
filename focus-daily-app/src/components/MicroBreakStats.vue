<template>
  <div class="micro-break-stats bg-white rounded-lg shadow p-6">
    <h3 class="text-lg font-semibold text-gray-800 mb-4 flex items-center">
      <svg class="w-5 h-5 text-orange-600 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
      </svg>
      微休息统计
    </h3>

    <!-- 主要统计指标 -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
      <div class="text-center p-3 bg-green-50 rounded-lg">
        <div class="text-2xl font-bold text-green-600">{{ stats.completed }}</div>
        <div class="text-sm text-green-700">已完成</div>
      </div>
      
      <div class="text-center p-3 bg-orange-50 rounded-lg">
        <div class="text-2xl font-bold text-orange-600">{{ stats.skipped }}</div>
        <div class="text-sm text-orange-700">已跳过</div>
      </div>
      
      <div class="text-center p-3 bg-blue-50 rounded-lg">
        <div class="text-2xl font-bold text-blue-600">{{ Math.round(stats.completion_rate * 100) }}%</div>
        <div class="text-sm text-blue-700">完成率</div>
      </div>
      
      <div class="text-center p-3 bg-purple-50 rounded-lg">
        <div class="text-2xl font-bold text-purple-600">{{ stats.total_triggered }}</div>
        <div class="text-sm text-purple-700">总触发</div>
      </div>
    </div>

    <!-- 完成率图表 -->
    <div class="mb-6">
      <div class="flex justify-between items-center mb-2">
        <span class="text-sm font-medium text-gray-700">完成率趋势</span>
        <span class="text-sm text-gray-500">{{ getCompletionRateLevel() }}</span>
      </div>
      
      <div class="w-full bg-gray-200 rounded-full h-3">
        <div 
          class="h-3 rounded-full transition-all duration-500"
          :class="getCompletionRateColor()"
          :style="{ width: `${Math.round(stats.completion_rate * 100)}%` }"
        ></div>
      </div>
      
      <div class="flex justify-between text-xs text-gray-500 mt-1">
        <span>0%</span>
        <span>50%</span>
        <span>100%</span>
      </div>
    </div>

    <!-- 疲劳度监测 -->
    <div class="mb-6">
      <div class="flex justify-between items-center mb-2">
        <span class="text-sm font-medium text-gray-700">疲劳度监测</span>
        <span class="text-sm" :class="getFatigueTextColor()">
          {{ getFatigueLevel() }}
        </span>
      </div>
      
      <div class="w-full bg-gray-200 rounded-full h-3">
        <div 
          class="h-3 rounded-full transition-all duration-500"
          :class="getFatigueColor()"
          :style="{ width: `${Math.round(stats.fatigue_level * 100)}%` }"
        ></div>
      </div>
      
      <div class="flex justify-between text-xs text-gray-500 mt-1">
        <span>轻松</span>
        <span>适中</span>
        <span>疲劳</span>
      </div>
    </div>

    <!-- 智能调度状态 -->
    <div class="mb-6">
      <div class="flex justify-between items-center mb-3">
        <span class="text-sm font-medium text-gray-700">智能调度</span>
        <span class="text-xs px-2 py-1 rounded" :class="getAdjustmentBadgeClass()">
          {{ getAdjustmentStatus() }}
        </span>
      </div>
      
      <div class="grid grid-cols-2 gap-4">
        <div class="text-center p-2 bg-gray-50 rounded">
          <div class="text-lg font-semibold text-gray-800">{{ adjustmentFactorDisplay }}</div>
          <div class="text-xs text-gray-600">调整因子</div>
        </div>
        
        <div class="text-center p-2 bg-gray-50 rounded">
          <div class="text-lg font-semibold text-gray-800">{{ nextBreakDisplay }}</div>
          <div class="text-xs text-gray-600">下次休息</div>
        </div>
      </div>
    </div>

    <!-- 效果评估 -->
    <div class="border-t pt-4">
      <h4 class="text-sm font-medium text-gray-700 mb-3">效果评估</h4>
      
      <div class="space-y-3">
        <!-- 配合度评估 -->
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-600">配合度</span>
          <div class="flex items-center space-x-2">
            <div class="flex space-x-1">
              <div v-for="i in 5" :key="i" 
                   :class="i <= getCooperationStars() ? 'text-yellow-400' : 'text-gray-300'"
                   class="text-sm">★</div>
            </div>
            <span class="text-xs text-gray-500">({{ getCooperationStars() }}/5)</span>
          </div>
        </div>

        <!-- 健康建议 -->
        <div class="p-3 bg-blue-50 border border-blue-200 rounded-lg">
          <div class="flex items-start space-x-2">
            <svg class="w-4 h-4 text-blue-600 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            <div>
              <div class="text-sm font-medium text-blue-800">健康建议</div>
              <div class="text-xs text-blue-700 mt-1">{{ getHealthRecommendation() }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

// Props
interface MicroBreakStatsData {
  total_triggered: number
  completed: number
  skipped: number
  completion_rate: number
  fatigue_level: number
  adjustment_factor: number
  next_break_at: number | null
}

interface Props {
  stats: MicroBreakStatsData
  currentElapsed?: number
}

const props = withDefaults(defineProps<Props>(), {
  currentElapsed: 0
})

// 计算属性
const adjustmentFactorDisplay = computed(() => {
  const factor = props.stats.adjustment_factor
  if (factor > 1.1) return '已延长'
  if (factor < 0.9) return '已缩短'
  return '正常'
})

const nextBreakDisplay = computed(() => {
  if (!props.stats.next_break_at) return '--'
  
  const remaining = props.stats.next_break_at - props.currentElapsed
  if (remaining <= 0) return '即将到来'
  
  const minutes = Math.floor(remaining / 60)
  const seconds = remaining % 60
  
  if (minutes > 0) {
    return `${minutes}分${seconds}秒`
  }
  return `${seconds}秒`
})

// 方法
function getCompletionRateLevel(): string {
  const rate = props.stats.completion_rate
  if (rate >= 0.8) return '优秀'
  if (rate >= 0.6) return '良好'
  if (rate >= 0.4) return '一般'
  return '需改善'
}

function getCompletionRateColor(): string {
  const rate = props.stats.completion_rate
  if (rate >= 0.8) return 'bg-green-500'
  if (rate >= 0.6) return 'bg-blue-500'
  if (rate >= 0.4) return 'bg-yellow-500'
  return 'bg-red-500'
}

function getFatigueLevel(): string {
  const level = props.stats.fatigue_level
  if (level >= 0.7) return '高疲劳'
  if (level >= 0.4) return '中疲劳'
  if (level >= 0.2) return '轻疲劳'
  return '良好'
}

function getFatigueColor(): string {
  const level = props.stats.fatigue_level
  if (level >= 0.7) return 'bg-red-500'
  if (level >= 0.4) return 'bg-yellow-500'
  if (level >= 0.2) return 'bg-blue-500'
  return 'bg-green-500'
}

function getFatigueTextColor(): string {
  const level = props.stats.fatigue_level
  if (level >= 0.7) return 'text-red-600'
  if (level >= 0.4) return 'text-yellow-600'
  if (level >= 0.2) return 'text-blue-600'
  return 'text-green-600'
}

function getAdjustmentStatus(): string {
  const factor = props.stats.adjustment_factor
  if (factor > 1.1) return '自动延长'
  if (factor < 0.9) return '自动缩短'
  return '标准模式'
}

function getAdjustmentBadgeClass(): string {
  const factor = props.stats.adjustment_factor
  if (factor > 1.1) return 'bg-green-100 text-green-800'
  if (factor < 0.9) return 'bg-orange-100 text-orange-800'
  return 'bg-blue-100 text-blue-800'
}

function getCooperationStars(): number {
  const rate = props.stats.completion_rate
  if (rate >= 0.9) return 5
  if (rate >= 0.7) return 4
  if (rate >= 0.5) return 3
  if (rate >= 0.3) return 2
  return 1
}

function getHealthRecommendation(): string {
  const rate = props.stats.completion_rate
  const fatigue = props.stats.fatigue_level
  
  if (fatigue >= 0.7) {
    return '疲劳度较高，建议增加休息频率，多做眼部和肩颈放松运动。'
  }
  
  if (rate < 0.5) {
    return '微休息完成率偏低，建议调整工作强度，重视眼部健康保护。'
  }
  
  if (rate >= 0.8 && fatigue < 0.3) {
    return '休息习惯很好！继续保持这种健康的工作节奏。'
  }
  
  return '保持规律的微休息，有助于提高专注效率和眼部健康。'
}
</script> 
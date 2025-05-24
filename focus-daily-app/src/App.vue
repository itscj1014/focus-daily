<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { UserSettings, FocusSession, AudioConfig, TodayStats } from './types/models';
import { SessionType, AudioType } from './types/models';
import {
  getUserSettings,
  updateUserSettings,
  resetUserSettingsToDefault,
  createFocusSession,
  completeFocusSession,
  getRecentFocusSessions,
  getTodayStats,
  deleteFocusSession,
  getAllAudioConfigs,
  toggleAudioConfig
} from './api';
import TimerSystemTest from './TimerSystemTest.vue';

const greetMsg = ref("");
const name = ref("");

// 测试模式切换
const testMode = ref<'data' | 'timer'>('timer');

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

// 响应式数据
const loading = ref(false);
const error = ref<string | null>(null);
const userSettings = ref<UserSettings | null>(null);
const sessionHistory = ref<FocusSession[]>([]);
const todayStats = ref<TodayStats | null>(null);
const audioConfigs = ref<AudioConfig[]>([]);

// 用户设置测试函数
async function testLoadUserSettings() {
  loading.value = true;
  error.value = null;
  try {
    userSettings.value = await getUserSettings();
  } catch (err) {
    error.value = err instanceof Error ? err.message : '加载用户设置失败';
  } finally {
    loading.value = false;
  }
}

async function testUpdateUserSettings() {
  loading.value = true;
  error.value = null;
  try {
    userSettings.value = await updateUserSettings({
      focus_duration_minutes: 60,
      notifications_enabled: true
    });
  } catch (err) {
    error.value = err instanceof Error ? err.message : '更新用户设置失败';
  } finally {
    loading.value = false;
  }
}

async function testResetUserSettings() {
  loading.value = true;
  error.value = null;
  try {
    userSettings.value = await resetUserSettingsToDefault();
  } catch (err) {
    error.value = err instanceof Error ? err.message : '重置用户设置失败';
  } finally {
    loading.value = false;
  }
}

// 专注会话测试函数
async function testCreateFocusSession() {
  loading.value = true;
  error.value = null;
  try {
    const newSession = await createFocusSession({
      session_type: SessionType.Focus,
      duration_seconds: 90 * 60 // 90分钟
    });
    sessionHistory.value.unshift(newSession);
  } catch (err) {
    error.value = err instanceof Error ? err.message : '创建专注会话失败';
  } finally {
    loading.value = false;
  }
}

async function testCreateMicroBreak() {
  loading.value = true;
  error.value = null;
  try {
    const newSession = await createFocusSession({
      session_type: SessionType.MicroBreak,
      duration_seconds: 15 // 15秒，便于测试
    });
    sessionHistory.value.unshift(newSession);
  } catch (err) {
    error.value = err instanceof Error ? err.message : '创建微休息失败';
  } finally {
    loading.value = false;
  }
}

async function testLoadSessionHistory() {
  loading.value = true;
  error.value = null;
  try {
    sessionHistory.value = await getRecentFocusSessions(10);
  } catch (err) {
    error.value = err instanceof Error ? err.message : '加载会话历史失败';
  } finally {
    loading.value = false;
  }
}

async function testGetTodayStats() {
  loading.value = true;
  error.value = null;
  try {
    todayStats.value = await getTodayStats();
  } catch (err) {
    error.value = err instanceof Error ? err.message : '获取今日统计失败';
  } finally {
    loading.value = false;
  }
}

async function testCompleteSession(sessionId: string) {
  loading.value = true;
  error.value = null;
  try {
    await completeFocusSession(sessionId);
    await testLoadSessionHistory();
    await testGetTodayStats();
  } catch (err) {
    error.value = err instanceof Error ? err.message : '完成会话失败';
  } finally {
    loading.value = false;
  }
}

async function testDeleteSession(sessionId: string) {
  loading.value = true;
  error.value = null;
  try {
    await deleteFocusSession(sessionId);
    sessionHistory.value = sessionHistory.value.filter(s => s.id !== sessionId);
    await testGetTodayStats();
  } catch (err) {
    error.value = err instanceof Error ? err.message : '删除会话失败';
  } finally {
    loading.value = false;
  }
}

// 音频配置测试函数
async function testLoadAudioConfigs() {
  loading.value = true;
  error.value = null;
  try {
    audioConfigs.value = await getAllAudioConfigs();
  } catch (err) {
    error.value = err instanceof Error ? err.message : '加载音频配置失败';
  } finally {
    loading.value = false;
  }
}

async function testToggleAudio() {
  loading.value = true;
  error.value = null;
  try {
    if (audioConfigs.value.length > 0) {
      const config = audioConfigs.value[0];
      await toggleAudioConfig(config.config_type, !config.enabled);
      await testLoadAudioConfigs();
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : '切换音频配置失败';
  } finally {
    loading.value = false;
  }
}

// 工具函数
function getSessionTypeText(type: SessionType): string {
  switch (type) {
    case SessionType.Focus:
      return '专注时间';
    case SessionType.LongBreak:
      return '长休息';
    case SessionType.MicroBreak:
      return '微休息';
    default:
      return '未知';
  }
}

function getAudioTypeText(type: AudioType): string {
  switch (type) {
    case AudioType.FocusStart:
      return '专注开始';
    case AudioType.FocusEnd:
      return '专注结束';
    case AudioType.LongBreakStart:
      return '长休息开始';
    case AudioType.LongBreakEnd:
      return '长休息结束';
    case AudioType.MicroBreakStart:
      return '微休息开始';
    default:
      return '未知';
  }
}

function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;
  
  if (hours > 0) {
    return `${hours}小时${minutes}分钟`;
  } else if (minutes > 0) {
    return `${minutes}分钟${secs}秒`;
  } else {
    return `${secs}秒`;
  }
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleString('zh-CN');
}

function clearError() {
  error.value = null;
}
</script>

<template>
  <div class="min-h-screen bg-gray-100 p-8">
    <div class="max-w-4xl mx-auto">
      <!-- 测试模式切换 -->
      <div class="bg-white rounded-lg shadow p-4 mb-6">
        <div class="flex items-center justify-between">
          <h1 class="text-3xl font-bold text-gray-900">Focus Daily - 功能验证测试</h1>
          <div class="flex space-x-2">
            <button
              @click="testMode = 'timer'"
              :class="testMode === 'timer' ? 'bg-blue-600 text-white' : 'bg-gray-200 text-gray-700'"
              class="px-4 py-2 rounded font-medium transition-colors"
            >
              定时器系统测试
            </button>
            <button
              @click="testMode = 'data'"
              :class="testMode === 'data' ? 'bg-blue-600 text-white' : 'bg-gray-200 text-gray-700'"
              class="px-4 py-2 rounded font-medium transition-colors"
            >
              数据层测试
            </button>
          </div>
        </div>
        <p class="text-gray-600 mt-2">
          {{ testMode === 'timer' ? '测试定时器核心功能：90分钟专注、20分钟长休息、微休息系统' : '测试数据库操作：用户设置、会话管理、音频配置' }}
        </p>
      </div>

      <!-- 定时器系统测试 -->
      <TimerSystemTest v-if="testMode === 'timer'" />

      <!-- 数据层测试 -->
      <div v-else>
        <h2 class="text-2xl font-bold text-gray-800 mb-6">数据层功能测试</h2>
        <div class="bg-white rounded-lg shadow p-6 mb-6">
          <h2 class="text-xl font-semibold mb-4">用户设置测试</h2>
          <div class="space-y-4">
            <button
              @click="testLoadUserSettings"
              class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded mr-2"
              :disabled="loading"
            >
              加载用户设置
            </button>
            <button
              @click="testUpdateUserSettings"
              class="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded mr-2"
              :disabled="loading"
            >
              更新设置
            </button>
            <button
              @click="testResetUserSettings"
              class="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded"
              :disabled="loading"
            >
              重置设置
            </button>
          </div>
          <div v-if="userSettings" class="mt-4 p-4 bg-gray-50 rounded">
            <pre class="text-sm">{{ JSON.stringify(userSettings, null, 2) }}</pre>
          </div>
        </div>

        <!-- 专注会话测试 -->
        <div class="bg-white rounded-lg shadow p-6 mb-6">
          <h2 class="text-xl font-semibold mb-4">专注会话测试</h2>
          <div class="space-y-4">
            <button
              @click="testCreateFocusSession"
              class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded mr-2"
              :disabled="loading"
            >
              创建90分钟专注会话
            </button>
            <button
              @click="testCreateMicroBreak"
              class="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded mr-2"
              :disabled="loading"
            >
              创建15秒微休息
            </button>
            <button
              @click="testLoadSessionHistory"
              class="bg-purple-500 hover:bg-purple-600 text-white px-4 py-2 rounded mr-2"
              :disabled="loading"
            >
              加载会话历史
            </button>
            <button
              @click="testGetTodayStats"
              class="bg-yellow-500 hover:bg-yellow-600 text-white px-4 py-2 rounded"
              :disabled="loading"
            >
              获取今日统计
            </button>
          </div>
          
          <div v-if="sessionHistory.length > 0" class="mt-4">
            <h3 class="text-lg font-medium mb-2">会话历史</h3>
            <div class="space-y-2">
              <div
                v-for="session in sessionHistory"
                :key="session.id"
                class="p-3 bg-gray-50 rounded flex justify-between items-center"
              >
                <div>
                  <span class="font-medium">{{ getSessionTypeText(session.session_type) }}</span>
                  <span class="text-gray-600 ml-2">{{ formatDuration(session.duration_seconds) }}</span>
                  <span class="text-sm text-gray-500 ml-2">{{ formatDate(session.created_at) }}</span>
                </div>
                <div class="flex space-x-2">
                  <button
                    v-if="!session.completed"
                    @click="testCompleteSession(session.id)"
                    class="bg-green-500 hover:bg-green-600 text-white px-3 py-1 rounded text-sm"
                    :disabled="loading"
                  >
                    完成
                  </button>
                  <button
                    @click="testDeleteSession(session.id)"
                    class="bg-red-500 hover:bg-red-600 text-white px-3 py-1 rounded text-sm"
                    :disabled="loading"
                  >
                    删除
                  </button>
                </div>
              </div>
            </div>
          </div>

          <div v-if="todayStats" class="mt-4 p-4 bg-blue-50 rounded">
            <h3 class="text-lg font-medium mb-2">今日统计</h3>
            <div class="grid grid-cols-3 gap-4">
              <div>
                <div class="text-2xl font-bold text-blue-600">{{ todayStats.focus_count }}</div>
                <div class="text-sm text-gray-600">专注次数</div>
              </div>
              <div>
                <div class="text-2xl font-bold text-green-600">{{ todayStats.break_count }}</div>
                <div class="text-sm text-gray-600">休息次数</div>
              </div>
              <div>
                <div class="text-2xl font-bold text-purple-600">{{ formatDuration(todayStats.total_focus_time) }}</div>
                <div class="text-sm text-gray-600">总专注时间</div>
              </div>
            </div>
          </div>
        </div>

        <!-- 音频配置测试 -->
        <div class="bg-white rounded-lg shadow p-6 mb-6">
          <h2 class="text-xl font-semibold mb-4">音频配置测试</h2>
          <div class="space-y-4">
            <button
              @click="testLoadAudioConfigs"
              class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded mr-2"
              :disabled="loading"
            >
              加载音频配置
            </button>
            <button
              @click="testToggleAudio"
              class="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded"
              :disabled="loading"
            >
              切换音频开关
            </button>
          </div>
          <div v-if="audioConfigs.length > 0" class="mt-4 space-y-2">
            <div
              v-for="config in audioConfigs"
              :key="config.id"
              class="p-3 bg-gray-50 rounded flex justify-between items-center"
            >
              <div>
                <span class="font-medium">{{ getAudioTypeText(config.config_type) }}</span>
                <span class="text-gray-600 ml-2">音量: {{ config.volume }}</span>
                <span :class="config.enabled ? 'text-green-600' : 'text-red-600'" class="ml-2">
                  {{ config.enabled ? '已启用' : '已禁用' }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- 错误显示 -->
        <div v-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          <strong>错误：</strong> {{ error }}
          <button @click="clearError" class="ml-2 text-red-500 hover:text-red-700">
            ✕
          </button>
        </div>

        <!-- 加载指示器 -->
        <div v-if="loading" class="text-center">
          <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
          <p class="mt-2 text-gray-600">正在处理...</p>
        </div>
      </div>
    </div>
  </div>
</template>
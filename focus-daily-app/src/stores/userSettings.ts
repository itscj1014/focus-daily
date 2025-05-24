import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { UserSettings, UpdateUserSettings, Theme } from '../types/models';
import { getUserSettings, updateUserSettings, resetUserSettingsToDefault } from '../api';

export const useUserSettingsStore = defineStore('userSettings', () => {
  // 状态
  const settings = ref<UserSettings | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // 计算属性
  const isLoaded = computed(() => settings.value !== null);
  const currentTheme = computed(() => settings.value?.theme || 'System');
  const focusDuration = computed(() => settings.value?.focus_duration_minutes || 90);
  const longBreakDuration = computed(() => settings.value?.long_break_duration_minutes || 20);
  const microBreakMinInterval = computed(() => settings.value?.micro_break_min_interval_minutes || 3);
  const microBreakMaxInterval = computed(() => settings.value?.micro_break_max_interval_minutes || 5);
  const microBreakDuration = computed(() => settings.value?.micro_break_duration_seconds || 15);
  const notificationsEnabled = computed(() => settings.value?.notifications_enabled || true);
  const autoStart = computed(() => settings.value?.auto_start || false);
  const language = computed(() => settings.value?.language || 'zh-CN');

  // 动作
  async function loadSettings() {
    loading.value = true;
    error.value = null;
    
    try {
      settings.value = await getUserSettings();
    } catch (err) {
      error.value = err instanceof Error ? err.message : '加载设置失败';
      console.error('加载用户设置失败:', err);
    } finally {
      loading.value = false;
    }
  }

  async function updateSettings(updates: UpdateUserSettings) {
    loading.value = true;
    error.value = null;
    
    try {
      settings.value = await updateUserSettings(updates);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '更新设置失败';
      console.error('更新用户设置失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function resetToDefault() {
    loading.value = true;
    error.value = null;
    
    try {
      settings.value = await resetUserSettingsToDefault();
    } catch (err) {
      error.value = err instanceof Error ? err.message : '重置设置失败';
      console.error('重置用户设置失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  // 具体设置更新方法
  async function updateTheme(theme: Theme) {
    await updateSettings({ theme });
  }

  async function updateLanguage(language: string) {
    await updateSettings({ language });
  }

  async function updateAutoStart(autoStart: boolean) {
    await updateSettings({ auto_start: autoStart });
  }

  async function updateFocusDuration(minutes: number) {
    await updateSettings({ focus_duration_minutes: minutes });
  }

  async function updateLongBreakDuration(minutes: number) {
    await updateSettings({ long_break_duration_minutes: minutes });
  }

  async function updateMicroBreakInterval(minMinutes: number, maxMinutes: number) {
    await updateSettings({ 
      micro_break_min_interval_minutes: minMinutes,
      micro_break_max_interval_minutes: maxMinutes
    });
  }

  async function updateMicroBreakDuration(seconds: number) {
    await updateSettings({ micro_break_duration_seconds: seconds });
  }

  async function updateNotifications(enabled: boolean) {
    await updateSettings({ notifications_enabled: enabled });
  }

  // 清除错误
  function clearError() {
    error.value = null;
  }

  return {
    // 状态
    settings,
    loading,
    error,
    
    // 计算属性
    isLoaded,
    currentTheme,
    focusDuration,
    longBreakDuration,
    microBreakMinInterval,
    microBreakMaxInterval,
    microBreakDuration,
    notificationsEnabled,
    autoStart,
    language,
    
    // 动作
    loadSettings,
    updateSettings,
    resetToDefault,
    updateTheme,
    updateLanguage,
    updateAutoStart,
    updateFocusDuration,
    updateLongBreakDuration,
    updateMicroBreakInterval,
    updateMicroBreakDuration,
    updateNotifications,
    clearError
  };
}); 
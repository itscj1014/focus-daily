import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { 
  FocusSession, 
  CreateFocusSession, 
  TodayStats 
} from '../types/models';
import { SessionType, SessionStatus } from '../types/models';
import { 
  createFocusSession, 
  completeFocusSession,
  getRecentFocusSessions,
  getTodayStats,
  deleteFocusSession
} from '../api';

export const useFocusSessionStore = defineStore('focusSession', () => {
  // 状态
  const currentSession = ref<FocusSession | null>(null);
  const sessionHistory = ref<FocusSession[]>([]);
  const todayStats = ref<TodayStats | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  
  // 定时器相关状态
  const sessionStatus = ref<SessionStatus>(SessionStatus.Idle);
  const remainingSeconds = ref(0);
  const timerId = ref<number | null>(null);

  // 计算属性
  const isSessionActive = computed(() => currentSession.value !== null);
  const isRunning = computed(() => sessionStatus.value === SessionStatus.Running);
  const isPaused = computed(() => sessionStatus.value === SessionStatus.Paused);
  const isCompleted = computed(() => sessionStatus.value === SessionStatus.Completed);
  const sessionProgress = computed(() => {
    if (!currentSession.value) return 0;
    const elapsed = currentSession.value.duration_seconds - remainingSeconds.value;
    return Math.min(100, (elapsed / currentSession.value.duration_seconds) * 100);
  });

  const formattedRemainingTime = computed(() => {
    const minutes = Math.floor(remainingSeconds.value / 60);
    const seconds = remainingSeconds.value % 60;
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  });

  const sessionTypeText = computed(() => {
    if (!currentSession.value) return '';
    switch (currentSession.value.session_type) {
      case SessionType.Focus:
        return '专注时间';
      case SessionType.LongBreak:
        return '长休息';
      case SessionType.MicroBreak:
        return '微休息';
      default:
        return '';
    }
  });

  // 动作
  async function startSession(input: CreateFocusSession) {
    loading.value = true;
    error.value = null;
    
    try {
      currentSession.value = await createFocusSession(input);
      remainingSeconds.value = currentSession.value.duration_seconds;
      sessionStatus.value = SessionStatus.Running;
      startTimer();
    } catch (err) {
      error.value = err instanceof Error ? err.message : '启动会话失败';
      console.error('启动会话失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  function pauseSession() {
    if (sessionStatus.value === SessionStatus.Running) {
      sessionStatus.value = SessionStatus.Paused;
      stopTimer();
    }
  }

  function resumeSession() {
    if (sessionStatus.value === SessionStatus.Paused) {
      sessionStatus.value = SessionStatus.Running;
      startTimer();
    }
  }

  async function completeCurrentSession() {
    if (!currentSession.value) return;
    
    loading.value = true;
    error.value = null;
    
    try {
      const completedSession = await completeFocusSession(currentSession.value.id);
      sessionStatus.value = SessionStatus.Completed;
      stopTimer();
      
      // 添加到历史记录
      sessionHistory.value.unshift(completedSession);
      
      // 重置当前会话
      currentSession.value = null;
      remainingSeconds.value = 0;
      
      // 刷新今日统计
      await loadTodayStats();
    } catch (err) {
      error.value = err instanceof Error ? err.message : '完成会话失败';
      console.error('完成会话失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  function stopSession() {
    sessionStatus.value = SessionStatus.Idle;
    stopTimer();
    currentSession.value = null;
    remainingSeconds.value = 0;
  }

  function startTimer() {
    if (timerId.value) return;
    
    timerId.value = window.setInterval(() => {
      if (remainingSeconds.value > 0) {
        remainingSeconds.value--;
      } else {
        // 时间到，自动完成会话
        completeCurrentSession();
      }
    }, 1000);
  }

  function stopTimer() {
    if (timerId.value) {
      clearInterval(timerId.value);
      timerId.value = null;
    }
  }

  async function loadSessionHistory(limit: number = 20) {
    loading.value = true;
    error.value = null;
    
    try {
      sessionHistory.value = await getRecentFocusSessions(limit);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '加载会话历史失败';
      console.error('加载会话历史失败:', err);
    } finally {
      loading.value = false;
    }
  }

  async function loadTodayStats() {
    try {
      todayStats.value = await getTodayStats();
    } catch (err) {
      console.error('加载今日统计失败:', err);
    }
  }

  async function deleteSession(sessionId: string) {
    loading.value = true;
    error.value = null;
    
    try {
      await deleteFocusSession(sessionId);
      
      // 从历史记录中移除
      sessionHistory.value = sessionHistory.value.filter(s => s.id !== sessionId);
      
      // 刷新今日统计
      await loadTodayStats();
    } catch (err) {
      error.value = err instanceof Error ? err.message : '删除会话失败';
      console.error('删除会话失败:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  // 快捷启动方法
  async function startFocusSession(durationMinutes: number = 90) {
    await startSession({
      session_type: SessionType.Focus,
      duration_seconds: durationMinutes * 60
    });
  }

  async function startLongBreak(durationMinutes: number = 20) {
    await startSession({
      session_type: SessionType.LongBreak,
      duration_seconds: durationMinutes * 60
    });
  }

  async function startMicroBreak(durationSeconds: number = 15) {
    await startSession({
      session_type: SessionType.MicroBreak,
      duration_seconds: durationSeconds
    });
  }

  function clearError() {
    error.value = null;
  }

  // 清理函数，在组件卸载时调用
  function cleanup() {
    stopTimer();
  }

  return {
    // 状态
    currentSession,
    sessionHistory,
    todayStats,
    loading,
    error,
    sessionStatus,
    remainingSeconds,
    
    // 计算属性
    isSessionActive,
    isRunning,
    isPaused,
    isCompleted,
    sessionProgress,
    formattedRemainingTime,
    sessionTypeText,
    
    // 动作
    startSession,
    pauseSession,
    resumeSession,
    completeCurrentSession,
    stopSession,
    loadSessionHistory,
    loadTodayStats,
    deleteSession,
    startFocusSession,
    startLongBreak,
    startMicroBreak,
    clearError,
    cleanup
  };
}); 
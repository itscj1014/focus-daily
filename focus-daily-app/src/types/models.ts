// 会话类型枚举
export enum SessionType {
  Focus = "Focus",
  LongBreak = "LongBreak",
  MicroBreak = "MicroBreak"
}

// 主题枚举
export enum Theme {
  Light = "Light",
  Dark = "Dark",
  System = "System"
}

// 音频类型枚举
export enum AudioType {
  FocusStart = "FocusStart",
  FocusEnd = "FocusEnd",
  LongBreakStart = "LongBreakStart",
  LongBreakEnd = "LongBreakEnd",
  MicroBreakStart = "MicroBreakStart"
}

// 专注会话接口
export interface FocusSession {
  id: string;
  start_time: string;
  end_time?: string;
  duration_seconds: number;
  session_type: SessionType;
  completed: boolean;
  created_at: string;
  updated_at: string;
}

// 创建专注会话输入
export interface CreateFocusSession {
  session_type: SessionType;
  duration_seconds: number;
}

// 用户设置接口
export interface UserSettings {
  id: string;
  theme: Theme;
  language: string;
  auto_start: boolean;
  focus_duration_minutes: number;
  long_break_duration_minutes: number;
  micro_break_min_interval_minutes: number;
  micro_break_max_interval_minutes: number;
  micro_break_duration_seconds: number;
  notifications_enabled: boolean;
  created_at: string;
  updated_at: string;
}

// 更新用户设置输入
export interface UpdateUserSettings {
  theme?: Theme;
  language?: string;
  auto_start?: boolean;
  focus_duration_minutes?: number;
  long_break_duration_minutes?: number;
  micro_break_min_interval_minutes?: number;
  micro_break_max_interval_minutes?: number;
  micro_break_duration_seconds?: number;
  notifications_enabled?: boolean;
}

// 音频配置接口
export interface AudioConfig {
  id: string;
  config_type: AudioType;
  file_path?: string;
  is_default: boolean;
  volume: number;
  enabled: boolean;
  created_at: string;
  updated_at: string;
}

// 创建音频配置输入
export interface CreateAudioConfig {
  config_type: AudioType;
  file_path?: string;
  volume: number;
  enabled: boolean;
}

// 今日统计数据
export interface TodayStats {
  focus_count: number;
  break_count: number;
  total_focus_time: number; // 秒
}

// API响应通用类型
export type ApiResponse<T> = {
  success: boolean;
  data?: T;
  error?: string;
};

// 会话状态枚举
export enum SessionStatus {
  Idle = "Idle",
  Running = "Running",
  Paused = "Paused",
  Completed = "Completed"
} 
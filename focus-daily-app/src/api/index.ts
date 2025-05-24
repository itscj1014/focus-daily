import Database from '@tauri-apps/plugin-sql';
import type {
  FocusSession,
  CreateFocusSession,
  UserSettings,
  UpdateUserSettings,
  AudioConfig,
  CreateAudioConfig,
  AudioType,
  TodayStats,
  SessionType
} from '../types/models';

// 数据库实例
let db: Database | null = null;

/**
 * 获取数据库连接
 */
async function getDatabase(): Promise<Database> {
  if (!db) {
    db = await Database.load('sqlite:focus_daily.db');
  }
  return db;
}

// ============ 专注会话相关API ============

/**
 * 创建新的专注会话
 */
export async function createFocusSession(input: CreateFocusSession): Promise<FocusSession> {
  const database = await getDatabase();
  const id = generateUUID();
  const now = new Date().toISOString();
  
  await database.execute(
    `INSERT INTO focus_sessions (id, start_time, duration_seconds, session_type, completed, created_at, updated_at) 
     VALUES ($1, $2, $3, $4, 0, $5, $6)`,
    [id, now, input.duration_seconds, input.session_type, now, now]
  );

  return {
    id,
    start_time: now,
    end_time: undefined,
    duration_seconds: input.duration_seconds,
    session_type: input.session_type,
    completed: false,
    created_at: now,
    updated_at: now,
  };
}

/**
 * 完成专注会话
 */
export async function completeFocusSession(sessionId: string): Promise<FocusSession> {
  const database = await getDatabase();
  const now = new Date().toISOString();
  
  await database.execute(
    'UPDATE focus_sessions SET end_time = $1, completed = 1, updated_at = $2 WHERE id = $3',
    [now, now, sessionId]
  );
  
  return await getFocusSessionById(sessionId);
}

/**
 * 获取指定ID的会话
 */
export async function getFocusSessionById(sessionId: string): Promise<FocusSession> {
  const database = await getDatabase();
  const result = await database.select<FocusSession[]>(
    'SELECT * FROM focus_sessions WHERE id = $1 LIMIT 1',
    [sessionId]
  );
  
  if (result.length === 0) {
    throw new Error('会话未找到');
  }
  
  return result[0];
}

/**
 * 获取最近的会话列表
 */
export async function getRecentFocusSessions(limit: number = 10): Promise<FocusSession[]> {
  const database = await getDatabase();
  return await database.select<FocusSession[]>(
    'SELECT * FROM focus_sessions ORDER BY created_at DESC LIMIT $1',
    [limit]
  );
}

/**
 * 获取今日会话统计
 */
export async function getTodayStats(): Promise<TodayStats> {
  const database = await getDatabase();
  const today = new Date().toISOString().split('T')[0];
  
  const result = await database.select<Array<{ session_type: string; count: number; total_seconds: number }>>(
    `SELECT 
       session_type,
       COUNT(*) as count,
       SUM(CASE WHEN completed = 1 THEN duration_seconds ELSE 0 END) as total_seconds
     FROM focus_sessions 
     WHERE date(created_at) = $1
     GROUP BY session_type`,
    [today]
  );
  
  let focusCount = 0;
  let breakCount = 0;
  let totalFocusTime = 0;
  
  for (const row of result) {
    if (row.session_type === 'Focus') {
      focusCount = row.count;
      totalFocusTime = row.total_seconds;
    } else if (row.session_type === 'LongBreak' || row.session_type === 'MicroBreak') {
      breakCount += row.count;
    }
  }
  
  return {
    focus_count: focusCount,
    break_count: breakCount,
    total_focus_time: totalFocusTime
  };
}

/**
 * 删除会话
 */
export async function deleteFocusSession(sessionId: string): Promise<void> {
  const database = await getDatabase();
  await database.execute('DELETE FROM focus_sessions WHERE id = $1', [sessionId]);
}

// ============ 用户设置相关API ============

/**
 * 获取用户设置
 */
export async function getUserSettings(): Promise<UserSettings> {
  const database = await getDatabase();
  const result = await database.select<UserSettings[]>(
    'SELECT * FROM user_settings WHERE id = $1 LIMIT 1',
    ['default_settings']
  );
  
  if (result.length === 0) {
    throw new Error('用户设置未找到');
  }
  
  return result[0];
}

/**
 * 更新用户设置
 */
export async function updateUserSettings(input: UpdateUserSettings): Promise<UserSettings> {
  const database = await getDatabase();
  const now = new Date().toISOString();
  
  const updates = [];
  const values = [];
  let paramIndex = 1;
  
  if (input.theme !== undefined) {
    updates.push(`theme = $${paramIndex++}`);
    values.push(input.theme);
  }
  
  if (input.language !== undefined) {
    updates.push(`language = $${paramIndex++}`);
    values.push(input.language);
  }
  
  if (input.auto_start !== undefined) {
    updates.push(`auto_start = $${paramIndex++}`);
    values.push(input.auto_start ? 1 : 0);
  }
  
  if (input.focus_duration_minutes !== undefined) {
    updates.push(`focus_duration_minutes = $${paramIndex++}`);
    values.push(input.focus_duration_minutes);
  }
  
  if (input.long_break_duration_minutes !== undefined) {
    updates.push(`long_break_duration_minutes = $${paramIndex++}`);
    values.push(input.long_break_duration_minutes);
  }
  
  if (input.micro_break_min_interval_minutes !== undefined) {
    updates.push(`micro_break_min_interval_minutes = $${paramIndex++}`);
    values.push(input.micro_break_min_interval_minutes);
  }
  
  if (input.micro_break_max_interval_minutes !== undefined) {
    updates.push(`micro_break_max_interval_minutes = $${paramIndex++}`);
    values.push(input.micro_break_max_interval_minutes);
  }
  
  if (input.micro_break_duration_seconds !== undefined) {
    updates.push(`micro_break_duration_seconds = $${paramIndex++}`);
    values.push(input.micro_break_duration_seconds);
  }
  
  if (input.notifications_enabled !== undefined) {
    updates.push(`notifications_enabled = $${paramIndex++}`);
    values.push(input.notifications_enabled ? 1 : 0);
  }
  
  if (updates.length > 0) {
    updates.push(`updated_at = $${paramIndex++}`);
    values.push(now);
    values.push('default_settings');
    
    const query = `UPDATE user_settings SET ${updates.join(', ')} WHERE id = $${paramIndex}`;
    await database.execute(query, values);
  }
  
  return await getUserSettings();
}

/**
 * 重置用户设置为默认值
 */
export async function resetUserSettingsToDefault(): Promise<UserSettings> {
  const database = await getDatabase();
  const now = new Date().toISOString();
  
  await database.execute(
    `UPDATE user_settings SET
       theme = $1,
       language = $2,
       auto_start = $3,
       focus_duration_minutes = $4,
       long_break_duration_minutes = $5,
       micro_break_min_interval_minutes = $6,
       micro_break_max_interval_minutes = $7,
       micro_break_duration_seconds = $8,
       notifications_enabled = $9,
       updated_at = $10
     WHERE id = $11`,
    ['System', 'zh-CN', 0, 90, 20, 3, 5, 15, 1, now, 'default_settings']
  );
  
  return await getUserSettings();
}

// ============ 音频配置相关API ============

/**
 * 获取所有音频配置
 */
export async function getAllAudioConfigs(): Promise<AudioConfig[]> {
  const database = await getDatabase();
  return await database.select<AudioConfig[]>('SELECT * FROM audio_configs ORDER BY config_type');
}

/**
 * 根据音频类型获取配置
 */
export async function getAudioConfigByType(audioType: AudioType): Promise<AudioConfig> {
  const database = await getDatabase();
  const result = await database.select<AudioConfig[]>(
    'SELECT * FROM audio_configs WHERE config_type = $1 LIMIT 1',
    [audioType]
  );
  
  if (result.length === 0) {
    throw new Error('音频配置未找到');
  }
  
  return result[0];
}

/**
 * 创建或更新音频配置
 */
export async function createOrUpdateAudioConfig(input: CreateAudioConfig): Promise<AudioConfig> {
  const database = await getDatabase();
  const now = new Date().toISOString();
  
  await database.execute(
    `UPDATE audio_configs SET
       file_path = $1,
       is_default = $2,
       volume = $3,
       enabled = $4,
       updated_at = $5
     WHERE config_type = $6`,
    [
      input.file_path || null,
      input.file_path ? 0 : 1,
      input.volume,
      input.enabled ? 1 : 0,
      now,
      input.config_type
    ]
  );
  
  return await getAudioConfigByType(input.config_type);
}

/**
 * 启用/禁用音频配置
 */
export async function toggleAudioConfig(audioType: AudioType, enabled: boolean): Promise<AudioConfig> {
  const database = await getDatabase();
  const now = new Date().toISOString();
  
  await database.execute(
    'UPDATE audio_configs SET enabled = $1, updated_at = $2 WHERE config_type = $3',
    [enabled ? 1 : 0, now, audioType]
  );
  
  return await getAudioConfigByType(audioType);
}

/**
 * 更新音频音量
 */
export async function updateAudioVolume(audioType: AudioType, volume: number): Promise<AudioConfig> {
  const database = await getDatabase();
  const now = new Date().toISOString();
  
  await database.execute(
    'UPDATE audio_configs SET volume = $1, updated_at = $2 WHERE config_type = $3',
    [volume, now, audioType]
  );
  
  return await getAudioConfigByType(audioType);
}

/**
 * 重置音频配置为默认
 */
export async function resetAudioConfigToDefault(audioType: AudioType): Promise<AudioConfig> {
  const input: CreateAudioConfig = {
    config_type: audioType,
    file_path: undefined,
    volume: 1.0,
    enabled: true,
  };
  
  return await createOrUpdateAudioConfig(input);
}

/**
 * 获取启用的音频配置
 */
export async function getEnabledAudioConfigs(): Promise<AudioConfig[]> {
  const database = await getDatabase();
  return await database.select<AudioConfig[]>(
    'SELECT * FROM audio_configs WHERE enabled = 1 ORDER BY config_type'
  );
}

// ============ 工具函数 ============

/**
 * 生成UUID
 */
function generateUUID(): string {
  return 'xxxx-xxxx-4xxx-yxxx'.replace(/[xy]/g, function(c) {
    const r = Math.random() * 16 | 0;
    const v = c === 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

/**
 * 通用错误处理函数
 */
export function handleApiError(error: unknown): string {
  if (typeof error === 'string') {
    return error;
  }
  if (error instanceof Error) {
    return error.message;
  }
  return '未知错误';
}

/**
 * 带错误处理的API调用包装器
 */
export async function safeApiCall<T>(
  apiCall: () => Promise<T>
): Promise<{ success: boolean; data?: T; error?: string }> {
  try {
    const data = await apiCall();
    return { success: true, data };
  } catch (error) {
    return { success: false, error: handleApiError(error) };
  }
} 
use tauri_plugin_sql::{Builder, Migration, MigrationKind};

/// 数据库初始化和迁移
pub fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "创建focus_sessions表",
            sql: r#"
                CREATE TABLE focus_sessions (
                    id TEXT PRIMARY KEY,
                    start_time TEXT NOT NULL,
                    end_time TEXT,
                    duration_seconds INTEGER NOT NULL,
                    session_type TEXT NOT NULL,
                    completed BOOLEAN NOT NULL DEFAULT 0,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );
            "#,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "创建user_settings表",
            sql: r#"
                CREATE TABLE user_settings (
                    id TEXT PRIMARY KEY,
                    theme TEXT NOT NULL DEFAULT 'System',
                    language TEXT NOT NULL DEFAULT 'zh-CN',
                    auto_start BOOLEAN NOT NULL DEFAULT 0,
                    focus_duration_minutes INTEGER NOT NULL DEFAULT 90,
                    long_break_duration_minutes INTEGER NOT NULL DEFAULT 20,
                    micro_break_min_interval_minutes INTEGER NOT NULL DEFAULT 3,
                    micro_break_max_interval_minutes INTEGER NOT NULL DEFAULT 5,
                    micro_break_duration_seconds INTEGER NOT NULL DEFAULT 15,
                    notifications_enabled BOOLEAN NOT NULL DEFAULT 1,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );
            "#,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "创建audio_configs表",
            sql: r#"
                CREATE TABLE audio_configs (
                    id TEXT PRIMARY KEY,
                    config_type TEXT NOT NULL,
                    file_path TEXT,
                    is_default BOOLEAN NOT NULL DEFAULT 1,
                    volume REAL NOT NULL DEFAULT 1.0,
                    enabled BOOLEAN NOT NULL DEFAULT 1,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );
            "#,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 4,
            description: "插入默认用户设置",
            sql: r#"
                INSERT INTO user_settings (
                    id, theme, language, auto_start, focus_duration_minutes,
                    long_break_duration_minutes, micro_break_min_interval_minutes,
                    micro_break_max_interval_minutes, micro_break_duration_seconds,
                    notifications_enabled, created_at, updated_at
                ) VALUES (
                    'default_settings', 'System', 'zh-CN', 0, 90, 20, 3, 5, 15, 1,
                    datetime('now'), datetime('now')
                );
            "#,
            kind: MigrationKind::Up,
        },
        Migration {
            version: 5,
            description: "插入默认音频配置",
            sql: r#"
                INSERT INTO audio_configs (id, config_type, file_path, is_default, volume, enabled, created_at, updated_at)
                VALUES 
                    ('focus_start_default', 'FocusStart', NULL, 1, 1.0, 1, datetime('now'), datetime('now')),
                    ('focus_end_default', 'FocusEnd', NULL, 1, 1.0, 1, datetime('now'), datetime('now')),
                    ('long_break_start_default', 'LongBreakStart', NULL, 1, 1.0, 1, datetime('now'), datetime('now')),
                    ('long_break_end_default', 'LongBreakEnd', NULL, 1, 1.0, 1, datetime('now'), datetime('now')),
                    ('micro_break_start_default', 'MicroBreakStart', NULL, 1, 1.0, 1, datetime('now'), datetime('now'));
            "#,
            kind: MigrationKind::Up,
        },
    ]
}

/// 获取SQL插件构建器
pub fn get_sql_plugin() -> Builder {
    Builder::default()
        .add_migrations("sqlite:focus_daily.db", get_migrations())
} 
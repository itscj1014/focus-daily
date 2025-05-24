use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// 性能指标类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MetricType {
    EventProcessingTime,    // 事件处理时间
    MemoryUsage,           // 内存使用量
    EventQueueSize,        // 事件队列大小
    TimerAccuracy,         // 定时器精度
    DatabaseOperationTime, // 数据库操作时间
    AudioLatency,          // 音频延迟
    UIResponseTime,        // UI响应时间
    CPUUsage,              // CPU使用率
}

/// 性能度量数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDataPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub unit: String,
    pub tags: HashMap<String, String>,
}

/// 性能统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub metric_type: MetricType,
    pub count: u64,
    pub min: f64,
    pub max: f64,
    pub average: f64,
    pub median: f64,
    pub p95: f64,
    pub p99: f64,
    pub last_updated: DateTime<Utc>,
}

/// 系统健康状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub overall_status: HealthStatus,
    pub event_system_health: HealthStatus,
    pub timer_system_health: HealthStatus,
    pub memory_health: HealthStatus,
    pub performance_score: f64,
    pub recommendations: Vec<String>,
    pub last_check: DateTime<Utc>,
}

/// 健康状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Excellent,  // 优秀
    Good,       // 良好
    Warning,    // 警告
    Critical,   // 严重
}

/// 性能警报
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub id: String,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub metric_type: MetricType,
    pub threshold_value: f64,
    pub actual_value: f64,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
}

/// 警报类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    HighLatency,        // 高延迟
    MemoryLeak,         // 内存泄漏
    EventBacklog,       // 事件积压
    TimerDrift,         // 定时器漂移
    HighCPU,           // 高CPU使用
    DiskSpace,         // 磁盘空间
    DatabaseSlow,      // 数据库缓慢
}

/// 警报严重程度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum AlertSeverity {
    Info = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
}

/// 性能监控器
pub struct PerformanceMonitor {
    /// 指标数据存储
    metrics: Arc<RwLock<HashMap<MetricType, Vec<MetricDataPoint>>>>,
    /// 性能统计
    stats: Arc<RwLock<HashMap<MetricType, PerformanceStats>>>,
    /// 当前警报
    alerts: Arc<RwLock<Vec<PerformanceAlert>>>,
    /// 系统健康状态
    health: Arc<RwLock<SystemHealth>>,
    /// 监控配置
    config: PerformanceConfig,
}

/// 性能监控配置
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// 数据保留时间
    pub data_retention: Duration,
    /// 最大数据点数量
    pub max_data_points: usize,
    /// 统计计算间隔
    pub stats_interval: Duration,
    /// 健康检查间隔
    pub health_check_interval: Duration,
    /// 警报阈值
    pub alert_thresholds: HashMap<MetricType, f64>,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        let mut thresholds = HashMap::new();
        thresholds.insert(MetricType::EventProcessingTime, 100.0); // 100ms
        thresholds.insert(MetricType::MemoryUsage, 500.0); // 500MB
        thresholds.insert(MetricType::EventQueueSize, 100.0); // 100 events
        thresholds.insert(MetricType::TimerAccuracy, 50.0); // 50ms drift
        thresholds.insert(MetricType::DatabaseOperationTime, 200.0); // 200ms
        thresholds.insert(MetricType::AudioLatency, 20.0); // 20ms
        thresholds.insert(MetricType::UIResponseTime, 16.7); // 60fps
        thresholds.insert(MetricType::CPUUsage, 80.0); // 80%
        
        Self {
            data_retention: Duration::from_secs(24 * 60 * 60), // 24 hours
            max_data_points: 10000,
            stats_interval: Duration::from_secs(5 * 60), // 5 minutes
            health_check_interval: Duration::from_secs(60), // 1 minute
            alert_thresholds: thresholds,
        }
    }
}

impl PerformanceMonitor {
    /// 创建新的性能监控器
    pub fn new(config: Option<PerformanceConfig>) -> Self {
        let config = config.unwrap_or_default();
        
        let monitor = Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
            health: Arc::new(RwLock::new(SystemHealth::default())),
            config,
        };
        
        // 启动定期任务
        monitor.start_background_tasks();
        
        monitor
    }
    
    /// 记录指标数据点
    pub async fn record_metric(&self, metric_type: MetricType, value: f64, unit: String, tags: Option<HashMap<String, String>>) {
        let data_point = MetricDataPoint {
            timestamp: Utc::now(),
            value,
            unit,
            tags: tags.unwrap_or_default(),
        };
        
        let mut metrics = self.metrics.write().await;
        let metric_data = metrics.entry(metric_type.clone()).or_insert_with(Vec::new);
        
        metric_data.push(data_point);
        
        // 清理旧数据
        if metric_data.len() > self.config.max_data_points {
            metric_data.drain(0..metric_data.len() - self.config.max_data_points);
        }
        
        // 检查警报阈值
        if let Some(threshold) = self.config.alert_thresholds.get(&metric_type) {
            if value > *threshold {
                self.trigger_alert(metric_type, value, *threshold).await;
            }
        }
    }
    
    /// 记录定时器操作
    pub async fn record_timer_operation<F, T>(&self, operation: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        let start = Instant::now();
        let result = operation.await;
        let duration = start.elapsed();
        
        self.record_metric(
            MetricType::EventProcessingTime,
            duration.as_millis() as f64,
            "ms".to_string(),
            None,
        ).await;
        
        result
    }
    
    /// 获取指标统计
    pub async fn get_stats(&self, metric_type: &MetricType) -> Option<PerformanceStats> {
        self.stats.read().await.get(metric_type).cloned()
    }
    
    /// 获取所有统计信息
    pub async fn get_all_stats(&self) -> HashMap<MetricType, PerformanceStats> {
        self.stats.read().await.clone()
    }
    
    /// 获取系统健康状态
    pub async fn get_system_health(&self) -> SystemHealth {
        self.health.read().await.clone()
    }
    
    /// 获取当前警报
    pub async fn get_active_alerts(&self) -> Vec<PerformanceAlert> {
        let alerts = self.alerts.read().await;
        alerts.iter().filter(|alert| !alert.resolved).cloned().collect()
    }
    
    /// 解决警报
    pub async fn resolve_alert(&self, alert_id: &str) {
        let mut alerts = self.alerts.write().await;
        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.resolved = true;
        }
    }
    
    /// 获取性能报告
    pub async fn get_performance_report(&self) -> PerformanceReport {
        let stats = self.get_all_stats().await;
        let health = self.get_system_health().await;
        let alerts = self.get_active_alerts().await;
        
        let summary = self.generate_summary(&stats, &health).await;
        
        PerformanceReport {
            timestamp: Utc::now(),
            stats,
            health,
            active_alerts: alerts,
            summary,
        }
    }
    
    /// 触发警报
    async fn trigger_alert(&self, metric_type: MetricType, value: f64, threshold: f64) {
        let alert = PerformanceAlert {
            id: uuid::Uuid::new_v4().to_string(),
            alert_type: self.metric_to_alert_type(&metric_type),
            severity: self.calculate_severity(value, threshold),
            message: format!("{:?} exceeded threshold: {:.2} > {:.2}", metric_type, value, threshold),
            metric_type,
            threshold_value: threshold,
            actual_value: value,
            timestamp: Utc::now(),
            resolved: false,
        };
        
        let mut alerts = self.alerts.write().await;
        alerts.push(alert);
        
        // 清理旧警报
        alerts.retain(|alert| {
            Utc::now().signed_duration_since(alert.timestamp).num_hours() < 24
        });
    }
    
    /// 启动后台任务
    fn start_background_tasks(&self) {
        let metrics = Arc::clone(&self.metrics);
        let stats = Arc::clone(&self.stats);
        let health = Arc::clone(&self.health);
        let config = self.config.clone();
        
        // 统计计算任务
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.stats_interval);
            loop {
                interval.tick().await;
                Self::calculate_statistics(&metrics, &stats).await;
            }
        });
        
        // 健康检查任务
        let metrics_health = Arc::clone(&self.metrics);
        let config_health = self.config.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config_health.health_check_interval);
            loop {
                interval.tick().await;
                Self::check_system_health(&metrics_health, &health).await;
            }
        });
        
        // 数据清理任务
        let metrics_cleanup = Arc::clone(&self.metrics);
        let config_cleanup = self.config.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60 * 60)); // 1 hour
            loop {
                interval.tick().await;
                Self::cleanup_old_data(&metrics_cleanup, &config_cleanup).await;
            }
        });
    }
    
    /// 计算统计信息
    async fn calculate_statistics(
        metrics: &Arc<RwLock<HashMap<MetricType, Vec<MetricDataPoint>>>>,
        stats: &Arc<RwLock<HashMap<MetricType, PerformanceStats>>>,
    ) {
        let metrics_guard = metrics.read().await;
        let mut stats_guard = stats.write().await;
        
        for (metric_type, data_points) in metrics_guard.iter() {
            if data_points.is_empty() {
                continue;
            }
            
            let values: Vec<f64> = data_points.iter().map(|dp| dp.value).collect();
            let mut sorted_values = values.clone();
            sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let count = values.len() as u64;
            let min = *sorted_values.first().unwrap_or(&0.0);
            let max = *sorted_values.last().unwrap_or(&0.0);
            let average = values.iter().sum::<f64>() / values.len() as f64;
            let median = Self::percentile(&sorted_values, 50.0);
            let p95 = Self::percentile(&sorted_values, 95.0);
            let p99 = Self::percentile(&sorted_values, 99.0);
            
            let performance_stats = PerformanceStats {
                metric_type: metric_type.clone(),
                count,
                min,
                max,
                average,
                median,
                p95,
                p99,
                last_updated: Utc::now(),
            };
            
            stats_guard.insert(metric_type.clone(), performance_stats);
        }
    }
    
    /// 计算百分位数
    fn percentile(sorted_values: &[f64], percentile: f64) -> f64 {
        if sorted_values.is_empty() {
            return 0.0;
        }
        
        let index = (percentile / 100.0 * (sorted_values.len() - 1) as f64) as usize;
        sorted_values[index.min(sorted_values.len() - 1)]
    }
    
    /// 检查系统健康状态
    async fn check_system_health(
        metrics: &Arc<RwLock<HashMap<MetricType, Vec<MetricDataPoint>>>>,
        health: &Arc<RwLock<SystemHealth>>,
    ) {
        let metrics_guard = metrics.read().await;
        let mut health_guard = health.write().await;
        
        // 简化的健康检查逻辑
        let event_health = if let Some(event_metrics) = metrics_guard.get(&MetricType::EventProcessingTime) {
            if event_metrics.is_empty() {
                HealthStatus::Good
            } else {
                let recent_avg = event_metrics.iter().rev().take(10).map(|dp| dp.value).sum::<f64>() / 10.0;
                if recent_avg < 50.0 {
                    HealthStatus::Excellent
                } else if recent_avg < 100.0 {
                    HealthStatus::Good
                } else if recent_avg < 200.0 {
                    HealthStatus::Warning
                } else {
                    HealthStatus::Critical
                }
            }
        } else {
            HealthStatus::Good
        };
        
        let timer_health = HealthStatus::Good; // 简化实现
        let memory_health = HealthStatus::Good; // 简化实现
        
        let overall_status = match (&event_health, &timer_health, &memory_health) {
            (HealthStatus::Critical, _, _) | (_, HealthStatus::Critical, _) | (_, _, HealthStatus::Critical) => HealthStatus::Critical,
            (HealthStatus::Warning, _, _) | (_, HealthStatus::Warning, _) | (_, _, HealthStatus::Warning) => HealthStatus::Warning,
            (HealthStatus::Good, HealthStatus::Good, HealthStatus::Good) => HealthStatus::Good,
            _ => HealthStatus::Excellent,
        };
        
        *health_guard = SystemHealth {
            overall_status,
            event_system_health: event_health,
            timer_system_health: timer_health,
            memory_health,
            performance_score: 85.0, // 简化计算
            recommendations: vec![], // 简化实现
            last_check: Utc::now(),
        };
    }
    
    /// 清理旧数据
    async fn cleanup_old_data(
        metrics: &Arc<RwLock<HashMap<MetricType, Vec<MetricDataPoint>>>>,
        config: &PerformanceConfig,
    ) {
        let cutoff_time = Utc::now() - chrono::Duration::from_std(config.data_retention).unwrap();
        let mut metrics_guard = metrics.write().await;
        
        for data_points in metrics_guard.values_mut() {
            data_points.retain(|dp| dp.timestamp > cutoff_time);
        }
    }
    
    /// 指标类型转换为警报类型
    fn metric_to_alert_type(&self, metric_type: &MetricType) -> AlertType {
        match metric_type {
            MetricType::EventProcessingTime => AlertType::HighLatency,
            MetricType::MemoryUsage => AlertType::MemoryLeak,
            MetricType::EventQueueSize => AlertType::EventBacklog,
            MetricType::TimerAccuracy => AlertType::TimerDrift,
            MetricType::DatabaseOperationTime => AlertType::DatabaseSlow,
            MetricType::AudioLatency => AlertType::HighLatency,
            MetricType::UIResponseTime => AlertType::HighLatency,
            MetricType::CPUUsage => AlertType::HighCPU,
        }
    }
    
    /// 计算警报严重程度
    fn calculate_severity(&self, value: f64, threshold: f64) -> AlertSeverity {
        let ratio = value / threshold;
        if ratio > 3.0 {
            AlertSeverity::Critical
        } else if ratio > 2.0 {
            AlertSeverity::Error
        } else if ratio > 1.5 {
            AlertSeverity::Warning
        } else {
            AlertSeverity::Info
        }
    }
    
    /// 生成性能摘要
    async fn generate_summary(&self, _stats: &HashMap<MetricType, PerformanceStats>, health: &SystemHealth) -> String {
        format!(
            "系统整体状态: {:?}, 性能评分: {:.1}/100",
            health.overall_status,
            health.performance_score
        )
    }
}

/// 性能报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub timestamp: DateTime<Utc>,
    pub stats: HashMap<MetricType, PerformanceStats>,
    pub health: SystemHealth,
    pub active_alerts: Vec<PerformanceAlert>,
    pub summary: String,
}

impl Default for SystemHealth {
    fn default() -> Self {
        Self {
            overall_status: HealthStatus::Good,
            event_system_health: HealthStatus::Good,
            timer_system_health: HealthStatus::Good,
            memory_health: HealthStatus::Good,
            performance_score: 80.0,
            recommendations: vec![],
            last_check: Utc::now(),
        }
    }
} 
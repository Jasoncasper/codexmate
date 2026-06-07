//! 路由配置结构定义

use serde::{Deserialize, Serialize};

pub const INTERNAL_OPENAI_PROVIDER_ID: &str = "openai";
pub const DEFAULT_OPENAI_MODEL_PATTERN: &str = "gpt*,o1*,o3*,o4*,o5*,codex-*";

/// 智能路由全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRouterConfig {
    /// 模型列表
    pub providers: Vec<SmartProvider>,
    /// 多模态回退模型名（当选中模型不支持视觉且请求有图片时使用）
    #[serde(default)]
    pub vision_fallback_model: String,
    /// 无匹配模型时的 fallback provider id
    #[serde(default)]
    pub fallback_provider: String,
    /// 是否自动压缩超大图片
    #[serde(default = "default_image_auto_compress")]
    pub image_auto_compress: bool,
    /// 图片压缩阈值（KB）
    #[serde(default = "default_image_max_size_kb")]
    pub image_max_size_kb: u64,
    /// 图片压缩后最长边（px）
    #[serde(default = "default_image_max_dimension")]
    pub image_max_dimension: u32,
    /// 故障转移配置
    #[serde(default)]
    pub fallback: FallbackConfig,
}

fn default_image_auto_compress() -> bool { true }
fn default_image_max_size_kb() -> u64 { 100 }
fn default_image_max_dimension() -> u32 { 1024 }

impl Default for SmartRouterConfig {
    fn default() -> Self {
        Self {
            providers: Vec::new(),
            vision_fallback_model: String::new(),
            fallback_provider: String::new(),
            image_auto_compress: true,
            image_max_size_kb: 100,
            image_max_dimension: 1024,
            fallback: FallbackConfig::default(),
        }
    }
}

/// 模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartProvider {
    /// 唯一标识
    pub id: String,
    /// 显示名称
    pub name: String,
    /// API 基础地址
    pub base_url: String,
    /// API 密钥
    pub api_key: String,
    /// 协议类型
    #[serde(default)]
    pub protocol: ProviderProtocol,
    /// 是否启用
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// 是否支持多模态（图片/视觉输入）
    #[serde(default)]
    pub supports_vision: bool,
    /// 使用完整 URL（不自动拼接 /chat/completions 等后缀）
    #[serde(default)]
    pub use_full_url: bool,
    /// 上游模型名（发给 API 的实际 model 倰），为空则用 id
    #[serde(default)]
    pub target_model: String,
    /// 模型名通配符模式（支持 * 通配），为空则仅精确匹配
    #[serde(default)]
    pub model_pattern: String,
    /// 是否为内置 provider（不可删除、不可编辑）
    #[serde(default)]
    pub builtin: bool,
}

fn default_true() -> bool {
    true
}

/// 供应商协议类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderProtocol {
    /// OpenAI Responses API (Codex 原生)
    Responses,
    /// OpenAI Chat Completions API
    ChatCompletions,
    /// Anthropic Messages API
    Anthropic,
    /// 自定义协议
    Custom,
}

impl Default for ProviderProtocol {
    fn default() -> Self {
        Self::Responses
    }
}

/// 故障转移配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_retry_delay")]
    pub retry_delay_ms: u64,
}

fn default_max_retries() -> u32 {
    3
}
fn default_retry_delay() -> u64 {
    1000
}

impl Default for FallbackConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_retries: default_max_retries(),
            retry_delay_ms: default_retry_delay(),
        }
    }
}

/// 请求上下文
#[derive(Debug, Clone)]
pub struct RequestContext {
    pub model: String,
    pub has_image: bool,
    pub has_tools: bool,
}

impl Default for RequestContext {
    fn default() -> Self {
        Self {
            model: String::new(),
            has_image: false,
            has_tools: false,
        }
    }
}

pub fn api_key_masked_str(key: &str) -> String {
    if key.is_empty() {
        String::new()
    } else if key.len() <= 8 {
        "****".to_string()
    } else {
        format!("{}...{}", &key[..4], &key[key.len() - 4..])
    }
}

pub fn is_internal_openai_provider(provider: &SmartProvider) -> bool {
    provider
        .id
        .trim()
        .eq_ignore_ascii_case(INTERNAL_OPENAI_PROVIDER_ID)
}

pub fn is_internal_openai_model_name(model: &str) -> bool {
    model
        .trim()
        .eq_ignore_ascii_case(INTERNAL_OPENAI_PROVIDER_ID)
}

pub fn normalize_router_config(mut config: SmartRouterConfig) -> SmartRouterConfig {
    let mut providers = Vec::with_capacity(config.providers.len());
    for provider in config.providers.into_iter() {
        if is_internal_openai_provider(&provider) {
            continue;
        }
        providers.push(provider);
    }
    if is_internal_openai_model_name(&config.fallback_provider) {
        config.fallback_provider.clear();
    }
    if is_internal_openai_model_name(&config.vision_fallback_model) {
        config.vision_fallback_model.clear();
    }
    config.providers = providers;
    config
}

pub fn scoped_model_id(provider: &SmartProvider) -> String {
    let target = if provider.target_model.trim().is_empty() {
        provider.id.trim()
    } else {
        provider.target_model.trim()
    };
    format!("{}:{target}", provider.id.trim())
}

pub fn split_scoped_model(model: &str) -> Option<(&str, &str)> {
    let (provider_id, target_model) = model.split_once(':')?;
    let provider_id = provider_id.trim();
    let target_model = target_model.trim();
    if provider_id.is_empty() || target_model.is_empty() {
        return None;
    }
    Some((provider_id, target_model))
}

pub fn provider_matches_model(provider: &SmartProvider, model: &str) -> bool {
    !provider.model_pattern.trim().is_empty()
        && model_matches_pattern(model, &provider.model_pattern)
}

/// 通配符模式匹配：支持逗号/分号/竖线分隔的多 pattern，单项支持前缀（gpt-*）和后缀（*-pro）。
pub fn model_matches_pattern(model: &str, pattern: &str) -> bool {
    pattern
        .split(|ch| matches!(ch, ',' | ';' | '|'))
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .any(|item| single_model_pattern_matches(model, item))
}

fn single_model_pattern_matches(model: &str, pattern: &str) -> bool {
    if let Some(prefix) = pattern.strip_suffix('*') {
        model.starts_with(prefix)
    } else if let Some(suffix) = pattern.strip_prefix('*') {
        model.ends_with(suffix)
    } else {
        model == pattern
    }
}

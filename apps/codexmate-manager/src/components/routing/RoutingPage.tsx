import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { confirm } from "@tauri-apps/plugin-dialog";
import { Network, RefreshCw, Save, Image } from "lucide-react";
import { NoticeToast } from "@/components/shared/NoticeToast";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ProviderList } from "./ProviderList";
import { Select } from "@/components/ui/select";
import type { RoutingConfigPayload, SmartProvider, SmartRouterConfig } from "./types";

type Status = "ok" | "failed" | string;
type CommandResult<T> = T & { status: Status; message: string };

export default function RoutingPage() {
  const [config, setConfig] = useState<SmartRouterConfig | null>(null);
  // configPath removed - unused
  const [activeTab, setActiveTab] = useState("models");
  const [notice, setNotice] = useState<{ type: "ok" | "error"; text: string } | null>(null);
  const [testingIndex, setTestingIndex] = useState<number | null>(null);
  const [savingIndex, setSavingIndex] = useState<number | null>(null);
  const [expandedIndex, setExpandedIndex] = useState<number | null>(null);

  const loadConfig = useCallback(async () => {
    try {
      const result = await invoke<CommandResult<RoutingConfigPayload>>("load_routing_config");
      if (result.status === "ok") {
        setConfig(result.config);
        // setConfigPath removed - unused
      }
    } catch (e) {
      console.error("Failed to load routing config:", e);
    }
  }, []);

  useEffect(() => {
    loadConfig();
  }, [loadConfig]);

  const addProvider = () => {
    if (!config) return;
    const newProvider: SmartProvider = {
      id: "",
      name: "新模型",
      base_url: "",
      api_key: "",
      protocol: "chat_completions",
      enabled: true,
      supports_vision: false,
      use_full_url: false,
      target_model: "",
      model_pattern: "",
      builtin: false,
      user_agent: "",
      max_context: 0,
      supports_large_context: false,
    };
    setConfig({ ...config, providers: [...config.providers, newProvider] });
    setExpandedIndex(config.providers.length);
  };

  const saveConfig = async (updatedConfig?: SmartRouterConfig) => {
    const cfg = updatedConfig ?? config;
    if (!cfg) return;
    const missingProvider = cfg.providers.find((provider) => !provider.id.trim());
    if (missingProvider) {
      setNotice({ type: "error", text: "请先填写所有模型名称后再保存" });
      return;
    }
    try {
      const result = await invoke<CommandResult<RoutingConfigPayload>>("save_routing_config", {
        config: cfg,
      });
      setNotice({ type: result.status === "ok" ? "ok" : "error", text: result.message });
      if (result.status === "ok") {
        setConfig(result.config);
        // setConfigPath removed - unused
      }
    } catch (e: any) {
      setNotice({ type: "error", text: String(e) });
    }
  };

  const saveProvider = async (index: number) => {
    if (!config) return;
    const provider = config.providers[index];
    if (!provider || provider.builtin || provider.id === "openai") {
      setNotice({ type: "error", text: "内置 provider 不可编辑" });
      return;
    }
    if (!provider.id.trim()) {
      setNotice({ type: "error", text: "请先填写模型名称" });
      return;
    }
    setSavingIndex(index);
    try {
      const result = await invoke<CommandResult<RoutingConfigPayload>>("upsert_provider", {
        provider,
      });
      setNotice({ type: result.status === "ok" ? "ok" : "error", text: result.message });
      if (result.status === "ok") {
        setConfig(result.config);
        // setConfigPath removed - unused
      }
    } catch (e: any) {
      setNotice({ type: "error", text: String(e) });
    } finally {
      setSavingIndex(null);
    }
  };

  const removeProvider = async (index: number) => {
    if (!config) return;
    const provider = config.providers[index];
    if (!provider || provider.builtin || provider.id === "openai") {
      setNotice({ type: "error", text: "内置 provider 不可删除" });
      return;
    }
    const confirmed = await confirm(
      `确定要删除模型「${provider.name}」吗？`,
      { title: "删除确认", kind: "warning" }
    );
    if (!confirmed) return;
    try {
      const result = await invoke<CommandResult<RoutingConfigPayload>>("delete_provider", {
        request: { providerId: provider.id },
      });
      setNotice({ type: result.status === "ok" ? "ok" : "error", text: result.message });
      if (result.status === "ok") {
        setConfig(result.config);
        // setConfigPath removed - unused
        setExpandedIndex((current) =>
          current === index ? null : current !== null && current > index ? current - 1 : current
        );
      }
    } catch (e: any) {
      setNotice({ type: "error", text: String(e) });
    }
  };

  const updateProvider = (index: number, updates: Partial<SmartProvider>) => {
    if (!config) return;
    const providers = [...config.providers];
    providers[index] = { ...providers[index], ...updates };
    setConfig({ ...config, providers });
  };

  const copyProvider = (index: number) => {
    if (!config) return;
    const source = config.providers[index];
    const copy: SmartProvider = {
      ...source,
      id: "",
      name: `${source.name} (副本)`,
      builtin: false,
    };
    setConfig({ ...config, providers: [...config.providers, copy] });
    setExpandedIndex(config.providers.length);
  };

  const toggleProviderEnabled = (index: number) => {
    if (!config) return;
    const provider = config.providers[index];
    if (!provider || provider.builtin || provider.id === "openai") {
      return;
    }
    updateProvider(index, { enabled: !provider.enabled });
  };

  const testProvider = async (index: number) => {
    if (!config) return;
    const provider = config.providers[index];
    setTestingIndex(index);
    try {
      const result = await invoke<CommandResult<any>>("test_smart_provider", { provider });
      setNotice({ type: result.status === "ok" ? "ok" : "error", text: result.message });
    } catch (e: any) {
      setNotice({ type: "error", text: String(e) });
    }
    setTestingIndex(null);
  };

  const visionModels = config?.providers.filter((p) => p.supports_vision) ?? [];

  if (!config) {
    return <div className="flex items-center justify-center h-64 text-muted-foreground">加载中...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-baseline gap-x-3 gap-y-1">
        <h2 className="text-2xl font-bold flex items-center gap-2">
          <Network className="h-6 w-6" />
          智能路由
        </h2>
        <p className="text-sm text-muted-foreground">
          配置模型与 API 的映射关系，支持多模态回退路由
        </p>
      </div>

      {notice && (
        <NoticeToast
          message={notice.text}
          variant={notice.type === "ok" ? "success" : "error"}
          autoDismiss={notice.type === "ok"}
          onDismiss={() => setNotice(null)}
        />
      )}

      <Tabs value={activeTab} onValueChange={setActiveTab}>
        <TabsList>
          <TabsTrigger value="models">模型 ({config.providers.length})</TabsTrigger>
          <TabsTrigger value="vision">代理规则</TabsTrigger>
        </TabsList>

        <TabsContent value="models">
          <ProviderList
            config={config}
            expandedIndex={expandedIndex}
            testingIndex={testingIndex}
            savingIndex={savingIndex}
            onToggleExpand={setExpandedIndex}
            onUpdate={updateProvider}
            onSave={(index) => void saveProvider(index)}
            onTest={(index) => void testProvider(index)}
            onCopy={copyProvider}
            onToggleEnabled={toggleProviderEnabled}
            onRemove={(index) => void removeProvider(index)}
            onAdd={addProvider}
          />
        </TabsContent>

        <TabsContent value="vision">
          <Card>
            <CardHeader>
              <CardTitle className="text-lg flex items-center gap-2">
                <Image className="h-5 w-5" /> 代理规则
              </CardTitle>
              <CardDescription>
                配置无匹配模型时的 fallback provider 和图片/视觉消息的路由规则
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-2 mb-6">
                <Label>Fallback Provider（无匹配模型时使用）</Label>
                <Select
                  value={config.fallback_provider || ""}
                  onChange={(e) => {
                    const updated = { ...config!, fallback_provider: e.target.value };
                    setConfig(updated);
                    void saveConfig(updated);
                  }}
                >
                  <option value="">不设置 fallback</option>
                  {config.providers.filter((p) => p.enabled).map((m) => (
                    <option key={m.id} value={m.id}>
                      {m.name} ({m.id})
                    </option>
                  ))}
                </Select>
                <p className="text-xs text-muted-foreground">
                  当选中的模型没有精确匹配或 pattern 匹配到任何 provider 时，使用此 provider 处理请求
                </p>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-lg flex items-center gap-2">
                <Image className="h-5 w-5" /> 多模态路由
              </CardTitle>
              <CardDescription>
                配置图片/视觉消息的路由规则：当选中模型不支持多模态时，自动回退到指定模型
              </CardDescription>
            </CardHeader>
            <CardContent>
              {visionModels.length === 0 ? (
                <p className="text-sm text-muted-foreground">
                  暂无支持多模态的模型。请先在"模型"标签中为模型勾选"支持多模态理解"。
                </p>
              ) : (
                <div className="space-y-2">
                  <Label>回退模型</Label>
                  <Select
                    value={config.vision_fallback_model}
                    onChange={(e) => {
                      const updated = { ...config!, vision_fallback_model: e.target.value };
                      setConfig(updated);
                      void saveConfig(updated);
                    }}
                  >
                    <option value="">不启用回退</option>
                    {visionModels.map((m) => (
                      <option key={m.id} value={m.id}>
                        {m.name} ({m.id})
                      </option>
                    ))}
                  </Select>

                  <p className="text-xs text-muted-foreground mt-2">
                    选择后，当请求包含图片但匹配的模型不支持多模态时，自动改用此模型处理
                  </p>
                </div>
              )}
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">图片压缩</CardTitle>
              <CardDescription>
                超过阈值的 base64 图片自动缩放为 JPEG，减少 token 消耗
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <label className="flex items-center gap-2 text-sm cursor-pointer">
                <input
                  type="checkbox"
                  checked={config.image_auto_compress}
                  onChange={(e) => {
                    const updated = { ...config!, image_auto_compress: e.target.checked };
                    setConfig(updated);
                    void saveConfig(updated);
                  }}
                />
                自动压缩超大图片
              </label>
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <Label className="text-xs">触发阈值（KB）</Label>
                  <Input
                    type="number"
                    value={config.image_max_size_kb}
                    onChange={(e) => {
                      const updated = { ...config!, image_max_size_kb: parseInt(e.target.value) || 100 };
                      setConfig(updated);
                      void saveConfig(updated);
                    }}
                    className="h-8 text-sm"
                    min={10}
                    max={10000}
                  />
                </div>
                <div>
                  <Label className="text-xs">最大尺寸（px，最长边）</Label>
                  <Input
                    type="number"
                    value={config.image_max_dimension}
                    onChange={(e) => {
                      const updated = { ...config!, image_max_dimension: parseInt(e.target.value) || 1024 };
                      setConfig(updated);
                      void saveConfig(updated);
                    }}
                    className="h-8 text-sm"
                    min={128}
                    max={4096}
                  />
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>

    </div>
  );
}

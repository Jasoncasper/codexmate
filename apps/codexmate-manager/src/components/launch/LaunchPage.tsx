import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Rocket, ArrowRight, Server } from "lucide-react";
import type { CodexMode } from "@/components/layout/Topbar";
import type { OverviewResult } from "@/lib/types";

export type { CodexMode } from "@/components/layout/Topbar";

interface LaunchPageProps {
  mode: CodexMode;
  overview: OverviewResult | null;
  onModeChange: (mode: CodexMode) => void;
  onRestartCodex: () => void;
}

export function LaunchPage({ mode, overview, onModeChange, onRestartCodex }: LaunchPageProps) {
  const latestLaunch = overview?.latest_launch;
  const isRunning = latestLaunch?.status === "running";

  return (
    <div className="space-y-6 max-w-2xl mx-auto py-6">
      <div className="text-center">
        <h2 className="text-2xl font-bold">CodexMate</h2>
        <p className="text-sm text-muted-foreground mt-1">
          选择运行模式并启动 Codex
        </p>
      </div>

      <div className="flex flex-col items-center gap-3">
        <div className="inline-flex rounded-lg border bg-muted p-1 shadow-inner">
          <button
            onClick={() => onModeChange("direct")}
            className={`px-5 py-2.5 rounded-md text-sm font-semibold transition-all ${
              mode === "direct"
                ? "bg-background text-foreground shadow-md ring-1 ring-border/50"
                : "text-muted-foreground hover:text-foreground"
            }`}
          >
            直连模式
          </button>
          <button
            onClick={() => onModeChange("proxy")}
            className={`px-5 py-2.5 rounded-md text-sm font-semibold transition-all ${
              mode === "proxy"
                ? "bg-background text-foreground shadow-md ring-1 ring-border/50"
                : "text-muted-foreground hover:text-foreground"
            }`}
          >
            代理模式
          </button>
        </div>
        <p className="text-xs text-muted-foreground">
          {mode === "direct"
            ? "直连模式：使用 Codex 官方 GPT 模型，不走代理"
            : "代理模式：通过 codexmate 代理使用三方模型（deepseek 等）"}
        </p>
      </div>

      <Button
        onClick={onRestartCodex}
        variant="default"
        className="mx-auto flex h-auto px-8 py-4 text-base font-semibold shadow-lg shadow-primary/25 hover:shadow-xl hover:shadow-primary/30 transition-shadow"
      >
        <Rocket className="h-5 w-5 mr-2" />
        重启 Codex
        <span className="ml-1 font-normal opacity-80">
          {mode === "proxy" ? "（代理·CDP注入）" : "（直连）"}
        </span>
      </Button>

      {overview && (
        <div className="grid gap-4 sm:grid-cols-2">
          <Card>
            <CardContent className="pt-4 space-y-3">
              <div className="flex items-center gap-2">
                <Server className="h-4 w-4 text-muted-foreground" />
                <span className="text-sm font-medium">运行状态</span>
              </div>
              <div className="space-y-2 text-sm">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">当前模式</span>
                  <Badge variant="outline">{mode === "proxy" ? "代理" : "直连"}</Badge>
                </div>
                {latestLaunch && (
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Codex 状态</span>
                    <Badge variant={isRunning ? "default" : "secondary"}>
                      {latestLaunch.status === "running"
                        ? "运行中"
                        : latestLaunch.status ?? "未知"}
                    </Badge>
                  </div>
                )}
                <div className="flex justify-between">
                  <span className="text-muted-foreground">CodexMate 版本</span>
                  <span className="font-mono text-xs">{overview.current_version}</span>
                </div>
                {overview.codex_version && (
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">Codex 版本</span>
                    <span className="font-mono text-xs">{overview.codex_version}</span>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="pt-4 space-y-3">
              <div className="flex items-center gap-2">
                <ArrowRight className="h-4 w-4 text-muted-foreground" />
                <span className="text-sm font-medium">快捷操作</span>
              </div>
              <div className="space-y-2 text-sm text-muted-foreground">
                <p>
                  前往「<strong className="text-foreground">智能路由</strong>」配置 API 供应商和模型映射
                </p>
                <p>
                  前往「<strong className="text-foreground">日志诊断</strong>」查看运行日志和诊断报告
                </p>
                <p>
                  前往「<strong className="text-foreground">关于</strong>」检查 CodexMate 更新
                </p>
              </div>
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}

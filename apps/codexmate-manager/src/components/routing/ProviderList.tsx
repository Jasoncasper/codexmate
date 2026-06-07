import { PackageOpen, Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ProviderCard } from "./ProviderCard";
import type { SmartRouterConfig, SmartProvider } from "./types";

interface ProviderListProps {
  config: SmartRouterConfig;
  expandedIndex: number | null;
  testingIndex: number | null;
  savingIndex: number | null;
  onToggleExpand: (index: number | null) => void;
  onUpdate: (index: number, updates: Partial<SmartProvider>) => void;
  onSave: (index: number) => void;
  onTest: (index: number) => void;
  onCopy: (index: number) => void;
  onToggleEnabled: (index: number) => void;
  onRemove: (index: number) => void;
  onAdd: () => void;
}

export function ProviderList({
  config,
  expandedIndex,
  testingIndex,
  savingIndex,
  onToggleExpand,
  onUpdate,
  onSave,
  onTest,
  onCopy,
  onToggleEnabled,
  onRemove,
  onAdd,
}: ProviderListProps) {
  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h3 className="text-lg font-semibold">模型列表</h3>
        <Button size="sm" onClick={onAdd}>
          <Plus className="h-4 w-4 mr-1" /> 添加模型
        </Button>
      </div>

      {config.providers.length === 0 ? (
        <div className="flex flex-col items-center justify-center gap-2 py-12 border rounded-lg border-dashed text-muted-foreground">
          <PackageOpen className="h-8 w-8" />
          <p className="text-sm">暂无模型供应商</p>
          <p className="text-xs">点击「添加模型」开始配置 API 路由</p>
          <Button size="sm" variant="outline" onClick={onAdd}>
            <Plus className="h-4 w-4 mr-1" /> 添加第一个模型
          </Button>
        </div>
      ) : (
        config.providers.map((provider, index) => (
          <ProviderCard
            key={index}
            provider={provider}
            index={index}
            isExpanded={expandedIndex === index}
            isTesting={testingIndex === index}
            isSaving={savingIndex === index}
            onToggleExpand={onToggleExpand}
            onUpdate={onUpdate}
            onSave={onSave}
            onTest={onTest}
            onCopy={onCopy}
            onToggleEnabled={onToggleEnabled}
            onRemove={onRemove}
          />
        ))
      )}
    </div>
  );
}

/**
 * MCP Tool: Obtenção da cadeia de proveniência SPICE
 */
export function getProvenanceTool(inferenceId: string): string {
  return `spice-provenance-chain:${inferenceId}`;
}

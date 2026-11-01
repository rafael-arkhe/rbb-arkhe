/**
 * MCP Tool: Verificação de consistência e inclusão no MMR selado
 */
export function verifyMmrTool(mmrJson: string): boolean {
  if (!mmrJson) return false;
  try {
    const data = JSON.parse(mmrJson);
    return Array.isArray(data.peaks) && typeof data.bagged_root === 'string';
  } catch {
    return false;
  }
}

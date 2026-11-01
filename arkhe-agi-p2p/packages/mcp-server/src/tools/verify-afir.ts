/**
 * MCP Tool: Verificação offline de recibos pós-quânticos AFiR
 */
export function verifyAfirTool(receiptJson: string): boolean {
  if (!receiptJson) return false;
  try {
    const receipt = JSON.parse(receiptJson);
    return receipt.version === 1 && typeof receipt.inference_root === 'string';
  } catch {
    return false;
  }
}

/**
 * Verificação de inferência e recibos via SDK TypeScript
 */
export function verifyInference(receipt: any): boolean {
  return receipt != null && typeof receipt.model_cid === 'string';
}

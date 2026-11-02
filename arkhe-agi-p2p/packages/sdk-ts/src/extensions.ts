/**
 * Camadas lógicas normativas da arquitetura Arkhe
 */
export enum ArkheLayer {
  IdentityAndContent = 'IdentityAndContent',
  ManifestsAndModels = 'ManifestsAndModels',
  ReceiptsAndEvidence = 'ReceiptsAndEvidence',
  NetworkAndDiscovery = 'NetworkAndDiscovery',
  InferenceAndExecution = 'InferenceAndExecution',
  VerificationAndFraud = 'VerificationAndFraud',
  EconomyAndGovernance = 'EconomyAndGovernance',
  ChainIntegration = 'ChainIntegration',
  BuildAndSupplyChain = 'BuildAndSupplyChain',
}

/**
 * Status de obrigatoriedade normativo
 */
export enum ExtensionRequirement {
  Mandatory = 'Mandatory',
  Recommended = 'Recommended',
  Optional = 'Optional',
  Restricted = 'Restricted',
  Prohibited = 'Prohibited',
}

export interface ExtensionMetadata {
  extension: string;
  layer?: ArkheLayer;
  requirement: ExtensionRequirement;
  contentAddressed: boolean;
  failClosed: boolean;
}

export const EXTENSION_REGISTRY: Record<string, ExtensionMetadata> = {
  // 1. Identidade e Conteúdo
  '.cid': { extension: '.cid', layer: ArkheLayer.IdentityAndContent, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },
  '.multihash': { extension: '.multihash', layer: ArkheLayer.IdentityAndContent, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.pub': { extension: '.pub', layer: ArkheLayer.IdentityAndContent, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.sig': { extension: '.sig', layer: ArkheLayer.IdentityAndContent, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.cert': { extension: '.cert', layer: ArkheLayer.IdentityAndContent, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.crl': { extension: '.crl', layer: ArkheLayer.IdentityAndContent, requirement: ExtensionRequirement.Recommended, contentAddressed: false, failClosed: false },
  '.keystore': { extension: '.keystore', layer: ArkheLayer.IdentityAndContent, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.seed': { extension: '.seed', layer: ArkheLayer.IdentityAndContent, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },

  // 2. Manifestos e Modelos
  '.arkmanifest': { extension: '.arkmanifest', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },
  '.arkmanifest.sig': { extension: '.arkmanifest.sig', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.gguf': { extension: '.gguf', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.gguf.part': { extension: '.gguf.part', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },
  '.gguf.index': { extension: '.gguf.index', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Recommended, contentAddressed: false, failClosed: false },
  '.quant': { extension: '.quant', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.lineage': { extension: '.lineage', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.eval': { extension: '.eval', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.dataset': { extension: '.dataset', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.tokenizer': { extension: '.tokenizer', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.vocab': { extension: '.vocab', layer: ArkheLayer.ManifestsAndModels, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },

  // 3. Recibos e Evidência
  '.afir': { extension: '.afir', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },
  '.afir.json': { extension: '.afir.json', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.afirtree': { extension: '.afirtree', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },
  '.wer': { extension: '.wer', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },
  '.mmr': { extension: '.mmr', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.mmrproof': { extension: '.mmrproof', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.consistency': { extension: '.consistency', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.checkpoint': { extension: '.checkpoint', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },
  '.attest': { extension: '.attest', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.arkbundle': { extension: '.arkbundle', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },
  '.wasm': { extension: '.wasm', layer: ArkheLayer.ReceiptsAndEvidence, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },

  // 4. Rede e Descoberta
  '.peers': { extension: '.peers', layer: ArkheLayer.NetworkAndDiscovery, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.peerledger': { extension: '.peerledger', layer: ArkheLayer.NetworkAndDiscovery, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.seeds': { extension: '.seeds', layer: ArkheLayer.NetworkAndDiscovery, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.dhtcache': { extension: '.dhtcache', layer: ArkheLayer.NetworkAndDiscovery, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.announce': { extension: '.announce', layer: ArkheLayer.NetworkAndDiscovery, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.gossip': { extension: '.gossip', layer: ArkheLayer.NetworkAndDiscovery, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.netpolicy': { extension: '.netpolicy', layer: ArkheLayer.NetworkAndDiscovery, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.pcap': { extension: '.pcap', layer: ArkheLayer.NetworkAndDiscovery, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },

  // 5. Inferência e Execução
  '.policy': { extension: '.policy', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.policy.sig': { extension: '.policy.sig', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.request': { extension: '.request', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.response': { extension: '.response', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.session': { extension: '.session', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.router': { extension: '.router', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.expertmap': { extension: '.expertmap', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.kv': { extension: '.kv', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.gpu': { extension: '.gpu', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.onnx': { extension: '.onnx', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.safetensors': { extension: '.safetensors', layer: ArkheLayer.InferenceAndExecution, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },

  // 6. Verificação e Fraudes
  '.fraudproof': { extension: '.fraudproof', layer: ArkheLayer.VerificationAndFraud, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.reexec': { extension: '.reexec', layer: ArkheLayer.VerificationAndFraud, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.zkproof': { extension: '.zkproof', layer: ArkheLayer.VerificationAndFraud, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.teequote': { extension: '.teequote', layer: ArkheLayer.VerificationAndFraud, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.verdict': { extension: '.verdict', layer: ArkheLayer.VerificationAndFraud, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.slash': { extension: '.slash', layer: ArkheLayer.VerificationAndFraud, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.golden': { extension: '.golden', layer: ArkheLayer.VerificationAndFraud, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },

  // 7. Economia e Governança
  '.constitution': { extension: '.constitution', layer: ArkheLayer.EconomyAndGovernance, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },
  '.constitution.sig': { extension: '.constitution.sig', layer: ArkheLayer.EconomyAndGovernance, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.proposal': { extension: '.proposal', layer: ArkheLayer.EconomyAndGovernance, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },
  '.vote': { extension: '.vote', layer: ArkheLayer.EconomyAndGovernance, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.canon': { extension: '.canon', layer: ArkheLayer.EconomyAndGovernance, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.emission': { extension: '.emission', layer: ArkheLayer.EconomyAndGovernance, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.reward': { extension: '.reward', layer: ArkheLayer.EconomyAndGovernance, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.stake': { extension: '.stake', layer: ArkheLayer.EconomyAndGovernance, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.epoch': { extension: '.epoch', layer: ArkheLayer.EconomyAndGovernance, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.gini': { extension: '.gini', layer: ArkheLayer.EconomyAndGovernance, requirement: ExtensionRequirement.Recommended, contentAddressed: false, failClosed: false },

  // 8. Integração com Chains
  '.anchor': { extension: '.anchor', layer: ArkheLayer.ChainIntegration, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.ibc': { extension: '.ibc', layer: ArkheLayer.ChainIntegration, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.ccip': { extension: '.ccip', layer: ArkheLayer.ChainIntegration, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.layerzero': { extension: '.layerzero', layer: ArkheLayer.ChainIntegration, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.abi': { extension: '.abi', layer: ArkheLayer.ChainIntegration, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.sol': { extension: '.sol', layer: ArkheLayer.ChainIntegration, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.move': { extension: '.move', layer: ArkheLayer.ChainIntegration, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.tx': { extension: '.tx', layer: ArkheLayer.ChainIntegration, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.receiptroot': { extension: '.receiptroot', layer: ArkheLayer.ChainIntegration, requirement: ExtensionRequirement.Mandatory, contentAddressed: true, failClosed: true },

  // 9. Build, Testes e Supply Chain
  '.toml': { extension: '.toml', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.rs': { extension: '.rs', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.ts': { extension: '.ts', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.cs': { extension: '.cs', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.csproj': { extension: '.csproj', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.wat': { extension: '.wat', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.json': { extension: '.json', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.lock': { extension: '.lock', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.sbom': { extension: '.sbom', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.provenance': { extension: '.provenance', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.sha256': { extension: '.sha256', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },
  '.sigstore': { extension: '.sigstore', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Recommended, contentAddressed: false, failClosed: false },
  '.log': { extension: '.log', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Optional, contentAddressed: false, failClosed: false },
  '.trace': { extension: '.trace', layer: ArkheLayer.BuildAndSupplyChain, requirement: ExtensionRequirement.Mandatory, contentAddressed: false, failClosed: true },

  // Proibidas ou Restritas
  '.dll': { extension: '.dll', requirement: ExtensionRequirement.Restricted, contentAddressed: false, failClosed: true },
  '.exe': { extension: '.exe', requirement: ExtensionRequirement.Restricted, contentAddressed: false, failClosed: true },
  '.so': { extension: '.so', requirement: ExtensionRequirement.Restricted, contentAddressed: false, failClosed: true },
  '.dylib': { extension: '.dylib', requirement: ExtensionRequirement.Restricted, contentAddressed: false, failClosed: true },
  '.pem': { extension: '.pem', requirement: ExtensionRequirement.Prohibited, contentAddressed: false, failClosed: true },
  '.env': { extension: '.env', requirement: ExtensionRequirement.Prohibited, contentAddressed: false, failClosed: true },
  '.bak': { extension: '.bak', requirement: ExtensionRequirement.Prohibited, contentAddressed: false, failClosed: true },
  '.tmp': { extension: '.tmp', requirement: ExtensionRequirement.Restricted, contentAddressed: false, failClosed: true },
  '.dump': { extension: '.dump', requirement: ExtensionRequirement.Prohibited, contentAddressed: false, failClosed: true },
};

/**
 * Validador fail-closed de extensões Arkhe
 */
export function validateArkheFilename(filename: string): { valid: boolean; error?: string; metadata?: ExtensionMetadata } {
  const lower = filename.toLowerCase();

  // Verifica compostas primeiro
  for (const ext of ['.arkmanifest.sig', '.afir.json', '.policy.sig', '.constitution.sig', '.gguf.index']) {
    if (lower.endsWith(ext)) {
      const meta = EXTENSION_REGISTRY[ext];
      return checkRequirement(filename, meta);
    }
  }

  if (lower.includes('.gguf.part')) {
    const meta = EXTENSION_REGISTRY['.gguf.part'];
    return checkRequirement(filename, meta);
  }

  // Verifica simples
  for (const [ext, meta] of Object.entries(EXTENSION_REGISTRY)) {
    if (lower.endsWith(ext)) {
      return checkRequirement(filename, meta);
    }
  }

  return {
    valid: false,
    error: `Extensão não reconhecida e não autorizada: '${filename}'`,
  };
}

function checkRequirement(filename: string, meta: ExtensionMetadata): { valid: boolean; error?: string; metadata?: ExtensionMetadata } {
  if (meta.requirement === ExtensionRequirement.Prohibited) {
    return {
      valid: false,
      error: `Extensão estritamente proibida pelo protocolo Arkhe: '${filename}'`,
      metadata: meta,
    };
  }

  if (meta.requirement === ExtensionRequirement.Restricted) {
    if (meta.extension === '.tmp') {
      return { valid: true, metadata: meta };
    }
    return {
      valid: false,
      error: `Binário/arquivo restrito sem atestação formal: '${filename}'`,
      metadata: meta,
    };
  }

  return {
    valid: true,
    metadata: meta,
  };
}

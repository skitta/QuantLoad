export interface Primer {
  name: string;
  concentration: number; // uM
}

export interface Recipe {
  mix: number; // ul
  primers: number; // ul
  cDNA: number; // ul
  water: number; // ul
}

export interface Samples {
  targets: string[];
  repeat: number;
  groups: string[];
}

export interface PrimersConfig {
  forward: Primer;
  reverse: Primer;
}

export interface QPCRConfig {
  samples: Samples;
  recipe: Recipe;
  primers: PrimersConfig;
}

export interface WorkingSolution {
  mix: number;
  forwardPrimer: number;
  reversePrimer: number;
  water: number;
  totalVolume: number;
}

export interface MasterMix {
  mix: number;
  forwardPrimer: number;
  reversePrimer: number;
  water: number;
  totalVolume: number;
}

export interface CalculationResult {
  totalReactions: number;
  workingSolutions: Record<string, WorkingSolution>;
  masterMix: MasterMix;
  totalcDNAVolume: number;
}
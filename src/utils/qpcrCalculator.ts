import { QPCRConfig, CalculationResult, WorkingSolution, MasterMix } from '../types/qpcr';

export function calculateQPCRVolumes(config: QPCRConfig): CalculationResult {
  const { samples, recipe } = config;
  
  // Calculate total number of reactions per target
  const reactionsPerTarget = samples.groups.length * samples.repeat;
  const totalReactions = reactionsPerTarget * samples.targets.length;
  
  // Calculate working solution for each target
  const workingSolutions: Record<string, WorkingSolution> = {};
  
  samples.targets.forEach(target => {
    const mixVolume = recipe.mix * reactionsPerTarget;
    const forwardPrimerVolume = recipe.primers * reactionsPerTarget;
    const reversePrimerVolume = recipe.primers * reactionsPerTarget;
    const waterVolume = recipe.water * reactionsPerTarget;
    const totalVolume = mixVolume + forwardPrimerVolume + reversePrimerVolume + waterVolume;
    
    workingSolutions[target] = {
      mix: mixVolume,
      forwardPrimer: forwardPrimerVolume,
      reversePrimer: reversePrimerVolume,
      water: waterVolume,
      totalVolume
    };
  });
  
  // Calculate master mix (without cDNA)
  const masterMix: MasterMix = {
    mix: recipe.mix * totalReactions,
    forwardPrimer: recipe.primers * totalReactions,
    reversePrimer: recipe.primers * totalReactions,
    water: recipe.water * totalReactions,
    totalVolume: (recipe.mix + recipe.primers * 2 + recipe.water) * totalReactions
  };
  
  // Calculate total cDNA volume
  const totalcDNAVolume = recipe.cDNA * totalReactions;
  
  return {
    totalReactions,
    workingSolutions,
    masterMix,
    totalcDNAVolume
  };
}

export function formatVolume(volume: number): string {
  return volume.toFixed(1) + ' ul';
}
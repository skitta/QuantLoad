<script setup lang="ts">
import { ref, reactive } from "vue";
import { calculateQPCRVolumes, formatVolume } from "./utils/qpcrCalculator";
import type { QPCRConfig, CalculationResult } from "./types/qpcr";
import { getCurrentWindow } from "@tauri-apps/api/window";

const config = reactive<QPCRConfig>({
  samples: {
    targets: ["geneA", "geneB", "geneC"],
    repeat: 3,
    groups: ["Control", "Treatment"]
  },
  recipe: {
    mix: 10,
    primers: 1,
    cDNA: 2,
    water: 6
  },
  primers: {
    forward: {
      name: "primerA",
      concentration: 100
    },
    reverse: {
      name: "primerB", 
      concentration: 100
    }
  }
});

const calculationResult = ref<CalculationResult | null>(null);
const activeSection = ref<'samples' | 'recipe' | 'primers' | 'results'>('samples');

function calculate() {
  calculationResult.value = calculateQPCRVolumes(config);
  activeSection.value = 'results';
}

function addTarget() {
  config.samples.targets.push(`gene${config.samples.targets.length + 1}`);
}

function removeTarget(index: number) {
  config.samples.targets.splice(index, 1);
}

function addGroup() {
  config.samples.groups.push(`Group${config.samples.groups.length + 1}`);
}

function removeGroup(index: number) {
  config.samples.groups.splice(index, 1);
}

// Window control functions
async function minimizeWindow() {
  const appWindow = getCurrentWindow();
  await appWindow.minimize();
}

async function maximizeWindow() {
  const appWindow = getCurrentWindow();
  await appWindow.toggleMaximize();
}

async function closeWindow() {
  const appWindow = getCurrentWindow();
  await appWindow.close();
}
</script>

<template>
  <main class="macos-container">
    <div class="window-titlebar">
      <div class="traffic-lights">
        <div class="traffic-light close" @click="closeWindow"></div>
        <div class="traffic-light minimize" @click="minimizeWindow"></div>
        <div class="traffic-light maximize" @click="maximizeWindow"></div>
      </div>
      <span class="title" data-tauri-drag-region>qPCR Calculator</span>
    </div>

    <div class="macos-content">
      <div class="sidebar">
        <div class="sidebar-header">
          <h2>Configuration</h2>
        </div>
        <nav class="sidebar-nav">
          <button :class="['nav-item', { active: activeSection === 'samples' }]" @click="activeSection = 'samples'">
            🧬 Samples
          </button>
          <button :class="['nav-item', { active: activeSection === 'recipe' }]" @click="activeSection = 'recipe'">
            🧪 Recipe
          </button>
          <button :class="['nav-item', { active: activeSection === 'primers' }]" @click="activeSection = 'primers'">
            🔬 Primers
          </button>
          <button :class="['nav-item', { active: activeSection === 'results' }]" @click="activeSection = 'results'" v-if="calculationResult">
            📊 Results
          </button>
        </nav>
      </div>

      <div class="main-content">
        <!-- Samples Configuration -->
        <div v-show="activeSection === 'samples'" class="content-section">
          <h2 class="section-title">Samples Configuration</h2>
          
          <div class="macos-form-group">
            <label class="macos-label">Target Genes</label>
            <div v-for="(_, index) in config.samples.targets" :key="index" class="input-row">
              <input 
                v-model="config.samples.targets[index]" 
                placeholder="Gene name" 
                class="macos-input"
              />
              <button @click="removeTarget(index)" class="macos-button secondary small">
                Remove
              </button>
            </div>
            <button @click="addTarget" class="macos-button secondary">
              ＋ Add Target
            </button>
          </div>

          <div class="macos-form-group">
            <label class="macos-label">Groups</label>
            <div v-for="(_, index) in config.samples.groups" :key="index" class="input-row">
              <input 
                v-model="config.samples.groups[index]" 
                placeholder="Group name" 
                class="macos-input"
              />
              <button @click="removeGroup(index)" class="macos-button secondary small">
                Remove
              </button>
            </div>
            <button @click="addGroup" class="macos-button secondary">
              ＋ Add Group
            </button>
          </div>

          <div class="macos-form-group">
            <label class="macos-label">Repeats per group</label>
            <input 
              type="number" 
              v-model.number="config.samples.repeat" 
              min="1" 
              class="macos-input"
            />
          </div>
        </div>

        <!-- Recipe Configuration -->
        <div v-show="activeSection === 'recipe'" class="content-section">
          <h2 class="section-title">Reaction Recipe (μl per reaction)</h2>
          
          <div class="macos-form-group">
            <label class="macos-label">Mix Volume</label>
            <input 
              type="number" 
              v-model.number="config.recipe.mix" 
              step="0.1" 
              min="0" 
              class="macos-input"
            />
          </div>

          <div class="macos-form-group">
            <label class="macos-label">Primers Volume (each)</label>
            <input 
              type="number" 
              v-model.number="config.recipe.primers" 
              step="0.1" 
              min="0" 
              class="macos-input"
            />
          </div>

          <div class="macos-form-group">
            <label class="macos-label">cDNA Volume</label>
            <input 
              type="number" 
              v-model.number="config.recipe.cDNA" 
              step="0.1" 
              min="0" 
              class="macos-input"
            />
          </div>

          <div class="macos-form-group">
            <label class="macos-label">Water Volume</label>
            <input 
              type="number" 
              v-model.number="config.recipe.water" 
              step="0.1" 
              min="0" 
              class="macos-input"
            />
          </div>
        </div>

        <!-- Primers Configuration -->
        <div v-show="activeSection === 'primers'" class="content-section">
          <h2 class="section-title">Primers Information</h2>
          
          <div class="macos-form-group">
            <label class="macos-label">Forward Primer Name</label>
            <input 
              v-model="config.primers.forward.name" 
              placeholder="Primer name" 
              class="macos-input"
            />
          </div>

          <div class="macos-form-group">
            <label class="macos-label">Forward Primer Concentration (μM)</label>
            <input 
              type="number" 
              v-model.number="config.primers.forward.concentration" 
              step="1" 
              min="0" 
              class="macos-input"
            />
          </div>

          <div class="macos-form-group">
            <label class="macos-label">Reverse Primer Name</label>
            <input 
              v-model="config.primers.reverse.name" 
              placeholder="Primer name" 
              class="macos-input"
            />
          </div>

          <div class="macos-form-group">
            <label class="macos-label">Reverse Primer Concentration (μM)</label>
            <input 
              type="number" 
              v-model.number="config.primers.reverse.concentration" 
              step="1" 
              min="0" 
              class="macos-input"
            />
          </div>
        </div>

        <!-- Results -->
        <div v-show="activeSection === 'results'" class="content-section" v-if="calculationResult">
          <h2 class="section-title">Calculation Results</h2>
          
          <div class="result-card">
            <h3 class="result-subtitle">Summary</h3>
            <div class="result-grid">
              <div class="result-item">
                <span class="result-label">Total Reactions</span>
                <span class="result-value">{{ calculationResult.totalReactions }}</span>
              </div>
              <div class="result-item">
                <span class="result-label">Total cDNA Volume</span>
                <span class="result-value">{{ formatVolume(calculationResult.totalcDNAVolume) }}</span>
              </div>
            </div>
          </div>

          <div class="result-card">
            <h3 class="result-subtitle">Master Mix (without cDNA)</h3>
            <div class="result-grid">
              <div class="result-item">
                <span class="result-label">Mix</span>
                <span class="result-value">{{ formatVolume(calculationResult.masterMix.mix) }}</span>
              </div>
              <div class="result-item">
                <span class="result-label">Forward Primer</span>
                <span class="result-value">{{ formatVolume(calculationResult.masterMix.forwardPrimer) }}</span>
              </div>
              <div class="result-item">
                <span class="result-label">Reverse Primer</span>
                <span class="result-value">{{ formatVolume(calculationResult.masterMix.reversePrimer) }}</span>
              </div>
              <div class="result-item">
                <span class="result-label">Water</span>
                <span class="result-value">{{ formatVolume(calculationResult.masterMix.water) }}</span>
              </div>
              <div class="result-item highlight">
                <span class="result-label">Total Volume</span>
                <span class="result-value">{{ formatVolume(calculationResult.masterMix.totalVolume) }}</span>
              </div>
            </div>
          </div>

          <div class="result-card">
            <h3 class="result-subtitle">Working Solutions per Target</h3>
            <div v-for="(solution, target) in calculationResult.workingSolutions" :key="target" class="target-card">
              <h4 class="target-name">{{ target }}</h4>
              <div class="result-grid">
                <div class="result-item">
                  <span class="result-label">Mix</span>
                  <span class="result-value">{{ formatVolume(solution.mix) }}</span>
                </div>
                <div class="result-item">
                  <span class="result-label">Forward Primer</span>
                  <span class="result-value">{{ formatVolume(solution.forwardPrimer) }}</span>
                </div>
                <div class="result-item">
                  <span class="result-label">Reverse Primer</span>
                  <span class="result-value">{{ formatVolume(solution.reversePrimer) }}</span>
                </div>
                <div class="result-item">
                  <span class="result-label">Water</span>
                  <span class="result-value">{{ formatVolume(solution.water) }}</span>
                </div>
                <div class="result-item highlight">
                  <span class="result-label">Total Volume</span>
                  <span class="result-value">{{ formatVolume(solution.totalVolume) }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="action-bar">
          <button @click="calculate" class="macos-button primary large">
            Calculate Volumes
          </button>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
/* macOS Window Styling */
.macos-container {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: rgba(245, 245, 247, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 10px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
  overflow: hidden;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.window-titlebar {
  background: #e8e8e8;
  padding: 10px 16px;
  display: flex;
  align-items: center;
  border-bottom: 1px solid #d0d0d0;
  user-select: none;
}

.traffic-lights {
  display: flex;
  gap: 6px;
  margin-right: 8px;
}

.traffic-light {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: 1px solid rgba(0, 0, 0, 0.1);
  cursor: pointer;
  transition: all 0.2s ease;
  opacity: 0.8;
  position: relative;
  z-index: 1000;
}

.traffic-light:hover {
  opacity: 1;
  transform: scale(1.1);
}

.traffic-light.close:hover::before {
  content: '×';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 8px;
  font-weight: bold;
  color: rgba(0, 0, 0, 0.6);
}

.traffic-light.minimize:hover::before {
  content: '−';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 10px;
  font-weight: bold;
  color: rgba(0, 0, 0, 0.6);
}

.traffic-light.maximize:hover::before {
  content: '+';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 8px;
  font-weight: bold;
  color: rgba(0, 0, 0, 0.6);
}

.traffic-light.close { background: #ff5f57; }
.traffic-light.minimize { background: #ffbd2e; }
.traffic-light.maximize { background: #28ca42; }

.title {
  font-size: 13px;
  color: #666;
  font-weight: 500;
}

.macos-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

/* Sidebar Styling */
.sidebar {
  width: 200px;
  background: #f0f0f0;
  border-right: 1px solid #e0e0e0;
  padding: 20px 0;
}

.sidebar-header {
  padding: 0 20px 15px 20px;
  border-bottom: 1px solid #e0e0e0;
  margin-bottom: 15px;
}

.sidebar-header h2 {
  font-size: 15px;
  font-weight: 600;
  color: #666;
  margin: 0;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.nav-item {
  background: none;
  border: none;
  padding: 10px 20px;
  text-align: left;
  font-size: 14px;
  color: #666;
  cursor: pointer;
  transition: all 0.2s ease;
  border-radius: 6px;
  margin: 0 8px;
}

.nav-item:hover {
  background: #e5e5e5;
}

.nav-item.active {
  background: #007aff;
  color: white;
  font-weight: 500;
}

/* Main Content */
.main-content {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
  background: transparent;
}

.content-section {
  max-width: 600px;
}

.section-title {
  font-size: 22px;
  font-weight: 600;
  color: #333;
  margin-bottom: 24px;
}

/* Form Styling */
.macos-form-group {
  margin-bottom: 20px;
}

.macos-label {
  display: block;
  font-size: 13px;
  font-weight: 600;
  color: #666;
  margin-bottom: 6px;
}

.macos-input {
  width: 100%;
  max-width: 300px;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.9);
  transition: all 0.2s ease;
}

.macos-input:focus {
  outline: none;
  border-color: #007aff;
  box-shadow: 0 0 0 2px rgba(0, 122, 255, 0.1);
}

.input-row {
  display: flex;
  gap: 8px;
  align-items: center;
  margin-bottom: 8px;
}

.input-row .macos-input {
  flex: 1;
}

/* Button Styling */
.macos-button {
  padding: 8px 16px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  background: rgba(255, 255, 255, 0.9);
}

.macos-button.primary {
  background: #007aff;
  color: white;
  border-color: #007aff;
}

.macos-button.primary:hover {
  background: #0056d6;
}

.macos-button.secondary {
  background: #f5f5f7;
  color: #666;
}

.macos-button.secondary:hover {
  background: #e5e5e7;
}

.macos-button.small {
  padding: 4px 8px;
  font-size: 12px;
}

.macos-button.large {
  padding: 12px 24px;
  font-size: 14px;
}

/* Results Styling */
.result-card {
  background: #f8f9fa;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 16px;
  border: 1px solid #e9ecef;
}

.result-subtitle {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin-bottom: 16px;
}

.result-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 12px;
}

.result-item {
  display: flex;
  justify-content: between;
  align-items: center;
  padding: 8px 12px;
  background: white;
  border-radius: 6px;
  border: 1px solid #e9ecef;
}

.result-item.highlight {
  background: #e8f4ff;
  border-color: #b8daff;
}

.result-label {
  font-size: 13px;
  color: #666;
  font-weight: 500;
  flex: 1;
}

.result-value {
  font-size: 13px;
  color: #333;
  font-weight: 600;
}

.target-card {
  background: rgba(255, 255, 255, 0.9);
  border-radius: 6px;
  padding: 16px;
  margin-bottom: 12px;
  border: 1px solid #e9ecef;
}

.target-name {
  font-size: 14px;
  font-weight: 600;
  color: #333;
  margin-bottom: 12px;
}

/* Action Bar */
.action-bar {
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid #e0e0e0;
}

/* Dark Mode Support */
@media (prefers-color-scheme: dark) {
  .macos-container {
    background: rgba(44, 44, 46, 0.95);
  }
  
  .window-titlebar {
    background: #3a3a3c;
    border-color: #404040;
  }
  
  .title {
    color: #ccc;
  }
  
  .sidebar {
    background: #2c2c2e;
    border-color: #404040;
  }
  
  .sidebar-header {
    border-color: #404040;
  }
  
  .sidebar-header h2 {
    color: #ccc;
  }
  
  .nav-item {
    color: #ccc;
  }
  
  .nav-item:hover {
    background: #3a3a3c;
  }
  
  .nav-item.active {
    background: #0a84ff;
    color: white;
  }
  
  .main-content {
    background: #1c1c1e;
  }
  
  .section-title {
    color: #fff;
  }
  
  .macos-label {
    color: #ccc;
  }
  
  .macos-input {
    background: #2c2c2e;
    border-color: #404040;
    color: #fff;
  }
  
  .macos-input:focus {
    border-color: #0a84ff;
    box-shadow: 0 0 0 2px rgba(10, 132, 255, 0.2);
  }
  
  .macos-button {
    background: #3a3a3c;
    border-color: #404040;
    color: #fff;
  }
  
  .macos-button.primary {
    background: #0a84ff;
    border-color: #0a84ff;
  }
  
  .macos-button.secondary {
    background: #3a3a3c;
    color: #ccc;
  }
  
  .result-card {
    background: #2c2c2e;
    border-color: #404040;
  }
  
  .result-item {
    background: #3a3a3c;
    border-color: #404040;
  }
  
  .result-item.highlight {
    background: #1a3a5f;
    border-color: #2a5a8f;
  }
  
  .result-label {
    color: #ccc;
  }
  
  .result-value {
    color: #fff;
  }
  
  .target-card {
    background: #3a3a3c;
    border-color: #404040;
  }
  
  .action-bar {
    border-color: #404040;
  }
}
</style>
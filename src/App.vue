<script setup lang="ts">
import { ref } from 'vue';
import { region, reconfigureSqs } from './stores/sqsStore';
import { useRouter } from 'vue-router';

const router = useRouter();
const showRegionModal = ref(false);
const awsRegions = [
  "us-east-1", "us-east-2", "us-west-1", "us-west-2",
  "af-south-1", "ap-east-1", "ap-south-1", "ap-northeast-1",
  "ap-northeast-2", "ap-northeast-3", "ap-southeast-1", "ap-southeast-2",
  "ca-central-1", "eu-central-1", "eu-west-1", "eu-west-2",
  "eu-west-3", "eu-north-1", "me-south-1", "sa-east-1"
];

const selectRegion = async (newRegion: string) => {
  showRegionModal.value = false;
  await reconfigureSqs(newRegion);
  router.go(0); // Reload the page to reflect changes
};
</script>

<template>
  <div class="min-h-screen bg-black text-gray-100 flex flex-col">
    <!-- Animated Background -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
      <div class="absolute inset-0 bg-gradient-to-br from-blue-900/10 via-black to-purple-900/10"></div>
      <div class="grid-overlay"></div>
    </div>

    <!-- Header -->
    <header class="relative bg-black/50 backdrop-blur-sm border-b border-cyan-500/20 shadow-lg shadow-cyan-500/5">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex items-center justify-between">
        <router-link to="/queues" class="cursor-pointer">
          <span class="text-xs font-mono text-cyan-400/60">Simple Queue Dashboard</span>
        </router-link>
        <div
          @click="showRegionModal = true"
          class="flex items-center gap-2 text-xs font-mono text-cyan-400/60 cursor-pointer hover:text-cyan-300"
        >
          <span class="animate-pulse">●</span>
          <span>{{ region }}</span>
        </div>
      </div>
    </header>

    <main class="relative flex-grow max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 w-full">
      <router-view />
    </main>

    <!-- Footer -->
    <footer class="relative bg-black/50 backdrop-blur-sm border-t border-cyan-500/20 shadow-lg shadow-cyan-500/5 mt-auto">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 text-center text-xs font-mono text-cyan-400/60">
        created by
        <a href="https://miguelms.es" target="_blank" rel="noopener noreferrer" class="hover:text-cyan-300">
          miguelms.es
        </a>
      </div>
    </footer>

    <!-- Region Selector Modal -->
    <div v-if="showRegionModal" class="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-50">
      <div class="bg-gray-900/80 border border-cyan-500/30 rounded-lg shadow-lg p-6 w-full max-w-sm">
        <h3 class="text-lg font-mono text-cyan-400 mb-4">Select AWS Region</h3>
        <div class="grid grid-cols-2 gap-2 max-h-96 overflow-y-auto custom-scrollbar">
          <button
            v-for="r in awsRegions"
            :key="r"
            @click="selectRegion(r)"
            class="w-full text-left px-4 py-3 rounded transition-all font-mono text-sm bg-black/30 border border-cyan-500/10 text-gray-400 hover:bg-cyan-500/10 hover:border-cyan-500/30 hover:text-cyan-300"
          >
            {{ r }}
          </button>
        </div>
        <button @click="showRegionModal = false" class="mt-4 w-full text-center py-2 text-xs font-mono text-cyan-400/60 hover:text-cyan-300">
          CLOSE
        </button>
      </div>
    </div>
  </div>
</template>

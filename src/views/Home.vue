<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { setConfigured } from '../stores/sqsStore';

const router = useRouter();
const endpoint = ref('http://localhost:4566');
const loading = ref(false);
const error = ref('');

const configureSqs = async () => {
  loading.value = true;
  error.value = '';
  try {
    await invoke('configure_sqs', { endpoint: endpoint.value });
    setConfigured(true);
    router.push('/queues');
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  configureSqs();
});
</script>

<template>
  <div class="max-w-md mx-auto mt-20">
    <div class="bg-gray-900/80 backdrop-blur-sm rounded-lg border border-cyan-500/30 shadow-lg shadow-cyan-500/10 p-6 animate-fade-in">
      <h2 class="text-xl font-semibold mb-4 text-cyan-400 font-mono flex items-center gap-2">
        <span class="text-cyan-500">›</span> Configure SQS
      </h2>
      <div class="space-y-4">
        <div>
          <label class="block text-xs font-mono text-cyan-400/70 mb-2 uppercase tracking-wider">
            Endpoint URL
          </label>
          <input
            v-model="endpoint"
            type="text"
            placeholder="http://localhost:4566"
            class="w-full px-4 py-2 border border-cyan-500/30 rounded bg-black/50 text-cyan-100 font-mono text-sm focus:outline-none focus:border-cyan-400 focus:ring-1 focus:ring-cyan-400 transition-all"
          />
        </div>
        <button
          @click="configureSqs"
          :disabled="loading"
          class="w-full bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 disabled:from-gray-700 disabled:to-gray-700 text-white font-mono font-medium py-3 px-4 rounded transition-all duration-200 transform hover:scale-[1.02] active:scale-[0.98] shadow-lg shadow-cyan-500/20 cursor-pointer"
        >
          {{ loading ? '⟳ CONNECTING...' : '▶ CONNECT' }}
        </button>
      </div>

      <!-- Error Display -->
      <div v-if="error" class="mt-4 bg-red-900/30 backdrop-blur-sm border border-red-500/50 rounded-lg p-4 animate-shake">
        <p class="text-red-400 text-sm font-mono flex items-center gap-2">
          <span class="animate-pulse">⚠</span> {{ error }}
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { isConfigured, setConfigured } from '../stores/sqsStore';

const router = useRouter();
const endpoint = ref('http://localhost:4566');
const region = ref('eu-west-1');
const loading = ref(false);
const error = ref('');
const success = ref(false);

const awsRegions = [
  "us-east-1", "us-east-2", "us-west-1", "us-west-2",
  "af-south-1", "ap-east-1", "ap-south-1", "ap-northeast-1",
  "ap-northeast-2", "ap-northeast-3", "ap-southeast-1", "ap-southeast-2",
  "ca-central-1", "eu-central-1", "eu-west-1", "eu-west-2",
  "eu-west-3", "eu-north-1", "me-south-1", "sa-east-1"
];

const configureSqs = async () => {
  loading.value = true;
  error.value = '';
  success.value = false;
  try {
    await invoke('configure_sqs', { endpoint: endpoint.value, region: region.value });
    setConfigured(true);
    success.value = true;
  } catch (e) {
    error.value = String(e);
    setConfigured(false);
  } finally {
    loading.value = false;
  }
};

const goToQueues = () => {
  router.push('/queues');
};

onMounted(() => {
  if (!isConfigured.value) {
    configureSqs();
  }
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
        <div>
          <label class="block text-xs font-mono text-cyan-400/70 mb-2 uppercase tracking-wider">
            AWS Region
          </label>
          <select
            v-model="region"
            class="w-full px-4 py-2 border border-cyan-500/30 rounded bg-black/50 text-cyan-100 font-mono text-sm focus:outline-none focus:border-cyan-400 focus:ring-1 focus:ring-cyan-400 transition-all"
          >
            <option v-for="r in awsRegions" :key="r" :value="r">{{ r }}</option>
          </select>
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

      <!-- Success and Navigation -->
      <div v-if="success || isConfigured" class="mt-4">
        <div v-if="success" class="bg-green-900/30 backdrop-blur-sm border border-green-500/50 rounded-lg p-4">
          <p class="text-green-400 text-sm font-mono flex items-center gap-2">
            <span class="animate-pulse">✔</span> Connection successful!
          </p>
        </div>
        <button
          @click="goToQueues"
          class="mt-4 w-full bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white font-mono font-medium py-3 px-4 rounded transition-all duration-200 transform hover:scale-[1.02] active:scale-[0.98] shadow-lg shadow-purple-500/20 cursor-pointer"
        >
          GO TO QUEUES →
        </button>
      </div>
    </div>
  </div>
</template>

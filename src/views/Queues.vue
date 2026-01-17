<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { isConfigured } from '../stores/sqsStore';

interface QueueInfo {
  name: string;
  url: string;
}

const router = useRouter();
const queues = ref<QueueInfo[]>([]);
const newQueueName = ref('');
const loading = ref(false);
const error = ref('');

onMounted(() => {
  if (!isConfigured.value) {
    router.push('/');
    return;
  }
  loadQueues();
});

const loadQueues = async () => {
  loading.value = true;
  error.value = '';
  try {
    queues.value = await invoke<QueueInfo[]>('list_queues');
    console.log("queues", queues.value)
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

const createQueue = async () => {
  if (!newQueueName.value.trim()) return;
  loading.value = true;
  error.value = '';
  try {
    await invoke('create_queue', { queueName: newQueueName.value });
    newQueueName.value = '';
    await loadQueues();
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

const selectQueue = (queue: QueueInfo) => {
  router.push({
    name: 'messages',
    params: { queueUrl: encodeURIComponent(queue.url) },
    query: { name: queue.name }
  });
};
</script>

<template>
  <div class="max-w-4xl mx-auto mt-8">
    <div class="bg-gray-900/80 backdrop-blur-sm rounded-lg border border-cyan-500/30 shadow-lg shadow-cyan-500/10 p-6 animate-fade-in">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold text-cyan-400 font-mono flex items-center gap-2">
          <span class="text-cyan-500">›</span> Queues
        </h2>
        <button
          @click="loadQueues"
          :disabled="loading"
          class="text-cyan-400 hover:text-cyan-300 text-xs font-mono transition-colors disabled:text-gray-600 cursor-pointer"
        >
          ⟳ REFRESH
        </button>
      </div>

      <div class="space-y-2 mb-4">
        <input
          v-model="newQueueName"
          type="text"
          placeholder="queue_name"
          @keypress.enter="createQueue"
          class="w-full px-3 py-2 border border-cyan-500/30 rounded bg-black/50 text-cyan-100 font-mono text-sm focus:outline-none focus:border-cyan-400 focus:ring-1 focus:ring-cyan-400 transition-all placeholder:text-gray-600"
        />
        <button
          @click="createQueue"
          :disabled="loading || !newQueueName.trim()"
          class="w-full bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-500 hover:to-emerald-500 disabled:from-gray-700 disabled:to-gray-700 text-white font-mono font-medium py-2 px-4 rounded transition-all text-sm shadow-lg shadow-green-500/20 cursor-pointer"
        >
          + CREATE QUEUE
        </button>
      </div>

      <div class="space-y-2 max-h-96 overflow-y-auto custom-scrollbar">
        <div v-if="loading" class="text-center text-cyan-400/60 py-12 font-mono text-sm">
          <div class="animate-pulse">⟳ LOADING QUEUES...</div>
        </div>
        <button
          v-for="queue in queues"
          :key="queue.url"
          @click="selectQueue(queue)"
          class="w-full text-left px-4 py-3 rounded transition-all font-mono text-sm bg-black/30 border border-cyan-500/10 text-gray-400 hover:bg-cyan-500/10 hover:border-cyan-500/30 hover:text-cyan-300 cursor-pointer"
        >
          <span class="text-cyan-500">›</span> {{ queue.name }}
        </button>
        <div v-if="queues.length === 0 && !loading" class="text-center text-cyan-400/40 py-12 font-mono text-sm">
          <div class="animate-pulse">[ NO QUEUES AVAILABLE ]</div>
        </div>
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

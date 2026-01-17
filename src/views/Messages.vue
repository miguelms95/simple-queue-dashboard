<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { isConfigured } from '../stores/sqsStore';

interface Message {
  id: string;
  body: string;
  receipt_handle?: string;
}

const route = useRoute();
const router = useRouter();
const queueUrl = ref('');
const queueName = ref('');
const messages = ref<Message[]>([]);
const newMessageBody = ref('');
const loading = ref(false);
const error = ref('');

onMounted(() => {
  if (!isConfigured.value) {
    router.push('/');
    return;
  }
  queueUrl.value = decodeURIComponent(route.params.queueUrl as string);
  queueName.value = route.query.name as string || 'Queue';
  loadMessages();
});

const loadMessages = async () => {
  loading.value = true;
  error.value = '';
  try {
    messages.value = await invoke<Message[]>('receive_messages', {
      queueUrl: queueUrl.value,
      maxMessages: 10
    });
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

const sendMessage = async () => {
  if (!newMessageBody.value.trim()) return;
  loading.value = true;
  error.value = '';
  try {
    await invoke('send_message', {
      queueUrl: queueUrl.value,
      messageBody: newMessageBody.value
    });
    newMessageBody.value = '';
    await loadMessages();
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

const deleteMessage = async (msg: Message) => {
  if (!msg.receipt_handle) return;
  loading.value = true;
  error.value = '';
  try {
    await invoke('delete_message', {
      queueUrl: queueUrl.value,
      receiptHandle: msg.receipt_handle
    });
    await loadMessages();
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div class="max-w-4xl mx-auto mt-8">
    <div class="bg-gray-900/80 backdrop-blur-sm rounded-lg border border-cyan-500/30 shadow-lg shadow-cyan-500/10 p-6 animate-fade-in">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold text-cyan-400 font-mono flex items-center gap-2">
          <span class="text-cyan-500">›</span> {{ queueName }}
        </h2>
        <button
          @click="loadMessages"
          :disabled="loading"
          class="text-cyan-400 hover:text-cyan-300 text-xs font-mono transition-colors disabled:text-gray-600 cursor-pointer"
        >
          ↓ RECEIVE MESSAGES
        </button>
      </div>

      <!-- Send Message -->
      <div class="mb-6 space-y-2">
        <textarea
          v-model="newMessageBody"
          placeholder='{"message": "payload"}'
          rows="3"
          class="w-full px-4 py-3 border border-cyan-500/30 rounded bg-black/50 text-cyan-100 font-mono text-sm focus:outline-none focus:border-cyan-400 focus:ring-1 focus:ring-cyan-400 transition-all placeholder:text-gray-600 custom-scrollbar"
        ></textarea>
        <button
          @click="sendMessage"
          :disabled="loading || !newMessageBody.trim()"
          class="w-full bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-500 hover:to-purple-500 disabled:from-gray-700 disabled:to-gray-700 text-white font-mono font-medium py-2 px-4 rounded transition-all shadow-lg shadow-blue-500/20 cursor-pointer"
        >
          ↑ SEND MESSAGE
        </button>
      </div>

      <!-- Messages List -->
      <div class="space-y-3 max-h-[500px] overflow-y-auto custom-scrollbar">
        <div
          v-for="msg in messages"
          :key="msg.id"
          class="bg-black/50 border border-cyan-500/20 rounded-lg p-4 hover:border-cyan-500/40 transition-all animate-slide-in"
        >
          <div class="flex items-start justify-between mb-2">
            <span class="text-xs text-cyan-400/60 font-mono">ID: {{ msg.id }}</span>
            <button
              v-if="msg.receipt_handle"
              @click="deleteMessage(msg)"
              class="text-red-400 hover:text-red-300 text-xs font-mono transition-colors cursor-pointer"
            >
              ✕ DELETE
            </button>
          </div>
          <pre class="text-sm text-gray-300 whitespace-pre-wrap break-words font-mono">{{ msg.body }}</pre>
        </div>
        <div v-if="messages.length === 0 && !loading" class="text-center text-cyan-400/40 py-12 font-mono text-sm">
          <div class="animate-pulse">[ NO MESSAGES AVAILABLE ]</div>
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

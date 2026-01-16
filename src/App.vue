<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface QueueInfo {
  name: string;
  url: string;
}

interface Message {
  id: string;
  body: string;
  receipt_handle?: string;
}

const darkMode = ref(false);
const configured = ref(false);
const endpoint = ref('http://localhost:4566');
const queues = ref<QueueInfo[]>([]);
const selectedQueue = ref<QueueInfo | null>(null);
const messages = ref<Message[]>([]);
const newQueueName = ref('');
const newMessageBody = ref('');
const loading = ref(false);
const error = ref('');

onMounted(() => {
  const saved = localStorage.getItem('darkMode');
  darkMode.value = saved === 'true';
  updateDarkMode();
});

const updateDarkMode = () => {
  if (darkMode.value) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
  localStorage.setItem('darkMode', darkMode.value.toString());
};

const toggleDarkMode = () => {
  darkMode.value = !darkMode.value;
  updateDarkMode();
};

const configureSqs = async () => {
  loading.value = true;
  error.value = '';
  try {
    await invoke('configure_sqs', { endpoint: endpoint.value });
    configured.value = true;
    await loadQueues();
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

const loadQueues = async () => {
  loading.value = true;
  error.value = '';
  try {
    queues.value = await invoke<QueueInfo[]>('list_queues');
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

const selectQueue = async (queue: QueueInfo) => {
  selectedQueue.value = queue;
  await loadMessages();
};

const loadMessages = async () => {
  if (!selectedQueue.value) return;
  loading.value = true;
  error.value = '';
  try {
    messages.value = await invoke<Message[]>('receive_messages', {
      queueUrl: selectedQueue.value.url,
      maxMessages: 10
    });
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

const sendMessage = async () => {
  if (!selectedQueue.value || !newMessageBody.value.trim()) return;
  loading.value = true;
  error.value = '';
  try {
    await invoke('send_message', {
      queueUrl: selectedQueue.value.url,
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
  if (!selectedQueue.value || !msg.receipt_handle) return;
  loading.value = true;
  error.value = '';
  try {
    await invoke('delete_message', {
      queueUrl: selectedQueue.value.url,
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
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors">
    <!-- Header -->
    <header class="bg-white dark:bg-gray-800 shadow">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex items-center justify-between">
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">SQS Dashboard</h1>
        <button
          @click="toggleDarkMode"
          class="p-2 rounded-lg bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
        >
          <span v-if="darkMode" class="text-yellow-400">☀️</span>
          <span v-else class="text-gray-700">🌙</span>
        </button>
      </div>
    </header>

    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <!-- Configuration -->
      <div v-if="!configured" class="max-w-md mx-auto">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h2 class="text-xl font-semibold mb-4 text-gray-900 dark:text-white">Configure SQS</h2>
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Endpoint URL
              </label>
              <input
                v-model="endpoint"
                type="text"
                placeholder="http://localhost:4566"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <button
              @click="configureSqs"
              :disabled="loading"
              class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white font-medium py-2 px-4 rounded-lg transition-colors"
            >
              {{ loading ? 'Connecting...' : 'Connect' }}
            </button>
          </div>
        </div>
      </div>

      <!-- Main Interface -->
      <div v-else class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Queues List -->
        <div class="lg:col-span-1">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
            <div class="flex items-center justify-between mb-4">
              <h2 class="text-xl font-semibold text-gray-900 dark:text-white">Queues</h2>
              <button
                @click="loadQueues"
                :disabled="loading"
                class="text-blue-600 dark:text-blue-400 hover:text-blue-700 text-sm"
              >
                Refresh
              </button>
            </div>

            <div class="space-y-2 mb-4">
              <input
                v-model="newQueueName"
                type="text"
                placeholder="New queue name"
                @keypress.enter="createQueue"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm focus:ring-2 focus:ring-blue-500"
              />
              <button
                @click="createQueue"
                :disabled="loading || !newQueueName.trim()"
                class="w-full bg-green-600 hover:bg-green-700 disabled:bg-gray-400 text-white font-medium py-2 px-4 rounded-lg transition-colors text-sm"
              >
                Create Queue
              </button>
            </div>

            <div class="space-y-2 max-h-96 overflow-y-auto">
              <button
                v-for="queue in queues"
                :key="queue.url"
                @click="selectQueue(queue)"
                :class="[
                  'w-full text-left px-4 py-3 rounded-lg transition-colors',
                  selectedQueue?.url === queue.url
                    ? 'bg-blue-100 dark:bg-blue-900 text-blue-900 dark:text-blue-100'
                    : 'bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white hover:bg-gray-100 dark:hover:bg-gray-600'
                ]"
              >
                {{ queue.name }}
              </button>
            </div>
          </div>
        </div>

        <!-- Messages -->
        <div class="lg:col-span-2">
          <div v-if="selectedQueue" class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
            <div class="flex items-center justify-between mb-4">
              <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
                {{ selectedQueue.name }}
              </h2>
              <button
                @click="loadMessages"
                :disabled="loading"
                class="text-blue-600 dark:text-blue-400 hover:text-blue-700 text-sm"
              >
                Receive Messages
              </button>
            </div>

            <!-- Send Message -->
            <div class="mb-6 space-y-2">
              <textarea
                v-model="newMessageBody"
                placeholder="Message body (JSON or text)"
                rows="3"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500"
              ></textarea>
              <button
                @click="sendMessage"
                :disabled="loading || !newMessageBody.trim()"
                class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white font-medium py-2 px-4 rounded-lg transition-colors"
              >
                Send Message
              </button>
            </div>

            <!-- Messages List -->
            <div class="space-y-3">
              <div
                v-for="msg in messages"
                :key="msg.id"
                class="bg-gray-50 dark:bg-gray-700 rounded-lg p-4"
              >
                <div class="flex items-start justify-between mb-2">
                  <span class="text-xs text-gray-500 dark:text-gray-400 font-mono">{{ msg.id }}</span>
                  <button
                    v-if="msg.receipt_handle"
                    @click="deleteMessage(msg)"
                    class="text-red-600 dark:text-red-400 hover:text-red-700 text-xs"
                  >
                    Delete
                  </button>
                </div>
                <pre class="text-sm text-gray-900 dark:text-white whitespace-pre-wrap break-words">{{ msg.body }}</pre>
              </div>
              <div v-if="messages.length === 0" class="text-center text-gray-500 dark:text-gray-400 py-8">
                No messages available
              </div>
            </div>
          </div>
          <div v-else class="bg-white dark:bg-gray-800 rounded-lg shadow p-12 text-center">
            <p class="text-gray-500 dark:text-gray-400">Select a queue to view messages</p>
          </div>
        </div>
      </div>

      <!-- Error Display -->
      <div v-if="error" class="mt-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
        <p class="text-red-800 dark:text-red-200 text-sm">{{ error }}</p>
      </div>
    </main>
  </div>
</template>
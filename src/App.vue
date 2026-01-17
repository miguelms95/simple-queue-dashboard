<script setup lang="ts">
import { region } from './stores/sqsStore';
</script>

<template>
  <div class="min-h-screen bg-black text-gray-100">
    <!-- Animated Background -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
      <div class="absolute inset-0 bg-gradient-to-br from-blue-900/10 via-black to-purple-900/10"></div>
      <div class="grid-overlay"></div>
    </div>

    <!-- Header -->
    <header class="relative bg-black/50 backdrop-blur-sm border-b border-cyan-500/20 shadow-lg shadow-cyan-500/5">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex items-center justify-between">
        <router-link to="/" class="cursor-pointer">
          <h1 class="text-2xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-blue-500 flex items-center gap-2 hover:from-cyan-300 hover:to-blue-400 transition-all">
            <span class="animate-pulse">▶</span>
            SQS Dashboard
          </h1>
        </router-link>
        <div class="flex items-center gap-2 text-xs font-mono text-cyan-400/60">
          <span class="animate-pulse">●</span>
          <span>{{ region }}</span>
        </div>
      </div>
    </header>

    <main class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="min-h-screen bg-gray-50 py-8">
    <div class="max-w-4xl mx-auto px-4">
      <!-- 标题区域 -->
      <div class="text-center mb-8">
        <h1 class="text-4xl font-bold text-gray-900 mb-2">Focus-Daily</h1>
        <p class="text-lg text-gray-600">Tauri + Vue3 + Tailwind CSS 开发环境测试</p>
      </div>

      <!-- 环境验证卡片 -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <!-- Tauri 测试 -->
        <div class="card">
          <h2 class="text-xl font-semibold text-gray-800 mb-4">🦀 Tauri 后端测试</h2>
          <form @submit.prevent="greet" class="space-y-4">
            <input 
              v-model="name" 
              placeholder="输入您的名字..." 
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <button type="submit" class="btn-primary w-full">
              调用 Rust 函数
            </button>
          </form>
          <p v-if="greetMsg" class="mt-4 p-3 bg-green-50 text-green-800 rounded-lg">
            {{ greetMsg }}
          </p>
        </div>

        <!-- Tailwind CSS 测试 -->
        <div class="card">
          <h2 class="text-xl font-semibold text-gray-800 mb-4">🎨 Tailwind CSS 测试</h2>
          <div class="space-y-3">
            <button class="btn-primary w-full">主要按钮</button>
            <button class="btn-secondary w-full">次要按钮</button>
            <div class="p-3 bg-blue-50 text-blue-800 rounded-lg">
              ✅ Tailwind CSS 样式正常工作
            </div>
          </div>
        </div>
      </div>

      <!-- 技术栈信息 -->
      <div class="card">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">📋 技术栈配置状态</h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="flex items-center space-x-2">
            <span class="w-3 h-3 bg-green-500 rounded-full"></span>
            <span class="text-gray-700">Vue 3 + TypeScript</span>
          </div>
          <div class="flex items-center space-x-2">
            <span class="w-3 h-3 bg-green-500 rounded-full"></span>
            <span class="text-gray-700">Tailwind CSS</span>
          </div>
          <div class="flex items-center space-x-2">
            <span class="w-3 h-3 bg-green-500 rounded-full"></span>
            <span class="text-gray-700">Pinia 状态管理</span>
          </div>
          <div class="flex items-center space-x-2">
            <span class="w-3 h-3 bg-green-500 rounded-full"></span>
            <span class="text-gray-700">Tauri 桌面框架</span>
          </div>
          <div class="flex items-center space-x-2">
            <span class="w-3 h-3 bg-green-500 rounded-full"></span>
            <span class="text-gray-700">Vite 构建工具</span>
          </div>
          <div class="flex items-center space-x-2">
            <span class="w-3 h-3 bg-green-500 rounded-full"></span>
            <span class="text-gray-700">Rust 后端</span>
          </div>
        </div>
      </div>

      <!-- 下一步计划 -->
      <div class="card mt-6">
        <h2 class="text-xl font-semibold text-gray-800 mb-4">🚀 下一步开发计划</h2>
        <ul class="space-y-2 text-gray-700">
          <li class="flex items-center space-x-2">
            <span class="w-2 h-2 bg-blue-500 rounded-full"></span>
            <span>配置 SQLite 数据库</span>
          </li>
          <li class="flex items-center space-x-2">
            <span class="w-2 h-2 bg-blue-500 rounded-full"></span>
            <span>实现定时器核心系统</span>
          </li>
          <li class="flex items-center space-x-2">
            <span class="w-2 h-2 bg-blue-500 rounded-full"></span>
            <span>开发音频播放功能</span>
          </li>
          <li class="flex items-center space-x-2">
            <span class="w-2 h-2 bg-blue-500 rounded-full"></span>
            <span>创建基础 UI 组件库</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
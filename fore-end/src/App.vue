<template>
  <div>
    <nav>
      <button @click="currentPage = 'ParameterPage'">参数调节</button>
      <button @click="currentPage = 'AddQQId'">权限设置</button>
      <button @click="currentPage = 'UsageStats'">用量统计</button>
    </nav>
    <div v-if="currentPage === 'ParameterPage'">
      <ParameterPage />
    </div>
    <div v-else-if="currentPage === 'AddQQId'">
      <AddQQId />
    </div>
    <div v-else-if="currentPage === 'UsageStats'">
      <UsageStats />
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from "vue";
import ParameterPage from "./components/ParameterPage.vue";
import AddQQId from "./components/AddQQId.vue";
import UsageStats from "./components/UsageStats.vue";

// 尝试从 localStorage 获取当前页面，如果没有则默认显示参数调节页面
const storedPage = localStorage.getItem('currentPage');
const currentPage = ref(storedPage || 'ParameterPage');

// 页面加载时，恢复之前的页面状态
onMounted(() => {
  const storedPage = localStorage.getItem('currentPage');
  if (storedPage) {
    currentPage.value = storedPage;
  }
});

// 监听 currentPage 的变化，将新值存储到 localStorage
watch(currentPage, (newPage) => {
  localStorage.setItem('currentPage', newPage);
});
</script>

<style lang="scss" scoped>
nav {
  margin-bottom: 16px;
  display: flex;
  justify-content: center;
  background-color: #f8f9fa;
  padding: 12px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

button {
  margin-right: 8px;
  padding: 8px 16px;
  cursor: pointer;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  transition: background-color 0.3s;
}

button:hover {
  background-color: #0056b3;
}

button:last-child {
  margin-right: 0;
}

div {
  padding: 16px;
  font-family: Arial, sans-serif;
  color: #333;
}
</style>
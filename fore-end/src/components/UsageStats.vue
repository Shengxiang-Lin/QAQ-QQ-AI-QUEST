<template>
  <div class="usage-stats-container">
    <h2>用量统计</h2>
    <p>DeepSeek API 请求数: {{ deepseekRequestCount }}</p>
    <p>DeepSeek 消耗的 Token 数: {{ deepseekTokenUsage }}</p>
    <p>豆包 API 请求数: {{ doubaoRequestCount }}</p>
    <p>豆包 消耗的 Token 数: {{ doubaoTokenUsage }}</p>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import axios from "axios";

const BASE_URL = `http://localhost:${__HOST_PORT__}`;

const deepseekRequestCount = ref(0);
const deepseekTokenUsage = ref(0);
const doubaoRequestCount = ref(0);
const doubaoTokenUsage = ref(0);

const fetchUsageStats = async () => {
  try {
    const response = await axios.get(`${BASE_URL}/usage_stats`);
    deepseekRequestCount.value = response.data.deepseek_request_count;
    deepseekTokenUsage.value = response.data.deepseek_token_usage;
    doubaoRequestCount.value = response.data.doubao_request_count;
    doubaoTokenUsage.value = response.data.doubao_token_usage;
  } catch (error) {
    console.error('获取用量统计信息失败：', error);
  }
};

let intervalId;

onMounted(() => {
  // 初始加载数据
  fetchUsageStats();
  // 每 5 秒（5000 毫秒）更新一次数据
  intervalId = setInterval(fetchUsageStats, 5000);
});

onUnmounted(() => {
  // 组件卸载时清除定时器
  clearInterval(intervalId);
});
</script>

<style lang="scss" scoped>
.usage-stats-container {
  max-width: 400px;
  margin: 0 auto;
  padding: 20px;
  border: 1px solid #ddd;
  border-radius: 8px;
  background-color: #f9f9f9;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

h2 {
  text-align: center;
  margin-bottom: 20px;
}

p {
  margin-bottom: 10px;
}
</style>
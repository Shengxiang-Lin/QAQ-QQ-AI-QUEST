<template>
  <div class="usage-stats-container">
    <h2>用量统计</h2>
    <p>DeepSeek API 请求数: {{ deepseekRequestCount }}</p>
    <p>DeepSeek 消耗的 Token 数: {{ deepseekTokenUsage }}</p>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import axios from "axios";

const BASE_URL = `http://localhost:${__HOST_PORT__}`;

const deepseekRequestCount = ref(0);
const deepseekTokenUsage = ref(0);

const fetchUsageStats = async () => {
  try {
    const response = await axios.get(`${BASE_URL}/usage_stats`);
    deepseekRequestCount.value = response.data.deepseek_request_count;
    deepseekTokenUsage.value = response.data.deepseek_token_usage;
  } catch (error) {
    console.error('获取用量统计信息失败：', error);
  }
};

onMounted(() => {
  fetchUsageStats();
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
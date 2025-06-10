<template>
  <div class="usage-stats-container">
    <!-- DeepSeek 卡片 -->
    <div class="card">
      <h2 class="title">DeepSeek 用量统计</h2>
      <div class="stat-item">
        <i class="fa fa-chart-bar"></i>
        <p>DeepSeek API 请求数: {{ deepseekRequestCount }}</p>
      </div>
      <div class="stat-item">
        <i class="fa fa-ticket"></i>
        <p>DeepSeek 消耗的 Token 数: {{ deepseekTokenUsage }}</p>
      </div>
    </div>
    <!-- 豆包卡片 -->
    <div class="card">
      <h2 class="title">豆包用量统计</h2>
      <div class="stat-item">
        <i class="fa fa-chart-bar"></i>
        <p>豆包 API 请求数: {{ doubaoRequestCount }}</p>
      </div>
      <div class="stat-item">
        <i class="fa fa-ticket"></i>
        <p>豆包 消耗的 Token 数: {{ doubaoTokenUsage }}</p>
      </div>
    </div>
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
// 引入 Font Awesome 图标库
@import url('https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css');

.usage-stats-container {
  display: flex;
  justify-content: center;
  // 修改此处，减少顶部的外边距，使卡片上移
  margin-top: 50px; 
  gap: 20px; 
  flex-wrap: wrap; 
}

.card {
  background-color: #fff;
  border-radius: 12px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  padding: 30px;
  width: 100%;
  max-width: 450px;
}

.title {
  text-align: center;
  margin-bottom: 25px;
  color: #333;
  font-size: 24px;
  font-weight: 600;
}

.stat-item {
  display: flex;
  align-items: center;
  margin-bottom: 15px;
}

.stat-item i {
  font-size: 20px;
  color: #007bff;
  margin-right: 15px;
}

.stat-item p {
  margin: 0;
  font-size: 16px;
  color: #555;
}
</style>
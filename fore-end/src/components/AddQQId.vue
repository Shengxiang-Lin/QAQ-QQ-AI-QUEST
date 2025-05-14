<template>
  <div class="list-container">
    <h2>管理列表</h2>
    {{ parameters.description }}
    <!-- 搜索框 -->
    <div class="form-group">
      <input
        type="text"
        v-model="searchQuery"
        placeholder="搜索..."
        class="search-input"
      />
    </div>

    <!-- 列表展示 -->
    <ul class="list">
      <li v-for="(item, index) in filteredList" :key="index" class="list-item">
        <input type="text" v-model="item.text" class="list-input" />
        <button @click="removeItem(index)" class="delete-button">删除</button>
      </li>
    </ul>

    <!-- 添加新元素 -->
    <div class="form-group">
      <input
        type="text"
        v-model="newItem"
        placeholder="添加新元素..."
        class="add-input"
      />
      <button @click="addItem" class="add-button">添加</button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from "vue";
import { getAllParameters, updateJS } from "@/utils.js";
const parameters = ref({});
const rawParameter = ref({});
onMounted(async () => {
  rawParameter.value = await getAllParameters();
  parameters.value = rawParameter.value["valid_QQid"];
  list.value = parameters.value.value;
});

// 列表数据
const list = ref([]);

// 搜索框绑定值
const searchQuery = ref("");

// 新元素输入框绑定值
const newItem = ref("");

// 计算过滤后的列表
const filteredList = computed(() => {
  return list.value.filter((item) =>
    item.text.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

// 添加新元素
const addItem = () => {
  if (!/^\d+$/.test(newItem.value)) {
    alert("参数值必须为纯数字！");
    return;
  }
  list.value.push({ text: newItem.value.trim() });
  newItem.value = "";
  submitChange("valid_QQid", {
    value: list.value,
    description: parameters.value.description,
    range: parameters.value.range,
    type: parameters.value.type,
  });
};

// 删除元素
const removeItem = (index) => {
  list.value.splice(index, 1);
  submitChange("valid_QQid", {
    value: list.value,
    description: parameters.value.description,
    range: parameters.value.range,
    type: parameters.value.type,
  });
};

const submitChange = async (name, UpdateParameter) => {
  rawParameter.value[name] = UpdateParameter;
  parameters.value = UpdateParameter;
  await updateJS(rawParameter.value);
  alert("参数已更新!");
};
</script>

<style lang="scss" scoped>
.list-container {
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

.form-group {
  margin-bottom: 15px;
}

.search-input,
.add-input,
.list-input {
  width: 100%;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  box-sizing: border-box;
  margin-bottom: 10px;
}

.list {
  list-style: none;
  padding: 0;
}

.list-item {
  display: flex;
  align-items: center;
  margin-bottom: 10px;
}

.list-input {
  flex: 1;
  margin-right: 10px;
}

.add-button,
.delete-button {
  padding: 8px 12px;
  background-color: #007bff;
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.add-button:hover,
.delete-button:hover {
  background-color: #0056b3;
}
</style>

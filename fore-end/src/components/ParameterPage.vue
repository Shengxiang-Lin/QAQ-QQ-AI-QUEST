<template>
  <div class="app-container">
    <h1 class="title">QQ AI Quest 参数设置</h1>
    <!-- 新增选择模型的下拉框 -->
    <select v-model="selectedModel" @change="submitModelChange">
      <option value="deepseek-chat">DeepSeek</option>
      <option value="doubao-1.5-vision-pro-32k-250115">Doubao</option>
    </select>
    <!-- 之前的下拉选择框 -->
    <select v-model="selectedConfig" @change="useSelectedConfig">
      <option value="" disabled>选择配置方案</option>
      <option v-for="(value, index) in configFiles" :key="value" :value="value">
        {{ configNames[index] }}
      </option>
    </select>
    <div class="parameters-container">
      <div
        v-for="(value, key) in filteredParameters"
        :key="key"
        class="parameter-item"
      >
        <Parameter
          :parameterName="key"
          :parameterDescription="value.description"
          :parameterValue="value.value"
          :parameterRange="value.range"
          :parameterType="value.type"
          :parameterHidden="key === 'deepseek_key' || key === 'doubao_key'"
          :bigSize="false"
          @submitParameter="submitChange"
        />
      </div>
    </div>
    <div v-if="parameters['default_prompt']" class="default-prompt-container">
      <Parameter
        :parameterName="'default_prompt'"
        :parameterDescription="parameters['default_prompt'].description"
        :parameterValue="parameters['default_prompt'].value"
        :parameterRange="parameters['default_prompt'].range"
        :parameterType="parameters['default_prompt'].type"
        :parameterHidden="false"
        :bigSize="true"
        @submitParameter="submitChange"
      />
    </div>
  </div>
</template>

<script setup>
import { getAllParameters, updateJS } from "@/utils.js";
import Parameter from "@/components/Parameter.vue";
import { ref, onMounted, computed } from "vue";
import axios from "axios";

const parameters = ref({});
const configFiles = ref([]);
const configNames = ["灵感源泉", "文案编辑", "聊天助手"];
const selectedConfig = ref("");
const selectedModel = ref("deepseek-chat"); // 默认选择 DeepSeek

const filteredParameters = computed(() => {
  let keys = Object.keys(parameters.value).filter((key) => {
    return key !== "default_prompt" && key !== "valid_QQid";
  });
  return keys.reduce((obj, key) => {
    obj[key] = parameters.value[key];
    return obj;
  }, {});
});

onMounted(async () => {
  parameters.value = await getAllParameters();
  console.log("parameters", parameters.value);
  // 获取 config_new 文件夹下的所有配置文件列表
  try {
    const response = await axios.get(
      `http://localhost:${__HOST_PORT__}/config_new_list`
    );
    configFiles.value = response.data;
  } catch (error) {
    console.error("获取配置文件列表失败：", error);
    alert("获取配置文件列表失败，请检查！");
  }
});

const submitChange = async (name, UpdateParameter) => {
  parameters.value[name] = UpdateParameter;
  await updateJS(parameters.value);
  alert("参数已更新!");
};

// 使用选中的配置方案
const useSelectedConfig = async () => {
  if (selectedConfig.value) {
    try {
      // 存储选中的配置文件
      localStorage.setItem('selectedConfig', selectedConfig.value);

      // 获取选中的配置文件内容
      const response = await axios.get(
        `http://localhost:${__HOST_PORT__}/config_new/${selectedConfig.value}`
      );
      const newConfig = response.data;

      // 更新网页上的参数
      parameters.value = newConfig;

      // 更新 config.json 的内容
      await updateJS(newConfig);
      alert(`已使用 ${selectedConfig.value} 的配置！`);
    } catch (error) {
      console.error("获取配置文件失败：", error);
      alert("获取配置文件失败，请检查！");
    }
  }
};

// 提交选择的模型
const submitModelChange = async () => {
  try {
    await axios.post(`http://localhost:${__HOST_PORT__}/update_model`, {
      model: selectedModel.value,
    });
    alert(
      `已切换到 ${
        selectedModel.value === "deepseek-chat" ? "DeepSeek" : "Doubao"
      } 模型！`
    );
  } catch (error) {
    console.error("切换模型失败：", error);
    alert("切换模型失败，请检查！");
  }
};
</script>

<style lang="scss" scoped>
.app-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px;
  background-color: #f9f9f9;
  font-family: Arial, sans-serif;
}

.title {
  font-size: 2.5rem;
  font-weight: bold;
  color: #4a90e2;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
  margin-bottom: 20px;
}

.parameters-container {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  justify-content: center;
  width: 100%;
}

.parameter-item {
  flex: 1 1 calc(33.333% - 20px);
  max-width: calc(33.333% - 20px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 10px;
  background-color: #fff;
  border-radius: 8px;
}

.default-prompt-container {
  margin-top: 30px;
  width: 500px;
  display: flex;
  justify-content: center;
  background-color: #fff;
  padding: 10px;
  border-radius: 8px;
}

select {
  margin-bottom: 20px;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
  cursor: pointer;
}
</style>

<template>
  <div class="app-container">
    <h1 class="title">QQ AI Quest 参数设置</h1>
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

const parameters = ref({});

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
});

const submitChange = async (name, UpdateParameter) => {
  parameters.value[name] = UpdateParameter;
  await updateJS(parameters.value);
  alert("参数已更新!");
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
</style>

<template>
  <div
    class="parameter-container"
    :style="{ width: props.bigSize ? '100%' : 'auto' }"
  >
    <h2>参数调整</h2>
    <p class="description" v-if="props.parameterDescription">
      {{ props.parameterDescription }}
    </p>
    <form @submit.prevent="submitParameter">
      <div class="form-group">
        <label for="parameterName">参数名：</label>
        <input
          :id="props.parameterName"
          type="text"
          v-model="parameter.name"
          disabled
        />
      </div>
      <div class="form-group">
        <label for="parameterValue">参数值：</label>
        <input
          v-if="!props.bigSize"
          :id="parameterValue + props.parameterName"
          :type="parameter.hidden ? 'password' : 'text'"
          v-model="parameter._value"
        />
        <textarea
          v-else
          v-model="parameter._value"
          class="big-input"
        ></textarea>
      </div>
      <div v-if="parameter.range" class="form-group">
        <small>范围：{{ parameter.range }}</small>
      </div>
      <button type="submit">提交</button>
    </form>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
const props = defineProps({
  parameterName: {
    type: String,
    required: true,
  },
  parameterDescription: {
    type: String,
    required: true,
  },
  parameterValue: {
    required: true,
  },
  parameterRange: {
    type: Array,
    default: () => null,
  },
  parameterType: {
    // string,int,float,bool
    type: String,
    required: true,
  },
  parameterHidden: {
    type: Boolean,
    default: false,
  },

  bigSize: {
    type: Boolean,
    default: false,
  },
});
const emit = defineEmits(["submitParameter"]);

const parameter = ref({
  name: props.parameterName,
  _value: props.parameterValue,
  range: props.parameterRange,
  hidden: props.parameterHidden,
});

const ParserValue = (value) => {
  if (props.parameterType === "bool") {
    if (value === "true") return true;
    else if (value === "false") return false;
    else return null;
  } else if (props.parameterType === "float") {
    value = Number.parseFloat(value);
    if (Number.isNaN(value)) return null;
    let mi = props.parameterRange[0],
      ma = props.parameterRange[1];
    if (mi <= value && value <= ma) return value;
    else return null;
  } else if (props.parameterType === "int") {
    value = Number.parseInt(value);
    if (Number.isNaN(value)) return null;
    let mi = props.parameterRange[0],
      ma = props.parameterRange[1];
    if (mi <= value && value <= ma) return value;
    else return null;
  } else if (props.parameterType === "string") {
    if (value.length > 0) return value;
    else return null;
  }
};

const submitParameter = () => {
  let new_value = ParserValue(parameter.value._value);
  if (new_value === null) {
    alert("参数值不合法！");
    return;
  } else {
    parameter.value._value = new_value;
  }
  emit("submitParameter", parameter.value.name, {
    value: parameter.value._value,
    description: props.parameterDescription,
    range: parameter.value.range,
    type: props.parameterType,
  });
};
</script>

<style lang="scss" scoped>
.parameter-container {
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

.description {
  text-align: center;
  margin-bottom: 15px;
  font-size: 14px;
  color: #666;
}

.form-group {
  margin-bottom: 15px;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

input[type="text"],
input[type="password"] {
  width: 100%;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  box-sizing: border-box;
}

.big-input {
  height: 300px;
  width: 100%;
  font-size: 16px;
  font-weight: 400;
  white-space: wrap;
}

button {
  width: 100%;
  padding: 10px;
  background-color: #007bff;
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background-color: #0056b3;
}
</style>

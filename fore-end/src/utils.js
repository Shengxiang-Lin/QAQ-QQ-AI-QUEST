import axios from 'axios';

const BASE_URL = `http://localhost:${__HOST_PORT__}`;


export const updateJS = async (updatedParameter) => {
    try {
        const response = await axios.post(`${BASE_URL}/update_config`, updatedParameter, {
            headers: {
                'Content-Type': 'application/json',
            },
        });

        console.log('参数提交成功！', response.data);
    } catch (error) {
        console.error('提交参数失败：', error);
    }
};

export const getAllParameters = async () => {
    try {
        const response = await axios.get(`${BASE_URL}/config`);
        return response.data;
    } catch (error) {
        console.error('获取所有参数失败：', error);
        return {};
    }
};
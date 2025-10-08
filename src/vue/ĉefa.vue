<template>
    <div class="ujo">
        <p>
            当前串口:
            <select>
                <option>COM1</option>
            </select>
        </p>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const count = ref(0)

const greet = async () => {
    try {
        const result = await invoke('greet', { name: 'Vue User' })
        console.log('Rust 响应:', result)
    } catch (error) {
        console.error('调用 Rust 失败:', error)
    }
}
</script>

<style>
.container {
    padding: 20px;
    text-align: center;
    font-family: Arial, sans-serif;
}

button {
    margin: 5px;
    padding: 10px 15px;
}
</style>
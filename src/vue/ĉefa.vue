<template>
    <div class="ujo">
        <p>
            当前串口:
            <select>
                <option>COM1</option>
            </select>
        </p>
    </div>
    <el-button plain  @click="dialogVisible = true">
        Click to open the Dialog
    </el-button>
    <el-dialog 
    v-model="dialogVisible"
    title="Tips"
    width="500"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    :show-close="false"
    >
        <span>This is a message</span>
        <template #footer>
            <div class="dialog-footer">
                <el-button @click="dialogVisible = false">Cancel</el-button>
                <el-button type="primary" @click="dialogVisible = false">
                    Confirm
                </el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref } from 'vue'

invoke('akiri_disponeblajn_seriaportojn');

const dialogVisible = ref(false)

</script>

import { invoke } from "@tauri-apps/api/core";
import { createApp } from 'vue'
import ĉefa from "../vue/ĉefa.vue";

createApp(ĉefa).mount('#aplikaĵo')
invoke('akiri_disponeblajn_seriaportojn');

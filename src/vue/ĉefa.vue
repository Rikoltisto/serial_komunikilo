<template>
    <div class="ujo">
        <p>
            当前串口:
            <select name="seriaj_havenoj">
                <option>COM1</option>
            </select>
        </p>
    </div>
    <el-dialog v-model="dialogo_videbla" title="检测到更新" width="500" :close-on-click-modal="false"
        :close-on-press-escape="false" :show-close="false">
        <span>
            <el-text class="mx-1" type="info">当前版本: {{ ĝisdatiga_informo?.nuna_versio }}</el-text><br />
            <el-text class="mx-1" type="success">最新版本: {{ ĝisdatiga_informo?.versio }}</el-text><br />
            <el-text class="mx-1" type="info">更新日期: {{ ĝisdatiga_informo?.dato }}</el-text><br />
            <el-text class="mx-1" type="info">更新说明: {{ ĝisdatiga_informo?.notoj }}</el-text>
        </span>
        <template #footer>
            <div>
                <el-button type="danger" @click="eliri">退出</el-button>
                <el-button type="success" @click="elŝuti_kaj_ĝisdatigi">更新</el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from 'vue';

interface VersiaInformo {
    versio: String,
    nuna_versio: String,
    notoj: String,
    dato: String,
}

let dialogo_videbla = ref(false);
let ĝisdatiga_informo = ref<VersiaInformo>();

onMounted(() => {
    kontroli_ĝisdatigojn();
})

function kontroli_ĝisdatigojn() {
    invoke("kontroli_ĝisdatigojn").then((rezulto) => {
        const rezulta_tabelo = rezulto as [boolean, VersiaInformo];
        if (rezulta_tabelo[0]) {
            ĝisdatiga_informo.value = konverti_tempon(rezulta_tabelo[1]);
            dialogo_videbla.value = true;
        }
    });
}

function konverti_tempon(ĝisdatiga_informo: VersiaInformo):VersiaInformo {
    //Dividi per Spaco.
    const dato = ĝisdatiga_informo.dato.split(' ');
    const [jaro, monato, tago] = dato[0].split('-');
    
    //Konverti Tempo-Formaton.
    ĝisdatiga_informo.dato = `${monato}月${tago}日${jaro}年`;
    
    return ĝisdatiga_informo;
}

function eliri() {
    invoke("eliri");
}

function elŝuti_kaj_ĝisdatigi() {
    invoke("elŝuti_kaj_ĝisdatigi");
}

</script>

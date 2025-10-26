<template>
  <el-dialog
    v-model="dialogo_mesaĝo_videblas"
    title="检测到更新"
    width="500"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    :show-close="false"
  >
    <span>
      <el-text class="mx-1" type="info"
        >当前版本: {{ ĝisdatiga_informo?.nuna_versio }}</el-text
      ><br />
      <el-text class="mx-1" type="success"
        >最新版本: {{ ĝisdatiga_informo?.versio }}</el-text
      ><br />
      <el-text class="mx-1" type="info"
        >更新日期: {{ ĝisdatiga_informo?.dato }}</el-text
      ><br />
      <el-text class="mx-1" type="info"
        >更新说明: {{ ĝisdatiga_informo?.notoj }}</el-text
      >
    </span>
    <template #footer>
      <div>
        <el-button type="danger" @click="eliri">退出</el-button>
        <el-button type="success" @click="elŝuti">更新</el-button>
      </div>
    </template>
  </el-dialog>
  <el-dialog
    v-model="ĝisdatiga_dialogo_videblas"
    title="正在更新中"
    width="500"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    :show-close="false"
  >
    <span>
      <el-text class="mx-1" type="info"
        >版本变更:
        {{ ĝisdatiga_informo?.nuna_versio }}&nbsp;&nbsp;>&nbsp;&nbsp;{{
          ĝisdatiga_informo?.versio
        }}</el-text
      ><br />
      <el-text class="mx-1" type="info">下载进度: </el-text>
      <el-progress
        :percentage="procentaĵo"
        :stroke-width="15"
        :status="procentaĵo === 100 ? 'success' : undefined"
        striped
        striped-flow
        :duration="10"
      />
    </span>
  </el-dialog>
</template>

<script setup lang="ts">
import { Channel, invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

interface VersiaInformo {
  versio: String;
  nuna_versio: String;
  notoj: String;
  dato: String;
}

type ElŝutaEvento =
  | {
      evento: "Komencita";
      datumo: {
        enhava_longo: number;
      };
    }
  | {
      evento: "Progreso";
      datumo: {
        elŝutita: number;
      };
    }
  | {
      evento: "Finita";
      datumo: null;
    };

let dialogo_mesaĝo_videblas = ref(false);
let ĝisdatiga_dialogo_videblas = ref(false);
let ĝisdatiga_informo = ref<VersiaInformo>();
let pri_evento = new Channel<ElŝutaEvento>();
let enhava_longo = ref<number>(0);
let procentaĵo = ref<number>();
let dosiera_vojo = ref<string>("");

onMounted(async () => {
  kontroli_ĝisdatigojn();
});

pri_evento.onmessage = async (mesaĝo) => {
  switch (mesaĝo.evento) {
    case "Komencita":
      dialogo_mesaĝo_videblas.value = false;
      ĝisdatiga_dialogo_videblas.value = true;
      enhava_longo.value = mesaĝo.datumo.enhava_longo;
      break;
    case "Progreso":
      procentaĵo.value = Math.min(
        100,
        Number(
          ((mesaĝo.datumo.elŝutita / enhava_longo.value) * 100).toFixed(1),
        ),
      );
      break;
    case "Finita":
      procentaĵo.value = 100;
      await dormi(3000);
      instali();
      break;
  }
};

function kontroli_ĝisdatigojn() {
  invoke("kontroli_ĝisdatigojn").then((rezulto) => {
    if (rezulto != null) {
      ĝisdatiga_informo.value = konverti_tempon(rezulto as VersiaInformo);
      dialogo_mesaĝo_videblas.value = true;
    }
  });
}

function konverti_tempon(ĝisdatiga_informo: VersiaInformo): VersiaInformo {
  //Dividi per Spaco.
  const dato = ĝisdatiga_informo.dato.split(" ");
  const [jaro, monato, tago] = dato[0].split("-");

  //Konverti Tempo-Formaton.
  ĝisdatiga_informo.dato = `${monato}月${tago}日${jaro}年`;

  return ĝisdatiga_informo;
}

async function elŝuti() {
  dosiera_vojo.value = await invoke("elŝuti", { pri_evento });
}

function eliri() {
  invoke("eliri");
}

// function restartigi() {
//   invoke("restartigi");
// }

function dormi(milisekundoj: number): Promise<void> {
  return new Promise((solvi) => setTimeout(solvi, milisekundoj));
}

async function instali() {
  await invoke("instali", { dosiera_vojo: dosiera_vojo.value });
}
</script>

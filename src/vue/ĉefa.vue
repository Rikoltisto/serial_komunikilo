<template>
  <div class="update-checker-container p-6">
    <el-dialog v-model="dialogo_mesaĝo_videblas" title="检测到新版本" width="480px" :close-on-click-modal="false"
      :close-on-press-escape="false" :show-close="false" class="rounded-lg shadow-2xl">
      <div class="space-y-3 pt-0">
        <div class="space-y-3 text-sm">
          <div class="flex items-center justify-between">
            <span class="text-gray-600 font-medium">当前版本:</span>
            <span
              class="inline-flex items-center px-3 py-1 text-xs font-semibold rounded-full bg-gray-200 text-gray-700">
              {{ ĝisdatiga_informo?.nuna_versio }}
            </span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-green-700 font-bold">最新版本:</span>
            <span
              class="inline-flex items-center px-3 py-1 text-sm font-bold rounded-full bg-green-100 text-green-700 border border-green-300">
              {{ ĝisdatiga_informo?.versio }}
            </span>
          </div>
          <div class="flex items-center justify-between border-b pb-3 border-gray-200">
            <span class="text-gray-600 font-medium">发布日期:</span>
            <span class="font-medium text-gray-700">
              {{ ĝisdatiga_informo?.dato }}
            </span>
          </div>
          <el-text tag="p" type="info"
            class="flex items-center justify-between !mt-4 p-2 rounded-lg bg-blue-50/70 border-l-4 border-blue-500">
            <span class="flex items-center space-x-2 text-blue-800">
              <el-icon :size="16">
                <Tickets />
              </el-icon>
              <span class="font-semibold">更新日志：</span>
            </span>
            <el-button type="primary" @click="ĝisdatiga_ĵurnala_videbleco = true" link
              class="font-bold hover:underline text-blue-600">
              点击查看详细变更
            </el-button>
          </el-text>
        </div>
      </div>
      <template #footer>
        <div class="flex justify-end space-x-3 pt-2">
          <el-button type="danger" plain @click="eliri" size="large" class="rounded-lg hover:shadow-md transition">
            退出应用
          </el-button>
          <el-button type="success" @click="elŝuti" size="large" class="rounded-lg hover:shadow-lg transition">
            立即更新
          </el-button>
        </div>
      </template>
    </el-dialog>
    <el-dialog v-model="ĝisdatiga_dialogo_videblas" title="正在下载新版本..." width="450px" :close-on-click-modal="false"
      :close-on-press-escape="false" :show-close="false" class="rounded-lg shadow-2xl">
      <div class="p-2 space-y-4">
        <el-text type="info" class="text-base block">
          版本变更：
          <span class="font-bold text-blue-600 ml-1">{{ ĝisdatiga_informo?.nuna_versio }} → {{ ĝisdatiga_informo?.versio
          }}</span>
        </el-text>
        <el-text type="info" class="text-sm block mb-2">
          下载进度：
        </el-text>
        <el-progress :percentage="procentaĵo" :stroke-width="20" :status="procentaĵo === 100 ? 'success' : undefined"
          striped striped-flow :duration="10" :text-inside="true" />
      </div>
    </el-dialog>
    <el-dialog v-model="ĝisdatiga_ĵurnala_videbleco" title="最新版本更新日志" width="500px" :close-on-click-modal="true"
      :close-on-press-escape="true" class="rounded-lg shadow-xl">
      <div class="p-1">
        <el-scrollbar max-height="350px" class="log-scrollbar">
          <el-timeline>
            <el-timeline-item timestamp="v2.1.0 (2025-10-01)" type="success" placement="top">
              <el-card>
                <h4>功能优化与新增</h4>
                <p>新增：支持深色模式切换。</p>
                <p>优化：提升数据加载速度 30%。</p>
              </el-card>
            </el-timeline-item>
          </el-timeline>
        </el-scrollbar>
      </div>
    </el-dialog>
  </div>
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
let ĝisdatiga_ĵurnala_videbleco = ref(false);

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
      await dormi(3000);
      instali();
      break;
  }
};

async function kontroli_ĝisdatigojn() {
  let rezulto = await invoke("kontroli_ĝisdatigojn");
  if (rezulto != null) {
    ĝisdatiga_informo.value = konverti_tempon(rezulto as VersiaInformo);
    dialogo_mesaĝo_videblas.value = true;
  }
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

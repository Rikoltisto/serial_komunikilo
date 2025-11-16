<template>
  <div class="h-screen w-full p-4 md:p-8 overflow-hidden">
    <div class="flex flex-col md:flex-row gap-6 w-full h-full max-w-7xl mx-auto">
      <el-card
        class="w-full md:w-80 lg:w-96 shadow-xl rounded-2xl h-full flex flex-col transition-all duration-300 hover:shadow-2xl"
        :body-style="{ padding: '24px', 'flex-grow': 1, 'display': 'flex', 'flex-direction': 'column' }">
        <template #header>
          <div class="flex items-center justify-between">
            <span class="text-xl font-extrabold text-indigo-800">串口设置</span>
            <el-icon :size="24" class="text-indigo-600">
              <Setting />
            </el-icon>
          </div>
        </template>
        <el-form :model="agordoj" label-position="top" class="flex flex-col flex-grow space-y-3">
          <el-form-item label="串口选择" class="mb-5">
            <div class="flex w-full gap-2">
              <el-select v-model="agordoj.elektita_pordo" placeholder="请选择串口" class="flex-grow rounded-lg"
                :disabled="ĉu_konektita">
                <el-option v-for="port in portList" :key="port.value" :label="port.label" :value="port.value" />
              </el-select>
              <el-button :icon="Refresh" circle @click="refreŝigi_pordojn" :disabled="ĉu_konektita"
                class="bg-blue-50 hover:bg-blue-100 border-blue-200 text-blue-600" />
            </div>
          </el-form-item>
          <el-form-item label="波特率" class="mb-5">
            <el-select v-model="agordoj.bodrapido" placeholder="请选择波特率" class="w-full" :disabled="ĉu_konektita">
              <el-option v-for="rate in baudRateOptions" :key="rate" :label="rate" :value="rate" />
            </el-select>
          </el-form-item>
          <div class="grid grid-cols-2 gap-4">
            <el-form-item label="数据位">
              <el-select v-model="agordoj.datumbitoj" class="w-full" :disabled="ĉu_konektita">
                <el-option label="8" :value="8" />
                <el-option label="7" :value="7" />
                <el-option label="6" :value="6" />
              </el-select>
            </el-form-item>
            <el-form-item label="停止位">
              <el-select v-model="agordoj.haltobitoj" class="w-full" :disabled="ĉu_konektita">
                <el-option label="1" :value="1" />
                <el-option label="1.5" :value="1.5" />
                <el-option label="2" :value="2" />
              </el-select>
            </el-form-item>
          </div>
          <el-form-item label="校验位" class="mb-auto">
            <el-select v-model="agordoj.pareco" class="w-full" :disabled="ĉu_konektita">
              <el-option label="None" value="none" />
              <el-option label="Even" value="even" />
              <el-option label="Odd" value="odd" />
            </el-select>
          </el-form-item>
          <el-form-item class="mt-8 pt-4 border-t border-gray-200">
            <el-button :type="ĉu_konektita ? 'danger' : 'success'" @click="ŝalti_konekton"
              class="w-full text-base font-bold transition-transform transform hover:scale-[1.01]"
              :icon="ĉu_konektita ? CircleCloseFilled : CircleCheckFilled" size="large">
              {{ ĉu_konektita ? '断开连接' : '打开串口' }}
            </el-button>
          </el-form-item>
        </el-form>
      </el-card>
      <div class="flex-grow flex flex-col gap-6 h-full min-h-[500px] md:min-h-0">
        <el-card class="flex-grow flex flex-col shadow-xl rounded-2xl transition-all duration-300 hover:shadow-2xl"
          :body-style="{ padding: '24px', display: 'flex', flexDirection: 'column', height: '100%' }">
          <template #header>
            <div class="flex items-center justify-between">
              <span class="text-xl font-extrabold text-indigo-800">数据接收</span>
              <div class="flex items-center gap-4">
                <el-switch v-model="kontroliloj.deksesume_ricevi" active-text="HEX接收" inactive-text="ASCII"
                  size="large" />
                <el-switch v-model="kontroliloj.aŭtomata_rulumado" active-text="自动滚动" size="large" />
                <el-button type="danger" :icon="Delete" @click="malplenigi_ricevitaĵon" plain round
                  class="font-medium hover:bg-red-50">清空接收</el-button>
              </div>
            </div>
          </template>
          <el-input v-model="ricevitaj_datumoj" type="textarea" :rows="15" readonly placeholder="等待接收数据..."
            class="flex-grow receive-textarea-custom" ref="receiveTextarea" />
        </el-card>
        <el-card class="shadow-xl rounded-2xl transition-all duration-300 hover:shadow-2xl"
          :body-style="{ padding: '24px' }">
          <template #header>
            <div class="flex items-center justify-between">
              <span class="text-xl font-extrabold text-indigo-800">数据发送</span>
              <div class="flex items-center gap-4">
                <el-switch v-model="kontroliloj.deksesume_sendi" active-text="HEX发送" inactive-text="ASCII"
                  size="large" />
                <el-switch v-model="kontroliloj.aŭtomate_sendi" active-text="定时发送" @change="ŝalti_aŭtomaton_sendi"
                  size="large" />
                <div class="flex items-center">
                  <el-input-number v-model="kontroliloj.send_intervalo" :min="100" :step="100" controls-position="right"
                    class="w-32" :disabled="!kontroliloj.aŭtomate_sendi" />
                  <span class="text-sm text-gray-500 ml-2">ms</span>
                </div>
              </div>
            </div>
          </template>
          <div class="flex gap-4">
            <el-input v-model="sendi_datumojn" placeholder="请输入要发送的数据..." class="flex-grow" clearable size="large" />
            <el-button type="primary" :icon="Position" @click="sendiSeridatumojn" :disabled="!ĉu_konektita"
              class="font-medium transition-transform transform hover:scale-[1.05]" size="large">
              发送
            </el-button>
          </div>
        </el-card>
      </div>
    </div>
    <div class="update-checker-container p-6">
      <el-dialog v-model="dialogo_mesaĝo_videblas" title="检测到新版本" width="480px" :close-on-click-modal="false"
        :close-on-press-escape="false" :show-close="false" class="rounded-xl shadow-3xl">
        <div class="space-y-4 pt-0">
          <div class="space-y-3 text-sm">
            <div class="flex items-center justify-between p-2 bg-gray-50 rounded-lg">
              <span class="text-gray-600 font-medium">当前版本:</span>
              <span
                class="inline-flex items-center px-4 py-1 text-xs font-semibold rounded-full bg-gray-200 text-gray-700 border border-gray-300">
                {{ ĝisdatiga_informo?.nuna_versio }}
              </span>
            </div>
            <div class="flex items-center justify-between p-2 bg-green-50 rounded-lg border border-green-200">
              <span class="text-green-700 font-bold">最新版本:</span>
              <span
                class="inline-flex items-center px-4 py-1 text-sm font-bold rounded-full bg-green-200 text-green-800">
                {{ ĝisdatiga_informo?.versio }}
              </span>
            </div>
            <div class="flex items-center justify-between border-b pb-3 border-gray-200 mt-4">
              <span class="text-gray-600 font-medium">发布日期:</span>
              <span class="font-medium text-gray-700">
                {{ ĝisdatiga_informo?.dato }}
              </span>
            </div>
            <div
              class="flex items-center justify-between mt-4 p-3 rounded-xl bg-blue-50/70 border-l-4 border-blue-500 transition-all hover:shadow-sm">
              <span class="flex items-center space-x-2 text-blue-800">
                <el-icon :size="18" class="flex-shrink-0">
                  <Ticket />
                </el-icon>
                <span class="font-semibold tracking-wide">更新日志：</span>
              </span>
              <el-button type="primary" link @click="ĝisdatiga_ĵurnala_videbleco = true"
                class="!font-bold hover:!underline !text-blue-600">
                点击查看详细变更
              </el-button>
            </div>
          </div>
        </div>
        <template #footer>
          <div class="flex justify-end space-x-3 pt-2">
            <el-button type="danger" plain @click="eliri" size="large" class="rounded-xl hover:shadow-md transition">
              退出应用
            </el-button>
            <el-button type="success" @click="elŝuti" size="large"
              class="rounded-xl font-bold hover:shadow-lg transition">
              立即更新
            </el-button>
          </div>
        </template>
      </el-dialog>
      <el-dialog v-model="ĝisdatiga_dialogo_videblas" title="正在下载新版本..." width="450px" :close-on-click-modal="false"
        :close-on-press-escape="false" :show-close="false" class="rounded-xl shadow-2xl">
        <div class="p-2 space-y-4">
          <el-text type="info" class="text-base block">
            版本变更：
            <span class="font-bold text-indigo-600 ml-1">{{ ĝisdatiga_informo?.nuna_versio }} → {{
              ĝisdatiga_informo?.versio
              }}</span>
          </el-text>
          <br />
          <el-text type="info" class="text-sm block mb-2">
            下载进度：
          </el-text>
          <el-progress :percentage="procentaĵo" :stroke-width="20" :status="procentaĵo === 100 ? 'success' : undefined"
            striped striped-flow :duration="10" :text-inside="true" />
        </div>
      </el-dialog>
      <el-dialog v-model="ĝisdatiga_ĵurnala_videbleco" :title="'版本更新日志: ' + ĝisdatiga_informo?.versio" width="500px"
        :close-on-click-modal="true" :close-on-press-escape="true" class="rounded-xl shadow-xl">
        <div class="p-1">
          <el-scrollbar max-height="350px" class="log-scrollbar">
            <el-timeline>
              <el-timeline-item :timestamp="ĝisdatiga_informo?.versio + ' (' + ĝisdatiga_informo?.dato + ')'"
                type="success" placement="top">
                <el-card class="shadow-sm">
                  <h4>功能优化与新增</h4>
                  <p>新增：支持深色模式切换。</p>
                  <p>优化：提升数据加载速度 30%。</p>
                  <p>修复：修正了定时发送逻辑中的一个小错误。</p>
                </el-card>
              </el-timeline-item>
              <el-timeline-item timestamp="其他版本日志" type="info" placement="top">
                <el-card class="shadow-sm">
                  <p>如需查看历史版本更新日志，请访问官方网站。</p>
                </el-card>
              </el-timeline-item>
            </el-timeline>
          </el-scrollbar>
        </div>
      </el-dialog>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Channel, invoke } from "@tauri-apps/api/core";
import { onMounted, reactive, ref } from "vue";
import { Refresh, Delete, Position, Setting, CircleCloseFilled, CircleCheckFilled, Ticket } from "@element-plus/icons-vue"

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
//Ĝisdatigi.
let dialogo_mesaĝo_videblas = ref(false);
let ĝisdatiga_dialogo_videblas = ref(false);
let ĝisdatiga_informo = ref<VersiaInformo>();
let pri_evento = new Channel<ElŝutaEvento>();
let enhava_longo = ref<number>(0);
let procentaĵo = ref<number>();
let dosiera_vojo = ref<string>("");
let ĝisdatiga_ĵurnala_videbleco = ref(false);
//Ĉefa Interfacо.
let agordoj = reactive({
  elektita_pordo: '',
  bodrapido: 115200,
  datumbitoj: 8,
  haltobitoj: 1,
  pareco: 'none',
});

const portList = ref([
  { label: 'COM1 - USB-SERIAL CH340', value: 'COM1' },
  { label: 'COM3 - 蓝牙串口', value: 'COM3' },
  { label: 'COM5', value: 'COM5' },
]);

const baudRateOptions = ref([
  9600, 19200, 38400, 57600, 115200, 921600
]);

const kontroliloj = reactive({
  deksesume_ricevi: false,
  aŭtomata_rulumado: true,
  deksesume_sendi: false,
  aŭtomate_sendi: false,
  send_intervalo: 1000,
});

let ĉu_konektita = ref(false);
let ricevitaj_datumoj = ref('');
let sendi_datumojn = ref('');

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

async function refreŝigi_pordojn() {

}

async function ŝalti_konekton() {

}

async function malplenigi_ricevitaĵon() {

}

async function ŝalti_aŭtomaton_sendi() {

}

async function sendiSeridatumojn() {

}
</script>

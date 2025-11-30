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
        <el-form :model="settings" label-position="top" class="flex flex-col flex-grow space-y-3">
          <el-form-item label="串口选择" class="mb-5">
            <div class="flex w-full gap-2">
              <el-select v-model="settings.selected_serial_port" placeholder="请选择串口" class="flex-grow rounded-lg"
                :disabled="is_connected">
                <el-option v-for="port in port_list" :label="port" :value="port" />
              </el-select>
              <el-button :icon="Refresh" circle @click="refresh_serial_ports" :disabled="is_connected"
                class="bg-blue-50 hover:bg-blue-100 border-blue-200 text-blue-600" />
            </div>
          </el-form-item>
          <el-form-item label="波特率" class="mb-5">
            <el-select v-model="settings.baud_rate" placeholder="请选择波特率" class="w-full" :disabled="is_connected">
              <el-option v-for="rate in baud_rate_options" :key="rate" :label="rate" :value="rate" />
            </el-select>
          </el-form-item>
          <div class="grid grid-cols-2 gap-4">
            <el-form-item label="数据位">
              <el-select v-model="settings.data_bits" class="w-full" :disabled="is_connected">
                <el-option label="8" :value="8" />
                <el-option label="7" :value="7" />
                <el-option label="6" :value="6" />
              </el-select>
            </el-form-item>
            <el-form-item label="停止位">
              <el-select v-model="settings.stop_bits" class="w-full" :disabled="is_connected">
                <el-option label="1" :value="1" />
                <el-option label="1.5" :value="1.5" />
                <el-option label="2" :value="2" />
              </el-select>
            </el-form-item>
          </div>
          <el-form-item label="校验位" class="mb-auto">
            <el-select v-model="settings.pareco" class="w-full" :disabled="is_connected">
              <el-option label="None" value="none" />
              <el-option label="Even" value="even" />
              <el-option label="Odd" value="odd" />
            </el-select>
          </el-form-item>
          <el-form-item class="mt-8 pt-4 border-t border-gray-200">
            <el-button :type="is_connected ? 'danger' : 'success'" @click="trigger_connection"
              class="w-full text-base font-bold transition-transform transform hover:scale-[1.01]"
              :icon="is_connected ? CircleCloseFilled : CircleCheckFilled" size="large">
              {{ is_connected ? '断开连接' : '打开串口' }}
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
                <el-switch v-model="controller.receive_hex" active-text="HEX接收" inactive-text="ASCII" size="large" />
                <el-switch v-model="controller.auto_scroll" active-text="自动滚动" size="large" />
                <el-button type="danger" :icon="Delete" @click="clear_receive_buffer" plain round
                  class="font-medium hover:bg-red-50">清空接收</el-button>
              </div>
            </div>
          </template>
          <el-input v-model="received_data" type="textarea" :rows="15" readonly placeholder="等待接收数据..."
            class="flex-grow receive-textarea-custom" ref="receiveTextarea" />
        </el-card>
        <el-card class="shadow-xl rounded-2xl transition-all duration-300 hover:shadow-2xl"
          :body-style="{ padding: '24px' }">
          <template #header>
            <div class="flex items-center justify-between">
              <span class="text-xl font-extrabold text-indigo-800">数据发送</span>
              <div class="flex items-center gap-4">
                <el-switch v-model="controller.send_hex" active-text="HEX发送" inactive-text="ASCII" size="large" />
                <el-switch v-model="controller.auto_send" active-text="定时发送" @change="enable_auto_send" size="large" />
                <div class="flex items-center">
                  <el-input-number v-model="controller.send_interval" :min="100" :step="100" controls-position="right"
                    class="w-32" :disabled="!controller.auto_send" />
                  <span class="text-sm text-gray-500 ml-2">ms</span>
                </div>
              </div>
            </div>
          </template>
          <div class="flex gap-4">
            <el-input v-model="send_data" placeholder="请输入要发送的数据..." class="flex-grow" clearable size="large" />
            <el-button type="primary" :icon="Position" @click="send_serial_data" :disabled="!is_connected"
              class="font-medium transition-transform transform hover:scale-[1.05]" size="large">
              发送
            </el-button>
          </div>
        </el-card>
      </div>
    </div>
    <div class="update-checker-container p-6">
      <el-dialog v-model="update_message_visibility" title="检测到新版本" width="480px" :close-on-click-modal="false"
        :close-on-press-escape="false" :show-close="false" class="rounded-xl shadow-3xl">
        <div class="space-y-4 pt-0">
          <div class="space-y-3 text-sm">
            <div class="flex items-center justify-between p-2 bg-gray-50 rounded-lg">
              <span class="text-gray-600 font-medium">当前版本:</span>
              <span
                class="inline-flex items-center px-4 py-1 text-xs font-semibold rounded-full bg-gray-200 text-gray-700 border border-gray-300">
                {{ update_information?.current_version }}
              </span>
            </div>
            <div class="flex items-center justify-between p-2 bg-green-50 rounded-lg border border-green-200">
              <span class="text-green-700 font-bold">最新版本:</span>
              <span
                class="inline-flex items-center px-4 py-1 text-sm font-bold rounded-full bg-green-200 text-green-800">
                {{ update_information?.version }}
              </span>
            </div>
            <div class="flex items-center justify-between border-b pb-3 border-gray-200 mt-4">
              <span class="text-gray-600 font-medium">发布日期:</span>
              <span class="font-medium text-gray-700">
                {{ update_information?.date }}
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
              <el-button type="primary" link @click="update_log_visibility = true"
                class="!font-bold hover:!underline !text-blue-600">
                点击查看详细变更
              </el-button>
            </div>
          </div>
        </div>
        <template #footer>
          <div class="flex justify-end space-x-3 pt-2">
            <el-button type="danger" plain @click="exit" size="large" class="rounded-xl hover:shadow-md transition">
              退出应用
            </el-button>
            <el-button type="success" @click="download" size="large"
              class="rounded-xl font-bold hover:shadow-lg transition">
              立即更新
            </el-button>
          </div>
        </template>
      </el-dialog>
      <el-dialog v-model="update_download_message_visibility" title="正在下载新版本..." width="450px"
        :close-on-click-modal="false" :close-on-press-escape="false" :show-close="false" class="rounded-xl shadow-2xl">
        <div class="p-2 space-y-4">
          <el-text type="info" class="text-base block">
            版本变更：
            <span class="font-bold text-indigo-600 ml-1">{{ update_information?.current_version }} → {{
              update_information?.version
              }}</span>
          </el-text>
          <el-text type="info" class="text-sm block mb-2">
            下载进度：
          </el-text>
          <el-progress :percentage="percent" :stroke-width="20" :status="percent === 100 ? 'success' : undefined"
            striped striped-flow :duration="10" :text-inside="true" />
        </div>
      </el-dialog>
      <el-dialog v-model="update_log_visibility" :title="'版本更新日志: ' + update_information?.version" width="500px"
        :close-on-click-modal="true" :close-on-press-escape="true" class="rounded-xl shadow-xl">
        <div class="p-1">
          <el-scrollbar max-height="350px" class="log-scrollbar">
            <el-timeline class="pl-1">
              <el-timeline-item :timestamp="update_information?.version + ' (' + update_information?.date + ')'"
                type="success" placement="top">
                <el-card class="shadow-sm">
                  <div v-html="get_rendered_content()" class="markdown-body"></div>
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
import MarkdownIt from "markdown-it";
import "github-markdown-css/github-markdown.css";
import { ElNotification } from "element-plus";
import 'element-plus/es/components/notification/style/css'

interface UpdateMetadata {
  version: String;
  current_version: String;
  note: String;
  date: String;
}

type DownloadEvent =
  | {
    event: "Started";
    data: {
      content_length: number;
    };
  }
  | {
    event: "Progress";
    data: {
      downloaded: number;
    };
  }
  | {
    event: "Finished";
    data: null;
  };
//Update Interface.
let update_message_visibility = ref(false);
let update_download_message_visibility = ref(false);
let update_information = ref<UpdateMetadata>({
  version: "",
  current_version: "",
  note: "",
  date: "",
});
let on_events = new Channel<DownloadEvent>();
let content_length = ref<number>(0);
let percent = ref<number>();
let file_path = ref<string>("");
let update_log_visibility = ref(false);
//Create a Markdown parser.
let md = new MarkdownIt();


//Main Interface.
let settings = reactive({
  selected_serial_port: '',
  baud_rate: 115200,
  data_bits: 8,
  stop_bits: 1,
  pareco: 'none',
});

let port_list = ref<String[]>();

const baud_rate_options = ref([
  9600, 19200, 38400, 57600, 115200, 921600
]);

const controller = reactive({
  receive_hex: false,
  auto_scroll: true,
  send_hex: false,
  auto_send: false,
  send_interval: 1000,
});

let is_connected = ref(false);
let received_data = ref('');
let send_data = ref('');

onMounted(async () => {
  check_for_updates();
  refresh_serial_ports();
});

on_events.onmessage = async (message) => {
  switch (message.event) {
    case "Started":
      update_message_visibility.value = false;
      update_download_message_visibility.value = true;
      content_length.value = message.data.content_length;
      break;
    case "Progress":
      percent.value = Math.min(
        100,
        Number(
          ((message.data.downloaded / content_length.value) * 100).toFixed(1),
        ),
      );
      break;
    case "Finished":
      await sleep(3000);
      install();
      break;
  }
};

async function check_for_updates() {
  let result = await invoke("check_for_updates");
  if (result != null) {
    update_information.value = format_time(result as UpdateMetadata);
    update_message_visibility.value = true;
  }
}

function format_time(update_information: UpdateMetadata): UpdateMetadata {
  //Split by space.
  const date = update_information.date.split(" ");
  const [year, month, day] = date[0].split("-");

  //Convert the time format.
  update_information.date = `${month}月${day}日${year}年`;

  return update_information;
}

async function download() {
  file_path.value = await invoke("download", { on_events });
}

function exit() {
  invoke("exit");
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function install() {
  await invoke("install", { file_path: file_path.value });
}

function get_rendered_content() {
  return md.render(String(update_information.value.note));
}
//Main Interface.
async function refresh_serial_ports() {
  let result = await invoke<String[]>("get_all_serial_port");

  port_list.value = result;

  ElNotification({
    title: '成功',
    message: '串口刷新成功',
    type: 'success',
    duration: 1000,
  })
}

async function trigger_connection() {

}

async function clear_receive_buffer() {

}

async function enable_auto_send() {

}

async function send_serial_data() {

}
</script>

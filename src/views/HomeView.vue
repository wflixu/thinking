<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import pkg from './../../package.json'
import { invoke } from '@tauri-apps/api';
import { register } from '@tauri-apps/api/globalShortcut';
import { appWindow } from '@tauri-apps/api/window';
import { emit, listen } from '@tauri-apps/api/event';

const version = computed(() => {
  return pkg.version
})

const count = ref(0)

const onDecrease = () => {
  count.value -= 1;
}
const onIncrease = () => {
  count.value += 1;
}

const onStart = async () => {

  await register('Ctrl+A', async () => {
    console.log('Shortcut triggered');
    await emit('frontend-loaded', { loggedIn: true, token: 'authToken' });
    await appWindow.show()
  });
}

onMounted(async () => {
  const unlisten = await listen<string>('frontend-loaded', (event) => {
    console.log(`Got error in window ${event.windowLabel}, payload: ${event.payload}`);
     
  });

  return () => {
    unlisten()
  };

})
</script>

<template>
  <main>
    <h1>Thinking</h1>
    <h3>版本：{{ version }}</h3>
    <div>
      <h2> counter</h2>
      <button @click="onIncrease"> 加 + </button><span>{{ count }}</span><button @click="onDecrease"> 减 -</button>
    </div>
    <div>
      <button @click="onStart">start</button>
    </div>
  </main>
</template>

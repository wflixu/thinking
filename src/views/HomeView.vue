<script setup lang="ts">
import { computed, ref } from 'vue';
import pkg from './../../package.json'
const version = computed(()=>{
  return pkg.version
})

const count = ref(0)

const onDecrease = () =>{
  count.value -= 1;
}
const onIncrease = () =>{
  count.value +=1;
}
</script>

<template>
  <main>
    <h1>Thinking</h1>
    <h3>版本：{{version}}</h3>
    <div>
      <h2> counter</h2>
      <button @click="onIncrease" > 加 + </button><span>{{count}}</span><button @click="onDecrease"> 减 -</button>
    </div>
  </main>
</template>

<script setup >
import { ref, onMounted } from 'vue';
import { mxGraph } from 'thgraph';



onMounted(() => {
  const container = document.getElementById('container');
  // console.log(container);
  main(container);
});

function main(container) {
  // var obj = new Object();
  // obj.foo = "Foo";
  // obj.bar = "Bar";
  //  var enc = new mxCodec();
  //  var node = enc.encode(obj);
  //  console.log('#**@'.repeat(10));
  //  console.log(node);

  // Creates the graph inside the given container
  var graph = new mxGraph(container);

  // Enables rubberband selection
  // new mxRubberband(graph);

  // Gets the default parent for inserting new cells. This
  // is normally the first child of the root (ie. layer 0).
  var parent = graph.getDefaultParent();

  // Adds cells to the model in a single step
  graph.getModel().beginUpdate();
  try {
    var v1 = graph.insertVertex(parent, null, 'Hello,', 20, 20, 80, 30);
    // var v2 = graph.insertVertex(parent, null, 'World!', 200, 150, 80, 30);
    // var e1 = graph.insertEdge(parent, null, '', v1, v2);
  } finally {
    // Updates the display
    graph.getModel().endUpdate();
  }

  console.log(graph, graph.getModel());
}

</script>

<template>
  <main class="home">
    <div class="top px-4">menubar</div>
    <div class="toolbar px-4">tools</div>
    <div class="sidebar">sidebar</div>
    <div class="content">
      <div id="container"></div>
    </div>
    <div class="format"></div>
  </main>
</template>
<style lang="less" scoped>
.home {
  min-height: 100vh;
  background: #eee;
  display: grid;
  grid-template-columns: 240px 1fr 240px;
  grid-template-rows: 32px 40px 1fr;
  .top {
    height: 32px;
    background: #ccc;
    display: flex;
    align-items: center;
    grid-column-start: 1;
    grid-column-end: 4;
    grid-row-start: 1;
    grid-row-end: 2;
  }
  .toolbar {
    grid-column-start: 1;
    grid-column-end: 4;
    grid-row-start: 2;
    grid-row-end: 3;
    background: #ddd;
    display: flex;
  }
  .sidebar {
    grid-column-start: 1;
    grid-column-end: 2;
    grid-row-start: 3;
    grid-row-end: 4;
    background: white;
  }
  .content {
    grid-column-start: 2;
    grid-column-end: 3;
    grid-row-start: 3;
    grid-row-end: 4;
    background: #f1f1f1;
    
  }
}

#container {
  height: 100%;
}
</style>

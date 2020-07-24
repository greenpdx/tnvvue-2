<template>
  <div class="tree-view">
    <!-- div v-if="top" class="tv-node">
      <span>Total</span><span> {{ top.showVal() }} </span><br>
      <input
        type="range"
        min="-10000000"
        max="10000000"
        v-bind:value="difVal">
      <span>{{ difVal }}</span>
    </div -->
    <div class="tnvtree">
      <div v-for="(node, idx) in nodes" class="tv-node" :key="node.idx">
        <div class="contain">
          <div class="indent" @click="onExpand($event, node)">
            <span v-show="node.expand">&#9660;&nbsp;&nbsp;</span>
            <span v-show="!node.expand">&#9658;&nbsp;&nbsp;</span>
          </div>

          <tree-view-node
            class="tvclass"
            :nodeIdx="node.idx"
            :pos="idx">
            <!--v-on:chgParent="top.chgParent" -->
            <!-- slider-node v-show="selected":node="node"></slider-node -->
          </tree-view-node>
        </div>
        <div v-if="node.expand">
          <div v-for="(bnode, bidx) in child(node.idx)" :key="bnode.idx" class="l2">
            <div class="contain">
              <div class="indent" @click="onExpand($event, bnode)">
                <span v-show="bnode.expand">&#9660;&nbsp;&nbsp;</span>
                <span v-show="!bnode.expand">&#9658;&nbsp;&nbsp;</span>
              </div>
              <tree-view-node
                :nodeIdx="bnode.idx"
                :pos="bidx">
                <!--v-on:chgParent="top.chgParent" -->
                <!-- slider-node v-show="selected":node="node"></slider-node -->
              </tree-view-node>
            </div>
            <div v-if="bnode.expand">
              <div v-for="(cnode, cidx) in child(bnode.idx)" :key="cnode.idx" class="l2">
                <div class="contain">
                  <tree-view-node
                    :nodeIdx="cnode.idx"
                    :pos="cidx">
                    <!--v-on:chgParent="top.chgParent" -->
                    <!-- slider-node v-show="selected":node="node"></slider-node -->
                  </tree-view-node>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'

// import Node from '@/lib/Node'
// import SliderNode from './SliderNode'
import TreeViewNode from './TreeViewNode'

export default {
  name: 'TreeView',
  components: {
    TreeViewNode,
  //  SliderNode
  },

  props: {
    raw: {
//      type: Node
    }
  },

  data () {
    return {

      //nodes: [],
      //node: null,
      total: 0,
      difVal: 0
    }
  },

  beforeCreate () {
  },

  reducer (val, node, idx, tre) {
    if (node.chld.lengh == 0) {
      return node.val
    }
    let sum = node.chld.reduce(this.reduce)
    return sum
  },

  created () {
    console.log("TV",this.tree[0].chld)
    //this.nodes = this.top.children
    //this.total = this.top.total
    //this.top.tree = this
    //this.top.lockVal = 1
    //console.log('tree', this.nodes.length)
  },

  methods: {
    // return all the children nodes
    child(nidx) {
       let ns = this.tree[nidx].chld.map(i => this.tree[i])
       return ns.sort((a,b) => { return a.val - b.val})
    },
    onExpand(evt, node) {

      console.log(evt, node)
      if (node.expand) {
        node.expand = false
        //this.setExpand(this.node)
      } else {
        node.expand = true
        //this.setExpand(this.node)
      }
      this.$forceUpdate()
    }

  },

  updated () {
    //this.nodes = this.tree
  },

  computed: {
    ...mapGetters({
    }),
    tree () {
      return this.$root.tree
    },
    accts () {
      return this.$root.accts
    },
    nodes () {
      return this.$root.tree[0].chld.map(i => this.tree[i])
    },
  }
}
</script>

<style scoped>
.tree-view {
  display: inline-block;
  width: 100%;
  height: 100%;
}
.tv-node {
  display: block;
  float: left;
  width: 45em;
}
.tnvtree {
  display: inline-block;
  height: 100%;
  overflow-y: scroll;
}
.top-slide {
  float: left;
}
.l2 {
  margin-left: .5em;
}
.indent {
  flex: 1;
  text-align: left;
  cursor: zoom-in;
  width: 3em;
  padding-right: 1em;
}
.tvclass {
  float: right
}
.contain {
  display: flex;
  height: 1em;
  display-outside: block;
}
</style>

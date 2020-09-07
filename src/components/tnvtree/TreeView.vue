<template>
  <div class="tree-view">
    <div class="tv-node">
      <!-- span>Total</span><span> {{ top.showVal() }} </span><br>
      <input
        type="range"
        min="-10000000"
        max="10000000"
        v-bind:value="difVal">
      <span>{{ difVal }}</span -->
      <button @click="btnClk">Collapse</button> {{ sum }}
    </div>
    <div class="tnvtree">
      <div v-for="(node, idx) in nodes" class="tv-node" :key="node.idx">
        <div class="contain">
          <div class="indent" @click="onExpand($event, node)">
            <span v-show="node.expand">&#9660;</span>
            <span v-show="!node.expand">&#9658;</span>
          </div>

          <tree-view-node
            class="tvclass"
            @TreeNodeSel="nodeSel"
            :gSel="gSel"
            :gExp="gExp"
            :nodeIdx="node.idx"
            :pos="idx"
            :sum="sum">
            <!--v-on:chgParent="top.chgParent" -->
            <!-- slider-node v-show="selected":node="node"></slider-node -->
          </tree-view-node>
        </div>
        <div v-show="node.expand">
          <!--div v-for="(bnode, bidx) in child(node.chld)" :key="bnode.idx" class="l2"-->
          <div v-for="(bnode, bidx) in node.chld" :key="bnode.idx" class="l2">
            <div class="contain">
              <div class="indent" @click="onExpand($event, bnode)">
                <span v-show="bnode.expand">&#9660;</span>
                <span v-show="!bnode.expand">&#9658;</span>
              </div>
              <tree-view-node
                @TreeNodeSel="nodeSel"
                :gSel="gSel"
                :gExp="gExp"
                :nodeIdx="bnode.idx"
                :pos="bidx"
                :sum="sum">
                <!--v-on:chgParent="top.chgParent" -->
                <!-- slider-node v-show="selected":node="node"></slider-node -->
              </tree-view-node>
            </div>
            <div v-show="bnode.expand">
              <!--div v-for="(cnode, cidx) in child(bnode.chld)" :key="cnode.idx" class="l2"-->
              <div v-for="(cnode, cidx) in bnode.chld" :key="cnode.idx" class="l2">
                <div class="contain">
                  <div class="indent leaf"></div>
                  <tree-view-node
                    @TreeNodeSel="nodeSel"
                    :gSel="gSel"
                    :gExp="gExp"
                    :nodeIdx="cnode.idx"
                    :pos="cidx"
                    :sum="sum">
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
      difVal: 0,
      gSel: -1,
      gExp: -1,
      dbgcnt: 4,
      sum: 0,
      once: true,
      top: null
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
    //console.log('TVTV')
    let node = this.getNode(0)
    let ss = node.chld.map(i => this.getNode(i))
    let sum = 0
    this.top = node
    for (let a of ss) {
      let asum = 0
      let bb = a.chld.map(i => this.getNode(i))
      for (let b of bb) {
        let cc = b.chld.map(i => this.getNode(i))
        let bsum= 0
        for (let c of cc) {
          bsum += c.val
        }
        b.chld = cc.sort((a,b) => { return b.val - a.val})
        //console.log("ONCE", b.chld)
        //for (let i in cc) {
        //  console.log(cc[i].val)
        //}
        b.val = bsum
        asum += bsum
            //console.log(".")
      }
      a.chld = bb.sort((a,b)=>{ return b.val - a.val})
      a.val = asum
      sum += asum
    }
    node.val = sum
    this.sum = sum
    console.log('SUM', sum)
        //this.sum = sum
    node.chld = ss.sort((a,b)=>{ return b.val - a.val})
    //console.log("TV",this.getNode(0).chld)
    //this.nodes = this.top.children
    //this.total = this.top.total
    //this.top.tree = this
    //this.top.lockVal = 1
    //console.log('tree', this.nodes.length)
  },

  methods: {
    colapse (base) {
      let chld = this.top.chld
      for (let node of chld) {
        node.expand = false
        for (let chd of node.chld ) {
          chd.expand = false
        }
      }
    },
    sortVals (a, b, zot) {
      let node = this.getNode(0)
      let sum = 0
      for (let a of node.chld) {
        let asum = 0
        for (let b of a.chld) {
          let bsum= 0
          for (let c of b.chld) {
            bsum += c.val
          }
          b.chld.sort((a,b) => { return b.val - a.val})
          b.val = bsum
          asum += bsum
        }
        a.chld.sort((a,b)=>{ return b.val - a.val})
        a.val = asum
        sum += asum
      }
      node.val = sum
      node.chld.sort((a,b)=>{ return b.val - a.val})
    },
    btnClk () {
      console.log('BTNCLK')
      let zot = this.nodes[3]
      this.colapse(this.top)
    },

    nodeSel (idx, val) {
      if (val === false ) {
        this.selected = -1
      } else {
        this.selected = idx
      }
      console.log('NS',idx,val)
    },
    // return all the children nodes
    child(chld) {
      //let node = this.getNode(nidx)

      //let ns = node.chld
      chld.sort((a,b) => { return b.val - a.val})
      if (this.dbgcnt > 0 ) {
        console.log('CHLD',chld)
        this.dbgcnt -= 1
      }
      return chld
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
    },
    addsum () {

    }
},

  updated () {
    //this.nodes = this.tree
  },

  computed: {
    ...mapGetters([
      'getNodes',
      'getNode',
      'setNode'
    ]),
    tree () {
      return this.getNodes
    },
    accts () {
      return this.$root.accts
    },
    nodes () {
      console.log("NODES base")
      let node = this.getNode(0)
      let ss = node.chld //.slice(10,11)
      // console.log(ss)
      let zot = []
      ss.sort((a,b) => { return b.val -a.val })
      //console.log('TVnodes',ns)
      return ss
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
.leaf {
  margin-right: 1em;
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

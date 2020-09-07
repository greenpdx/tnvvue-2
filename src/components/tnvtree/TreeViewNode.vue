<template>
  <div class="tree-view-node">
    <div class="tnv-node">
      <drag @dragstart="ds" @dragend="ds" :data="node">
        <div class="tnv-line" @click="selClick($event, node.idx)" @contextmenu.prevent="selClick($event, node.idx)">
          <span class="tnv-amount"> {{ nodeval() }}</span>
          <span>{{ percent() }} </span> 
          <span class="tnv-name"> {{ name }} </span>
          <span class="tnv-idx"> {{ prtkey() }} </span>
        </div>
        </drag>
        <slider-node
          v-if="node.select"
          :node="node"></slider-node>
      </div>
  </div>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
import Node from '@/lib/Node'
import { Drag, Drop, DropMask } from "vue-easy-dnd"

import SliderNode from './SliderNode'

let tmpvar = {}

export default {
  name: 'TreeViewNode',

  components: {
    SliderNode,
    Drag
  },

  props: {
    nodeIdx: Number,
    level: {
      default: 0
    },
    gSel: Number,
    gExp: Number,
    sum: Number
  },

  data () {
    return {
      node: null,
      total: 0,
      locked: false,
      change: false,
      default: 0,
      children: [],
      value: 0,

    }
  },

  created () {
    let node = this.getNode(this.nodeIdx)
    //console.log('TVNc',this.sum, this.nodeIdx, node)
    //let node = Object.assign({}, this.$root.tree[this.nodeIdx])
    node['select'] = false
    node['expand'] = false
    node['hover'] = false
    this.default = 0
    this.value = 0
    if (node.leaf != -1) {
      let leaf = this.$root.accts[node.leaf]
      let val = leaf.value[43]/1000
      //console.log(val, leaf)
      this.default = val
      this.value = val
    } 
    //node['default'] = this.value
    //node['nodeval'] = this.nodeval
    node['self'] = this
    this.node = node
    this.setNode(node)
    tmpvar = node
    //this.node['showVal'] = this.showVal

    //this.node['showVal'] = this.showVal
    //this.total = this.node.total
    //this.children = this.node.chld
    //console.log('TVNcr',this)
  },

  mounted () {
  
    this.change = true
  },
  updated () {
    //this.dbgPrt('tnodeUpdate')
  },

  methods: {
    ...mapActions([
      'setSelect',
      'setHover',
      'setExpand'
    ]),
    ds (evt) {
      //evt.data = this.node
      //evt.type = "text"
      console.log('DS', this.nodeIdx, evt)
    },
    percent () {
      return  (100 * this.nodeval() / this.sum).toFixed(4)
    },
    prtkey () {
      let node = this.node
      let key = node.key
      let str = ""
      if (key.ccode != -1) {
        str = ',' + key.ccode.toString()
      }
      if (key.bcode != -1) {
        str = ',' + key.bcode.toString() + str
      }
      str = key.acode.toString() + str
      return str
    },
    nodeval () {
      //console.log(node)
      //let node = this.node
      let sum = 0
      //if (node.leaf === -1) {
      //  for (let c of node.chld) {
          //let n = this.getNode(c)
          //let v = this.nodeval(n)
          //console.log(c, n, v)
      //    sum += c.val
      //  }
      //} else {
        sum = this.node.val
        //console.log('NVN', node.idx,  sum)
      //}
      return sum
    },

    /*showVal() {
      let sum = 0
      let chld =  this.node.chld
      if (chld.length > 0) {
        for (c in chld) {
          sum += this.showVal
        }
      } else 
        return this.value
      return sum
    },*/
    selClick (evt, idx) {
      console.log(idx, evt, this.node.key)
      let node = this.node



    /*  show slider
      console.log(tmpvar)
      if (this.gSel >= 0) {
        console.log('SELBSY')
        return
      }
      if (this.node.select) {
        this.node.select = false
        this.$emit('TreeNodeSel', this.nodeIdx ,false)
        //this.setSelect(node)
      } else {
        this.node.select = true
        this.$emit('TreeNodeSel', this.nodeIdx, true)
        //this.setSelect(node)
      }
      this.$forceUpdate()
      console.log("SEL",this.selected, this.node.select) */
    },
  },

  computed: {
    ...mapGetters([
      'rawData',
      'getNodeByIdx',
      'getNodes',
      'getNode',
      'setNode'
    ]),
    leafval () {
      return this.$root.accts[this.node.leaf]
    },
/*    nodeval () {
      //console.log(this.node)
      let node = this.node
      let sum = 0
      if (node.leaf === -1) {
        for (let c of node.chld) {
          let n = this.getNode(c)
          //console.log(c, n)
          sum += n.val
        }
      } else {
        sum = this.value
      }
      return sum
    }, */
    expanded: function () {
      return this.node.expand
    },
    selected () {
      return this.node.select
    },
    name: function () {
      return this.node.name
    },
    hovered: function () {
      return this.node.hover
    },
    //nodes: function () {
    //  return this.children
    //},
    indent: function () {
      let lvl = 'level0'
      switch (this.level) {
        case 2:
          lvl = 'level2'
          break
        case 1:
          lvl = 'level1'
          break
        default:
      }
      return lvl
    }
  }
}
</script>

<style scoped>
.tree-view-node {
  display: inline-block;
  float: left;
  width: 45em;
}
.tvn-expand {
  display: inline;
}
.noexpand {
  float: left;
  text-align: right;
  width: 3em;
  cursor: cell;
}
.tvn-expander {
  float: left;
  text-align: left;
  cursor: pointer;
}
.level0 {
  float: left;
  text-align: left;
  cursor: pointer;
  width: 3em;
}
.level1 {
  float: left;
  text-align: left;
  cursor: pointer;
  width: 2em;
  padding-left: 1em;
}
.level2 {
  float: left;
  text-align: left;
  cursor: pointer;
  width: 1em;
  padding-left: 2em;
}
.tnv-node {
  display: inline-block;
  float: left;
  width: 100%
}
.tnv-line {
  text-align: left;
  cursor: cell;
  white-space: nowrap;
  overflow: hidden;
}
.tnv-amount {
  display: inline-block;
  width: 5em;
  text-align: right;
  margin-right: 1em;
}
.tnv-name {
}
.tnv-idx {
  width: 2em;
  opacity: 60%;
  color: #444;
}

</style>

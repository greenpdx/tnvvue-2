<template>
  <div class="tree-view-node">
    <div class="tnv-node">
        <div class="tnv-line" @click="selClick">
          <span class="tnv-amount"> {{ nodeval }}</span>
          <span class="tnv-name"> {{ name }} </span>
          <span class="tnv-idx"> {{ node.idx }} </span>
        </div>
        <slider-node
          v-if="node.select"
          :node="node"></slider-node>
      </div>
  </div>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
import Node from '@/lib/Node'

import SliderNode from './SliderNode'

let tmpvar = {}

export default {
  name: 'TreeViewNode',

  components: {
    SliderNode
  },

  props: {
    nodeIdx: Number,
    level: {
      default: 0
    }
  },

  data () {
    return {
      node: null,
      total: 0,
      locked: false,
      change: false,
      value: 0,
      default: 0,
      children: []
    }
  },

  created () {
    //console.log(this.nodeIdx, this.$root.tree)
    let node = Object.assign({}, this.$root.tree[this.nodeIdx])
    node['select'] = false
    node['expand'] = true
    node['hover'] = false
    this.default = 0
    this.value = 0
    this.self = this
    if (node.leaf != -1) {
      let leaf = this.$root.accts[node.leaf]
      this.value = leaf.value['y2019']
      //console.log(this.value, leaf)
      this.default = this.value
    } else {
      this.value = 0
    }
    //node['nodeval'] = this.nodeval
    node['self'] = this
    this.node = node
    tmpvar = node
    //this.node['showVal'] = this.showVal

    //this.node['showVal'] = this.showVal
    //this.total = this.node.total
    //this.children = this.node.chld
    //console.log(this.node)
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
    showVal() {
      let sum = 0
      let chld =  this.node.chld
      if (chld.length > 0) {
        for (c in chld) {
          sum += this.showVal
        }
      } else 
        return this.value
      return sum
    },
    selClick () {
      console.log(tmpvar)
      if (this.node.select) {
        this.node.select = false
        //this.setSelect(node)
      } else {
        this.node.select = true
        //this.setSelect(node)
      }
      this.$forceUpdate()
      console.log("SEL",this.selected, this.node.select)
    },
  },

  computed: {
    ...mapGetters([
      'rawData',
      'getNodeByIdx'
    ]),
    leafval () {
      return this.$root.accts[this.node.leaf]
    },
    nodeval () {
      //console.log(this.node)
      let node = this.node
      let sum = 0
      if (node.leaf === -1) {
        for (let c of node.chld) {
          let n = this.$root.tree[c]
          console.log(c, n)
          //sum += n.nodeval()
        }
      } else {
        sum = this.value
      }
      return sum
    },
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
}
.tnv-amount {
  display: inline-block;
  width: 4em;
  text-align: right;
  margin-right: 1em;
}
.tnv-name {
}
.tnv-idx {
  width: 2em;
  opacity: 30%;
  color: #444;
}

</style>
